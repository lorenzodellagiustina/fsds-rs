extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

/// Implements `TryFrom<Value>` for a #struct and `From<#struct>` for `Value`.
///
/// Note that [`rmpv::Value`] must be in scope for the derive to work.
///
/// ## From<#struct> for Value implementation
///
/// The implementation of `From<#struct>` for `Value` will create a `Value::Map`
/// with the field names as keys and the field values as values.
///
/// Every field of the struct must implement `Into<Value>`.
///
/// ## TryFrom<Value> for #struct implementation
///
/// The implementation of `TryFrom<Value>` for `#struct` will try to convert a
/// `Value::Map` to a struct.
///
/// Every field of the struct must implement `TryFrom<Value>`. The struct must
/// have the same fields as the `Value::Map` keys.
#[proc_macro_derive(FromIntoValue)]
pub fn from_and_into_for_value_derive(input: TokenStream) -> TokenStream {
    // Parsing TokenStream into DeriveInput.
    let input = parse_macro_input!(input as DeriveInput);

    // Extracting the struct name.
    let name = input.ident;

    // Extracting the fields of the struct.
    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = &input.data
    {
        named
    } else {
        // Works only for structs with named fields.
        unimplemented!();
    };

    // ----------------------- //
    // FROM<#struct> FOR VALUE //
    // ----------------------- //

    // Converting the struct fields into `Value`s.
    let field_from_impl = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            vec.push((stringify!(#field_name).into(), value.#field_name.into()));
        }
    });

    // From<#struct> for Value implementation.
    let from_impl = quote! {
        impl From<#name> for Value {
            fn from(value: #name) -> Self {
                let mut vec = Vec::new();

                #(#field_from_impl)*

                Value::Map(vec)
            }
        }
    };

    // -------------------------- //
    // TRYFROM<VALUE> FOR #struct //
    // -------------------------- //

    let try_from_impl = {
        // Converting the `Value::Map` fields into the struct fields.
        let fields_def = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                let pos = map.iter().position(|(k, _)| k
                    .as_str()
                    .unwrap_or("Value::Map should contain only String keys to be converted to a struct.")
                    == stringify!(#field_name)
                ).ok_or(anyhow::anyhow!("Field {} not found in Value::Map.", stringify!(#field_name)))?;
                let #field_name = map
                    .remove(pos)
                    .1
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("Every field of {} should be convertible to Value.", stringify!(#name)))?;
            }
        });

        // Populating the struct fields.
        let fields = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                #field_name
            }
        });

        // TryFrom<Value> for #struct implementation.
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

    // Expanding the macro.
    let expanded = quote! {
        #from_impl
        #try_from_impl
    };

    // Returning the generated impl.
    TokenStream::from(expanded)
}
