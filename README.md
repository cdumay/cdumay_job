# cdumay_job

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A library to follow job execution using validation and checks steps.

## Quickstart

```toml
[dependencies]
cdumay_error = { git = "https://github.com/cdumay/cdumay-errors-rs"}
cdumay_result = { git = "https://github.com/cdumay/cdumay-result-rs", features = ["cdumay-error"]}
cdumay_job = { git = "https://github.com/cdumay/cdumay-job-rs" }
serde_json = "1.0"
serde-value = "0.5"
```

```rust
extern crate cdumay_error;
extern crate cdumay_result;
extern crate cdumay_job;
extern crate serde_json;
extern crate serde_value;
extern crate env_logger;
extern crate hostname;

use cdumay_error::ErrorRepr;
use cdumay_job::messages::MessageRepr;
use cdumay_job::status::Status;
use cdumay_job::task::Task;
use cdumay_result::{ResultRepr, ResultProps};
use serde_value::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Hello {
    message: MessageRepr,
    status: Status,
    result: ResultRepr,
}

impl Task for Hello {
    fn new(message: &MessageRepr, result: Option<ResultRepr>) -> Hello {
        Hello {
            message: message.clone(),
            result: result.unwrap_or(message.result().clone()),
            status: Status::Pending,
        }
    }
    fn status(&self) -> Status { self.status.clone() }
    fn status_mut(&mut self) -> &mut Status { &mut self.status }

    fn message(&self) -> MessageRepr { self.message.clone() }
    fn message_mut(&mut self) -> &mut MessageRepr { &mut self.message }
    fn result(&self) -> ResultRepr { self.result.clone() }
    fn result_mut(&mut self) -> &mut ResultRepr { &mut self.result }

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
        Ok({
            let mut res = ResultRepr::from(&self.message());
            *res.stdout_mut() = Some(format!("Hello {} from {}", user, host));
            res
        })
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
```
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

- **cdumay_result**: https://github.com/cdumay/cdumay-result-rs
- **cdumay_error**: https://github.com/cdumay/cdumay-errors-rs

## Project Links

- Issues: https://github.com/cdumay/cdumay-job-rs/issues
- Documentation: not available yet
