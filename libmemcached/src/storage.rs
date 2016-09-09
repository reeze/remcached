
use std::result;

pub type Key = Vec<u8>;
pub type Value = Vec<u8>;

trait Engine {
    fn get(&self, key: &Key) -> Result<Option<Value>>;
    fn set(&self, key: &Key, value: &Value) -> Result<()>;
}


#[derive(Debug)]
pub enum Error {
    EnternalError,
    UnknowError,
}

pub type Result<T> = result::Result<T, Error>;

struct MemStore {

}

impl MemStore {
    fn new() -> Self {
        MemStore{}
    }
}

impl Engine for MemStore {
    fn get(&self, key: &Key) -> Result<Option<Value>> {
        Ok(Some(vec!()))
    }

    fn set(&self, key: &Key, value: &Value) -> Result<()> {
        Ok(())
    }
}

pub struct Storage {
    engine: Box<Engine>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            engine: Box::new(MemStore::new()) // only mem storage for now
        }
    }

    pub fn get(&self, key: &Key) -> Result<Option<Value>> {
        self.engine.get(key)
    }

    pub fn set(&self, key: &Key, value: &Value) -> Result<()> {
        self.engine.set(key, value)
    }
}
