/// Represents the execution status of a task or operation.
///
/// This enum is commonly used to track and serialize the state of processes in a pipeline,
/// job scheduler, or any system involving transitions between execution states.
///
/// ## Conversions
///
/// - From [`serde_value::Value`] → [`Status`] (only string values are supported).
/// - From [`Status`] → [`String`] or [`serde_value::Value`].
///
/// ## Display
///
/// Implements [`Display`] to show a human-readable version of the status.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Status {
    /// The task has not started yet (default).
    Pending,
    /// The task is currently executing.
    Running,
    /// The task completed successfully.
    Success,
    /// The task encountered an error and did not complete successfully.
    Failed,
}

impl Default for Status {
    fn default() -> Self {
        Status::Pending
    }
}

impl From<&serde_value::Value> for Status {
    /// Converts a [`serde_value::Value`] (expected to be a string) into a [`Status`].
    ///
    /// Unrecognized or non-string values default to [`Status::Pending`].
    ///
    /// # Example
    ///
    /// ```
    /// use serde_value::Value;
    /// use cdumay_job::Status;
    ///
    /// let status = Status::from(&Value::String("SUCCESS".into()));
    /// assert_eq!(status, Status::Success);
    ///
    /// let unknown = Status::from(&Value::String("UNKNOWN".into()));
    /// assert_eq!(unknown, Status::Pending);
    /// ```
    fn from(value: &serde_value::Value) -> Status {
        match value {
            serde_value::Value::String(data) => match data.as_str() {
                "RUNNING" => Status::Running,
                "SUCCESS" => Status::Success,
                "FAILED" => Status::Failed,
                _ => Status::Pending,
            },
            _ => Status::Pending,
        }
    }
}

impl From<Status> for String {
    /// Converts a [`Status`] into its uppercase string representation.
    ///
    /// # Example
    ///
    /// ```
    /// use cdumay_job::Status;
    ///
    /// let s: String = Status::Failed.into();
    /// assert_eq!(s, "FAILED");
    /// ```
    fn from(status: Status) -> String {
        match status {
            Status::Pending => "PENDING".to_string(),
            Status::Running => "RUNNING".to_string(),
            Status::Success => "SUCCESS".to_string(),
            Status::Failed => "FAILED".to_string(),
        }
    }
}

impl From<Status> for serde_value::Value {
    /// Converts a [`Status`] into a [`serde_value::Value`] string.
    ///
    /// # Example
    ///
    /// ```
    /// use serde_value::Value;
    /// use cdumay_job::Status;
    ///
    /// let val: Value = Status::Running.into();
    /// assert_eq!(val, Value::String("RUNNING".into()));
    /// ```
    fn from(status: Status) -> serde_value::Value {
        serde_value::Value::String(String::from(status))
    }
}

impl core::fmt::Display for Status {
    /// Displays the [`Status`] as its uppercase string form.
    ///
    /// # Example
    ///
    /// ```
    /// use cdumay_job::Status;
    ///
    /// let status = Status::Success;
    /// println!("{}", status); // Outputs: SUCCESS
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(f, "{}", String::from(self.clone()))
    }
}
