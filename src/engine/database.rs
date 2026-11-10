use std::collections::HashMap;

use crate::engine::DbError;

pub struct Database {
    // todo
    data: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        return Self {
            data: HashMap::new(),
        };
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        return self.data.get(key).map(String::as_str);
    }

    pub fn delete(&mut self, key: &str) -> bool {
        return self.data.remove(key).is_some();
    }

    pub fn exists(&self, key: &str) -> bool {
        return self.data.contains_key(key);
    }

    pub fn increment(&mut self, key: &str) -> Result<i64, DbError> {
        let current: i64 = match self.get(key) {
            Some(value) => value.parse::<i64>().map_err(|_| DbError::InvalidInteger)?,
            None => 0,
        };

        let next: i64 = current.checked_add(1).ok_or(DbError::IntegerOverflow)?;

        self.set(key.to_owned(), next.to_string());
        return Ok(next);
    }

    pub fn key_type(&self, key: &str) -> Option<&'static str> {
        match self.data.contains_key(key) {
            true => Some("string"),
            false => None
        }
    }

    pub fn flush(&mut self) {
        self.data.clear();
    }
}