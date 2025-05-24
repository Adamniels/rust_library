pub struct Node {
    key: i32,
    value: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(key: i32, value: String) -> Self {
        Node {
            key,
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
            key: self.key.clone(),
            value: self.value.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

// Funktioner:
//  - [X] is_empty -> bool
//  - [X] size -> usize
//  - [X] insert
//  - [ ] contains_value -> bool
//  - [ ] contains_key -> bool
//  - [ ] remove -> value
//  - [ ] in_order -> sorted list with all nodes
//  - [ ] height -> usize
//  - [ ] map - apply a function on all nodes in the tree

pub struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, node: Node) {
        let mut current = &mut self.root;

        loop {
            match current {
                Some(node_ref) => {
                    if node_ref.key == node.key {
                        node_ref.value = node.value;
                        return;
                    } else if node_ref.key < node.key {
                        current = &mut node_ref.right;
                    } else {
                        current = &mut node_ref.left;
                    }
                }
                None => {
                    *current = Some(Box::new(node));
                    return;
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Some(_) = self.root {
            return false;
        }
        return true;
    }

    pub fn size(&self) -> usize {
        if self.root.is_none() {
            return 0;
        }

        let mut count = 0;
        let mut stack = Vec::new();

        stack.push(self.root.as_ref());

        while let Some(Some(node)) = stack.pop() {
            count += 1;
            if let Some(_) = node.left {
                stack.push(node.left.as_ref());
            }
            if let Some(_) = node.right {
                stack.push(node.right.as_ref());
            }
        }

        count
    }

    pub fn contains_key(&self, key: i32) -> bool {
        // can do smart search
        todo!("contains key")
    }

    pub fn contains_value(&self, value: String) -> bool {
        let mut stack = Vec::new();

        if let Some(root) = self.root.as_ref() {
            stack.push(root.as_ref());
        }

        while let Some(node) = stack.pop() {
            if node.value == value {
                return true;
            }

            if let Some(right) = node.get_right() {
                stack.push(right);
            }

            if let Some(left) = node.get_left() {
                stack.push(left);
            }
        }

        false
    }
}
