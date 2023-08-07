### MULTIPLY MATRICES


Write a function in Rust that takes two matrices and returns their product. You can assume that the matrices will be of the correct size for multiplication, that is, the number of columns of the first matrix is equal to the number of rows of the second matrix.

#### Prototype:

```rust
fn multiply_matrices(mat1: Vec<Vec<f64>>, mat2: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // your code is here
}
```

#### Input data examples:

```rust
let mat1 = vec![
    vec![1.0, 2.0],
    vec![3.0, 4.0]
];
let mat2 = vec![
    vec![5.0, 6.0],
    vec![7.0, 8.0]
];
```

```rust
let result = vec![
    vec![19.0, 22.0],
    vec![43.0, 50.0]
];
```
