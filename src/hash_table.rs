#[derive(Clone)] // TODO: varför?
pub struct Entry {
    key: String,
    value: String,
}

impl Entry {
    pub fn new(key: String, value: String) -> Self {
        Entry { key, value }
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn take_value(self) -> String {
        self.value
    }

    pub fn clone_value(&self) -> String {
        self.value.clone()
    }
}

/// simple hash function for testing
pub fn simple_hash(s: &str, buckets: usize) -> usize {
    let mut hash: usize = 0;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as usize);
    }
    hash % buckets
}

// Functions
//  - [X] insert
//  - [X] size, number of entries -> usize
//  - [X] is empty -> bool
//  - [X] get value -> Option<&T>
//  - [X] Remove/take value -> Option<T>
//  - [ ] clone value -> Option<T>  TODO:(next)
//  - [ ] contains key -> bool
//  - [ ] clear
//  - [ ] keys -> Vec<&T>
//  - [ ] values -> Vec<&T>

pub struct HashTable {
    table: Vec<Vec<Entry>>,
    buckets: usize,
    hash_func: fn(&str, usize) -> usize,
}

impl HashTable {
    pub fn new(buckets: usize, hash_func: fn(&str, usize) -> usize) -> Self {
        HashTable {
            table: vec![vec![]; buckets],
            buckets,
            hash_func,
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        let bucket_nr = (self.hash_func)(&key, self.buckets);
        self.table[bucket_nr].push(Entry { key, value });
    }

    pub fn size(&self) -> usize {
        let mut size = 0;
        self.table.iter().for_each(|buck| size += buck.len());

        size
    }

    pub fn is_empty(&self) -> bool {
        self.table.iter().all(|buck| buck.is_empty())
    }

    pub fn get_value(&self, key: &str) -> Option<&str> {
        let bucket_nr = (self.hash_func)(&key, self.buckets);
        let bucket = &self.table[bucket_nr];
        let result = bucket.iter().find(|k| k.get_key() == key);
        match result {
            Some(entry) => {
                return Some(entry.get_value());
            }
            None => {
                return None;
            }
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        let bucket_nr = (self.hash_func)(&key, self.buckets);
        let bucket = &mut self.table[bucket_nr];
        let result = bucket.iter().position(|entry| entry.get_key() == key);
        match result {
            Some(index) => {
                let entry = bucket.remove(index);
                Some(entry.take_value())
            }
            None => {
                return None;
            }
        }
    }
}
