#![feature(try_trait)]
extern crate cdumay_error;
extern crate cdumay_result;
extern crate env_logger;
extern crate futures;
extern crate hostname;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate serde_value;
extern crate uuid;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

pub mod messages;
pub mod operation;
pub mod task;
pub mod status;