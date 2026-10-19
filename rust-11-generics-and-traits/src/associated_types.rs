trait Container {
    type Item;

    fn first(&self) -> Option<&Self::Item>;
}

impl<T> Container for Vec<T> {
    type Item = T;

    fn first(&self) -> Option<&Self::Item> {
        self.as_slice().first()
    }
}

fn announce<T: Container>(container: &T)
where
    T::Item: std::fmt::Debug,
{
    println!("First item: {:?}", container.first());
}

pub fn run() {
    println!("\n--- Associated types ---");

    announce(&vec![String::from("associated"), String::from("type")]);
    announce(&Vec::<i32>::new());
}
