const TAX_RATE: f64 = 0.18;

fn calculate_total(price: f64) -> f64 {
    let tax = price * TAX_RATE;
    let total = price + tax;

    total
}

pub fn run() {
    println!("\n--- 4. SCOPE AND VARIABLES ---");

    let course = "Rust";

    {
        let lesson = "Functions";

        println!("Course: {}", course);
        println!("Lesson: {}", lesson);
    }

    // `lesson` is not accessible outside its block.
    println!("Course outside the block: {}", course);

    // Shadowing: a new variable reuses the same name.
    let value = "100";
    println!("Original value: {}", value);

    let value: i32 = value.parse().expect("Invalid integer");
    println!("Parsed value: {}", value);

    let price = 200.0;
    let total = calculate_total(price);

    println!("Price: {:.2}", price);
    println!("Tax rate: {:.0}%", TAX_RATE * 100.0);
    println!("Total including tax: {:.2}", total);
}