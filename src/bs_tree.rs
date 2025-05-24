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

    pub fn get_key(&self) -> i32 {
        self.key
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
//  - [X] contains_value -> bool
//  - [X] contains_key -> bool
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
        let mut current = &self.root;

        loop {
            match current {
                Some(node) => {
                    if key == node.key {
                        return true;
                    } else if key < node.key {
                        current = &node.left;
                    } else {
                        current = &node.right;
                    }
                }
                None => {
                    return false;
                }
            }
        }
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

    pub fn remove(&mut self, key: i32) -> Option<Box<Node>> {
        Self::remove_helper(&mut self.root, key)
    }
    // helper functions for remove
    fn remove_helper(current: &mut Option<Box<Node>>, key: i32) -> Option<Box<Node>> {
        match current {
            Some(node) => {
                if key < node.key {
                    return Self::remove_helper(&mut node.left, key);
                } else if key > node.key {
                    return Self::remove_helper(&mut node.right, key);
                } else {
                    let mut target = current.take().unwrap(); // flytta ut noden så vi får ägarskap

                    return match (target.left.take(), target.right.take()) {
                        (None, None) => {
                            // Inga barn – bara ta bort noden
                            // take() sätter None istället därför vi inte behöver uppdatera parent
                            Some(target)
                        }
                        (Some(child), None) | (None, Some(child)) => {
                            // Ett barn – returnera barnet
                            *current = Some(child);
                            Some(target)
                        }
                        (Some(left), Some(right)) => {
                            // Två barn – hitta minsta i höger subträd
                            let (mut min, new_right) = Self::remove_min(right);
                            min.left = Some(left);
                            min.right = new_right;
                            *current = Some(min);
                            Some(target)
                        }
                    };
                }
            }
            None => None, // noden finns inte
        }
    }

    fn remove_min(mut node: Box<Node>) -> (Box<Node>, Option<Box<Node>>) {
        match node.left {
            None => {
                // Vi har hittat minsta nod
                let right = node.right.take();
                (node, right)
            }
            Some(_) => {
                // Fortsätt söka till vänster
                let (min, new_left) = Self::remove_min(node.left.take().unwrap());
                node.left = new_left;
                (min, Some(node))
            }
        }
    }
}
