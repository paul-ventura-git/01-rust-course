pub fn run() {
    println!("\n--- UTF-8 strings ---");

    let mut greeting = String::from("Hello");
    greeting.push(',');
    greeting.push_str(" Rust");
    greeting.push('!');
    println!("{greeting}");

    let combined = ["safe", "and", "owned"].join(" ");
    println!("Concatenated with join: {combined}");

    let multilingual = String::from("नमस्ते");
    println!(
        "The UTF-8 string has {} bytes and {} Unicode scalar values.",
        multilingual.len(),
        multilingual.chars().count()
    );
    for character in multilingual.chars() {
        print!("{character} ");
    }
    println!();
}
