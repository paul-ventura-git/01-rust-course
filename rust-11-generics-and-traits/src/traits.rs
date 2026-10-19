trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

struct Post {
    username: String,
    body: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{} posted: {}", self.username, self.body)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

pub fn run() {
    println!("\n--- Defining and implementing traits ---");

    notify(&Article {
        title: String::from("Traits describe shared behavior"),
        author: String::from("The Rust team"),
    });
    notify(&Post {
        username: String::from("ferris"),
        body: String::from("Traits can have multiple implementations."),
    });
}
