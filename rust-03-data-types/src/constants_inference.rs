const MAX_USERS: u32 = 100;
const COURSE_NAME: &str = "Rust Programming";

pub fn run() {
    println!("\n--- 5. CONSTANTS AND TYPE INFERENCE ---");

    // Type inference
    let age = 25;          // Inferred as i32
    let price = 29.99;     // Inferred as f64
    let active = true;     // Inferred as bool
    let language = "Rust"; // Inferred as &str

    println!("Age: {}", age);
    println!("Price: {}", price);
    println!("Active: {}", active);
    println!("Language: {}", language);

    // Constants
    println!("Course: {}", COURSE_NAME);
    println!("Maximum users: {}", MAX_USERS);

    // Explicit type annotation
    let quantity: u32 = 15;
    println!("Quantity: {}", quantity);

    // Variable shadowing
    let value = "42";
    println!("Original value: {}", value);

    let value: i32 = value.parse().expect("Expected a valid integer");

    println!("Converted value: {}", value);

    // Conditional expression
    let status = if quantity <= MAX_USERS {
        "Within capacity"
    } else {
        "Capacity exceeded"
    };

    println!("Status: {}", status);
}