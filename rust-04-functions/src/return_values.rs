fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn calculate_average(total: f64, count: u32) -> f64 {
    if count == 0 {
        return 0.0;
    }

    total / count as f64
}

fn get_status(score: i32) -> &'static str {
    if score >= 60 {
        "Passed"
    } else {
        "Failed"
    }
}

fn get_min_max(numbers: &[i32]) -> Option<(i32, i32)> {
    if numbers.is_empty() {
        return None;
    }

    let mut min = numbers[0];
    let mut max = numbers[0];

    for &number in numbers {
        if number < min {
            min = number;
        }

        if number > max {
            max = number;
        }
    }

    Some((min, max))
}

pub fn run() {
    println!("\n--- 3. FUNCTION RETURN VALUES ---");

    let product = multiply(6, 7);
    println!("6 * 7 = {}", product);

    let average = calculate_average(450.0, 5);
    println!("Average: {:.2}", average);

    let status = get_status(75);
    println!("Student status: {}", status);

    let numbers = [12, 5, 30, 8, 17];

    match get_min_max(&numbers) {
        Some((min, max)) => {
            println!("Minimum: {}", min);
            println!("Maximum: {}", max);
        }
        None => println!("The array is empty."),
    }
}