use rayon::prelude::*;

pub fn matrix_multiplication(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
    assert_eq!(a.len(), b.len());
    assert_eq!(a[0].len(), b[0].len());
    let n = a.len();

    (0..n)
        .into_par_iter()
        .map(|i| {
            (0..n)
                .map(|j| (0..n).map(|k| a[i][k] * b[k][j]).sum())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let a = vec![vec![1, 2], vec![3, 4]];

        let b = vec![vec![5, 6], vec![7, 8]];

        let result = matrix_multiplication(&a, &b);
        assert_eq!(result, vec![vec![19, 22], vec![43, 50],]);
    }

    #[test]
    fn test_identity() {
        let a = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];

        let b = vec![vec![5, 2, 1], vec![9, 4, 8], vec![7, 6, 3]];

        let result = matrix_multiplication(&a, &b);
        assert_eq!(result, b);
    }

    #[test]
    fn test_zeros() {
        let a = vec![vec![0, 0], vec![0, 0]];

        let b = vec![vec![4, 3], vec![2, 1]];

        let result = matrix_multiplication(&a, &b);
        assert_eq!(result, vec![vec![0, 0], vec![0, 0],]);
    }

    #[test]
    #[should_panic]
    fn test_non_square() {
        let a = vec![vec![1, 2]];

        let b = vec![vec![3], vec![4]];

        matrix_multiplication(&a, &b);
    }
}
