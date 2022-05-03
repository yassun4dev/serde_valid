use crate::errors::{fields_errors_tokens, new_type_errors_tokens, Errors};
use crate::types::{Field, UnnamedField};
use crate::validator::{extract_meta_validator, FieldValidators};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use std::iter::FromIterator;
use syn::parse_quote;

pub fn expand_unnamed_struct_derive(
    input: &syn::DeriveInput,
    fields: &syn::FieldsUnnamed,
) -> Result<TokenStream, Errors> {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let validators = TokenStream::from_iter(
        collect_unnamed_fields_validators(fields)?
            .iter()
            .map(|validator| validator.generate_tokens()),
    );
    let errors = if fields.unnamed.len() != 1 {
        fields_errors_tokens()
    } else {
        new_type_errors_tokens()
    };

    Ok(quote!(
        impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
            fn validate(&self) -> Result<(), ::serde_valid::validation::Errors> {
                let mut __errors = ::serde_valid::validation::MapErrors::new();

                #validators

                if __errors.is_empty() {
                    Result::Ok(())
                } else {
                    Result::Err(#errors)
                }
            }
        }
    ))
}

pub fn collect_unnamed_fields_validators<'a>(
    fields: &'a syn::FieldsUnnamed,
) -> Result<Vec<FieldValidators<'a, UnnamedField<'a>>>, Errors> {
    fields
        .unnamed
        .iter()
        .enumerate()
        .map(collect_unnamed_field_validators)
        .collect()
}

fn collect_unnamed_field_validators<'a>(
    (index, field): (usize, &'a syn::Field),
) -> Result<FieldValidators<'a, UnnamedField<'a>>, Errors> {
    let unnamed_field = UnnamedField::new(index, field);

    let validators = unnamed_field
        .attrs()
        .iter()
        .filter(|attribute| attribute.path == parse_quote!(validate))
        .map(|attribute| extract_meta_validator(&unnamed_field, attribute))
        .collect::<Result<Vec<_>, _>>()?;

    let mut field_validators = FieldValidators::new(Cow::Owned(unnamed_field));
    validators
        .into_iter()
        .for_each(|validator| field_validators.push(validator));

    Ok(field_validators)
}
