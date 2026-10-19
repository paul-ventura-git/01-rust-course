#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn sum(list: &List) -> i32 {
    match list {
        List::Cons(value, next) => value + sum(next),
        List::Nil => 0,
    }
}

pub fn run() {
    println!("\n--- Box<T> and heap allocation ---");

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("A recursive list: {list:?}");
    println!("Sum: {}", sum(&list));
}
