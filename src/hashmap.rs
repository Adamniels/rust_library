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

pub struct HashMap {
    table: Vec<Vec<Entry>>,
    buckets: usize,
    hash_func: fn(&str, usize) -> usize,
}

impl HashMap {
    pub fn new(buckets: usize, hash_func: fn(&str, usize) -> usize) -> Self {
        HashMap {
            table: vec![vec![]; buckets],
            buckets,
            hash_func,
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        let bucket_to_insert_into = (self.hash_func)(&key, self.buckets);
        self.table[bucket_to_insert_into].push(Entry { key, value });
    }

    pub fn size(&self) -> usize {
        let mut size = 0;
        self.table.iter().for_each(|buck| size += buck.len());

        size
    }

    pub fn is_empty(&self) -> bool {
        self.table.iter().all(|buck| buck.is_empty())
    }
}
