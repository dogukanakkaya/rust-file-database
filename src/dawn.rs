use std::collections::HashMap;
use std::fs;
use std::io::{Result as IoResult};

pub struct Dawn {
    db: String,
    data: HashMap<String, String>
}

impl Dawn {
    pub fn new(db: String) -> Self {
        let data = fs::read_to_string(&db).expect("Something went wrong reading the database");
        let pairs = create_pairs(data);

        Self {
            db,
            data: pairs
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        // self.data.entry(key).or_insert(value);

        // overrides the old value if same key already exists
        self.data.insert(key, value);
    }
    
    pub fn get(&mut self, key: String) -> Option<&String> {
        self.data.get(&*key)
    }

    pub fn delete(&mut self, key: String) -> Option<String> {
        self.data.remove(&*key)
    }

    pub fn flush(&self) -> IoResult<()> {
        fs::write(&self.db, self.data_to_string())
    }

    fn data_to_string(&self) -> String {
        let mut data = String::new();

        for (key, value) in &self.data {
            data.push_str(&format!("{}:{}\n", key, value))
        }

        data
    }
}

fn create_pairs<'a>(data: String) -> HashMap<String, String> {
    let mut pairs: HashMap<String, String> = HashMap::new();

    for line in data.lines() {
        if let Some(i) = line.find(':') {
            pairs.insert(line[..i].to_string(), line[i + 1..].to_string());
        }
    }

    pairs
}