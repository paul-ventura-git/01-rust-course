pub fn run() {
    println!("\n--- 2. COMPOUND TYPES ---");

    // Tuple: values can have different types
    let employee: (&str, i32, f64) = ("Alice", 101, 3500.50);

    println!("Employee name: {}", employee.0);
    println!("Employee ID: {}", employee.1);
    println!("Employee salary: {:.2}", employee.2);

    // Tuple destructuring
    let (name, id, salary) = employee;

    println!(
        "Employee: {}, ID: {}, Salary: {:.2}",
        name, id, salary
    );

    // Array: elements have the same type and fixed length
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];

    println!("First number: {}", numbers[0]);
    println!("Array length: {}", numbers.len());

    // Iterate over the array
    let mut total: i32 = 0;

    for number in &numbers {
        total += *number;
    }

    println!("Sum of numbers: {}", total);

    // Create an array with repeated values
    let zeros: [i32; 3] = [0; 3];

    println!("First zero: {}", zeros[0]);
    println!("Number of zeros: {}", zeros.len());
}