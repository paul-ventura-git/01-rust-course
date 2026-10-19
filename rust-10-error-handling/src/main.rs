mod custom_errors;
mod option_type;
mod result_type;

fn main() {
    println!("======================================");
    println!(" LESSON 10: ERROR HANDLING");
    println!("======================================");

    option_type::run();
    result_type::run();
    custom_errors::run();

    println!("\nLesson completed successfully!");
}
