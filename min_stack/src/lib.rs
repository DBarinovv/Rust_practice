pub struct MinStack {
    // Your code here
}

impl MinStack {
    pub fn new() -> Self {
        unimplemented!();
        // Your code here
    }

    pub fn push(&mut self, x: i32) {
        unimplemented!();
        // Your code here
    }

    pub fn pop(&mut self) -> Option<i32> {
        unimplemented!();
        // Your code here
    }

    pub fn top(&self) -> Option<i32> {
        unimplemented!();
        // Your code here
    }

    pub fn get_min(&self) -> Option<i32> {
        unimplemented!();
        // Your code here
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut stack = MinStack::new();
        stack.push(3);
        stack.push(1);
        stack.push(4);
        assert_eq!(stack.get_min(), Some(1));
        assert_eq!(stack.top(), Some(4));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.get_min(), Some(1));
    }
}
