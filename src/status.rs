use core::fmt;

use cdumay_core::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Pending,
    Running,
    Success,
    Failed,
}


impl From<&Value> for Status {
    fn from(value: &Value) -> Status {
        match value {
            Value::String(data) => match data.as_str() {
                "RUNNING" => Status::Running,
                "SUCCESS" => Status::Success,
                "FAILED" => Status::Failed,
                _ => Status::Pending
            },
            _ => Status::Pending
        }
    }
}

impl From<Status> for String {
    fn from(status: Status) -> String {
        match status {
            Status::Pending => "PENDING".to_string(),
            Status::Running => "RUNNING".to_string(),
            Status::Success => "SUCCESS".to_string(),
            Status::Failed => "FAILED".to_string(),
        }
    }
}

impl From<Status> for Value {
    fn from(status: Status) -> Value {
        Value::String(String::from(status))
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", String::from(self.clone()))
    }
}