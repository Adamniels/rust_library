use std::io;

pub struct Queue {
    data: Vec<String>,
}

impl Queue {
    pub fn new() -> Self {
        Queue { data: vec![] }
    }

    pub fn enqueue(&mut self, item: String) {
        self.data.push(item);
    }

    pub fn dequeue(&mut self) -> Option<String> {
        if self.is_empty() {
            return None;
        }
        Some(self.data.remove(0))
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn peek(&self) -> Option<&str> {
        self.data.get(0).map(|str| str.as_str())
    }
}
