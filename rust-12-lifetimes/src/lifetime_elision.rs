static COURSE_NAME: &str = "Rust";

fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn static_name() -> &'static str {
    COURSE_NAME
}

pub fn run() {
    println!("\n--- Lifetime elision and 'static ---");

    let title = String::from("Lifetimes make borrowing explicit");
    println!("First word: {}", first_word(&title));
    println!("A string with a program-long lifetime: {}", static_name());
    println!("Annotations describe valid relationships; they do not extend lifetimes.");
}
