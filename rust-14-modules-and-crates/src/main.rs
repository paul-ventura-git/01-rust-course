use rust_14_modules_and_crates::{hosting, module_paths, restaurant};

fn main() {
    println!("=======================================");
    println!(" LESSON 14: MODULES AND CRATES");
    println!("=======================================");

    restaurant::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    module_paths::show_visibility();

    println!("\nLesson completed successfully!");
    println!("This package has a library crate and a binary crate.");
}
