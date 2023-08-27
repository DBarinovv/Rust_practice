pub fn add<T, U, V>(x: T, y: U) -> V
where
    T: Into<V>,
    U: Into<V>,
    V: std::ops::Add<Output = V>,
{
    x.into() + y.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x: u8 = 5;
        let y: u16 = 8;

        let z: u16 = add(x, y);
        assert_eq!(z, 13);
    }
}
