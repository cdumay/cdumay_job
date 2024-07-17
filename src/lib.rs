//! A library to follow job execution using validation and checks steps.
//!
//! ## Quickstart
//!
//! _Cargo.toml_:
//! ```toml
//! [dependencies]
//! cdumay_error = "0.3"
//! cdumay_result = "0.3"
//! cdumay_job = "0.3"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! hostname = "0.4"
//! ```
//!
//! _main.rs_:
//!
//! ```rust
//! extern crate cdumay_error;
//! extern crate cdumay_job;
//! extern crate cdumay_result;
//! extern crate hostname;
//! extern crate serde_json;
//!
//! use cdumay_error::{Error, JsonError};
//! use cdumay_result::{JsonResult, ResultBuilder};
//! use cdumay_result::Result;
//! use serde_json::Value;
//!
//! use cdumay_job::{Message, MessageInfo, Status, TaskExec, TaskInfo};
//!
//! //! //! //!
//! #[derive(Clone)]
//! pub struct Hello {
//!     message: Message,
//!     status: Status,
//!     result: Result,
//! }
//!
//! impl TaskInfo for Hello {
//!
//!     fn path() -> String { module_path!().to_string() }
//!     fn status(&self) -> Status { self.status.clone() }
//!     fn status_mut(&mut self) -> &mut Status { &mut self.status }
//!
//!     fn message(&self) -> Message { self.message.clone() }
//!     fn message_mut(&mut self) -> &mut Message { &mut self.message }
//!     fn result(&self) -> Result { self.result.clone() }
//!     fn result_mut(&mut self) -> &mut Result { &mut self.result }
//! }
//!
//!
//! impl TaskExec for Hello {
//!     fn required_params<'a>() -> Option<Vec<&'a str>> {
//!         Some(vec!["user"])
//!     }
//!     fn run(&mut self) -> std::result::Result<Result, Error> {
//!         let default = "John Smith".to_string();
//!         let host = hostname::get().unwrap_or("localhost".into());
//!
//!         let user = match self.search_data("user") {
//!             Some(Value::String(data)) => data.clone(),
//!             _ => default,
//!         };
//!         Ok(ResultBuilder::from(&self.message())
//!             .stdout(format!("Hello {} from {}", user, host))
//!             .build()
//!         )
//!     }
//! }
//!
//! impl Hello {
//!     fn new(message: &Message, result: Option<Result>) -> Hello {
//!         Hello {
//!             message: message.clone(),
//!             result: result.unwrap_or(message.result().clone()),
//!             status: Status::Pending,
//!         }
//!     }
//! }
//!
//! fn main() {
//!     use std::collections::BTreeMap;
//!     let mut task = Hello::new(
//!         &Message::new(
//!             None,
//!             "hello",
//!             Some({
//!                 let mut params = BTreeMap::new();
//!                 params.insert("user".to_string(), Value::String("Cedric".to_string()));
//!                 params
//!             }),
//!             None,
//!             None,
//!         ),
//!         None,
//!     );
//!     println!("{}", serde_json::to_string_pretty(&JsonResult::from(task.execute(None))).unwrap());
//! }
//! ```
//! **Log Output (using RUST_LOG=debug)**
//! ```text
//! [2019-02-01T16:02:04Z DEBUG cdumay_job::task] hello[39131d5b-a149-4a84-b183-c5eed1ef1ed1] - PreRun
//! [2019-02-01T16:02:04Z DEBUG cdumay_job::task] hello[39131d5b-a149-4a84-b183-c5eed1ef1ed1] - SetStatus: status updated 'PENDING' -> 'RUNNING'
//! [2019-02-01T16:02:04Z DEBUG cdumay_job::task] hello[39131d5b-a149-4a84-b183-c5eed1ef1ed1] - Run: Result: Ok(0, stdout: None)
//! [2019-02-01T16:02:04Z DEBUG cdumay_job::task] hello[39131d5b-a149-4a84-b183-c5eed1ef1ed1] - PostRun: Result: Ok(0, stdout: Some("Hello Cedric from cdumay-desk"))
//! [2019-02-01T16:02:04Z DEBUG cdumay_job::task] hello[39131d5b-a149-4a84-b183-c5eed1ef1ed1] - SetStatus: status updated 'RUNNING' -> 'SUCCESS'
//! [2019-02-01T16:02:04Z INFO  cdumay_job::task] hello[39131d5b-a149-4a84-b183-c5eed1ef1ed1] - Success: Result: Ok(0, stdout: Some("Hello Cedric from cdumay-desk"))
//! ```
//! **Result**
//! ```json
//! {
//!   "uuid": "1a0c9711-2bff-48f1-9d7e-cdcbe498e9e8",
//!   "retcode": 0,
//!   "stdout": "Hello Cedric from cdumay-desk",
//!   "retval": {}
//! }
//! ```
//!
//! ## Dependencies Links
//!
//! - **cdumay_result**: https://github.com/cdumay/rust-cdumay_result
//! - **cdumay_error**: https://github.com/cdumay/rust-cdumay_error
//!
//! ## Project Links
//!
//! - Issues: https://github.com/cdumay/rust-cdumay_job/issues
//! - Documentation: https://docs.rs/cdumay_job
extern crate cdumay_error;
extern crate cdumay_result;
extern crate core;
extern crate futures;
extern crate hostname;
#[macro_use]
extern crate log;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate uuid;

pub use messages::{Message, MessageInfo};
pub use status::Status;
pub use task::{TaskExec, TaskInfo};
pub use result::Result;

mod messages;
mod operation;
mod task;
mod status;
mod result;