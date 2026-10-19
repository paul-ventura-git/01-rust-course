pub fn run() {
    println!("\n--- 1. SCALAR TYPES ---");

    // Integer types
    let age: i32 = 25;
    let population: u64 = 1_000_000;

    // Floating-point types
    let price: f64 = 19.99;
    let temperature: f32 = 23.5;

    // Boolean type
    let is_active: bool = true;

    // Character type: a single Unicode scalar value
    let initial: char = 'R';
    let symbol: char = '€';

    println!("Age: {}", age);
    println!("Population: {}", population);
    println!("Price: {:.2}", price);
    println!("Temperature: {}", temperature);
    println!("Active: {}", is_active);
    println!("Initial: {}", initial);
    println!("Symbol: {}", symbol);

    // Arithmetic with integers
    let a: i32 = 10;
    let b: i32 = 3;

    println!("Addition: {}", a + b);
    println!("Subtraction: {}", a - b);
    println!("Multiplication: {}", a * b);
    println!("Integer division: {}", a / b);
    println!("Remainder: {}", a % b);

    // Conditional expression using a scalar value
    if age >= 18 {
        println!("The person is an adult.");
    } else {
        println!("The person is a minor.");
    }
}