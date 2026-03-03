use std::collections::HashMap;

pub struct Store {
    map: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> String {
        match self.map.get(key) {
            Some(value) => value.clone(),
            None => String::from("DOES NOT EXIST"),
        }
    }

    pub fn set(&mut self, key: &str, value: String) -> String {
        self.map.insert(String::from(key), value);
        String::from("OK")
    }

    pub fn delete(&mut self, key: &str) -> String {
        match self.map.remove(key) {
            Some(value) => value,
            None => String::from("DOES NOT EXIST"),
        }
    }
}
