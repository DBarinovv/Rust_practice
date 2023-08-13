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

pub fn vec_loop<T>(v: &Vec<T>)
where
    T: std::fmt::Debug + std::ops::Mul + Square<T>,
    <T as Square<T>>::Output: std::fmt::Debug,
{
    v.iter()
        .for_each(|elem| println!("{:?}, {:?}", elem, elem.square()));
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
}
