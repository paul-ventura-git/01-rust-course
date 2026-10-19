pub fn run() {
    println!("\n--- 3. STRINGS ---");

    // String slice
    let language: &str = "Rust";

    // Owned, growable string
    let mut message: String = String::from("Hello");

    println!("Language: {}", language);
    println!("Message: {}", message);

    // Append text
    message.push_str(", world!");
    println!("Updated message: {}", message);

    // Concatenate strings
    let first_name = String::from("Alice");
    let last_name = String::from("Smith");

    let full_name = format!("{} {}", first_name, last_name);

    println!("Full name: {}", full_name);

    // String length in bytes
    let text = String::from("Programming");

    println!("Text: {}", text);
    println!("Length in bytes: {}", text.len());

    // Convert a number to a string
    let age: i32 = 25;
    let age_text: String = age.to_string();

    println!("Age as text: {}", age_text);

    // Check whether a string contains a substring
    if full_name.contains("Alice") {
        println!("The name contains Alice.");
    }
}