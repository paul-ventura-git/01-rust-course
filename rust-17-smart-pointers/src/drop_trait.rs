struct Resource {
    name: &'static str,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Dropping resource '{}'.", self.name);
    }
}

pub fn run() {
    println!("\n--- Drop and deterministic cleanup ---");

    let resource = Resource {
        name: "lesson demo",
    };
    println!("Resource is alive: {}", resource.name);
    drop(resource);
    println!("The value was dropped explicitly.");
}
