pub struct CircularQueue<T> {
    // Your fields here
}

impl<T> CircularQueue<T> {
    pub fn new(size: usize) -> Self {
        unimplemented!();
        // Your code here
    }

    pub fn enqueue(&mut self, item: T) -> Result<(), &'static str> {
        unimplemented!();
        // Your code here
    }

    pub fn dequeue(&mut self) -> Option<T> {
        unimplemented!();
        // Your code here
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!();
        // Your code here
    }

    pub fn is_empty(&self) -> bool {
        unimplemented!();
        // Your code here
    }

    pub fn is_full(&self) -> bool {
        unimplemented!();
        // Your code here
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
