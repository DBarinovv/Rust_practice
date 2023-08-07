pub fn multiply_matrices(mat1: Vec<Vec<f64>>, mat2: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let rows = mat1.len();
    let cols = mat2[0].len();

    if rows != cols {
        panic!("Number of columns in mat1 must be equal to the number of rows in mat2");
    }
    let mut res = vec![vec![0.0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            res[i][j] = (0..mat1[0].len()).map(|k| mat1[i][k] * mat2[k][j]).sum();

            // let mut val = 0.0;

            // for k in 0..mat1[0].len() {
            //     val += mat1[i][k] * mat2[k][j];
            // }

            // res[i][j] = val;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square() {
        let mat1 = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let mat2 = vec![vec![5.0, 6.0], vec![7.0, 8.0]];

        let mult = multiply_matrices(mat1, mat2);
        assert_eq!(mult, (vec![vec![19.0, 22.0], vec![43.0, 50.0],]));
    }

    #[test]
    fn rectabgle() {
        let mat1 = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];

        let mat2 = vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]];

        let result = multiply_matrices(mat1, mat2);
        assert_eq!(result, vec![vec![58.0, 64.0], vec![139.0, 154.0]]);
    }
}
