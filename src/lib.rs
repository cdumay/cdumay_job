//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_job on crates.io](https://img.shields.io/crates/v/cdumay_job)](https://crates.io/crates/cdumay_job)
//! [![cdumay_job on docs.rs](https://docs.rs/cdumay_job/badge.svg)](https://docs.rs/cdumay_job)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_job)
//!
//! A library to follow job execution using validation and checks steps.
//!
//! ## Quickstart
//!
//! _Cargo.toml_:
//! ```toml
//! [dependencies]
//! cdumay_error = "1.0"
//! cdumay_result = "1.0"
//! cdumay_job = "1.0"
//! serde = "1.0"
//! serde_json = "1.0"
//! serde-value = "0.7"
//! env_logger = "0.11"
//! hostname = "0.4"
//! ```
//!
//! _main.rs_:
//! ```rust
//! use cdumay_error::Error;
//! use cdumay_job::{Message, MessageBuilder, Status, TaskExec, TaskInfo};
//! use cdumay_result::{Result, ResultBuilder};
//! use serde_value::Value;
//! use std::collections::HashMap;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! pub struct Params {
//!     user: String
//! }
//!
//! #[derive(Clone)]
//! pub struct Hello {
//!     message: Message,
//!     status: Status,
//!     result: Result,
//! }
//!
//! impl TaskInfo for Hello {
//!     fn new(msg: &Message, result: Option<Result>) -> Hello {
//!         Hello {
//!             message: msg.clone(),
//!             result: result.unwrap_or(msg.result.clone()),
//!             status: Status::Pending,
//!         }
//!     }
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
//!     fn run(&mut self) -> cdumay_error::Result<Result> {
//!         let default = "John Smith".to_string();
//!         let host = match hostname::get() {
//!             Ok(os_string) => os_string.to_string_lossy().to_string(),
//!             Err(_) => "localhost".to_string()
//!         };
//!
//!         let params: Params = match self.message().params {
//!             Some(params) => params.deserialize_into().unwrap(),
//!             None => Params {user: "undef".to_string()}
//!         };
//!         Ok(ResultBuilder::from(&self.message())
//!             .stdout(format!("Hello {} from {}", params.user, host))
//!             .build()
//!         )
//!     }
//! }
//!
//! fn main() {
//!     use std::collections::BTreeMap;
//!     env_logger::init();
//!     let message = MessageBuilder::new("hello".to_string())
//!         .params({
//!             let params = Params {user: "Cedric".to_string()};
//!             serde_value::to_value(params).unwrap()
//!         }).build();
//!
//!     let mut task = Hello::new(&message, None);
//!     println!("{}", serde_json::to_string_pretty(&task.execute(None)).unwrap());
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
//! ## Macros
//!
//! To automatically generate implementations for tasks, this create defines macros
//!
//! The following code reuse the previous example
//!
//! ```rust
//! use cdumay_job::{define_task, MessageBuilder, TaskExec,TaskInfo };
//! use log::info;
//! use serde::{Deserialize, Serialize};
//! use std::collections::BTreeMap;
//! use cdumay_result::ResultBuilder;
//!
//! #[derive(Serialize, Deserialize)]
//! pub struct Params {
//!     user: String
//! }
//!
//! define_task!(Hello);
//!
//! impl TaskExec for Hello {
//!     fn run(&mut self) -> cdumay_error::Result<cdumay_result::Result> {
//!         let default = "John Smith".to_string();
//!         let host = match hostname::get() {
//!             Ok(os_string) => os_string.to_string_lossy().to_string(),
//!             Err(_) => "localhost".to_string()
//!         };
//!
//!         let params: Params = match self.message().params {
//!             Some(params) => params.deserialize_into().unwrap(),
//!             None => Params {user: "undef".to_string()}
//!         };
//!         Ok(ResultBuilder::from(&self.message())
//!             .stdout(format!("Hello {} from {}", params.user, host))
//!             .build()
//!         )
//!     }
//! }
//!
//!
//! fn main() {
//!     env_logger::init();
//!     let message = MessageBuilder::new("hello".to_string())
//!         .params({
//!             let params = Params {user: "Cedric".to_string()};
//!             serde_value::to_value(params).unwrap()
//!         }).build();
//!
//!     let mut task = Hello::new(&message, None);
//!     println!("{}", serde_json::to_string_pretty(&task.execute(None)).unwrap());
//! }
//! ```
//!

pub use messages::{Message, MessageBuilder};
pub use status::Status;
pub use task::{TaskExec, TaskInfo};

mod messages;
mod operation;
mod status;
mod task;
#[macro_use]
mod macros;
