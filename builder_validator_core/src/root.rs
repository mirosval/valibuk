use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Attribute, Error, Token};

use crate::field::ValidatedFieldDeriv;

#[derive(Debug)]
pub struct ValidatedDeriv<'a> {
    visibility: &'a syn::Visibility,
    name: &'a syn::Ident,
    unvalidated_name: syn::Ident,
    validators_name: syn::Ident,
    generics: &'a syn::Generics,
    custom_validation_error_ty: syn::Type,
    fields: Vec<ValidatedFieldDeriv<'a>>,
}

impl<'a> ValidatedDeriv<'a> {
    pub fn new(
        ast: &'a syn::DeriveInput,
        fields: impl Iterator<Item = &'a syn::Field>,
    ) -> Result<ValidatedDeriv<'a>, Error> {
        let unvalidated_name = syn::Ident::new(
            &format!("Unvalidated{}", ast.ident),
            proc_macro2::Span::call_site(),
        );
        let validators_name = syn::Ident::new(
            &format!("{}Validators", ast.ident),
            proc_macro2::Span::call_site(),
        );
        let custom_validation_error_ty: syn::Type = Self::validation_error_from_attrs(&ast.attrs);
        let fields = fields
            .enumerate()
            .map(|(_, f)| ValidatedFieldDeriv::new(&f, custom_validation_error_ty.clone()))
            .collect::<Result<_, _>>()?;
        Ok(ValidatedDeriv {
            visibility: &ast.vis,
            name: &ast.ident,
            unvalidated_name,
            validators_name,
            generics: &ast.generics,
            fields,
            custom_validation_error_ty,
        })
    }

    fn validation_error_from_attrs(attrs: &[Attribute]) -> syn::Type {
        attrs
            .iter()
            .filter(|a| a.path.is_ident("validation_error"))
            .map(|a| {
                let err: syn::Type = a.parse_args().expect("parse validation_error");
                err
            })
            .last()
            .unwrap_or(parse_quote! {
                ::std::string::String
            })
    }

    pub fn validated_impl(&self) -> Result<TokenStream, Error> {
        let mut b_generics = self.generics.clone();
        b_generics
            .params
            .push(syn::GenericParam::Type(syn::TypeParam {
                attrs: Vec::new(),
                ident: syn::Ident::new("ValidationError", proc_macro2::Span::call_site()),
                colon_token: Some(Token![:](proc_macro2::Span::call_site())),
                bounds: syn::punctuated::Punctuated::new(),
                eq_token: None,
                default: None,
            }));
        let unvalidated_struct = self.build_unvalidated_struct()?;
        let validate_impl = self.build_validate_impl()?;
        Ok(quote! {
            #unvalidated_struct
            #validate_impl
        })
    }

    fn build_unvalidated_struct(&self) -> Result<TokenStream, Error> {
        let vis = &self.visibility;
        let name = &self.unvalidated_name;
        let fields = self
            .fields
            .iter()
            .map(|f| f.build_unvalidated_struct_repr());
        Ok(quote! {
            #vis struct #name {
                #( #fields, )*
            }
        })
    }

    fn build_validate_impl(&self) -> Result<TokenStream, Error> {
        let name = &self.name;
        let unvalidated_name = &self.unvalidated_name;
        let ety = &self.custom_validation_error_ty;
        // let validator_fields = &self.validator_fields();
        let has_any_validated_fields = self.fields.iter().any(|f| f.field_validator.is_some());
        let body = if has_any_validated_fields {
            let match_validator_calls = &self.match_validator_calls();
            let match_validator_ok = &self.match_validator_ok();
            let match_validator_nok = &self.match_validator_nok();
            let match_validator_error_push = &self.match_validator_error_push();
            let constructor = self.constructor();
            quote! {
                match (#match_validator_calls) {
                    (#match_validator_ok) => Ok(#constructor),
                    (#match_validator_nok) => {
                        let mut errors: Vec<#ety> = Vec::new();
                        #match_validator_error_push
                        Err(errors)
                    }
                }
            }
        } else {
            let constructor = self.unvalidated_constructor();
            quote! {
                Ok(#constructor)
            }
        };
        Ok(quote! {
            impl #name {
                pub fn from_unvalidated(
                    unvalidated: #unvalidated_name
                ) -> ::core::result::Result<#name, Vec<#ety>> {
                    #body
                }
            }
        })
    }

    fn constructor(&self) -> TokenStream {
        let name = self.name;
        let fields = self.fields.iter().map(|f| f.name);
        quote! {
            #name {
                #( #fields, )*
            }
        }
    }

    fn unvalidated_constructor(&self) -> TokenStream {
        let name = self.name;
        let fields = self.fields.iter().map(|f| {
            let name = f.name;
            quote! {
                #name: unvalidated.#name
            }
        });
        quote! {
            #name {
                #( #fields, )*
            }
        }
    }

    fn validator_fields(&self) -> TokenStream {
        let validators_name = &self.validators_name;
        let ety = &self.custom_validation_error_ty;
        let validator_fields = self.fields.iter().map(|f| f.field_struct_def());
        quote! {
            let validators = #validators_name::<#ety> {
                #( #validator_fields, )*
            };
        }
    }

    fn match_validator_calls(&self) -> TokenStream {
        let fields = self.fields.iter().map(|f| {
            let field = f.name;
            let validator = &f.field_validator;
            if let Some(v) = validator {
                quote! {
                    (#v)(unvalidated.#field)
                }
            } else {
                quote! {
                    unvalidated.#field
                }
            }
        });
        quote! {
            #( #fields, )*
        }
    }

    fn match_validator_ok(&self) -> TokenStream {
        let fields = self.fields.iter().map(|f| {
            let name = f.name;
            let validator = &f.field_validator;
            if validator.is_some() {
                quote! {
                    Ok(#name)
                }
            } else {
                quote! {
                    #name
                }
            }
        });
        quote! {
            #( #fields, )*
        }
    }

    fn match_validator_nok(&self) -> TokenStream {
        let fields = self.fields.iter().map(|f| f.name);
        quote! {
            #( #fields, )*
        }
    }

    fn match_validator_error_push(&self) -> TokenStream {
        let fields = self.fields.iter().map(|f| {
            let name = f.name;
            let validator = &f.field_validator;
            if validator.is_some() {
                quote! {
                    if let Err(e) = #name {
                        errors.push(e);
                    }
                }
            } else {
                quote! {}
            }
        });
        quote! {
            #( #fields )*
        }
    }
}
