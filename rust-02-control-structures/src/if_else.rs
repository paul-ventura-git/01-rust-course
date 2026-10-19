pub fn run() {
    println!("\n--- 1. IF, ELSE IF, ELSE ---");

    let age: i32 = 20;

    if age < 13 {
        println!("You are a child.");
    } else if age < 18 {
        println!("You are a teenager.");
    } else {
        println!("You are an adult.");
    }

    // IF as an expression
    let number: i32 = 7;

    let result = if number % 2 == 0 {
        "even"
    } else {
        "odd"
    };

    println!("The number {} is {}.", number, result);

    // Multiple conditions
    let temperature: i32 = 25;
    let raining: bool = false;

    if temperature > 20 && !raining {
        println!("The weather is suitable for a walk.");
    } else {
        println!("Consider other activities.");
    }
}