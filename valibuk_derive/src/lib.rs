use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use valibuk_core::valibuk_core;

/// The main macro enabling validation on a struct.
///
/// Only structs with named fields are supported at this time
///
/// The available attributes:
/// `validator` is set on a field and specifies the function to be run for validation, the function
/// should return `Result<T, E>`, where T is the type of the field under validation and E is the
/// error type set by `validation_error` attribute, or `String` by default.
#[proc_macro_error]
#[proc_macro_derive(Validated, attributes(validator, validation_error))]
pub fn valibuk_derive(input: TokenStream) -> TokenStream {
    valibuk_core(input.into()).into()
}
