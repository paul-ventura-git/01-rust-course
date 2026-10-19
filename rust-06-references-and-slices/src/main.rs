mod references;
mod slices;
mod string_slices;

fn main() {
    println!("==========================================");
    println!(" LESSON 06: REFERENCES AND SLICES");
    println!("==========================================");

    references::run();
    slices::run();
    string_slices::run();

    println!("\nLesson completed successfully!");
}
