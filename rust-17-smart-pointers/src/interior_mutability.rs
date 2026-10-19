use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    println!("\n--- RefCell<T> interior mutability ---");

    let shared_log = Rc::new(RefCell::new(Vec::new()));
    let writer_a = Rc::clone(&shared_log);
    let writer_b = Rc::clone(&shared_log);

    writer_a.borrow_mut().push(String::from("first message"));
    writer_b.borrow_mut().push(String::from("second message"));
    println!("Shared log: {:?}", shared_log.borrow());
    println!("RefCell checks borrowing rules at runtime.");
}
