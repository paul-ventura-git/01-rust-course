fn character_count(text: &String) -> usize {
    text.len()
}

pub fn run() {
    println!("--- 4. Immutable borrowing ---");

    let message = String::from("Borrowing does not take ownership");
    let length = character_count(&message); // Pass a reference, not ownership.

    println!("Message: {message}");
    println!("Length in bytes: {length}");
    println!("The function borrowed the string and did not take ownership.\n");
}
