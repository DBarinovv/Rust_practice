pub fn update_and_print(x: &mut u8) {
    println!("x inside was = {x}");
    *x += 1;
    println!("x inside now = {x}");
}

//=============================================

pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    pub fn birthday(&mut self) {
        self.age += 1;
    }

    pub fn greet(&self) {
        println!("Hello {}", self.name);
    }
}

//=============================================

pub trait Square<T> {
    type Output;
    fn square(&self) -> Self::Output;
}

impl<T, K> Square<T> for K
where
    K: std::ops::Mul,
    // <K as std::ops::Mul>::Output: std::fmt::Debug,
{
    type Output = <K as std::ops::Mul>::Output;
    fn square(&self) -> Self::Output {
        unimplemented!()
        // *self * *self // still need Copy trait
    }
}

pub fn vec_loop<T>(v: &[T])
where
    T: std::fmt::Debug + std::ops::Mul + Square<T>,
    <T as Square<T>>::Output: std::fmt::Debug,
{
    v.iter()
        .for_each(|elem| println!("{:?}, {:?}", elem, elem.square()));
}

//=============================================

pub fn sort_evens(v: &mut [i32]) {
    v.sort_by(|a, b| match (a % 2 == 0, b % 2 == 0) {
        (true, true) => a.cmp(b),
        _ => std::cmp::Ordering::Equal,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_and_print() {
        let mut x = 5;
        println!("x outside was = {x}");
        update_and_print(&mut x);
        println!("x outside now = {x}");
    }

    #[test]
    fn test_person() {
        let mut person = Person::new("Katya".to_owned(), 5);

        assert_eq!(person.age, 5);
        person.birthday();
        assert_eq!(person.age, 6);

        person.greet();
    }

    #[test]
    fn test_empty() {
        let mut v: Vec<i32> = vec![];
        sort_evens(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn test_all_odd() {
        let mut v = vec![1, 3, 5, 7, 9];
        sort_evens(&mut v);
        assert_eq!(v, vec![1, 3, 5, 7, 9]); // No change
    }

    #[test]
    fn test_all_even() {
        let mut v = vec![8, 6, 4, 2];
        sort_evens(&mut v);
        assert_eq!(v, vec![2, 4, 6, 8]); // Sorted
    }

    #[test]
    fn test_mixed() {
        let mut v = vec![3, 2, 1, 6, 4, 5];
        sort_evens(&mut v);
        assert_eq!(v, vec![3, 2, 1, 4, 6, 5]); // Only even numbers sorted
    }

    #[test]
    fn test_duplicates() {
        let mut v = vec![2, 4, 2, 4];
        sort_evens(&mut v);
        assert_eq!(v, vec![2, 2, 4, 4]); // Sorted, including duplicates
    }
}
