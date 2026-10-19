// Function without parameters or return value
fn greet() {
    println!("Hello from Rust!");
}

// Function with a parameter
fn greet_user(name: &str) {
    println!("Hello, {}!", name);
}

// Function with multiple statements
fn display_course_info() {
    println!("Course: Rust Programming");
    println!("Lesson: Functions");
    println!("Level: Beginner");
}

pub fn run() {
    println!("\n--- 1. BASIC FUNCTIONS ---");

    greet();

    greet_user("Alice");
    greet_user("Bob");

    display_course_info();
}