#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn run() {
    println!("\n--- Destructuring tuples and structs ---");

    let coordinates = (3, 8, -2);
    let (x, y, z) = coordinates;
    println!("Tuple {coordinates:?} has x={x}, y={y}, z={z}.");

    let point = Point { x: 0, y: 7 };
    match point {
        Point { x: 0, y } => println!("Point lies on the y axis at {y}."),
        Point { x, y: 0 } => println!("Point lies on the x axis at {x}."),
        Point { x, y } => println!("Point is at ({x}, {y})."),
    }

    let values = [10, 20, 30, 40];
    let [first, middle @ .., last] = values;
    println!("Array edges are {first} and {last}; middle is {middle:?}.");
}
