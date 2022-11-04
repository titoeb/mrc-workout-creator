use serde::{Deserialize, Serialize};
use std::{num::ParseFloatError, ops::Add};
/// A Floating point number that can only take non-negativevalues.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    InvalidStringToParse {
        message: String,
    },
}
impl From<ParseFloatError> for InvalidPositiveFloatError {
    fn from(parse_error: ParseFloatError) -> Self {
        Self::InvalidStringToParse {
            message: format!("{}", parse_error),
        }
    }
}

impl PositiveFloat {
    /// Try to create a new positive floating point number.
    pub fn new(float: f64) -> Result<Self, InvalidPositiveFloatError> {
        if float >= 0.0 {
            return Ok(Self { float });
        }
        Err(InvalidPositiveFloatError::ProvidedNonPositiveNumber { number: float })
    }
    /// Convert the positive float to a number that can be displayed in the crm-file.
    pub fn to_crm(&self) -> String {
        format!("{:.2}", self.float)
    }
    /// Extract the floating point number from the float
    pub fn to_float(&self) -> f64 {
        self.float
    }
}
impl TryFrom<String> for PositiveFloat {
    type Error = InvalidPositiveFloatError;
    fn try_from(potential_float: String) -> Result<Self, Self::Error> {
        let float = potential_float.trim().parse::<f64>()?;
        Self::new(float)
    }
}

impl TryFrom<&mut String> for PositiveFloat {
    type Error = InvalidPositiveFloatError;
    fn try_from(potential_float: &mut String) -> Result<Self, Self::Error> {
        Self::try_from(potential_float.to_string())
    }
}

impl Add for PositiveFloat {
    type Output = Self;
    /// Add a positive floating-point number
    /// to a another one.
    fn add(self, other: Self) -> Self {
        PositiveFloat {
            float: self.float + other.float,
        }
    }
}

impl From<PositiveFloat> for String {
    fn from(positive_float: PositiveFloat) -> Self {
        positive_float.to_crm()
    }
}

#[cfg(test)]
mod test {
    use super::{InvalidPositiveFloatError, PositiveFloat};
    use crate::testing::serialize_deserialize;

    #[test]
    fn create_valid_positive_float() {
        assert_eq!(PositiveFloat::new(10.0), Ok(PositiveFloat { float: 10.0 }));
    }

    #[test]
    fn cannot_create_invalid_positive_from_zero() {
        assert_eq!(PositiveFloat::new(0.0), Ok(PositiveFloat { float: 0.0 }))
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
            PositiveFloat { float: 1.0 } + PositiveFloat { float: 2.0 },
            PositiveFloat { float: 3.0 }
        )
    }

    #[test]
    fn test_serialization() {
        let point_to_serialize = PositiveFloat { float: 1.0 };
        assert_eq!(
            point_to_serialize,
            serialize_deserialize(&point_to_serialize)
        )
    }
    #[test]
    fn valid_conversion_from_string() {
        assert_eq!(
            PositiveFloat::try_from(String::from("1.0")).unwrap(),
            PositiveFloat { float: 1.0 }
        )
    }
    #[test]
    fn valid_conversion_from_string_with_int() {
        assert_eq!(
            PositiveFloat::try_from(String::from("3")).unwrap(),
            PositiveFloat { float: 3.0 }
        )
    }
    #[test]
    fn valid_conversion_from_string_with_spaces() {
        assert_eq!(
            PositiveFloat::try_from(String::from(" 90.0 ")).unwrap(),
            PositiveFloat { float: 90.0 }
        )
    }
    #[test]
    fn invalid_conversion_from_string_no_float() {
        assert_eq!(
            PositiveFloat::try_from(String::from("abcd")),
            Err(InvalidPositiveFloatError::InvalidStringToParse {
                message: String::from("invalid float literal")
            })
        )
    }
    #[test]
    fn invalid_conversion_from_string_no_valid_positive_string() {
        assert_eq!(
            PositiveFloat::try_from(String::from("-1.0")),
            Err(InvalidPositiveFloatError::ProvidedNonPositiveNumber { number: -1.0 })
        )
    }
}
