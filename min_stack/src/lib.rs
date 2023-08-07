use std::collections::VecDeque;

pub struct MinStack<T: Ord + Clone> {
    stack: VecDeque<T>,
    mins: VecDeque<T>,
}

impl<T: Ord + Clone> MinStack<T> {
    pub fn new() -> Self {
        MinStack {
            stack: VecDeque::<T>::new(),
            mins: VecDeque::<T>::new(),
        }
    }

    pub fn push(&mut self, x: T) {
        if self.mins.is_empty() || x <= *self.mins.back().unwrap() {
            self.mins.push_back(x.clone());
        }

        self.stack.push_back(x);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        let popped = self.stack.pop_back();
        if popped.as_ref() == self.mins.back() {
            self.mins.pop_back();
        }
        popped
    }
    

    pub fn top(&self) -> Option<&T> {
        self.stack.back()
    }

    pub fn get_min(&self) -> Option<&T> {
        self.mins.back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_stack() {
        let mut stack: MinStack<i32> = MinStack::new();
        assert_eq!(stack.top(), None);
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.get_min(), None);
    }

    #[test]
    fn test_single_element() {
        let mut stack = MinStack::new();
        stack.push(5);
        assert_eq!(stack.top(), Some(&5));
        assert_eq!(stack.get_min(), Some(&5));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.get_min(), None);
    }

    #[test]
    fn test_duplicate_minimum() {
        let mut stack = MinStack::new();
        stack.push(3);
        stack.push(1);
        stack.push(1);
        assert_eq!(stack.get_min(), Some(&1));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.get_min(), Some(&1));
    }

    #[test]
    fn test_push_pop_sequence() {
        let mut stack = MinStack::new();
        stack.push(4);
        stack.push(2);
        stack.push(8);
        stack.push(1);
        assert_eq!(stack.get_min(), Some(&1));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.get_min(), Some(&2));
        assert_eq!(stack.pop(), Some(8));
        assert_eq!(stack.get_min(), Some(&2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.get_min(), Some(&4));
    }

    #[test]
    fn test_mixed_types() {
        let mut stack = MinStack::new();
        stack.push("z");
        stack.push("b");
        stack.push("a");
        assert_eq!(stack.get_min(), Some(&"a"));
        assert_eq!(stack.top(), Some(&"a"));
    }
}
