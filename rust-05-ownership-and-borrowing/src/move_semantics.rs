pub fn run() {
    println!("--- 2. Move semantics ---");

    let first = String::from("Hello, Rust!");
    let second = first; // Ownership moves from `first` to `second`.

    // `first` can no longer be used here because its ownership moved.
    println!("second = {second}");

    // Copy types such as integers behave differently:
    let x = 10;
    let y = x; // The integer value is copied.
    println!("x = {x}, y = {y}\n");
}
