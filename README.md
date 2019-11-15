# cdumay_job

[![Build Status](https://travis-ci.org/cdumay/rust-cdumay_job.svg?branch=master)](https://travis-ci.org/cdumay/rust-cdumay_job)
[![Latest version](https://img.shields.io/crates/v/cdumay_job.svg)](https://crates.io/crates/cdumay_job)
[![Documentation](https://docs.rs/cdumay_job/badge.svg)](https://docs.rs/cdumay_job)
![License](https://img.shields.io/crates/l/cdumay_job.svg)

A library to follow job execution using validation and checks steps.

## Quickstart

_Cargo.toml_:
```toml
[dependencies]
cdumay_error = { git = "https://github.com/cdumay/rust-cdumay_errors" }
cdumay_result = { git = "https://github.com/cdumay/rust-cdumay_result" }
cdumay_job = { git = "https://github.com/cdumay/rust-cdumay_job" }
serde = "1.0"
serde_derive = "1.0"
serde_json = "1.0"
serde-value = "0.6"
env_logger = "0.7"
hostname = "0.2"
```

_main.rs_:
```rust
extern crate cdumay_error;
extern crate cdumay_job;
extern crate cdumay_result;
extern crate env_logger;
extern crate hostname;
extern crate serde_json;
extern crate serde_value;

use std::collections::HashMap;

use cdumay_error::ErrorRepr;
use cdumay_job::{KserMessage, MessageRepr, Status, TaskExec, TaskInfo};
use cdumay_result::{ResultBuilder, ResultRepr};
use serde_value::Value;

#[derive(Clone)]
pub struct Hello {
    message: MessageRepr,
    status: Status,
    result: ResultRepr,
}

impl TaskInfo for Hello {
    fn new(message: &MessageRepr, result: Option<ResultRepr>) -> Hello {
        Hello {
            message: message.clone(),
            result: result.unwrap_or(message.result().clone()),
            status: Status::Pending,
        }
    }

    fn path() -> String { module_path!().to_string() }
    fn status(&self) -> Status { self.status.clone() }
    fn status_mut(&mut self) -> &mut Status { &mut self.status }

    fn message(&self) -> MessageRepr { self.message.clone() }
    fn message_mut(&mut self) -> &mut MessageRepr { &mut self.message }
    fn result(&self) -> ResultRepr { self.result.clone() }
    fn result_mut(&mut self) -> &mut ResultRepr { &mut self.result }
}


impl TaskExec for Hello {
    fn required_params<'a>() -> Option<Vec<&'a str>> {
        Some(vec!["user"])
    }
    fn run(&mut self) -> Result<ResultRepr, ErrorRepr> {
        let default = "John Smith".to_string();
        let host = hostname::get_hostname().unwrap_or("localhost".to_string());

        let user = match self.search_data("user") {
            Some(Value::String(data)) => data.clone(),
            _ => default,
        };
        Ok(ResultBuilder::from(&self.message())
            .stdout(format!("Hello {} from {}", user, host))
            .build()
        )
    }
}

fn main() {
    env_logger::init();

    let mut task = Hello::new(
        &MessageRepr::new(
            None,
            "hello",
            Some({
                let mut params = HashMap::new();
                params.insert("user".to_string(), Value::String("Cedric".to_string()));
                params
            }),
            None,
            None,
        ),
        None,
    );
    println!("{}", serde_json::to_string_pretty(&task.execute(None)).unwrap());
}
```
**Log Output (using RUST_LOG=debug)**
```rust
[2019-02-01T16:02:04Z DEBUG cdumay_job::task] hello[39131d5b-a149-4a84-b183-c5eed1ef1ed1] - PreRun
[2019-02-01T16:02:04Z DEBUG cdumay_job::task] hello[39131d5b-a149-4a84-b183-c5eed1ef1ed1] - SetStatus: status updated 'PENDING' -> 'RUNNING'
[2019-02-01T16:02:04Z DEBUG cdumay_job::task] hello[39131d5b-a149-4a84-b183-c5eed1ef1ed1] - Run: Result: Ok(0, stdout: None)
[2019-02-01T16:02:04Z DEBUG cdumay_job::task] hello[39131d5b-a149-4a84-b183-c5eed1ef1ed1] - PostRun: Result: Ok(0, stdout: Some("Hello Cedric from cdumay-desk"))
[2019-02-01T16:02:04Z DEBUG cdumay_job::task] hello[39131d5b-a149-4a84-b183-c5eed1ef1ed1] - SetStatus: status updated 'RUNNING' -> 'SUCCESS'
[2019-02-01T16:02:04Z INFO  cdumay_job::task] hello[39131d5b-a149-4a84-b183-c5eed1ef1ed1] - Success: Result: Ok(0, stdout: Some("Hello Cedric from cdumay-desk"))
```
**Result**
```json
{
  "uuid": "1a0c9711-2bff-48f1-9d7e-cdcbe498e9e8",
  "retcode": 0,
  "stdout": "Hello Cedric from cdumay-desk",
  "retval": {}
}
```

## Dependencies Links

- **cdumay_result**: https://github.com/cdumay/rust-cdumay_result
- **cdumay_error**: https://github.com/cdumay/rust-cdumay_error

## Project Links

- Issues: https://github.com/cdumay/rust-cdumay_job/issues
- Documentation: https://docs.rs/cdumay_job

License: MIT
