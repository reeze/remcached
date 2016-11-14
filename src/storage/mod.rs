
use std::result;

pub mod engine;

pub use self::engine::{Engine, Type, MemStore};

pub type Key = Vec<u8>;
pub type Value = Vec<u8>;

pub type Result<T> = result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
    EnternalError,
    UnknowError,
}

pub struct Storage {
    engine: Box<Engine>,
}

impl Storage {
    pub fn new(t: Type) -> Self {
        let engine = match t {
            Type::Memory => MemStore::new()
            // maybe more engine later
        };

        Storage {
            engine: Box::new(engine)
        }
    }
}
