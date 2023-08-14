### Exercise 1: Immutable and Mutable Borrows
Write a function `update_and_print` that takes a mutable reference to an integer and increments it by one. Then, print the value both inside and outside the function.

### Exercise 2: Borrowing and Function Calls
Create a struct `Person` with fields `name` and `age`. Write two functions:
- `birthday`: Takes a mutable reference to a `Person` and increases their age by one.
- `greet`: Takes an immutable reference to a `Person` and prints a greeting including their name.

Create a `Person` instance and call both functions to test.

### Exercise 3: Borrowing and Iteration
Create a vector of integers and write a loop that prints each value and its square. Do this without cloning the vector or any of its elements. Hint: consider using `.iter()` method.

### Exercise 4: Borrow Checker Challenge
Here's a tricky one: Write a function that takes a mutable reference to a vector of integers and sorts the even numbers in descending order while leaving the odd numbers in their original places.

For example:
```rust
let mut nums = vec![3, 4, 2, 5, 8, 7];
sort_evens(&mut nums);
assert_eq!(nums, vec![3, 8, 4, 5, 2, 7]);
```
Hint: This will require careful handling of mutable and immutable borrows.
