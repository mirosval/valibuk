use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Error};

#[derive(Debug)]
pub(crate) struct ValidatedFieldDeriv<'a> {
    pub(crate) name: &'a syn::Ident,
    pub(crate) ty: &'a syn::Type,
    pub(crate) custom_validation_error_ty: syn::Type,
    pub(crate) field_validator: Option<syn::Ident>,
}

impl<'a> ValidatedFieldDeriv<'a> {
    pub fn new(field: &'a syn::Field, error: syn::Type) -> Result<ValidatedFieldDeriv<'a>, Error> {
        if let Some(ref name) = field.ident {
            let field_validator = field
                .attrs
                .iter()
                .filter(|a| a.path.is_ident("validator"))
                .map(|f| {
                    let args: syn::Ident = f.parse_args().expect("attribue parsing failed");
                    args
                })
                .last();
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

    pub fn build_unvalidated_struct_repr(&self) -> TokenStream {
        let name = self.name;
        let ty = self.ty;
        quote! {
            pub #name: #ty
        }
    }

    pub fn build_validators_struct_repr(&self) -> TokenStream {
        let name = self.name;
        let ty = self.ty;
        let ety = &self.custom_validation_error_ty;
        quote! {
            pub #name: fn(#ty) -> ::core::result::Result<#ty, #ety>
        }
    }

    pub fn field_struct_def(&self) -> TokenStream {
        let field_name = &self.name;
        let field_validator = &self.field_validator;
        if let Some(v) = field_validator {
            quote! {
                #field_name: #v
            }
        } else {
            quote! {}
        }
    }
}
