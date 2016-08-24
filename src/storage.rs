
trait StorageEngine {
    fn new() -> Self;
    fn set(&self, key: &str, value: str);
    fn get(&self, key: &str) -> Option<&str>;
}

pub struct Storage {

}

// TODO use a plugable engine
impl Storage {
    pub fn new() -> Storage {
        Storage{}
    }

    pub fn set(&self, key: &str, value: &str)
    {
    
    }
}
