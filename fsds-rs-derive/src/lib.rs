extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

/// Implements From for Value for an iterable struct.
#[proc_macro_derive(FromIntoValue)]
pub fn from_and_into_for_value_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let from_impl = quote! {
        impl From<#name> for Value {
            fn from(value: #name) -> Self {
                let vec = value
                    .iter()
                    .map(|(k, v)| {
                        (k.into(), any_to_value(v))
                    })
                    .collect();

                Value::Map(vec)
            }
        }
    };

    let fields = if let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named, .. }), .. }) = &input.data {
        named
    } else {
        // Works only for structs with named fields.
        unimplemented!();
    };

    let try_from_impl = {

        let fields_def = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                let pos = map.iter().position(|(k, _)| k
                    .as_str()
                    .unwrap_or("") // TODO: throw instead this error for better
                    // debugging: Value::Map should be Vec<(String, _)> to be converted to a struct, but {} was found as a key
                    == stringify!(#field_name)
                ).ok_or(anyhow::anyhow!("Field {} not found in Value::Map.", stringify!(#field_name)))?;
                let #field_name = map
                    .remove(pos)
                    .1
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("Every field of {} should be convertible to Value.", stringify!(#name)))?;
            }
        });

        let fields = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                #field_name
            }
        });

        quote! {
            impl TryFrom<Value> for #name {
                type Error = anyhow::Error;

                fn try_from(value: Value) -> Result<Self, Self::Error> {
                    match value {
                        Value::Map(mut map) => {
                            #(#fields_def)*
                            if map.is_empty() {
                                Ok(#name {
                                    #(#fields),*
                                })
                            } else {
                                Err(anyhow::anyhow!("Value::Map contains extra fields: {:?}", map))
                            }
                        }
                        _ => Err(anyhow::anyhow!("Value should be a Map to be converted to {}", stringify!(#name))),
                    }
                }
            }
        }
    };

    let expanded = quote! {
        #from_impl
        #try_from_impl
    };

    TokenStream::from(expanded)
}