use crate::{Status, TaskExec, TaskInfo};
use cdumay_core::Error;
use log::{debug, error, info};

pub trait OperationInfo: TaskInfo {
    fn tasks(&self) -> &Vec<Box<dyn TaskExec>>;
    fn tasks_mut(&mut self) -> &mut Vec<Box<dyn TaskExec>>;
}

pub trait OperationExec: OperationInfo {
    fn check_required_params(&mut self) -> Result<crate::Result, Error> {
        for task in self.tasks_mut() {
            task.check_required_params()?;
        }
        Ok(self.result())
    }

    fn label_result(&self, action: &str, result: &Result<crate::Result, Error>) -> String {
        format!(
            "{} => {}",
            self.label(Some(action)),
            match result {
                Ok(data) => format!("{data}"),
                Err(error) => format!("{error}"),
            }
        )
    }

    fn label(&self, action: Option<&str>) -> String {
        format!(
            "{}[{}]{}",
            self.path(),
            self.uuid(),
            match action {
                Some(data) => format!(" - {}", data),
                None => String::new(),
            }
        )
    }
    fn _post_init(&mut self) -> Result<crate::Result, Error> {
        debug!("{}", self.label(Some("PostInit-Start")));
        let result = self.post_init(self.new_result());
        debug!("{}", self.label_result("PostInit-End", &result));
        Ok(result?)
    }

    fn post_init(&mut self, result: crate::Result) -> Result<crate::Result, Error> {
        Ok(result)
    }

    fn _pre_run(&mut self) -> Result<crate::Result, Error> {
        debug!("{}", self.label(Some("PreRun-Start")));
        let result = self.pre_run(self.new_result());
        debug!("{}", self.label_result("PreRun-End", &result));
        Ok(result?)
    }

    fn pre_run(&mut self, result: crate::Result) -> Result<crate::Result, Error> {
        Ok(result)
    }

    fn _run(&mut self) -> Result<crate::Result, Error> {
        info!("{}", self.label(Some("Run-Start")));
        self._set_status(Status::Running)?;
        let result = self.run(self.new_result());
        info!("{}", self.label_result("Run-End", &result));
        Ok(result?)
    }

    fn run(&mut self, mut result: crate::Result) -> Result<crate::Result, Error> {
        for task in self.tasks_mut() {
            if task.status() != Status::Success {
                result = task.unsafe_execute(Some(result))?;
            }
        }
        Ok(result)
    }

    fn _post_run(&mut self) -> Result<crate::Result, Error> {
        debug!("{}", self.label(Some("PostRun-Start")));
        let result = self.post_run(self.new_result());
        debug!("{}", self.label_result("PostRun-End", &result));
        Ok(result?)
    }
    fn post_run(&mut self, result: crate::Result) -> Result<crate::Result, Error> {
        Ok(result)
    }

    fn _on_error(&mut self, error: &Error) -> Result<crate::Result, Error> {
        debug!("{}", self.label(Some("OnError-Start")));
        self._set_status(Status::Failed)?;
        *self.result_mut() = &self.result() + &crate::Result::from(error.clone());
        let result = self.on_error(error, self.new_result());
        debug!("{}", self.label_result("OnError-End", &result));
        Ok(self.result())
    }
    fn on_error(&mut self, _error: &Error, result: crate::Result) -> Result<crate::Result, Error> {
        Ok(result)
    }
    fn _on_success(&mut self) -> Result<crate::Result, Error> {
        debug!("{}", self.label(Some("OnSuccess-Start")));
        self._set_status(Status::Success)?;
        let result = self.on_success(self.new_result());
        debug!("{}", self.label_result("OnSuccess-End", &result));
        Ok(result?)
    }
    fn on_success(&mut self, result: crate::Result) -> Result<crate::Result, Error> {
        Ok(result)
    }

    fn unsafe_execute(&mut self, result: Option<crate::Result>) -> Result<crate::Result, Error> {
        if let Some(data) = result {
            *self.result_mut() = &self.result() + &data;
        }
        *self.result_mut() = &self.result() + &self.check_required_params()?;
        *self.result_mut() = &self.result() + &self._post_init()?;
        *self.result_mut() = &self.result() + &self._pre_run()?;
        *self.result_mut() = &self.result() + &self._run()?;
        *self.result_mut() = &self.result() + &self._post_run()?;
        *self.result_mut() = &self.result() + &self._on_success()?;
        Ok(self.result())
    }
    fn execute(&mut self, result: Option<crate::Result>) -> crate::Result {
        info!("{}", self.label(Some("TaskExecution-Start".into())));
        match self.unsafe_execute(result) {
            Ok(result) => {
                info!("{} => {}", self.label(Some("TaskExecution-End")), &result);
                result
            }
            Err(err) => {
                let result = self._on_error(&err).unwrap_or_else(|err| crate::Result::from(err));
                error!("{} => {}", self.label(Some("TaskExecution-End")), &result);
                result
            }
        }
    }
    fn _set_status(&mut self, status: Status) -> Result<crate::Result, Error> {
        debug!("{}: status updated '{}' -> '{}'", self.label(Some("SetStatus")), self.status(), &status);
        self.set_status(status)
    }
    fn set_status(&mut self, status: Status) -> Result<crate::Result, Error> {
        *self.status_mut() = status;
        Ok(self.result())
    }

    fn _pre_build(&mut self) -> Result<crate::Result, Error> {
        debug!("{}", self.label(Some("PreBuild-Start")));
        let result = self.post_run(self.new_result());
        debug!("{}", self.label_result("PreBuild-End", &result));
        Ok(result?)
    }
    fn pre_build(&mut self, result: crate::Result) -> Result<crate::Result, Error> {
        Ok(result)
    }
    fn build_tasks(&self) -> Vec<Box<dyn TaskExec>> {
        vec![]
    }
    fn build(&mut self) -> Result<crate::Result, Error> {
        *self.result_mut() = &self.result() + &self._pre_build()?;
        *self.tasks_mut() = self.build_tasks();
        debug!("{}: {} task(s) found", self.label(Some("Build")), self.tasks().len());
        self.finalize()
    }
    fn finalize(&self) -> Result<crate::Result, Error> {
        let mut result = self.result();
        for task in self.tasks() {
            result = &result + &task.finalize()?;
        }
        Ok(result)
    }

    fn launch(&mut self, result: Option<crate::Result>) -> Result<crate::Result, Error> {
        self.launch_next(None, result)
    }
    fn launch_next(&mut self, task: Option<Box<dyn TaskExec>>, result: Option<crate::Result>) -> Result<crate::Result, Error> {
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
                false => Ok({
                    self.result_mut().stderr = Some("Nothing to do, empty operation !".to_string());
                    self.result()
                }),
            },
        }
    }
    fn next(&mut self, _task: &Box<dyn TaskExec>) -> Option<Box<dyn TaskExec>> {
        unimplemented!("To implement for remote execution")
    }
}
