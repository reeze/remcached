use std::fmt::Debug;

use storage::{Key, Value, Result};


pub trait Engine: Send + Debug {
    fn get(&self, key: &Key) -> Result<Option<Value>>;
    fn set(&self, key: &Key, value: &Value) -> Result<()>;


    fn clone(&self) -> Box<Engine + 'static>;
}

#[derive(Debug, Clone)]
pub struct MemStore {

}

pub enum Type {
    Memory,
}

impl MemStore {
    pub fn new() -> Self {
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

    fn clone(&self) -> Box<Engine> {
        box MemStore {}
    }
}
