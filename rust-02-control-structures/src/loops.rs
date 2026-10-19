// loops.rs

pub fn run() {
    println!("\n--- 3. LOOPS ---");

    // FOR loop: iterate over an array
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];

    println!("FOR loop:");

    for number in numbers {
        println!("Number: {}", number);
    }

    // WHILE loop: repeat while a condition is true
    println!("\nWHILE loop:");

    let mut counter: i32 = 1;

    while counter <= 3 {
        println!("Counter: {}", counter);
        counter += 1;
    }

    // LOOP: repeat indefinitely until stopped
    println!("\nLOOP:");

    let mut value: i32 = 1;

    loop {
        println!("Value: {}", value);

        if value == 3 {
            break;
        }

        value += 1;
    }
}