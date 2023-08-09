fn main() {
    let s = String::from("Rust is fun!");
    take_ownership(s);
    // Try to print s here and see what happens
}

fn take_ownership(s: String) {
    // Print the string here
}

//=======================================================

// fn main() {
//     let s = String::from("Rust is great!");
//     borrow_string(&s);
//     // Print s again here to show that ownership was not taken
// }

// fn borrow_string(s: &String) {
//     // Print the string here
// }

//=======================================================

// fn main() {
//     let s1 = "Rust programming";
//     let s2 = "is awesome!";
//     let result = longest_word(s1, s2);
//     println!("The longest word is: {}", result);
// }

// fn longest_word<'a>(x: &'a str, y: &'a str) -> &'a str {
//     // Return the longest string
// }
