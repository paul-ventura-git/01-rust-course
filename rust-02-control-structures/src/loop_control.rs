pub fn run() {
    println!("\n--- 4. BREAK AND CONTINUE ---");

    // BREAK: terminate the loop
    println!("BREAK example:");

    for number in 1..=10 {
        if number == 5 {
            break;
        }

        println!("Number: {}", number);
    }

    // CONTINUE: skip the current iteration
    println!("\nCONTINUE example:");

    for number in 1..=5 {
        if number == 3 {
            continue;
        }

        println!("Number: {}", number);
    }

    // Skip even numbers
    println!("\nOdd numbers:");

    for number in 1..=10 {
        if number % 2 == 0 {
            continue;
        }

        println!("{}", number);
    }
}