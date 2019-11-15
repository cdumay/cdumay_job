use std::ops::Add;

use cdumay_error::{ErrorBuilder, ErrorRepr, GenericErrors};
use cdumay_result::ResultRepr;
use serde_value::Value;

use crate::{Message, MessageRepr, Status};

pub trait TaskInfo {
    fn new(message: &MessageRepr, result: Option<ResultRepr>) -> Self;
    fn path() -> String;
    fn status(&self) -> Status;
    fn status_mut(&mut self) -> &mut Status;
    fn message(&self) -> MessageRepr;
    fn message_mut(&mut self) -> &mut MessageRepr;
    fn result(&self) -> ResultRepr;
    fn result_mut(&mut self) -> &mut ResultRepr;
}

pub trait TaskExec: TaskInfo {
    /***********************************************************************************************
    // Search a value using message().params(), message().result() & self.result()
    */
    fn search_data(&self, key: &str) -> Option<Value> {
        match self.message().params().get(key) {
            Some(value) => Some(value.clone()),
            _ => match self.message().result().retval.get(key) {
                Some(value) => Some(value.clone()),
                _ => match self.result().retval.get(key) {
                    Some(value) => Some(value.clone()),
                    _ => None,
                }
            }
        }
    }
    /***********************************************************************************************
    // Required parameters
    */
    fn required_params<'a>() -> Option<Vec<&'a str>> { None }
    /***********************************************************************************************
    // Method to check required parameters ( Message.params() <=> Task::required_params() )
    */
    fn check_required_params(&mut self) -> Result<ResultRepr, ErrorRepr> {
        match Self::required_params() {
            None => Ok(self.result()),
            Some(required_fields) => {
                for attr in required_fields {
                    if self.message().params().get(attr) == None {
                        return Err(
                            ErrorBuilder::from(GenericErrors::VALIDATION_ERROR)
                                .message(format!("required field '{}' not set!", attr))
                                .build()
                        );
                    }
                }
                Ok(self.result())
            }
        }
    }
    /***********************************************************************************************
    // Method to format log prefix (=label)
    */
    fn label(&self, action: Option<&str>) -> String {
        format!(
            "{}[{}]{}", self.message().entrypoint(), self.message().uuid(), match action {
                Some(data) => format!(" - {}", data),
                None => String::new()
            }
        )
    }
    /***********************************************************************************************
    // Post Init - Trigger launched just after initialization, it perform checks
    */
    fn _post_init(&mut self) -> Result<ResultRepr, ErrorRepr> {
        *self.result_mut() = &self.result() + &self.check_required_params()?;
        self.post_init()
    }
    fn post_init(&mut self) -> Result<ResultRepr, ErrorRepr> {
        Ok(ResultRepr::from(&self.message()))
    }
    /***********************************************************************************************
    // Pre Run - Trigger launched just before running the task
    */
    fn _pre_run(&mut self) -> Result<ResultRepr, ErrorRepr> {
        debug!("{}", self.label(Some("PreRun")));
        self.pre_run()
    }
    fn pre_run(&mut self) -> Result<ResultRepr, ErrorRepr> {
        Ok(ResultRepr::from(&self.message()))
    }
    /***********************************************************************************************
    // Run - Trigger which represent the task body. It usually overwrite
    */
    fn _run(&mut self) -> Result<ResultRepr, ErrorRepr> {
        *self.result_mut() = &self.result() + &self._set_status(Status::Running)?;
        debug!("{}: {}", self.label(Some("Run")), self.result());
        self.run()
    }
    fn run(&mut self) -> Result<ResultRepr, ErrorRepr> {
        Ok(ResultRepr::from(&self.message()))
    }
    /***********************************************************************************************
    // Post Run - Trigger launched just after running the task
    */
    fn _post_run(&mut self) -> Result<ResultRepr, ErrorRepr> {
        debug!("{}: {}", self.label(Some("PostRun")), self.result());
        self.post_run()
    }
    fn post_run(&mut self) -> Result<ResultRepr, ErrorRepr> {
        Ok(ResultRepr::from(&self.message()))
    }
    /***********************************************************************************************
    // On Error - Trigger raised if any error is raised
    */
    fn _on_error(&mut self, error: &ErrorRepr) -> Result<ResultRepr, ErrorRepr> {
        *self.result_mut() = &self.result() + &self._set_status(Status::Failed)?;
        *self.result_mut() = &self.result() + &ResultRepr::from(error.clone());
        error!("{}: {}", self.label(Some("Failed")), self.result());
        self.on_error(error)
    }
    fn on_error(&mut self, error: &ErrorRepr) -> Result<ResultRepr, ErrorRepr> {
        Ok(ResultRepr::from(&self.message()).add(&ResultRepr::from(error.clone())))
    }
    /***********************************************************************************************
    // On Success - Trigger launched if the task has succeeded
    */
    fn _on_success(&mut self) -> Result<ResultRepr, ErrorRepr> {
        *self.result_mut() = &self.result() + &self._set_status(Status::Success)?;
        info!("{}: {}", self.label(Some("Success")), self.result());
        self.on_success()
    }
    fn on_success(&mut self) -> Result<ResultRepr, ErrorRepr> {
        Ok(self.result())
    }
    /***********************************************************************************************
    // Unsafe Execute - Method to call to get a Result of the task execution.
    // NOTE: the trigger on_error is not called!
    */
    fn unsafe_execute(&mut self, result: Option<ResultRepr>) -> Result<ResultRepr, ErrorRepr> {
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
    fn execute(&mut self, result: Option<ResultRepr>) -> ResultRepr {
        match self.unsafe_execute(result) {
            Ok(result) => result,
            Err(err) => match self._on_error(&err) {
                Ok(result) => result,
                Err(err) => ResultRepr::from(err)
            }
        }
    }
    /***********************************************************************************************
    // Status - Methods to update the status of the task. it can be overwrite to perform action such
    // as database save ...
    */
    fn _set_status(&mut self, status: Status) -> Result<ResultRepr, ErrorRepr> {
        debug!("{}: status updated '{}' -> '{}'", self.label(Some("SetStatus")), self.status(), &status);
        self.set_status(status)
    }
    fn set_status(&mut self, status: Status) -> Result<ResultRepr, ErrorRepr> {
        *self.status_mut() = status;
        Ok(ResultRepr::from(&self.message()))
    }
    /***********************************************************************************************
    // Send: Send back to kafka (used by operations)
    */
    fn send(&self, result: Option<ResultRepr>) -> Result<ResultRepr, ErrorRepr> {
        match result {
            Some(result) => Ok(ResultRepr::from(&self.message()).add(&result)),
            None => Ok(ResultRepr::from(&self.message()))
        }
    }
    /***********************************************************************************************
    // Finalize: Finalize the task, use by operation to perform database save or so one.
    */
    fn finalize(&self) -> Result<ResultRepr, ErrorRepr> {
        Ok(ResultRepr::from(&self.message()))
    }
}
