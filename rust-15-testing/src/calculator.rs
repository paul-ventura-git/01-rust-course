/// Returns the arithmetic mean, or `None` when the input is empty.
///
/// ```
/// use rust_15_testing::calculator::average;
/// assert_eq!(average(&[2.0, 4.0, 6.0]), Some(4.0));
/// ```
pub fn average(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    Some(values.iter().sum::<f64>() / values.len() as f64)
}

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::{add, average};

    #[test]
    fn adds_positive_and_negative_values() {
        assert_eq!(add(7, -2), 5);
    }

    #[test]
    fn averages_values() {
        assert_eq!(average(&[2.0, 4.0, 6.0]), Some(4.0));
    }

    #[test]
    fn empty_average_is_none() {
        assert_eq!(average(&[]), None);
    }

    #[test]
    #[should_panic(expected = "expected the same value")]
    fn demonstrates_expected_panic() {
        assert_eq!(add(1, 1), 3, "expected the same value");
    }
}
