#![crate_name = "remcached"]
#![crate_type = "rlib"]

#![feature(box_syntax)]

#[macro_use]
extern crate log;
extern crate futures;
extern crate tokio_core;
extern crate chrono;


pub mod server;
pub mod storage;
pub mod protocol;
pub mod logger;

mod util;

pub use storage::{Key, Value};
pub use server::Server;
