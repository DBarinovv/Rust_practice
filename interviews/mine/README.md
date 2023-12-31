## Task: Number Associative Mining and Trading Inc (NAMT)

### Imaginary:
Welcome to Number Associative Mining and Trading Inc, or NAMT for short! Your duties are to make sure our turtles, working in difficult environments, are making correct associations, and stopping their operations when the tunnel is about to crumble. For that, our mathematicians finally found with absolute certainty the pattern the numbers mined follow before going too far.

### The Problem:
Here's how the mathematicians have written the solution, they did write it in understandable words for once:
> "The first 100 numbers of the mine are always secure, but after that, the next number is only safe if it is the sum of 2 numbers in the previous 100"

### Examples:
In that case, the number 127 will make the mine crumble, as there is no 2-number combination in the last 5 numbers that sum to 127.
```rust
#[test]
fn window_5() {
    let nums = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
    let window_size = 5;
    assert_eq!(find_unstable_number(nums, window_size), 127); // Expected Output: 127
}
```
### Requirements
Find ways to make it as fast as possible, while also keeping it as efficient as possible.
