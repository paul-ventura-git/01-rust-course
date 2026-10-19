pub fn run() {
    println!("--- 3. Cloning a value ---");

    let original = String::from("Ownership");
    let copy = original.clone(); // Explicitly duplicates the String data.

    println!("original = {original}");
    println!("copy     = {copy}");
    println!("Both strings can be used because clone() created a separate value.\n");
}
