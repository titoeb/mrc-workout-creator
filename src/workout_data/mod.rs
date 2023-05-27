/// Defining the top-level structs to build an individual workout.
pub mod workout;

/// The individual parts or a workout.
pub mod effort;

pub mod from_mrc;

pub trait ToMRC {
    fn to_mrc(&self) -> String;
}

impl ToMRC for f64 {
    fn to_mrc(&self) -> String {
        format!("{:.2}", self)
    }
}

#[cfg(test)]
mod test {
    use super::ToMRC;

    #[test]
    fn single_digit_is_printed_with_two_digits() {
        assert_eq!(0.0.to_mrc(), "0.00")
    }
    #[test]
    fn two_digits_are_printed_with_two_digits() {
        assert_eq!(0.12.to_mrc(), "0.12")
    }
    #[test]
    fn three_digits_are_printed_with_two_digits() {
        assert_eq!(0.123.to_mrc(), "0.12")
    }
}
