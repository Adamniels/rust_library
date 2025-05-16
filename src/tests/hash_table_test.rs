use crate::hash_table::{simple_hash, Entry, HashTable};

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
    let mut hash_table = HashTable::new(17, simple_hash);

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

    let mut value = hash_table.get_value("key1");

    match value {
        Some(value) => {
            assert_eq!("value1", value, "First get_value after insert")
        }
        None => {
            panic!("expected Some")
        }
    }
    value = hash_table.get_value("non existing");

    assert_eq!(None, value, "Trying non existing key")
}

#[test]
fn hashtable_remove_simple_test() {
    let mut hash_table = HashTable::new(17, simple_hash);

    hash_table.insert("key1".to_string(), "value1".to_string());

    assert!(!hash_table.is_empty(), "not empty before remove");
    assert_eq!(1, hash_table.size(), "size 1 before remove");

    let mut value = hash_table.remove("key1");

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
    value = hash_table.remove("non existing");

    assert_eq!(None, value, "Trying non existing key")
}

#[test]
fn hashtable_clone_simple_test() {
    let mut hash_table = HashTable::new(17, simple_hash);

    hash_table.insert("key1".to_string(), "value1".to_string());

    assert!(!hash_table.is_empty(), "not empty before clone");
    assert_eq!(1, hash_table.size(), "size 1 before clone");

    let mut value = hash_table.clone("key1");

    assert!(!hash_table.is_empty(), "still not empty after clone");
    assert_eq!(1, hash_table.size(), "size still 1 after remove");

    match value {
        Some(value) => {
            assert_eq!("value1", value, "First get_value after insert")
        }
        None => {
            panic!("expected Some")
        }
    }
    value = hash_table.clone("non existing");

    assert_eq!(None, value, "Trying non existing key");

    value = hash_table.remove("key1");

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

    hash_table.insert("key1".to_string(), "value1".to_string());

    assert!(
        hash_table.contains_key("key1"),
        "contains after inserted key1"
    );
    assert!(
        hash_table.contains_value("value1"),
        "contains after inserted value1"
    );

    assert!(
        !hash_table.contains_key("non existing"),
        "dont contain uninserted key"
    );
    assert!(
        !hash_table.contains_value("non existing"),
        "dont contain uninserted value"
    );

    hash_table.remove("key1");

    assert!(
        !hash_table.contains_key("key1"),
        "dont contain after removing entry"
    );
    assert!(
        !hash_table.contains_value("value1"),
        "dont contain after removing entry"
    );
}
