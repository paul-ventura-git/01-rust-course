fn calculate_length(text: &str) -> usize {
    text.len()
}

fn append_suffix(text: &mut String, suffix: &str) {
    text.push_str(suffix);
}

pub fn run() {
    println!("\n--- Shared and mutable references ---");

    let mut message = String::from("References");
    let length = calculate_length(&message);
    println!("'{message}' has {length} bytes.");

    append_suffix(&mut message, " are borrowed");
    println!("After a mutable borrow: {message}");

    let first = &message;
    let second = &message;
    println!("Two shared references can coexist: '{first}' and '{second}'.");

    let mutable = &mut message;
    mutable.push('!');
    println!("A mutable reference is exclusive while it is in use: {mutable}");
}
