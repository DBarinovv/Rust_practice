// fn main() {
//     let s = String::from("Rust is fun!");
//     take_ownership(s);
//     // println!("s = {}", s);
// }

// fn take_ownership(s: String) {
//     println!("s = {}", s);
// }

//=======================================================

// fn main() {
//     let s = String::from("Rust is great!");
//     borrow_string(&s);
//     println!("s = {}", s);
// }

// fn borrow_string(s: &String) {
//     println!("s = {}", s);
// }

//=======================================================

// fn main() {
//     let s1 = "Rust programming";
//     let s2 = "is awesome!";
//     let result = longest_word(s1, s2);
//     println!("The longest word is: {}", result);
// }

// fn longest_word<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         return x;
//     }

//     y
// }
