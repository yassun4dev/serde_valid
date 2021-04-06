use super::inner_extract_array_unique_items_validator;
use crate::types::Field;
use crate::validator::Validator;
use quote::quote;

pub fn extract_array_unique_items_validator_from_meta_path<F: Field>(field: &F) -> Validator {
    if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(
            extract_array_unique_items_validator_from_meta_path(&option_field),
        ))
    } else {
        let message =
            quote!(::serde_valid::validation::error::UniqueItemsParams::to_default_message);
        Validator::Normal(inner_extract_array_unique_items_validator(
            field.name(),
            field.ident(),
            message,
        ))
    }
}