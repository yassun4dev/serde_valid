use super::{inner_extract_array_unique_items_validator, VALIDATION_LABEL};
use crate::abort::abort_required_list_argument;
use crate::types::Field;
use crate::validator::common::{check_common_list_argument, extract_message_tokens};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_array_unique_items_validator_from_meta_list<F: Field>(
    field: &F,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> Validator {
    if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(
            extract_array_unique_items_validator_from_meta_list(
                &option_field,
                attribute,
                meta_list,
            ),
        ))
    } else {
        Validator::Normal(inner_extract_array_unique_items_validator_from_meta_list(
            field.name(),
            field.ident(),
            attribute,
            meta_list,
        ))
    }
}

fn inner_extract_array_unique_items_validator_from_meta_list(
    field_name: &str,
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> TokenStream {
    let syn::MetaList { nested, .. } = meta_list;

    let message = extract_message_tokens(VALIDATION_LABEL, field_ident, attribute, nested)
        .unwrap_or(quote!(
            ::serde_valid::validation::error::ItemsParams::to_default_message
        ));
    if nested.is_empty() && !check_common_list_argument(meta_list) {
        abort_required_list_argument(
            VALIDATION_LABEL,
            &["message_fn"],
            field_ident,
            attribute.span(),
            meta_list,
        )
    }
    inner_extract_array_unique_items_validator(field_name, field_ident, message)
}