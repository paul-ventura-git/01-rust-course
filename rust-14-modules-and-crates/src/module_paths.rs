mod kitchen {
    pub(super) fn prepare_meal() {
        println!("The kitchen is preparing a meal.");
    }
}

pub fn show_visibility() {
    kitchen::prepare_meal();
    crate::restaurant::serve_order();
    crate::restaurant::front_of_house::greet_guest();
    println!("Use `pub` to expose an item outside its module.");
}
