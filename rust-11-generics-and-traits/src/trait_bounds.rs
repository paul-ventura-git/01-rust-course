use std::fmt::{Debug, Display};

fn compare_and_print<T: Debug + Display + PartialOrd>(left: &T, right: &T) {
    if left >= right {
        println!("{left:?} is at least {right:?}");
    } else {
        println!("{left:?} is less than {right:?}");
    }
}

fn describe<T>(value: &T)
where
    T: Debug + Display,
{
    println!("Display: {value}; Debug: {value:?}");
}

pub fn run() {
    println!("\n--- Trait bounds and where clauses ---");

    compare_and_print(&8, &5);
    describe(&String::from("Readable in two formats"));
}
