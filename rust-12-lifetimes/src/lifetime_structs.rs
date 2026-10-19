struct Excerpt<'a> {
    text: &'a str,
}

impl Excerpt<'_> {
    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    fn text(&self) -> &str {
        self.text
    }
}

pub fn run() {
    println!("\n--- Lifetimes in structs and methods ---");

    let source = String::from("A struct can hold a borrowed string slice.");
    let excerpt = Excerpt {
        text: source.split('.').next().unwrap_or(""),
    };
    println!("Excerpt: '{}'", excerpt.text());
    println!("Words: {}", excerpt.word_count());
}
