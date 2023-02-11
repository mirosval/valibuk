use proc_macro2::TokenStream;
use quote::quote;
use root::ValidatedDeriv;
use syn::{parse2, parse_quote, spanned::Spanned, DeriveInput, Error, Token};

mod field;
mod root;

pub fn builder_validator_core(input: TokenStream) -> TokenStream {
    let input = match parse2::<DeriveInput>(input) {
        Ok(i) => i,
        Err(e) => return e.to_compile_error(),
    };
    let derived = match inner_derive(&input) {
        Ok(i) => i,
        Err(e) => return e.to_compile_error(),
    };
    quote!(#derived)
}

fn inner_derive(ast: &syn::DeriveInput) -> Result<TokenStream, Error> {
    //let validator_error_type = ast.attrs.iter().find(|a| a.parse_args());
    let data = match &ast.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => {
                let validated_deriv = ValidatedDeriv::new(ast, fields.named.iter())?;
                let validated_impl = validated_deriv.validated_impl()?;
                quote! {
                    #validated_impl
                }
            }
            syn::Fields::Unnamed(_) => {
                return Err(Error::new(
                    ast.span(),
                    "Correct-by-construction Validator is not supported on tuple Structs",
                ))
            }
            syn::Fields::Unit => {
                return Err(Error::new(
                    ast.span(),
                    "Correct-by-construction Validator is not supported on unit Structs",
                ))
            }
        },
        syn::Data::Enum(_) => {
            return Err(Error::new(
                ast.span(),
                "Correct-by-construction Validator is not supported on Enums",
            ))
        }
        syn::Data::Union(_) => {
            return Err(Error::new(
                ast.span(),
                "Correct-by-construction Validator is not supported on Unions",
            ))
        }
    };
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_tokens_eq::assert_tokens_eq;

    #[test]
    fn test_minimal_no_validated_fields() {
        let before = quote! {
            struct A {
                a: i32
            }
        };
        let after = builder_validator_core(before);
        let expected = quote! {
            struct UnvalidatedA {
                pub a: i32,
            }
            impl A {
                pub fn from_unvalidated(
                    unvalidated: UnvalidatedA,
                ) -> ::core::result::Result<A, Vec<::std::string::String>> {
                    Ok(A { a: unvalidated.a })
                }
            }
        };
        assert_tokens_eq!(&expected, &after);
    }

    #[test]
    fn test_minimal_no_validated_fields_custom_error() {
        let before = quote! {
            #[validation_error(E)]
            struct A {
                a: i32
            }
        };
        let after = builder_validator_core(before);
        let expected = quote! {
            struct UnvalidatedA {
                pub a: i32,
            }
            impl A {
                pub fn from_unvalidated(
                    unvalidated: UnvalidatedA,
                ) -> ::core::result::Result<A, Vec<E>> {
                    Ok(A { a: unvalidated.a })
                }
            }
        };
        assert_tokens_eq!(&expected, &after);
    }
}
