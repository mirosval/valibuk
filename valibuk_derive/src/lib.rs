use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use valibuk_core::valibuk_core;

#[proc_macro_error]
#[proc_macro_derive(Validated, attributes(validator, validation_error))]
pub fn valibuk_derive(input: TokenStream) -> TokenStream {
    valibuk_core(input.into()).into()
}
