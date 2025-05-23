use crate::Status;
use log::{debug, error, info};
use serde_value::Value;

/// A trait that provides structured access to task-related metadata, parameters,
/// execution status, and result handling.
///
/// This trait is designed to encapsulate a common interface for managing and
/// tracking long-running or asynchronous tasks. It requires serialization and
/// deserialization capabilities for the parameter and metadata types, making it
/// suitable for persistence or communication over the network.
pub trait TaskInfo {
    /// The type used for task parameters.
    ///
    /// This type must implement `serde::Serialize` and `serde::DeserializeOwned`
    /// to support serialization and deserialization.
    type ParamType: serde::Serialize + serde::de::DeserializeOwned;

    /// The type used for storing task metadata.
    ///
    /// This type must also implement `serde::Serialize` and `serde::DeserializeOwned`.
    type MetadataType: serde::Serialize + serde::de::DeserializeOwned;

    /// Returns the static path associated with this task type.
    ///
    /// This path can represent a routing key, a resource location, or a task identifier
    /// used for classification or dispatching.
    fn path() -> String;

    /// Returns the current status of the task.
    ///
    /// The `Status` type typically includes state information such as
    /// `Pending`, `Running`, `Success`, or `Failed`.
    fn status(&self) -> Status;

    /// Returns a mutable reference to the task's status.
    ///
    /// This allows the task's status to be updated.
    fn status_mut(&mut self) -> &mut Status;

    /// Returns the UUID of the task instance.
    ///
    /// This identifier is used to uniquely distinguish a task.
    fn uuid(&self) -> uuid::Uuid;

    /// Returns the [`cdumay_job::Result`] of the task.
    ///
    /// The result includes output values, success/failure information, and
    /// optionally a return value map.
    fn result(&self) -> crate::Result;

    /// Returns a mutable reference to the task's result.
    ///
    /// This allows modification of the result, such as adding output values
    /// or marking the task as successful/failed.
    fn result_mut(&mut self) -> &mut crate::Result;

    /// Returns a reference to the task's metadata.
    ///
    /// Metadata may include non-functional or contextual information about the task.
    fn metadata(&self) -> &Self::MetadataType;

    /// Returns a mutable reference to the task's metadata.
    ///
    /// This enables updating the task's metadata during execution.
    fn metadata_mut(&mut self) -> &mut Self::MetadataType;

    /// Returns the parameters used to initialize or configure the task.
    ///
    /// This typically includes user-defined inputs or execution arguments.
    fn params(&self) -> Self::ParamType;

    /// Attempts to retrieve a value from the task's result by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look for in the result's return value map.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(Value))` if the key exists, `Ok(None)` if it does not,
    /// or an error if the operation fails.
    fn search_result(&self, key: &str) -> Option<Value> {
        self.result().retval.get(key).cloned()
    }

    /// Creates a new empty result object associated with this task.
    ///
    /// The result is initialized with the current task's UUID and default values.
    fn new_result(&self) -> crate::Result {
        crate::ResultBuilder::default().uuid(self.uuid()).build()
    }
}

pub trait TaskExec: TaskInfo {
    fn entrypoint() -> String {
        Self::path()
    }
    fn check_required_params(&mut self) -> Result<crate::Result, cdumay_core::Error> {
        Ok(self.result())
    }
    fn label_result(&self, action: &str, result: &Result<crate::Result, cdumay_core::Error>) -> String {
        format!(
            "{} => {}",
            self.label(Some(action.to_string())),
            match result {
                Ok(data) => format!("{data}"),
                Err(error) => format!("{error}"),
            }
        )
    }
    fn label(&self, action: Option<String>) -> String {
        format!(
            "{}[{}]{}",
            Self::entrypoint(),
            self.uuid(),
            match action {
                Some(data) => format!(" - {}", data),
                None => String::new(),
            }
        )
    }
    fn _post_init(&mut self) -> Result<crate::Result, cdumay_core::Error> {
        debug!("{}", self.label(Some("PostInit-Start".into())));
        let result = self.post_init(self.new_result());
        debug!("{}", self.label_result("PostInit-End", &result));
        Ok(result?)
    }
    fn post_init(&mut self, result: crate::Result) -> Result<crate::Result, cdumay_core::Error> {
        Ok(result)
    }
    fn _pre_run(&mut self) -> Result<crate::Result, cdumay_core::Error> {
        debug!("{}", self.label(Some("PreRun-Start".into())));
        let result = self.pre_run(self.new_result());
        debug!("{}", self.label_result("PreRun-End", &result));
        Ok(result?)
    }
    fn pre_run(&mut self, result: crate::Result) -> Result<crate::Result, cdumay_core::Error> {
        Ok(result)
    }
    fn _run(&mut self) -> Result<crate::Result, cdumay_core::Error> {
        info!("{}", self.label(Some("Run-Start".into())));
        self._set_status(Status::Running)?;
        let result = self.run(self.new_result());
        info!("{}", self.label_result("Run-End", &result));
        Ok(result?)
    }
    fn run(&mut self, result: crate::Result) -> Result<crate::Result, cdumay_core::Error> {
        Ok(result)
    }
    fn _post_run(&mut self) -> Result<crate::Result, cdumay_core::Error> {
        debug!("{}", self.label(Some("PostRun-Start".into())));
        let result = self.post_run(self.new_result());
        debug!("{}", self.label_result("PostRun-End", &result));
        Ok(result?)
    }
    fn post_run(&mut self, result: crate::Result) -> Result<crate::Result, cdumay_core::Error> {
        Ok(result)
    }
    fn _on_error(&mut self, error: &cdumay_core::Error) -> Result<crate::Result, cdumay_core::Error> {
        debug!("{}", self.label(Some("OnError-Start".into())));
        self._set_status(Status::Failed)?;
        *self.result_mut() = &self.result() + &crate::Result::from(error.clone());
        let result = self.on_error(error, self.new_result());
        debug!("{}", self.label_result("OnError-End", &result));
        Ok(self.result())
    }
    fn on_error(&mut self, _error: &cdumay_core::Error, result: crate::Result) -> Result<crate::Result, cdumay_core::Error> {
        Ok(result)
    }
    fn _on_success(&mut self) -> Result<crate::Result, cdumay_core::Error> {
        debug!("{}", self.label(Some("OnSuccess-Start".into())));
        self._set_status(Status::Success)?;
        let result = self.on_success(self.new_result());
        debug!("{}", self.label_result("OnSuccess-End", &result));
        Ok(result?)
    }
    fn on_success(&mut self, result: crate::Result) -> Result<crate::Result, cdumay_core::Error> {
        Ok(result)
    }
    fn unsafe_execute(&mut self, result: Option<crate::Result>) -> Result<crate::Result, cdumay_core::Error> {
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
                info!("{} => {}", self.label(Some("TaskExecution-End".to_string())), &result);
                result
            }
            Err(err) => {
                let result = self._on_error(&err).unwrap_or_else(|err| crate::Result::from(err));
                error!("{} => {}", self.label(Some("TaskExecution-End".to_string())), &result);
                result
            }
        }
    }
    fn _set_status(&mut self, status: Status) -> Result<crate::Result, cdumay_core::Error> {
        debug!(
            "{}: status updated '{}' -> '{}'",
            self.label(Some("SetStatus".to_string())),
            self.status(),
            &status
        );
        self.set_status(status)
    }
    fn set_status(&mut self, status: Status) -> Result<crate::Result, cdumay_core::Error> {
        *self.status_mut() = status;
        Ok(self.result())
    }
    fn send(&self, result: Option<crate::Result>) -> Result<crate::Result, cdumay_core::Error> {
        Ok(result.unwrap_or(self.result()))
    }
    fn finalize(&self) -> Result<crate::Result, cdumay_core::Error> {
        Ok(self.result())
    }
}
