use std::collections::HashMap;

pub fn run() {
    println!("\n--- Hash maps ---");

    let teams = [(String::from("blue"), 10), (String::from("red"), 15)];
    let mut scores: HashMap<String, i32> = teams.into_iter().collect();
    scores.insert(String::from("green"), 8);

    if let Some(score) = scores.get("blue") {
        println!("Blue team score: {score}");
    }

    scores.entry(String::from("blue")).or_insert(0);
    scores.entry(String::from("yellow")).or_insert(0);

    for (team, score) in &scores {
        println!("{team}: {score}");
    }
}
