fn take_ownership(value: String) {
    println!("Inside take_ownership(): {value}");
} // `value` goes out of scope here.

fn give_back(value: String) -> String {
    value // Return ownership to the caller.
}

pub fn run() {
    println!("--- 6. Ownership and functions ---");

    let first = String::from("A value");
    take_ownership(first);
    // `first` is no longer usable here because ownership was transferred.

    let second = String::from("Returned value");
    let second = give_back(second);
    println!("Ownership returned to main: {second}\n");
}
