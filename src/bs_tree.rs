pub struct Node {
    nr: i32,
    value: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(nr: i32, value: String) -> Self {
        Node {
            nr,
            value,
            left: None,
            right: None,
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn destroy(self) -> String {
        self.value
    }

    pub fn set_left_node(&mut self, node: Node) {
        self.left = Some(Box::new(node));
    }

    pub fn set_right_node(&mut self, node: Node) {
        self.right = Some(Box::new(node));
    }

    pub fn get_left(&self) -> Option<&Box<Node>> {
        self.left.as_ref()
    }

    pub fn get_right(&self) -> Option<&Box<Node>> {
        self.right.as_ref()
    }
}

// kan egentligen använda #[derive(Clone)]
impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            nr: self.nr.clone(),
            value: self.value.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

// Funktioner:
//  - [ ] is_empty
//  - [ ] size
//  - [ ] insert
//  - [ ] contains
//  - [ ] remove
//  - [ ] in_order -> sorterad lista med alla
//  - [ ] height
//  - [ ] map - gör en funktion på alla element i trädet

pub struct BinarySearchTree {
    root: Option<Node>,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, node: Node) {
        todo!("insert")
    }

    pub fn is_empty(&self) -> bool {
        todo!("is empty")
    }

    pub fn size(&self) -> usize {
        todo!("size")
    }
}
