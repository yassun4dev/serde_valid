use crate::traits::IsUnique;

/// Unique validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#unique_items>
pub trait ValidateUniqueItems {
    fn validate(&self) -> Result<(), crate::UniqueItemsErrorParams>;
}

impl<T> ValidateUniqueItems for Vec<T>
where
    T: std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
{
    fn validate(&self) -> Result<(), crate::UniqueItemsErrorParams> {
        if self.is_unique() {
            Ok(())
        } else {
            Err(crate::UniqueItemsErrorParams::new())
        }
    }
}

impl<T, const N: usize> ValidateUniqueItems for [T; N]
where
    T: std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
{
    fn validate(&self) -> Result<(), crate::UniqueItemsErrorParams> {
        if self.is_unique() {
            Ok(())
        } else {
            Err(crate::UniqueItemsErrorParams::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_unique_items_array_type_is_true() {
        assert!(ValidateUniqueItems::validate(&[1, 2, 3, 4]).is_ok());
    }

    #[test]
    fn test_validate_array_unique_items_vec_type_is_true() {
        assert!(ValidateUniqueItems::validate(&vec![1, 2, 3, 4]).is_ok());
    }

    #[test]
    fn test_validate_array_unique_items_is_false() {
        assert!(ValidateUniqueItems::validate(&[1, 2, 3, 3]).is_err());
    }
}
