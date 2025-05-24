use crate::bs_tree::BinarySearchTree;
use crate::bs_tree::Node;

#[test]
fn test_node_all_methods() {
    // Skapa root-nod
    let mut root = Node::new(10, String::from("root"));

    // Kontrollera att get_value fungerar
    assert_eq!(root.get_value(), "root");

    // Skapa barnnoder
    let left_child = Node::new(5, String::from("left"));
    let right_child = Node::new(15, String::from("right"));

    // Lägg till barnnoder
    root.set_left_node(left_child);
    root.set_right_node(right_child);

    // Kontrollera att barnnoderna sattes korrekt
    assert!(root.get_left().is_some());
    assert!(root.get_right().is_some());

    assert_eq!(root.get_left().as_ref().unwrap().get_value(), "left");
    assert_eq!(root.get_right().as_ref().unwrap().get_value(), "right");

    // Testa kloning
    let clone = root.clone();
    assert_eq!(clone.get_value(), "root");
    assert_eq!(clone.get_left().as_ref().unwrap().get_value(), "left");
    assert_eq!(clone.get_right().as_ref().unwrap().get_value(), "right");

    // Testa destroy (tar över ägande)
    let to_destroy = Node::new(99, String::from("destroy_me"));
    let destroyed_value = to_destroy.destroy();
    assert_eq!(destroyed_value, "destroy_me");
    // (Man kan inte använda to_destroy här efteråt – ownership är borta!)
}

#[test]
fn test_is_empty_insert_size() {
    // Skapa ett tomt träd
    let mut tree = BinarySearchTree::new();

    // Det borde vara tomt direkt efter skapande
    assert!(tree.is_empty(), "Tree should be empty at start");
    assert_eq!(tree.size(), 0, "Tree size should be 0 at start");

    // Lägg till en nod
    tree.insert(Node::new(10, "root".to_string()));
    assert!(!tree.is_empty(), "Tree should not be empty after insert");
    assert_eq!(tree.size(), 1, "Tree size should be 1 after one insert");

    // Lägg till fler noder
    tree.insert(Node::new(5, "left".to_string()));
    tree.insert(Node::new(15, "right".to_string()));
    tree.insert(Node::new(3, "left-left".to_string()));
    tree.insert(Node::new(7, "left-right".to_string()));

    assert_eq!(tree.size(), 5, "Tree size should be 5 after five inserts");
}

#[test]
fn test_contains_key() {
    let mut tree = BinarySearchTree::new();

    assert!(!tree.contains_key(10), "Don't contain key before inserts");

    tree.insert(Node::new(10, "root".to_string()));
    assert!(tree.contains_key(10), "Contain key after first insert");

    tree.insert(Node::new(5, "left".to_string()));
    tree.insert(Node::new(15, "right".to_string()));
    tree.insert(Node::new(3, "left-left".to_string()));
    tree.insert(Node::new(7, "left-right".to_string()));

    assert!(tree.contains_key(15), "Contain key after many inserts");
    assert!(tree.contains_key(5), "Contain key after many inserts");
    assert!(tree.contains_key(3), "Contain key after many inserts");
    assert!(tree.contains_key(7), "Contain key after many inserts");

    tree.insert(Node::new(7, "left-right-new".to_string()));
    assert!(tree.contains_key(7), "Contain key after overwrite");
}

#[test]
fn test_contains_value() {
    let mut tree = BinarySearchTree::new();

    assert!(
        !tree.contains_value("root".to_string()),
        "Don't contain value before inserts"
    );

    tree.insert(Node::new(10, "root".to_string()));
    assert!(
        tree.contains_value("root".to_string()),
        "Contain value after first insert"
    );

    tree.insert(Node::new(5, "left".to_string()));
    tree.insert(Node::new(15, "right".to_string()));
    tree.insert(Node::new(3, "left-left".to_string()));
    tree.insert(Node::new(7, "left-right".to_string()));

    assert!(
        tree.contains_value("right".to_string()),
        "Contain value after many inserts"
    );
    assert!(
        tree.contains_value("left".to_string()),
        "Contain value after many inserts"
    );
    assert!(
        tree.contains_value("left-left".to_string()),
        "Contain value after many inserts"
    );
    assert!(
        tree.contains_value("left-right".to_string()),
        "Contain value after many inserts"
    );

    tree.insert(Node::new(7, "left-right-new".to_string()));
    assert!(
        tree.contains_value("left-right-new".to_string()),
        "Contain updated value after overwrite"
    );
}

#[test]
fn test_remove_nodes() {
    let mut tree = BinarySearchTree::new();

    // Lägg till noder
    tree.insert(Node::new(10, "root".to_string()));
    tree.insert(Node::new(5, "left".to_string()));
    tree.insert(Node::new(15, "right".to_string()));
    tree.insert(Node::new(3, "left-left".to_string()));
    tree.insert(Node::new(7, "left-right".to_string()));

    assert_eq!(tree.size(), 5, "tree size before any remove");

    // Ta bort ett blad (left-left)
    let removed = tree.remove(3);
    assert!(removed.is_some(), "should remove existing leaf");
    assert_eq!(removed.unwrap().get_value(), "left-left");
    assert!(!tree.contains_key(3), "should not contain removed leaf");
    assert_eq!(tree.size(), 4, "tree size after removing leaf");

    // Ta bort nod med ett barn (left)
    let removed = tree.remove(5);
    assert!(removed.is_some(), "should remove node with one child");
    assert_eq!(removed.unwrap().get_value(), "left");
    assert!(!tree.contains_key(5), "should not contain removed node");
    assert_eq!(
        tree.size(),
        3,
        "tree size after removing node with one child"
    );

    // Ta bort nod med två barn (root)
    let removed = tree.remove(10);
    assert!(removed.is_some(), "should remove root with two children");
    assert_eq!(removed.unwrap().get_value(), "root");
    assert!(!tree.contains_key(10), "should not contain removed root");
    assert_eq!(tree.size(), 2, "tree size after removing root node");

    // Ta bort nod som inte finns
    let removed = tree.remove(999);
    assert!(removed.is_none(), "should return None for non-existent key");
    assert_eq!(
        tree.size(),
        2,
        "tree size should be unchanged after failed remove"
    );
}

#[test]
fn test_remove_node_with_two_children_deep_right_min() {
    let mut tree = BinarySearchTree::new();

    // Skapa detta träd:
    //         10
    //        /  \
    //      5     20
    //           /
    //         15
    //        /
    //      12
    //     /
    //    11   <- minsta noden i höger subträd (leftmost in right)

    tree.insert(Node::new(10, "root".to_string()));
    tree.insert(Node::new(5, "left".to_string()));
    tree.insert(Node::new(20, "right".to_string()));
    tree.insert(Node::new(15, "mid-right".to_string()));
    tree.insert(Node::new(12, "deep-left".to_string()));
    tree.insert(Node::new(11, "deepest-left".to_string()));

    assert_eq!(tree.size(), 6);

    let removed = tree.remove(10);
    assert!(removed.is_some(), "should remove root with two children");

    let removed_node = removed.unwrap();
    assert_eq!(removed_node.get_key(), 10);
    assert_eq!(removed_node.get_value(), "root");

    // Root ska nu vara 11 (minsta i höger subträd)
    assert!(tree.contains_key(11));
    assert_eq!(tree.size(), 5);
}
