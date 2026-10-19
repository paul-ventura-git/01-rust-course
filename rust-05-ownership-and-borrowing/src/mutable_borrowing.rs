fn add_exclamation(text: &mut String) {
    text.push('!');
}

pub fn run() {
    println!("--- 5. Mutable borrowing ---");

    let mut message = String::from("Hello");
    add_exclamation(&mut message);

    println!("{message}");

    // Rule of thumb: while a mutable reference is active, do not create
    // other references to the same value at the same time.
    println!("A mutable reference allows the borrowed value to be changed.\n");
}
