use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Error};

#[derive(Debug)]
pub(crate) struct ValidatedFieldDeriv<'a> {
    name: &'a syn::Ident,
    ty: &'a syn::Type,
    custom_validation_error_ty: syn::Type,
    field_validator: FieldValidator,
}

impl<'a> ValidatedFieldDeriv<'a> {
    pub fn new(field: &'a syn::Field, error: syn::Type) -> Result<ValidatedFieldDeriv<'a>, Error> {
        if let Some(ref name) = field.ident {
            let field_validator = Self::parse_field_validator(field); // dbg!(&field_validator);
            Ok(ValidatedFieldDeriv {
                name: &name,
                ty: &field.ty,
                custom_validation_error_ty: error,
                field_validator,
            })
        } else {
            Err(Error::new(field.span(), "Nameless field in struct"))
        }
    }

    fn parse_field_validator(field: &'a syn::Field) -> FieldValidator {
        field
            .attrs
            .iter()
            .filter(|a| a.path.is_ident("validator"))
            .map(FieldValidator::from)
            .last()
            .unwrap_or(FieldValidator::None)
    }

    /// Name of the field as token stream
    pub fn get_name(&self) -> TokenStream {
        let name = self.name;
        quote!(#name)
    }

    /// True when the field has a validator attached
    pub fn is_validated(&self) -> bool {
        self.field_validator.is_some()
    }

    /// Used to construct the validated instance from the unvalidated
    ///
    /// When there are no validators attached, its a simple field copy
    pub fn build_unvalidated_constructor(&self) -> TokenStream {
        let name = self.name;
        quote! {
            #name: unvalidated.#name
        }
    }

    /// Emits code to execute the validator attached to field, if any
    ///
    /// The emitted code should yield a value of the type Result<T, E>
    /// where [T][ValidatedFieldDeriv.ty] is the type of the current field and E is the error type
    /// of the current field
    pub fn build_match_validator_call(&self) -> TokenStream {
        let field = self.name;
        let validator = &self.field_validator;
        match validator {
            crate::field::FieldValidator::Ident(v) => quote! {
                (#v)(unvalidated.#field)
            },
            crate::field::FieldValidator::Closure(v) => quote! {
                (#v)(unvalidated.#field)
            },
            crate::field::FieldValidator::None => quote! {
                unvalidated.#field
            },
        }
    }

    /// Builds the PatExpr that matches when the validator was successful
    ///
    /// This is used in the match expr to collect all the validated fields
    pub fn build_match_validator_ok(&self) -> TokenStream {
        let name = self.name;
        let validator = &self.field_validator;
        if validator.is_some() {
            quote! {
                ::std::result::Result::Ok(#name)
            }
        } else {
            quote! {
                #name
            }
        }
    }

    /// Builds error handling for when the validator fails
    pub fn build_validator_error_push(&self) -> TokenStream {
        let name = self.name;
        let validator = &self.field_validator;
        if validator.is_some() {
            quote! {
                if let ::std::result::Result::Err(e) = #name {
                    errors.push(e);
                }
            }
        } else {
            quote! {}
        }
    }

    /// Emits dummy code that fails to compile when the declared
    /// type of the custom error does not match the signature of
    /// the validator for this field.
    pub fn build_field_assertions(&self) -> TokenStream {
        let ty = self.ty;
        let err = &self.custom_validation_error_ty;
        let validator = &self.field_validator;
        match validator {
            FieldValidator::Ident(v) => quote! {
                let _: fn(#ty) -> ::std::result::Result<#ty, #err> = #v;
            },
            FieldValidator::Closure(v) => quote! {
                let _: fn(#ty) -> ::std::result::Result<#ty, #err> = #v;
            },
            FieldValidator::None => quote!(),
        }
    }

    /// Builds fields for the unvalidated struct
    pub fn build_unvalidated_struct_repr(&self) -> TokenStream {
        let name = self.name;
        let ty = self.ty;
        quote! {
            pub #name: #ty
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FieldValidator {
    Ident(syn::Ident),
    Closure(syn::ExprClosure),
    None,
}

impl FieldValidator {
    pub fn is_some(&self) -> bool {
        dbg!(self != &FieldValidator::None)
    }
}

impl From<&syn::Attribute> for FieldValidator {
    fn from(value: &syn::Attribute) -> Self {
        let ident = value
            .parse_args::<syn::Ident>()
            .map(|i| FieldValidator::Ident(i));
        let closure = value
            .parse_args::<syn::ExprClosure>()
            .map(|i| FieldValidator::Closure(i));
        ident.or(closure).unwrap_or(FieldValidator::None)
    }
}

#[cfg(test)]
mod test {
    use syn::parse_quote;

    use super::*;
    use assert_tokens_eq::assert_tokens_eq;

    /// Helper to extract the ValidatedFieldDeriv for the first field of the input struct
    fn first_field_deriv_from_struct<'a>(s: &'a syn::DeriveInput) -> ValidatedFieldDeriv<'a> {
        let fields = match &s.data {
            syn::Data::Struct(data) => match &data.fields {
                syn::Fields::Named(f) => f,
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        };
        ValidatedFieldDeriv::new(fields.named.iter().last().unwrap(), parse_quote!(String)).unwrap()
    }

    #[test]
    fn test_name() {
        let s: syn::DeriveInput = parse_quote! {
            struct A {
                a: i32
            }
        };
        let f = first_field_deriv_from_struct(&s);
        assert_tokens_eq!(
            &f.get_name(),
            &quote!(a),
            "get_name returns the name of the field "
        );
    }

    #[test]
    fn test_unvalidated_constructor() {
        let s: syn::DeriveInput = parse_quote! {
            struct A {
                a: i32
            }
        };
        let f = first_field_deriv_from_struct(&s).build_unvalidated_constructor();
        let actual: syn::ExprStruct = parse_quote! {
            B {
                #f
            }
        };
        let expected: syn::ExprStruct = parse_quote! {
            B {
                a: unvalidated.a,
            }
        };
        assert_tokens_eq!(
            &actual,
            &expected,
            "get_name returns the name of the field "
        );
    }

    #[test]
    fn test_is_validated() {
        let s: syn::DeriveInput = parse_quote! {
            struct A {
                a: i32
            }
        };
        let f = first_field_deriv_from_struct(&s);
        assert_eq!(f.is_validated(), false, "field a is not validated");
        let s: syn::DeriveInput = parse_quote! {
            struct A {
                #[validator(abc)]
                a: i32
            }
        };
        let f = first_field_deriv_from_struct(&s);
        assert_eq!(f.is_validated(), true, "field a is validated");
    }

    #[test]
    fn test_build_match_validator_call() {
        {
            // fn validator case
            let s: syn::DeriveInput = parse_quote! {
                struct A {
                    #[validator(abc)]
                    a: i32
                }
            };
            let f = first_field_deriv_from_struct(&s);
            let expected: syn::ExprCall = parse_quote! {
                (abc)(unvalidated.a)
            };
            assert_tokens_eq!(
                f.build_match_validator_call(),
                &expected,
                "validator call for fn validator"
            );
        }
        {
            // inline fn validator case
            let s: syn::DeriveInput = parse_quote! {
                struct A {
                    #[validator(|a| if a > 0 { Ok(a) } else { Err("err") })]
                    a: i32
                }
            };
            let f = first_field_deriv_from_struct(&s);
            let expected: syn::ExprCall = parse_quote! {
                (|a| if a > 0 { Ok(a) } else { Err("err") })(unvalidated.a)
            };
            assert_tokens_eq!(
                f.build_match_validator_call(),
                &expected,
                "validator call for fn validator"
            );
        }
        {
            // unvalidated case
            let s: syn::DeriveInput = parse_quote! {
                struct A {
                    a: i32
                }
            };
            let f = first_field_deriv_from_struct(&s);
            let expected: syn::ExprField = parse_quote! {
                unvalidated.a
            };
            assert_tokens_eq!(
                f.build_match_validator_call(),
                &expected,
                "validator call for unvalidated field"
            );
        }
    }

    #[test]
    fn test_build_match_validator_ok() {
        let s: syn::DeriveInput = parse_quote! {
            struct A {
                a: i32
            }
        };
        let f = first_field_deriv_from_struct(&s);
        let expected: syn::Pat = parse_quote! {
            a
        };
        assert_tokens_eq!(&f.build_match_validator_ok(), &expected, "_ pat");
        let s: syn::DeriveInput = parse_quote! {
            struct A {
                #[validator(abc)]
                a: i32
            }
        };
        let f = first_field_deriv_from_struct(&s);
        let expected: syn::Pat = parse_quote! {
            ::std::result::Result::Ok(a)
        };
        assert_tokens_eq!(&f.build_match_validator_ok(), &expected, "ok extractor pat");
    }

    #[test]
    fn test_build_validator_error_push() {
        let s: syn::DeriveInput = parse_quote! {
            struct A {
                a: i32
            }
        };
        let f = first_field_deriv_from_struct(&s);
        let expected: TokenStream = quote! {};
        assert_tokens_eq!(
            &f.build_validator_error_push(),
            &expected,
            "no error handling for unvalidated"
        );
        let s: syn::DeriveInput = parse_quote! {
            struct A {
                #[validator(abc)]
                a: i32
            }
        };
        let f = first_field_deriv_from_struct(&s);
        let expected: syn::Expr = parse_quote! {
            if let ::std::result::Result::Err(e) = a {
                errors.push(e);
            }
        };
        assert_tokens_eq!(
            &f.build_validator_error_push(),
            &expected,
            "if expr for validated"
        );
    }

    #[test]
    fn test_build_unvalidated_struct_repr() {
        let s: syn::DeriveInput = parse_quote! {
            struct A {
                a: i32
            }
        };
        let f = first_field_deriv_from_struct(&s).build_unvalidated_struct_repr();
        let actual: syn::ItemStruct = parse_quote! {
            struct B {
                #f
            }
        };
        let expected: syn::ItemStruct = parse_quote! {
            struct B {
                pub a: i32
            }
        };
        assert_tokens_eq!(&actual, &expected, "unvalidated struct field");
    }
}
