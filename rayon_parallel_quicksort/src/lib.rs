use rayon::prelude::*;

pub fn parallel_quicksort<T: Ord + Send + Copy + std::fmt::Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    let hi = &mut hi[1..];
    rayon::join(|| parallel_quicksort(lo), || parallel_quicksort(hi));
}

fn partition<T: Ord + Copy>(v: &mut [T]) -> usize {
    let pivot_index = v.len() / 2;
    v.swap(pivot_index, v.len() - 1); // Move pivot to the end

    let mut ind = 0;
    let mut last_ind = v.len() - 1;
    let pivot = v[last_ind];

    while ind < last_ind {
        if v[ind] >= pivot {
            v.swap(ind, last_ind - 1); // Swap with the element before the pivot
            last_ind -= 1;
        } else {
            ind += 1;
        }
    }

    v.swap(ind, v.len() - 1); // Move pivot to its final place
    ind
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut v: Vec<i32> = vec![];
        parallel_quicksort(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut v = vec![42];
        parallel_quicksort(&mut v);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn test_two_elements() {
        let mut v = vec![55, 22];
        parallel_quicksort(&mut v);
        assert_eq!(v, vec![22, 55]);
    }

    #[test]
    fn test_reversed() {
        let mut v = vec![5, 4, 3, 2, 1];
        parallel_quicksort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random() {
        let mut v = vec![10, 40, 30, 60, 20, 50];
        parallel_quicksort(&mut v);
        assert_eq!(v, vec![10, 20, 30, 40, 50, 60]);
    }

    #[test]
    fn test_duplicates() {
        let mut v = vec![10, 20, 10, 30, 20, 10];
        parallel_quicksort(&mut v);
        assert_eq!(v, vec![10, 10, 10, 20, 20, 30]);
    }

    #[test]
    fn test_large_random() {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let mut v: Vec<u32> = (0..1000).map(|_| rng.gen()).collect();
        let mut expected = v.clone();
        parallel_quicksort(&mut v);
        expected.sort();
        assert_eq!(v, expected);
    }
}
