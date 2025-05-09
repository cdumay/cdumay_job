use log::{debug, error, info};
use serde_value::Value;
use std::ops::Add;

use crate::{Message, Status};

pub trait TaskInfo {
    fn new(msg: &Message, result: Option<cdumay_result::Result>) -> Self;
    fn path() -> String;
    fn status(&self) -> Status;
    fn status_mut(&mut self) -> &mut Status;
    fn message(&self) -> Message;
    fn message_mut(&mut self) -> &mut Message;
    fn result(&self) -> cdumay_result::Result;
    fn result_mut(&mut self) -> &mut cdumay_result::Result;
    fn search_result(&self, key: &str) -> cdumay_error::Result<Option<Value>> {
        match self.message().result.retval.get(key) {
            Some(value) => Ok(Some(value.clone())),
            None => Ok(None),
        }
    }
    fn search_meta(&self, key: &str) -> cdumay_error::Result<Option<Value>> {
        match self.message().metadata.get(key) {
            Some(value) => Ok(Some(value.clone())),
            None => Ok(None),
        }
    }
    fn new_result(&self) -> cdumay_result::Result {
        cdumay_result::ResultBuilder::default().uuid(self.message().uuid).build()
    }
}

pub trait TaskExec: TaskInfo {
    fn entrypoint() -> String {
        Self::path()
    }
    /***********************************************************************************************
    // Method to check required parameters ( Message.params() <=> Task::required_params() )
     */
    fn check_required_params(&mut self) -> cdumay_error::Result<cdumay_result::Result> {
        Ok(self.result())
    }
    /***********************************************************************************************
    // Method to format log prefix (=label)
     */
    fn label(&self, action: Option<&str>) -> String {
        format!(
            "{}[{}]{}",
            Self::entrypoint(),
            self.message().uuid,
            match action {
                Some(data) => format!(" - {}", data),
                None => String::new(),
            }
        )
    }
    /***********************************************************************************************
    // Post Init - Trigger launched just after initialization, it perform checks
     */
    fn _post_init(&mut self) -> cdumay_error::Result<cdumay_result::Result> {
        *self.result_mut() = &self.result() + &self.check_required_params()?;
        self.post_init()
    }
    fn post_init(&mut self) -> cdumay_error::Result<cdumay_result::Result> {
        Ok(cdumay_result::ResultBuilder::from(&self.message()).build())
    }
    /***********************************************************************************************
    // Pre Run - Trigger launched just before running the task
     */
    fn _pre_run(&mut self) -> cdumay_error::Result<cdumay_result::Result> {
        debug!("{}", self.label(Some("PreRun")));
        self.pre_run()
    }
    fn pre_run(&mut self) -> cdumay_error::Result<cdumay_result::Result> {
        Ok(cdumay_result::ResultBuilder::from(&self.message()).build())
    }
    /***********************************************************************************************
    // Run - Trigger which represent the task body. It usually overwrites
     */
    fn _run(&mut self) -> cdumay_error::Result<cdumay_result::Result> {
        *self.result_mut() = &self.result() + &self._set_status(Status::Running)?;
        debug!("{}: {}", self.label(Some("Run")), self.result());
        self.run()
    }
    fn run(&mut self) -> cdumay_error::Result<cdumay_result::Result> {
        Ok(cdumay_result::ResultBuilder::from(&self.message()).build())
    }
    /***********************************************************************************************
    // Post Run - Trigger launched just after running the task
     */
    fn _post_run(&mut self) -> cdumay_error::Result<cdumay_result::Result> {
        debug!("{}: {}", self.label(Some("PostRun")), self.result());
        self.post_run()
    }
    fn post_run(&mut self) -> cdumay_error::Result<cdumay_result::Result> {
        Ok(cdumay_result::ResultBuilder::from(&self.message()).build())
    }
    /***********************************************************************************************
    // On Error - Trigger raised if any error is raised
     */
    fn _on_error(&mut self, error: &cdumay_error::Error) -> cdumay_error::Result<cdumay_result::Result> {
        *self.result_mut() = &self.result() + &self._set_status(Status::Failed)?;
        *self.result_mut() = &self.result() + &cdumay_result::Result::from(error.clone());
        error!("{}: {}", self.label(Some("Failed")), self.result());
        self.on_error(error)
    }
    fn on_error(&mut self, error: &cdumay_error::Error) -> cdumay_error::Result<cdumay_result::Result> {
        Ok(cdumay_result::ResultBuilder::from(&self.message())
            .build()
            .add(&cdumay_result::Result::from(error.clone())))
    }
    /***********************************************************************************************
    // On Success - Trigger launched if the task has succeeded
     */
    fn _on_success(&mut self) -> cdumay_error::Result<cdumay_result::Result> {
        *self.result_mut() = &self.result() + &self._set_status(Status::Success)?;
        info!("{}: {}", self.label(Some("Success")), self.result());
        self.on_success()
    }
    fn on_success(&mut self) -> cdumay_error::Result<cdumay_result::Result> {
        Ok(self.result())
    }
    /***********************************************************************************************
    // Unsafe Execute - Method to call to get a Result of the task execution.
    // NOTE: the trigger on_error is not called!
     */
    fn unsafe_execute(&mut self, result: Option<cdumay_result::Result>) -> cdumay_error::Result<cdumay_result::Result> {
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
    fn _set_status(&mut self, status: Status) -> cdumay_error::Result<cdumay_result::Result> {
        debug!("{}: status updated '{}' -> '{}'", self.label(Some("SetStatus")), self.status(), &status);
        self.set_status(status)
    }
    fn set_status(&mut self, status: Status) -> cdumay_error::Result<cdumay_result::Result> {
        *self.status_mut() = status;
        Ok(cdumay_result::ResultBuilder::from(&self.message()).build())
    }
    /***********************************************************************************************
    // Send: Send back to kafka (used by operations)
     */
    fn send(&self, result: Option<cdumay_result::Result>) -> cdumay_error::Result<cdumay_result::Result> {
        match result {
            Some(result) => Ok(cdumay_result::ResultBuilder::from(&self.message()).build().add(&result)),
            None => Ok(cdumay_result::ResultBuilder::from(&self.message()).build()),
        }
    }
    /***********************************************************************************************
    // Finalize: Finalize the task, use by operation to perform database save or so one.
     */
    fn finalize(&self) -> cdumay_error::Result<cdumay_result::Result> {
        Ok(cdumay_result::ResultBuilder::from(&self.message()).build())
    }
}
