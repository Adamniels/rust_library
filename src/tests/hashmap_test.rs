use crate::hashmap::{simple_hash, Entry, HashMap};

// TODO: skriva ut mina test resultat snyggare, med ett script, borde göra en makefile

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

////// Testing HashMap struct //////

/// Testing insert, size and is_empty
#[test]
fn hashmap_simple_test() {
    let mut hashMap = HashMap::new(17, simple_hash);

    assert_eq!(0, hashMap.size(), "Assert size is 0 intitialy");
    assert!(hashMap.is_empty(), "Assert hashMap is empty intitialy");

    hashMap.insert("key1".to_string(), "value1".to_string());

    assert_eq!(1, hashMap.size(), "Assert size is 1 after insert");
    assert!(!hashMap.is_empty(), "Assert not empty after insert");
}
