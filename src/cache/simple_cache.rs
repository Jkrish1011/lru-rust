use std::collections::HashMap;
use std::hash::Hash;

// Cache generic type which implements the Eq and Hash.
pub struct Cache<M, N> where M: Eq + Hash {
    // Stores the cached values
    storage: HashMap<M, N>,
}

impl<M, N> Cache<M, N> where M: Eq + Hash + Clone {
    // Acts like the constructor
    pub fn new() -> Self {
        Cache {
            storage: HashMap::new(),
        }
    }

    // Insertion
    pub fn set(&mut self, key: M, value: N) -> bool {
        self.storage.insert(key, value);
        true
    }

    // Fetch values - Returns an Option(Some or None)
    pub fn get(&self, key: &M) -> Option<&N> {
        self.storage.get(key)
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_simple_cache() {
        let mut cache = Cache::new();
        cache.set("Hi", "Hello");
        // cache.set(1, "Hello World!");
        cache.set("1", "56790");
        // cache.set(890, 13251920);
    
        let value = cache.get(&"Hi");
    
        match value {
            Some(v) => {
                println!("Value found : {}", v);
            },
            None => {
                println!("Value not found!");
            }
        }
    
        if let Some(value) = cache.get(&"2") {
            println!("Value found : {}", value);
        } else {
            println!("Value not found!");
        }
    }
}