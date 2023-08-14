mod bindings;

fn main() {
    unsafe {
        let result = bindings::add(3, 4);
        println!("3 + 4 = {}", result);
    }
}
