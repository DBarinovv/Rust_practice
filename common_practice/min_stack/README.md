### MinStack

Create a "MinStack" data structure that supports the following operations:

1. **push(x)**: Add an element `x` to the top of the stack.
2. **pop()**: Remove the element at the top of the stack. If the stack is empty, return `None`.
3. **top()**: Return the element at the top of the stack without removing it. If the stack is empty, return `None`.
4. **get_min()**: Return the minimum element in the stack without removing it. If the stack is empty, return `None`.

You are required to implement all these operations with \(O(1)\) time complexity.

#### Structure Prototype:

```rust
pub struct MinStack {
    // Your code here
}

impl MinStack {
    pub fn new() -> Self {
        // Your code here
    }

    pub fn push(&mut self, x: i32) {
        // Your code here
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Your code here
    }

    pub fn top(&self) -> Option<i32> {
        // Your code here
    }

    pub fn get_min(&self) -> Option<i32> {
        // Your code here
    }
}
```

#### Usage Example:

```rust
let mut stack = MinStack::new();
stack.push(3);
stack.push(1);
stack.push(4);
assert_eq!(stack.get_min(), Some(1));
assert_eq!(stack.top(), Some(4));
assert_eq!(stack.pop(), Some(4));
assert_eq!(stack.get_min(), Some(1));
```
