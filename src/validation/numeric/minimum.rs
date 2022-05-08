use crate::MinimumErrorParams;

/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
pub trait ValidateNumericMinimum<T>
where
    T: PartialOrd + PartialEq,
{
    fn validate(&self, minimum: T) -> Result<(), MinimumErrorParams>;
}

macro_rules! impl_validate_numeric_minimum {
    ($ty:ty) => {
        impl ValidateNumericMinimum<$ty> for $ty {
            fn validate(&self, minimum: $ty) -> Result<(), MinimumErrorParams> {
                if *self >= minimum {
                    Ok(())
                } else {
                    Err(MinimumErrorParams::new(minimum))
                }
            }
        }
    };
}

impl_validate_numeric_minimum!(i8);
impl_validate_numeric_minimum!(i16);
impl_validate_numeric_minimum!(i32);
impl_validate_numeric_minimum!(i64);
impl_validate_numeric_minimum!(i128);
impl_validate_numeric_minimum!(isize);
impl_validate_numeric_minimum!(u8);
impl_validate_numeric_minimum!(u16);
impl_validate_numeric_minimum!(u32);
impl_validate_numeric_minimum!(u64);
impl_validate_numeric_minimum!(u128);
impl_validate_numeric_minimum!(usize);
impl_validate_numeric_minimum!(f32);
impl_validate_numeric_minimum!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_minimum_is_true() {
        assert!(ValidateNumericMinimum::validate(&10, 9).is_ok());
        assert!(ValidateNumericMinimum::validate(&10, 10).is_ok());
    }

    #[test]
    fn test_validate_numeric_minimum_is_false() {
        assert!(ValidateNumericMinimum::validate(&5, 6).is_err());
    }

    #[test]
    fn test_validate_numeric_minimum_vec_is_true() {
        assert!(ValidateNumericMinimum::validate(&vec![5], 3).is_ok());
    }

    #[test]
    fn test_validate_numeric_minimum_vec_is_false() {
        assert!(ValidateNumericMinimum::validate(&vec![5], 7).is_err());
    }

    #[test]
    fn test_validate_numeric_minimum_array_is_true() {
        assert!(ValidateNumericMinimum::validate(&[5, 6, 7], 3).is_ok());
    }

    #[test]
    fn test_validate_numeric_minimum_array_is_false() {
        assert!(ValidateNumericMinimum::validate(&vec![5], 7).is_err());
    }

    #[test]
    fn test_validate_numeric_minimum_option_is_true() {
        assert!(ValidateNumericMinimum::validate(&Some(5), 3).is_ok());
    }

    #[test]
    fn test_validate_numeric_minimum_option_is_false() {
        assert!(ValidateNumericMinimum::validate(&Some(5), 7).is_err());
    }

    #[test]
    fn test_validate_numeric_minimum_specified_type() {
        assert!(ValidateNumericMinimum::validate(&0.5, 0.2).is_ok());
        assert!(ValidateNumericMinimum::validate(&5u8, 0).is_ok());
        assert!(ValidateNumericMinimum::validate(&4u16, 0).is_ok());
        assert!(ValidateNumericMinimum::validate(&6u32, 0).is_ok());
    }
}
