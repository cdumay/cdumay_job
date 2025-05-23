//! Builder pattern implementation for creating [`Result`] instances.

use std::collections::BTreeMap;
use serde_value::Value;
use crate::Result;
use uuid::Uuid;

/// A builder for creating [`Result`] instances with a fluent interface.
///
/// The builder provides methods to set all fields of a [`Result`] incrementally,
/// with sensible defaults when fields are not explicitly set.
///
/// # Examples
///
/// ```rust
/// use cdumay_job::ResultBuilder;
/// use std::collections::BTreeMap;
/// use serde_value::Value;
///
/// // Create a basic success result
/// let result = ResultBuilder::default()
///     .stdout("Operation successful".into())
///     .build();
///
/// // Create a result with custom return value
/// let result_with_data = ResultBuilder::default()
///     .stdout("Data processed".into())
///     .retval({
///         let mut data = BTreeMap::new();
///         data.insert("count".into(), Value::U8(42.into()));
///         data
///     })
///     .build();
/// ```
pub struct ResultBuilder {
    uuid: Uuid,
    retcode: u16,
    stdout: Option<String>,
    stderr: Option<String>,
    retval: BTreeMap<String, Value>,
}

impl Default for ResultBuilder {
    /// Creates a new [`ResultBuilder`] with default values:
    /// - A new random UUID v4
    /// - Return code 0 (success)
    /// - Empty stdout and stderr
    /// - Empty return value map
    fn default() -> ResultBuilder {
        ResultBuilder {
            uuid: Uuid::new_v4(),
            retcode: 0,
            stdout: Default::default(),
            stderr: Default::default(),
            retval: Default::default(),
        }
    }
}

impl ResultBuilder {
    /// Sets a custom UUID for the result.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The UUID to use for the result
    pub fn uuid(mut self, uuid: Uuid) -> Self {
        self.uuid = uuid;
        self
    }

    /// Sets the return code for the result.
    ///
    /// # Arguments
    ///
    /// * `retcode` - The return code to set (0 for success, other values for various error conditions)
    pub fn retcode(mut self, retcode: u16) -> Self {
        self.retcode = retcode;
        self
    }

    /// Sets the stdout content for the result.
    ///
    /// # Arguments
    ///
    /// * `stdout` - The stdout message to set
    pub fn stdout(mut self, stdout: String) -> Self {
        self.stdout = Some(stdout);
        self
    }

    /// Sets the stderr content for the result.
    ///
    /// # Arguments
    ///
    /// * `stderr` - The stderr message to set
    pub fn stderr(mut self, stderr: String) -> Self {
        self.stderr = Some(stderr);
        self
    }

    /// Sets the return value map for the result.
    ///
    /// # Arguments
    ///
    /// * `retval` - A map of string keys to serde values representing the operation's return data
    pub fn retval(mut self, retval: BTreeMap<String, Value>) -> Self {
        self.retval = retval;
        self
    }

    /// Builds and returns the final [`Result`] instance.
    pub fn build(self) -> Result {
        Result {
            uuid: self.uuid,
            retcode: self.retcode,
            stdout: self.stdout,
            stderr: self.stderr,
            retval: self.retval,
        }
    }
}
