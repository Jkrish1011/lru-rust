use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/*

LRU = An LRU cache evicts the least recently accessed item when the cache reaches its capacity to make room for new items.
VecDeque = The VecDeque will track the order of item usage, with the most recently used items at the front and the least recently used items at the back.

*/

pub struct LRUCache<K, V> where K: Eq + Hash + Clone {
    capacity: usize,
    storage: HashMap<K, V>,
    usage_order: VecDeque<K>
}

impl<K, V> LRUCache<K, V> where K: Eq + Hash + Clone {
    // Generate a new LRUCache
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Cache capacity must be greater than 0");
        LRUCache {
            capacity: capacity,
            storage: HashMap::new(),
            usage_order: VecDeque::new(),
        }
    }

    pub fn set(&mut self, key: K, value: V) -> () {
        self.storage.insert(key.clone(), value);
        self.update_usage(&key);

        // Check if cache has reached it's capacity. If so, remove the last item
        if self.storage.len() > self.capacity {
            if let Some(least_recently_used) = self.usage_order.pop_back() {
                self.storage.remove(&least_recently_used);
            }
        }
    }

    // To update the VecDeque - Will move the current item forward
    fn update_usage(&mut self, key: &K) -> () {
        // Remove the key if it already exists
        self.usage_order.retain(|existing_key| existing_key != key);

        // Insert it into the front - as it being used currently
        self.usage_order.push_front(key.clone());
    }

    // to fetch the key values
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.storage.contains_key(key) {
            self.update_usage(key);
            self.storage.get(key)
        } else {
            None
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn lru_cache_test() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.set("k1", "Hello");
        lru_cache.set("k2", "world");
        
        // this should move k1 to top.
        println!("Retrieved: {:?}", lru_cache.get(&"k1"));

        lru_cache.set("k3", "All world!");// Should evict k2

        let k2 = lru_cache.get(&"k2");
        if k2.is_some() {
            println!("Value of k2 is : {:?}", k2);
        } else {
            println!("k2 was removed!");
        }
    }
}