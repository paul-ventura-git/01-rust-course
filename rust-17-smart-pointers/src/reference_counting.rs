use std::rc::{Rc, Weak};

pub fn run() {
    println!("\n--- Rc<T> and Weak<T> shared ownership ---");

    let shared = Rc::new(String::from("shared data"));
    let second_owner = Rc::clone(&shared);
    let observer: Weak<String> = Rc::downgrade(&shared);

    println!("Strong owners: {}", Rc::strong_count(&shared));
    println!("Weak observers: {}", Rc::weak_count(&shared));
    println!("Observer can read: {:?}", observer.upgrade());

    drop(second_owner);
    drop(shared);
    println!(
        "After owners are dropped, observer upgrades to: {:?}",
        observer.upgrade()
    );
}
