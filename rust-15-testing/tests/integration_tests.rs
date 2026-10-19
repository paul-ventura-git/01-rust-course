use rust_15_testing::calculator::{add, average};

#[test]
fn public_functions_work_together() {
    let values = [add(1, 2) as f64, add(3, 3) as f64];
    assert_eq!(average(&values), Some(4.5));
}
