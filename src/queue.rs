// TODO: lägg till en iterator för denna klass

pub struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { data: vec![] }
    }

    pub fn enqueue(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
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

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }
}
