//! [![Build Status](https://travis-ci.org/cdumay/rust-cdumay_job.svg?branch=master)](https://travis-ci.org/cdumay/rust-cdumay_job)
//! [![Latest version](https://img.shields.io/crates/v/cdumay_job.svg)](https://crates.io/crates/cdumay_job)
//! [![Documentation](https://docs.rs/cdumay_job/badge.svg)](https://docs.rs/cdumay_job)
//! ![License](https://img.shields.io/crates/l/cdumay_job.svg)
//!
//! A library to follow job execution using validation and checks steps.
//!
//! ## Quickstart
//!
//! ```toml
//! [dependencies]
//! cdumay_error = { git = "https://github.com/cdumay/rust-cdumay_errors" }
//! cdumay_result = { git = "https://github.com/cdumay/rust-cdumay_result" }
//! cdumay_job = { git = "https://github.com/cdumay/rust-cdumay_job" }
//! serde_json = "1.0"
//! serde-value = "0.6"
//! ```
//!
//!
#![feature(try_trait)]
extern crate cdumay_error;
extern crate cdumay_result;
extern crate futures;
extern crate hostname;
#[macro_use]
extern crate log;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_value;
extern crate uuid;
extern crate core;

pub use messages::{KserMessage, MessageRepr};
pub use status::Status;
pub use task::{TaskExec, TaskInfo};

mod messages;
mod operation;
mod task;
mod status;