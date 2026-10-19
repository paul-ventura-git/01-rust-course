pub fn run() {
    println!("--- 1. Basic ownership rules ---");

    let name = String::from("Rust");
    println!("The string is: {name}");

    // `name` owns the String. When `name` goes out of scope,
    // Rust automatically releases the allocated memory.
    println!("Ownership means each value has an owner.\n");
}
