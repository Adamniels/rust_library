#[derive(Clone)]
pub struct Entry<K: PartialEq + Clone, V: PartialEq + Clone> {
    key: K,
    value: V,
}

impl<K: PartialEq + Clone, V: PartialEq + Clone> Entry<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Entry { key, value }
    }

    pub fn get_key(&self) -> &K {
        &self.key
    }

    pub fn get_value(&self) -> &V {
        &self.value
    }

    pub fn take_value(self) -> V {
        self.value
    }

    pub fn clone_value(&self) -> V {
        self.value.clone()
    }
}

/// simple hash function for testing
pub fn simple_hash(s: &String, buckets: usize) -> usize {
    let mut hash: usize = 0;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as usize);
    }
    hash % buckets
}

// Functions:
//  - [X] insert
//  - [X] size, number of entries -> usize
//  - [X] is empty -> bool
//  - [X] get value -> Option<&T>
//  - [X] Remove/take value -> Option<T>
//  - [X] clone value -> Option<T>
//  - [X] contains key -> bool
//  - [X] contains value -> bool
//
//  Maybe:
//      - clear
//      - keys -> Vec<&T>
//      - values -> Vec<&T>
//      - keys cloned
//      - values cloned

// TODO:
//   - kolla coverage

pub struct HashTable<K: PartialEq + Clone, V: PartialEq + Clone> {
    table: Vec<Vec<Entry<K, V>>>,
    buckets: usize,
    hash_func: fn(&K, usize) -> usize,
}

impl<K: PartialEq + Clone, V: PartialEq + Clone> HashTable<K, V> {
    pub fn new(buckets: usize, hash_func: fn(&K, usize) -> usize) -> Self {
        HashTable {
            table: vec![vec![]; buckets],
            buckets,
            hash_func,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let bucket_nr = {
            // &key lever bara inuti detta block, vilket gör att jag inte behöver clone på key
            // tidigare så behövdes det för att &key satt på en referens och då inte gick att
            // skicka in i hast tablet
            let key_ref = &key;
            (self.hash_func)(key_ref, self.buckets)
        };
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

    pub fn get_value(&self, key: &K) -> Option<&V> {
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

    pub fn remove(&mut self, key: &K) -> Option<V> {
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

    pub fn clone(&mut self, key: &K) -> Option<V> {
        let bucket_nr = (self.hash_func)(&key, self.buckets);
        let bucket = &mut self.table[bucket_nr];
        let result = bucket.iter().position(|entry| entry.get_key() == key);
        match result {
            Some(index) => {
                let clone_value = bucket[index].clone_value();
                Some(clone_value)
            }
            None => {
                return None;
            }
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let bucket_nr = (self.hash_func)(&key, self.buckets);
        let found = &self.table[bucket_nr]
            .iter()
            .find(|entry| entry.get_key() == key);

        match found {
            Some(..) => true,
            None => false,
        }
    }

    pub fn contains_value(&self, value: &V) -> bool {
        // kan använda any ist för alternativ lösning ovanför
        self.table
            .iter()
            .any(|bucket| bucket.iter().any(|entry| entry.get_value() == value))
    }
}
