#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PositiveFloat {
    float: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum InvalidPositiveFloatError {
    ProvidedNonPositiveNumber { number: f64 },
}

impl PositiveFloat {
    pub(crate) fn new(float: f64) -> Result<Self, InvalidPositiveFloatError> {
        if float > 0.0 {
            return Ok(Self { float });
        }
        Err(InvalidPositiveFloatError::ProvidedNonPositiveNumber { number: float })
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
}
