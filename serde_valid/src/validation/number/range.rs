#[derive(Debug)]
pub enum Limit<T> {
    Inclusive(T),
    Exclusive(T),
}

/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#id7>
pub fn validate_number_range<T>(
    value: T,
    minimum: Option<Limit<T>>,
    maximum: Option<Limit<T>>,
) -> bool
where
    T: PartialOrd + PartialEq,
{
    if let Some(max) = maximum {
        if match max {
            Limit::Inclusive(limit) => value > limit,
            Limit::Exclusive(limit) => value >= limit,
        } {
            return false;
        };
    }

    if let Some(min) = minimum {
        if match min {
            Limit::Inclusive(limit) => value < limit,
            Limit::Exclusive(limit) => value <= limit,
        } {
            return false;
        };
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_number_range_generic_ok() {
        // Unspecified generic type:
        assert!(validate_number_range(
            10,
            Some(Limit::Inclusive(-10)),
            Some(Limit::Inclusive(10))
        ));
        assert!(validate_number_range(
            0.0,
            Some(Limit::Inclusive(0.0)),
            Some(Limit::Inclusive(10.0))
        ));
        assert!(validate_number_range(
            10,
            Some(Limit::Inclusive(-10)),
            Some(Limit::Exclusive(11))
        ));
        assert!(validate_number_range(
            0.0,
            Some(Limit::Exclusive(-0.1)),
            Some(Limit::Inclusive(10.0))
        ));

        // Specified type:
        assert!(validate_number_range(
            5u8,
            Some(Limit::Inclusive(0)),
            Some(Limit::Inclusive(255))
        ));
        assert!(validate_number_range(
            4u16,
            Some(Limit::Inclusive(0)),
            Some(Limit::Inclusive(16))
        ));
        assert!(validate_number_range(
            6u32,
            Some(Limit::Inclusive(0)),
            Some(Limit::Inclusive(23))
        ));
    }

    #[test]
    fn test_validate_number_range_generic_fail() {
        assert!(!validate_number_range(
            5,
            Some(Limit::Inclusive(6)),
            Some(Limit::Inclusive(10))
        ));
        assert!(!validate_number_range(
            5,
            Some(Limit::Exclusive(5)),
            Some(Limit::Inclusive(10))
        ));
        assert!(!validate_number_range(
            10.0,
            Some(Limit::Inclusive(0.0)),
            Some(Limit::Inclusive(9.0))
        ));
        assert!(!validate_number_range(
            10.0,
            Some(Limit::Inclusive(0.0)),
            Some(Limit::Exclusive(10.0))
        ));
    }

    #[test]
    fn test_validate_number_range_generic_min_only() {
        assert_eq!(
            false,
            validate_number_range(5, Some(Limit::Inclusive(10)), None)
        );
        assert_eq!(
            true,
            validate_number_range(15, Some(Limit::Inclusive(10)), None)
        );
    }

    #[test]
    fn test_validate_number_range_generic_max_only() {
        assert_eq!(
            true,
            validate_number_range(5, None, Some(Limit::Inclusive(10)))
        );
        assert_eq!(
            false,
            validate_number_range(15, None, Some(Limit::Inclusive(10)))
        );
    }
}
