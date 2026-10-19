#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Point(i32, i32, i32);

pub fn run() {
    println!("\n--- Defining and creating structs ---");

    let user = User {
        username: String::from("rustacean"),
        email: String::from("rust@example.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("User: {user:?}");
    println!("{} is active: {}", user.username, user.active);
    println!("Contact {} ({} sign-in).", user.email, user.sign_in_count);

    let orange = Color(255, 128, 0);
    let origin = Point(0, 0, 0);
    println!(
        "Tuple struct color: {orange:?}; RGB: ({}, {}, {})",
        orange.0, orange.1, orange.2
    );
    println!(
        "Tuple struct point: {origin:?}; coordinates: ({}, {}, {})",
        origin.0, origin.1, origin.2
    );
}
