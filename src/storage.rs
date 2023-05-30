use std::collections::HashMap;
use std::fmt;

pub struct Storage {
    m: HashMap<String, String>,
}

pub struct ErrorUnique {
}

impl fmt::Debug for ErrorUnique {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

pub struct ErrorNotFound {
}

impl fmt::Debug for ErrorNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Storage {
    pub fn new() -> Storage {
        let map = HashMap::new();
        Storage{m: map}
    }
    
    pub fn set(&mut self, key: String, value: String, uniques: bool) -> Result<(), ErrorUnique> {
        if uniques {
            return Err(ErrorUnique {});
        }

        self.m.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<&String, ErrorNotFound> {
        match self.m.get(&key) {
            Some(v) => Ok(v),
            None => Err(ErrorNotFound {})
        }
    }
}