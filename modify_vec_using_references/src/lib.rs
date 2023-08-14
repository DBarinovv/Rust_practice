#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![1, 4, 3, 2, 5];
        let mut k: Vec<&mut i32> = Vec::new();

        // Iterate over mutable references to elements in `v`
        for x in &mut v {
            if *x % 2 == 0 {
                k.push(x);
            }
        }

        for elem in &mut k {
            **elem *= 2;
        }

        assert_eq!(v, [1, 8, 3, 4, 5]);
    }
}
