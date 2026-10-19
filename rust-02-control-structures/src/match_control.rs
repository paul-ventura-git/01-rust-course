pub fn run() {
    println!("\n--- 2. MATCH ---");

    let day: i32 = 3;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    // Match with ranges
    let score: i32 = 85;

    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        0..=59 => "F",
        _ => "Invalid score",
    };

    println!("Score: {}, grade: {}", score, grade);
}