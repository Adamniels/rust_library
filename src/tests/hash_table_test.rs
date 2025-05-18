use crate::hash_table::{Entry, HashTable, simple_hash};

// TODO: skriva ut mina test resultat snyggare, med ett script, hur enkelt är detta?

////// Testing entry struct //////
#[test]
fn entry_simple_test() {
    let entry = Entry::new("key1".to_string(), "value1".to_string());

    assert_eq!("key1", entry.get_key(), "Get the key");
    assert_eq!("value1", entry.get_value(), "Get the value");

    let clone_value = entry.clone_value();
    assert_eq!(
        "value1", clone_value,
        "Assert cloned value is the same as value"
    );
    assert_eq!(
        "value1",
        entry.get_value(),
        "Assert we still can get value after cloned"
    );

    let taken_value = entry.take_value();
    assert_eq!("value1", taken_value, "Assert taken value is correct value");
}

////// Testing HashTable struct //////

/// Testing insert, size and is_empty
#[test]
fn hashtable_simple_test() {
    let mut hash_table: HashTable<String, String> = HashTable::new(17, simple_hash);

    assert_eq!(0, hash_table.size(), "Assert size is 0 intitialy");
    assert!(
        hash_table.is_empty(),
        "Assert hash table is empty intitialy"
    );

    hash_table.insert("key1".to_string(), "value1".to_string());

    assert_eq!(1, hash_table.size(), "Assert size is 1 after insert");
    assert!(!hash_table.is_empty(), "Assert not empty after insert");
}

#[test]
fn hashtable_get_simple_test() {
    let mut hash_table = HashTable::new(17, simple_hash);

    hash_table.insert("key1".to_string(), "value1".to_string());

    let key1 = String::from("key1");
    let mut value = hash_table.get_value(&key1);

    match value {
        Some(value) => {
            assert_eq!("value1", value, "First get_value after insert")
        }
        None => {
            panic!("expected Some")
        }
    }
    value = hash_table.get_value(&"non existing".to_string());

    assert_eq!(None, value, "Trying non existing key")
}

#[test]
fn hashtable_remove_simple_test() {
    let mut hash_table = HashTable::new(17, simple_hash);

    let key = "key1".to_string();
    hash_table.insert(key.clone(), "value1".to_string());

    assert!(!hash_table.is_empty(), "not empty before remove");
    assert_eq!(1, hash_table.size(), "size 1 before remove");

    let mut value = hash_table.remove(&key);

    assert!(hash_table.is_empty(), "empty after remove");
    assert_eq!(0, hash_table.size(), "size 0 after remove");

    match value {
        Some(value) => {
            assert_eq!("value1", value, "First get_value after insert")
        }
        None => {
            panic!("expected Some")
        }
    }

    let non_existing = "non existing".to_string();
    value = hash_table.remove(&non_existing);

    assert_eq!(None, value, "Trying non existing key");
}

#[test]
fn hashtable_clone_simple_test() {
    let mut hash_table = HashTable::new(17, simple_hash);

    let key = "key1".to_string();
    hash_table.insert(key.clone(), "value1".to_string());

    assert!(!hash_table.is_empty(), "not empty before clone");
    assert_eq!(1, hash_table.size(), "size 1 before clone");

    let mut value = hash_table.clone(&key);

    assert!(!hash_table.is_empty(), "still not empty after clone");
    assert_eq!(1, hash_table.size(), "size still 1 after clone");

    match value {
        Some(value) => {
            assert_eq!("value1", value, "First get_value after insert")
        }
        None => {
            panic!("expected Some")
        }
    }

    let non_existing = "non existing".to_string();
    value = hash_table.clone(&non_existing);

    assert_eq!(None, value, "Trying non existing key");

    value = hash_table.remove(&key);

    assert!(hash_table.is_empty(), "empty after remove");
    assert_eq!(0, hash_table.size(), "size 0 after remove");

    match value {
        Some(value) => {
            assert_eq!("value1", value, "First get_value after insert")
        }
        None => {
            panic!("expected Some")
        }
    }
}

#[test]
fn hashtable_contains_simple_test() {
    let mut hash_table = HashTable::new(17, simple_hash);

    let key = "key1".to_string();
    let value_str = "value1".to_string();

    hash_table.insert(key.clone(), value_str.clone());

    assert!(
        hash_table.contains_key(&key),
        "contains after inserted key1"
    );
    assert!(
        hash_table.contains_value(&value_str),
        "contains after inserted value1"
    );

    let non_existing = "non existing".to_string();
    assert!(
        !hash_table.contains_key(&non_existing),
        "dont contain uninserted key"
    );
    assert!(
        !hash_table.contains_value(&non_existing),
        "dont contain uninserted value"
    );

    hash_table.remove(&key);

    assert!(
        !hash_table.contains_key(&key),
        "dont contain after removing entry"
    );
    assert!(
        !hash_table.contains_value(&value_str),
        "dont contain after removing entry"
    );
}
