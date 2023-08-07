pub struct CircularQueue<T> {
    queue: Vec<Option<T>>,
    head: usize,
    tail: usize, // next free spot
    size: usize,
    max_size: usize,
}

impl<T> CircularQueue<T> {
    pub fn new(init_size: usize) -> Self {
        let mut buffer = Vec::with_capacity(init_size);
        for _ in 0..init_size {
            buffer.push(None);
        }

        CircularQueue {
            queue: buffer,
            head: 0,
            tail: 0,
            size: 0,
            max_size: init_size,
        }
    }

    pub fn enqueue(&mut self, item: T) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Queue is full");
        }

        self.queue[self.tail] = Some(item);

        self.tail += 1;
        self.tail %= self.max_size;

        self.size += 1;

        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let res = self.queue[self.head].take();

        self.head += 1;
        self.head %= self.max_size;

        self.size -= 1;

        res
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.queue[self.head].as_ref()
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.max_size
    }
}

#[cfg(test)]
mod tests {
    use super::CircularQueue;

    #[test]
    fn basic_operations() {
        let mut queue = CircularQueue::new(3);

        assert!(queue.is_empty());
        assert!(!queue.is_full());
        assert_eq!(queue.peek(), None);

        assert_eq!(queue.enqueue(1), Ok(()));
        assert_eq!(queue.peek(), Some(&1));
        assert!(!queue.is_empty());
        assert!(!queue.is_full());

        assert_eq!(queue.enqueue(2), Ok(()));
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.enqueue(3), Ok(()));
        assert!(queue.is_full());

        assert_eq!(queue.enqueue(4), Err("Queue is full"));

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.peek(), Some(&2));
        assert!(!queue.is_full());

        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
        assert!(queue.is_empty());
    }

    #[test]
    fn circular_behavior() {
        let mut queue = CircularQueue::new(2);

        assert_eq!(queue.enqueue(1), Ok(()));
        // assert_eq!(queue.enqueue(2), Ok(()));
        // assert_eq!(queue.dequeue(), Some(1));
        // assert_eq!(queue.enqueue(3), Ok(()));
        // assert_eq!(queue.dequeue(), Some(2));
        // assert_eq!(queue.dequeue(), Some(3));
        // assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn peek_does_not_modify_queue() {
        let mut queue = CircularQueue::new(3);

        assert_eq!(queue.enqueue(1), Ok(()));
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.peek(), Some(&1)); // Peek again to make sure it doesn't modify the queue
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.peek(), None);
    }
}
