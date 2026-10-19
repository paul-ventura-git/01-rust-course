mod scalar_types;
mod compound_types;
mod strings;
mod type_conversion;
mod constants_inference;

fn main() {
    println!("====================================");
    println!(" LESSON 03: RUST DATA TYPES");
    println!("====================================");

    scalar_types::run();
    compound_types::run();
    strings::run();
    type_conversion::run();
    constants_inference::run();

    println!("\nLesson completed successfully!");
}