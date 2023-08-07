### Circular Queue

A circular queue (or ring buffer) is a data structure that uses a single, fixed-size buffer as if it were connected end-to-end. The useful property of a circular queue is that it can efficiently use storage without having to perform costly resizing operations.

You will need to implement a circular queue using an array. It should support the following operations:

1. **`enqueue(&mut self, item: T) -> Result<(), &'static str>`**: Adds an item to the back of the queue. Return an `Err` if the queue is full.

2. **`dequeue(&mut self) -> Option<T>`**: Removes and returns the item at the front of the queue. Return `None` if the queue is empty.

3. **`peek(&self) -> Option<&T>`**: Returns a reference to the front item without removing it. Return `None` if the queue is empty.

4. **`is_empty(&self) -> bool`**: Returns `true` if the queue is empty, `false` otherwise.

5. **`is_full(&self) -> bool`**: Returns `true` if the queue is full, `false` otherwise.

Please implement the circular queue as a Rust struct and provide these methods.

```rust
pub struct CircularQueue<T> {
    // Your fields here
}

impl<T> CircularQueue<T> {
    pub fn new(size: usize) -> Self {
        // Your code here
    }

    pub fn enqueue(&mut self, item: T) -> Result<(), &'static str> {
        // Your code here
    }

    pub fn dequeue(&mut self) -> Option<T> {
        // Your code here
    }

    pub fn peek(&self) -> Option<&T> {
        // Your code here
    }

    pub fn is_empty(&self) -> bool {
        // Your code here
    }

    pub fn is_full(&self) -> bool {
        // Your code here
    }
}
```
