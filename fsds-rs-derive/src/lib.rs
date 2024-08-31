extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Implements From for Value for an iterable struct.
#[proc_macro_derive(IntoValue)]
pub fn from_for_value_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl From<#name> for Value {
            fn from(value: #name) -> Self {
                let vec = value
                    .iter()
                    .map(|(k, v)| {
                        v.type_id();
                        (k.into(), any_to_value(v))
                    })
                    .collect();

                Value::Map(vec)
            }
        }
    };

    TokenStream::from(expanded)
}