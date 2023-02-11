use builder_validator_core::builder_validator_core;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_derive(Validated, attributes(validator, validation_error))]
pub fn builder_validator_derive(input: TokenStream) -> TokenStream {
    builder_validator_core(input.into()).into()
}
