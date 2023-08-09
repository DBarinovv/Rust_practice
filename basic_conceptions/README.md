#### Exercise 1: Ownership

Create a function `take_ownership` that takes ownership of a string and prints it. Test this function by passing a string to it.

```rust
fn main() {
    let s = String::from("Rust is fun!");
    take_ownership(s);
    // Try to print s here and see what happens
}

fn take_ownership(s: String) {
    // Print the string here
}
```

#### Exercise 2: Borrowing

Modify the previous exercise to use borrowing instead of taking ownership. Create a function `borrow_string` that borrows a string and prints it without taking ownership.

```rust
fn main() {
    let s = String::from("Rust is great!");
    borrow_string(&s);
    // Print s again here to show that ownership was not taken
}

fn borrow_string(s: &String) {
    // Print the string here
}
```

#### Exercise 3: Lifetimes

Write a function `longest_word` that takes two string slices and returns a reference to the longest one. Make sure to annotate the lifetimes correctly.

```rust
fn main() {
    let s1 = "Rust programming";
    let s2 = "is awesome!";
    let result = longest_word(s1, s2);
    println!("The longest word is: {}", result);
}

fn longest_word<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Return the longest string
}
```
