mod closures;
mod custom_iterator;
mod iterator_adaptors;
mod iterators;

fn main() {
    println!("===========================================");
    println!(" LESSON 13: ITERATORS AND CLOSURES");
    println!("===========================================");

    closures::run();
    iterators::run();
    iterator_adaptors::run();
    custom_iterator::run();

    println!("\nLesson completed successfully!");
}
