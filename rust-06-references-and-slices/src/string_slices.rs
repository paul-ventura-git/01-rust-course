fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

pub fn run() {
    println!("\n--- String slices ---");

    let sentence = String::from("Rust slices borrow part of a string");
    let word: &str = first_word(&sentence);
    println!("First word: '{word}'");
    println!(
        "String literals are also string slices: '{}'.",
        first_word("hello world")
    );

    let rust = &sentence[0..4];
    println!("A byte-range slice: '{rust}'");
    println!("Use character boundaries when slicing UTF-8 strings.");
}
