#![crate_name = "memcached"]
#![crate_type = "rlib"]

extern crate futures;
extern crate tokio_core;

pub mod server;
pub mod storage;
pub mod protocol;

mod util;

pub use storage::{Key, Value};
pub use server::Server;
