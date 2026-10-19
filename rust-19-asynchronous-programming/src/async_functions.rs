async fn load_configuration() -> &'static str {
    "configuration loaded"
}

async fn calculate_value() -> u32 {
    6 * 7
}

pub async fn run() -> String {
    println!("\n--- async functions and await ---");

    let configuration = load_configuration().await;
    let value = calculate_value().await;
    format!("{configuration}; calculated value is {value}")
}
