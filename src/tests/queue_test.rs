use crate::queue::Queue; // eller din Queue-struktur

#[test]
fn is_empty_simple_test() {
    let mut queue = Queue::new();
    assert_eq!(queue.is_empty(), true);

    queue.enqueue(String::from("item 1"));
    assert_eq!(queue.is_empty(), false);
}

#[test]
fn enqueue_simple_test() {
    let mut queue = Queue::new();

    queue.enqueue(String::from("item 1"));
}

#[test]
fn size_simple_test() {
    let mut queue = Queue::new();
    assert_eq!(queue.size(), 0);

    queue.enqueue(String::from("item 1"));
    assert_eq!(queue.size(), 1);

    queue.enqueue(String::from("item 2"));
    assert_eq!(queue.size(), 2);

    queue.enqueue(String::from("item 3"));
    assert_eq!(queue.size(), 3);
}

#[test]
fn dequeue_simple_test() {
    let mut queue = Queue::new();

    // should not be possible to dequeue with no items in the queue
    let mut item = queue.dequeue();
    assert_eq!(item, None);

    queue.enqueue(String::from("item 1"));

    // should now be possible to take one out
    item = queue.dequeue();
    match item {
        Some(item) => {
            assert_eq!(item, String::from("item 1"))
        }
        None => panic!("Expected some, got None"),
    }
}

#[test]
fn peek_test_simple() {
    let mut queue = Queue::new();

    // Peek into empty
    let mut peek_item = queue.peek();
    assert_eq!(peek_item, None);

    queue.enqueue(String::from("item 1"));

    peek_item = queue.peek();
    match peek_item {
        Some(item) => {
            assert_eq!(item, "item 1")
        }
        None => {
            panic!("Expected Some")
        }
    }
}

#[test]
fn queue_test() {
    let mut queue = Queue::new();

    assert_eq!(0, queue.size(), "Testing the that size is 0 initialy");
    assert!(queue.is_empty(), "Testing that it is empyt initialy");

    queue.enqueue(String::from("item 1"));

    assert_eq!(
        1,
        queue.size(),
        "Testing that size is 1 after first enqueue"
    );
    assert!(!queue.is_empty(), "Testing: not empty after first enqueue");

    let mut peek_item = queue.peek();
    match peek_item {
        Some(item) => {
            assert_eq!(item, "item 1", "Testing peek after first enqueue")
        }
        None => {
            panic!("Expected Some")
        }
    }

    let mut item = queue.dequeue();
    match item {
        Some(item) => {
            assert_eq!(
                item,
                String::from("item 1"),
                "Testing dequeue with one item"
            )
        }
        None => panic!("Expected some, got None"),
    }
    assert_eq!(0, queue.size(), "Testing queue size is now 0 after dequeue");
    assert!(queue.is_empty(), "Testing queue is empty after dequeue");

    queue.enqueue(String::from("item 1.1"));
    queue.enqueue(String::from("item 2"));
    queue.enqueue(String::from("item 3"));
    queue.enqueue(String::from("item 4"));

    assert_eq!(
        4,
        queue.size(),
        "queue size should be 4 after enqeue 4 items"
    );
}
