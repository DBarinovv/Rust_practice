fn main() {
    // Print the path to the C library
    println!("cargo:rustc-link-search=./");
    println!("cargo:rustc-link-lib=dylib=simple_math");
}
