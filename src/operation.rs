use std::ops::Add;

use crate::{Message, Status, TaskExec, TaskInfo};
use cdumay_error::{Error, Result};
use log::{debug, error, info};

pub trait Operation {
    type TasksItems: TaskExec;

    /***********************************************************************************************
    // Method to check required parameters ( Message.params() <=> Task::required_params() )
     */
    fn check_required_params(&mut self) -> Result<cdumay_result::Result> {
        Ok(self.result())
    }
    /***********************************************************************************************
    // Method to format log prefix (=label)
     */
    fn label(&self, action: Option<&str>) -> String {
        format!(
            "{}[{}]{}",
            self.message().entrypoint,
            self.message().uuid,
            match action {
                Some(data) => format!(" - {}", data),
                None => String::new(),
            }
        )
    }
    /***********************************************************************************************
    // Post Init - Trigger launched just after initialization, it performs checks
     */
    fn _post_init(&mut self) -> Result<cdumay_result::Result> {
        *self.result_mut() = &self.result() + &self.check_required_params()?;
        self.post_init()
    }
    fn post_init(&mut self) -> Result<cdumay_result::Result> {
        Ok(cdumay_result::ResultBuilder::from(&self.message()).build())
    }
    /***********************************************************************************************
    // Pre Run - Trigger launched just before running the task
     */
    fn _pre_run(&mut self) -> Result<cdumay_result::Result> {
        debug!("{}", self.label(Some("PreRun")));
        self.pre_run()
    }
    fn pre_run(&mut self) -> Result<cdumay_result::Result> {
        Ok(cdumay_result::ResultBuilder::from(&self.message()).build())
    }
    /***********************************************************************************************
    // Run - Trigger which represent the task body. It usually overwrites
     */
    fn _run(&mut self) -> Result<cdumay_result::Result> {
        *self.result_mut() = &self.result() + &self._set_status(Status::Running)?;
        debug!("{}: {}", self.label(Some("Run")), self.result());
        self.run()
    }
    fn run(&mut self) -> Result<cdumay_result::Result> {
        let mut result = self.result();
        for task in self.tasks_mut() {
            if task.status() != Status::Success {
                result = task.unsafe_execute(Some(result))?;
            }
        }
        Ok(result)
    }
    /***********************************************************************************************
    // Post Run - Trigger launched just after running the task
     */
    fn _post_run(&mut self) -> Result<cdumay_result::Result> {
        debug!("{}: {}", self.label(Some("PostRun")), self.result());
        self.post_run()
    }
    fn post_run(&mut self) -> Result<cdumay_result::Result> {
        Ok(cdumay_result::ResultBuilder::from(&self.message()).build())
    }
    /***********************************************************************************************
    // On Error - Trigger raised if any error is raised
     */
    fn _on_error(&mut self, error: &Error) -> Result<cdumay_result::Result> {
        *self.result_mut() = &self.result() + &self._set_status(Status::Failed)?;
        *self.result_mut() = &self.result() + &cdumay_result::Result::from(error.clone());
        error!("{}: {}", self.label(Some("Failed")), self.result());
        self.on_error(error)
    }
    fn on_error(&mut self, error: &Error) -> Result<cdumay_result::Result> {
        Ok(cdumay_result::ResultBuilder::from(&self.message())
            .build()
            .add(&cdumay_result::Result::from(error.clone())))
    }
    /***********************************************************************************************
    // On Success - Trigger launched if the task has succeeded
     */
    fn _on_success(&mut self) -> Result<cdumay_result::Result> {
        *self.result_mut() = &self.result() + &self._set_status(Status::Success)?;
        info!("{}: {}", self.label(Some("Success")), self.result());
        self.on_success()
    }
    fn on_success(&mut self) -> Result<cdumay_result::Result> {
        Ok(cdumay_result::ResultBuilder::from(&self.message()).build())
    }
    /***********************************************************************************************
    // Unsafe Execute - Method to call to get a Result of the task execution.
    // NOTE: the trigger on_error is not called!
     */
    fn unsafe_execute(&mut self, result: Option<cdumay_result::Result>) -> Result<cdumay_result::Result> {
        if let Some(data) = result {
            *self.result_mut() = &self.result() + &data;
        }
        *self.result_mut() = &self.result() + &self._post_init()?;
        *self.result_mut() = &self.result() + &self._pre_run()?;
        *self.result_mut() = &self.result() + &self._run()?;
        *self.result_mut() = &self.result() + &self._post_run()?;
        self._on_success()
    }
    /***********************************************************************************************
    // Execute - The method used by the registry
     */
    fn execute(&mut self, result: Option<cdumay_result::Result>) -> cdumay_result::Result {
        match self.unsafe_execute(result) {
            Ok(result) => result,
            Err(err) => match self._on_error(&err) {
                Ok(result) => result,
                Err(err) => cdumay_result::Result::from(err),
            },
        }
    }
    /***********************************************************************************************
    // Status - Methods to update the status of the task. it can be overwrite to perform action such
    // as database save ...
     */
    fn _set_status(&mut self, status: Status) -> Result<cdumay_result::Result> {
        debug!("{}: status updated '{}' -> '{}'", self.label(Some("SetStatus")), self.status(), &status);
        self.set_status(status)
    }
    fn set_status(&mut self, status: Status) -> Result<cdumay_result::Result> {
        *self.status_mut() = status;
        Ok(cdumay_result::ResultBuilder::from(&self.message()).build())
    }

    /***********************************************************************************************
    // On Pre Build - Trigger launched on operation building
     */
    fn _pre_build(&mut self) -> Result<cdumay_result::Result> {
        debug!("{}: {}", self.label(Some("PreBuild")), self.result());
        self.pre_build()
    }
    fn pre_build(&mut self) -> Result<cdumay_result::Result> {
        Ok(cdumay_result::ResultBuilder::from(&self.message()).build())
    }
    /***********************************************************************************************
    // Operation build
     */
    fn _build_tasks(&self) -> Vec<Self::TasksItems> {
        self.build_tasks()
    }
    fn build_tasks(&self) -> Vec<Self::TasksItems> {
        vec![]
    }
    fn build(&mut self) -> Result<cdumay_result::Result> {
        *self.result_mut() = &self.result() + &self._pre_build()?;
        *self.tasks_mut() = self._build_tasks();
        debug!("{}: {} task(s) found", self.label(Some("Build")), self.tasks().len());
        self.finalize()
    }
    /***********************************************************************************************
    // Finalize: Finalize the task, use by operation to perform database save or so one.
     */
    fn finalize(&self) -> Result<cdumay_result::Result> {
        let mut result = self.result();
        for task in self.tasks() {
            result = &result + &task.finalize()?;
        }
        Ok(result)
    }

    /***********************************************************************************************
    // Operation over kafka
     */
    fn launch(&mut self, result: Option<cdumay_result::Result>) -> Result<cdumay_result::Result> {
        self.launch_next(None, result)
    }
    fn launch_next(&mut self, task: Option<Self::TasksItems>, result: Option<cdumay_result::Result>) -> Result<cdumay_result::Result> {
        match task {
            Some(task) => match self.next(&task) {
                Some(next) => next.send(result),
                None => {
                    if let Some(result) = result {
                        *self.result_mut() = &self.result() + &result;
                    }
                    self._set_status(task.status())
                }
            },
            None => match self.tasks().len() > 0 {
                true => self.tasks()[0].send(result),
                false => Ok(cdumay_result::ResultBuilder::from(&self.message())
                    .stderr("Nothing to do, empty operation !".to_string())
                    .build()),
            },
        }
    }

    // to implement: constructor & property getters / setters
    fn new(message: &Message, result: Option<cdumay_result::Result>) -> Self;
    fn status(&self) -> Status;
    fn status_mut(&mut self) -> &mut Status;

    fn message(&self) -> Message;
    fn message_mut(&mut self) -> &mut Message;
    fn result(&self) -> cdumay_result::Result;
    fn result_mut(&mut self) -> &mut cdumay_result::Result;
    fn tasks(&self) -> &Vec<Self::TasksItems>;
    fn tasks_mut(&mut self) -> &mut Vec<Self::TasksItems>;

    // to implement
    fn next(&mut self, task: &Self::TasksItems) -> Option<Self::TasksItems>;
}
