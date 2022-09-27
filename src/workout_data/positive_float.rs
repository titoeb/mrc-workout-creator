/// A Floating point number that can only take positive values.
#[derive(Debug, Clone, PartialEq)]
pub struct PositiveFloat {
    float: f64,
}

/// Erros that occur during the creation of a positive floating-point number.
#[derive(Debug, Clone, PartialEq)]
pub enum InvalidPositiveFloatError {
    /// A non-positive number was provided.
    ProvidedNonPositiveNumber {
        /// Actual number.
        number: f64,
    },
}

impl PositiveFloat {
    /// Try to create a new positive floating point number.
    pub fn new(float: f64) -> Result<Self, InvalidPositiveFloatError> {
        if float > 0.0 {
            return Ok(Self { float });
        }
        Err(InvalidPositiveFloatError::ProvidedNonPositiveNumber { number: float })
    }
    /// Convert the positive float to a number that can be displayed in the crm-file.
    pub fn to_crm(&self) -> String {
        format!("{:.2}", self.float)
    }
    /// Add a positive floating-point number
    /// to a another one.
    pub fn add(&self, other: &PositiveFloat) -> Self {
        PositiveFloat {
            float: self.float + other.float,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{InvalidPositiveFloatError, PositiveFloat};

    #[test]
    fn create_valid_positive_float() {
        assert_eq!(PositiveFloat::new(10.0), Ok(PositiveFloat { float: 10.0 }));
    }

    #[test]
    fn cannot_create_invalid_positive_from_zero() {
        assert_eq!(
            PositiveFloat::new(0.0),
            Err(InvalidPositiveFloatError::ProvidedNonPositiveNumber { number: 0.0 })
        )
    }
    #[test]
    fn cannot_create_invalid_positive_from_negative_number() {
        assert_eq!(
            PositiveFloat::new(-1.0),
            Err(InvalidPositiveFloatError::ProvidedNonPositiveNumber { number: -1.0 })
        )
    }

    #[test]
    fn crm_format() {
        assert_eq!(
            PositiveFloat::new(10.0)
                .expect("A positive positive floating-point can be created")
                .to_crm(),
            "10.00"
        )
    }

    #[test]
    fn add() {
        assert_eq!(
            PositiveFloat { float: 1.0 }.add(&PositiveFloat { float: 2.0 }),
            PositiveFloat { float: 3.0 }
        )
    }
}
