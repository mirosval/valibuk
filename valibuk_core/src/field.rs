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

    pub fn build_unvalidated_constructor(&self) -> TokenStream {
        let name = self.name;
        quote! {
            #name: unvalidated.#name
        }
    }

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

    pub fn get_name(&self) -> TokenStream {
        let name = self.name;
        quote!(#name)
    }

    pub fn is_validated(&self) -> bool {
        dbg!(self.field_validator.is_some())
    }

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

    #[test]
    fn test_unvalidated() {
        let s: syn::DeriveInput = parse_quote! {
            struct A {
                a: i32
            }
        };
        let fields = match s.data {
            syn::Data::Struct(data) => match data.fields {
                syn::Fields::Named(f) => f,
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        };
        let f = ValidatedFieldDeriv::new(fields.named.iter().last().unwrap(), parse_quote!(String))
            .unwrap();
        dbg!(&f);
        let expected = quote!(a);
        let actual = f.get_name();
        assert_tokens_eq!(&actual, &expected);
    }
}
