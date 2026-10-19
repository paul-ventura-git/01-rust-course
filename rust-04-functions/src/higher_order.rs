// Accepts a function as an argument.
fn apply_operation(
    a: i32,
    b: i32,
    operation: fn(i32, i32) -> i32,
) -> i32 {
    operation(a, b)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Accepts a closure through the Fn trait.
fn calculate<F>(number: i32, operation: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operation(number)
}

pub fn run() {
    println!("\n--- 5. HIGHER-ORDER FUNCTIONS ---");

    let sum = apply_operation(10, 5, add);
    let product = apply_operation(10, 5, multiply);

    println!("Sum: {}", sum);
    println!("Product: {}", product);

    // Pass a closure directly.
    let doubled = calculate(8, |number| number * 2);
    let squared = calculate(8, |number| number * number);

    println!("Doubled: {}", doubled);
    println!("Squared: {}", squared);

    // Use a closure to filter values.
    let numbers = [1, 2, 3, 4, 5, 6];

    let even_numbers: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|number| number % 2 == 0)
        .collect();

    println!("Even numbers: {:?}", even_numbers);
}