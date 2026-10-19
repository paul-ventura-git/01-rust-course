fn longer<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

pub fn run() {
    println!("\n--- Lifetime annotations on functions ---");

    let announcement = "The longer string is:";
    let result;
    {
        let first = String::from("borrowed");
        let second = "references";
        result = longer(first.as_str(), second);
        println!("{announcement} {result}");
    }
    println!("The result is used only while its borrowed input is valid.");
}
