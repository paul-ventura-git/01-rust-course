fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn display_employee(name: &str, age: u32, salary: f64) {
    println!(
        "Name: {}, Age: {}, Salary: {:.2}",
        name, age, salary
    );
}

// Parameters are immutable by default.
// Use a mutable reference to modify the caller's variable.
fn increase_value(value: &mut i32) {
    *value += 10;
}

pub fn run() {
    println!("\n--- 2. FUNCTION PARAMETERS ---");

    let result = add(10, 20);
    println!("10 + 20 = {}", result);

    display_employee("Alice", 28, 3500.50);
    display_employee("Bob", 35, 4200.00);

    let mut number: i32 = 5;

    println!("Before modification: {}", number);

    increase_value(&mut number);

    println!("After modification: {}", number);
}