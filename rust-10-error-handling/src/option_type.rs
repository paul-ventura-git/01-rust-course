fn find_user(id: u32) -> Option<&'static str> {
    match id {
        1 => Some("Ada"),
        2 => Some("Grace"),
        _ => None,
    }
}

pub fn run() {
    println!("\n--- Option<T> for optional values ---");

    let name = find_user(1).unwrap_or("Guest");
    println!("User 1: {name}");

    match find_user(99) {
        Some(name) => println!("Found {name}."),
        None => println!("No user has that ID."),
    }

    let display_name = find_user(2).map(|name| name.to_uppercase());
    println!("Mapped optional name: {display_name:?}");
}
