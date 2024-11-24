use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct Registry {
    functions: Arc<Mutex<HashMap<String, String>>>,
}

impl Registry {
    pub fn register(&self, name: &str, code: &str) {
        self.functions.lock().unwrap().insert(name.to_string(), code.to_string());
    }

    pub fn get(&self, name: &str) -> Option<String> {
        self.functions.lock().unwrap().get(name).cloned()
    }
}
