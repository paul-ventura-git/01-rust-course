pub fn run() {
    println!("\n--- 4. TYPE CONVERSION ---");

    // Integer to floating-point
    let integer: i32 = 10;
    let decimal: f64 = integer as f64;

    println!("Integer: {}", integer);
    println!("As floating-point: {}", decimal);

    // Floating-point to integer
    // The fractional part is truncated.
    let price: f64 = 19.85;
    let rounded_down: i32 = price as i32;

    println!("Original price: {}", price);
    println!("Converted to integer: {}", rounded_down);

    // String to integer
    let number_text: &str = "123";

    match number_text.parse::<i32>() {
        Ok(number) => println!("Parsed integer: {}", number),
        Err(error) => println!("Parsing error: {}", error),
    }

    // String to floating-point
    let decimal_text: &str = "45.75";

    match decimal_text.parse::<f64>() {
        Ok(number) => println!("Parsed decimal: {}", number),
        Err(error) => println!("Parsing error: {}", error),
    }

    // Invalid conversion
    let invalid_text: &str = "hello";

    match invalid_text.parse::<i32>() {
        Ok(number) => println!("Parsed number: {}", number),
        Err(_) => println!("'hello' is not a valid integer."),
    }

    // Convert a number to text
    let quantity: i32 = 50;
    let quantity_text: String = quantity.to_string();

    println!("Quantity as text: {}", quantity_text);
}