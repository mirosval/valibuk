//! # Correct-by-construction validators without boilerplate
//!
//! Correct-by-construction is a pattern that leverages the type system to guard against bugs that can come from improperly validating inputs. It does so by having an "unvalidated" type and a "validated" type. The only way of obtaining an instance of the validated type is to run all the defined validations on the unvalidated type. Then the correctness is achieved by using the correct type.
//!
//! ## Minimal Example
//!
//! ```
//! use valibuk::Validated;
//!
//! // 1. Having a T -> Result<T, E> validator
//! fn is_positive(i: i32) -> Result<i32, String> {
//!     if i > 0 {
//!         Ok(i)
//!     } else {
//!         Err("wrong".to_string())
//!     }
//! }
//!
//! // 3. Derive (1) the `unvalidated` type and a `std::convert::TryFrom` trait
//! #[derive(Validated)]
//! // 2. And a struct
//! struct A {
//!     #[validator(is_positive)] // Apply the function from (1) as validator
//!     a: i32,
//! }
//!
//! fn main() {
//!     let i: i32 = 1;
//!     // 4. Construct the instance of the original type from the unvalidated version
//!     let a = A::try_from(UnvalidatedA { a: i }).expect("valid instance");
//!     assert_eq!(a.a, i);
//! }
//! ```
//!
//! ## Walkthrough
//!
//! 1. Use `#[derive(Validated)]` to mark the struct you want to use in your code
//! 2. Mark any fields you want validated using `#[validator(<fn_name>)]` where `fn_name` refers to
//!    a validator function already in scope. This fn should have the form of `fn(T) -> Result<T,
//!    E>` where `T` is the type of the field being validated and `E` is the error type you wish to
//!    use
//! 3. Then to actually construct an instance of your struct, use
//!    `A::try_from(UnvalidatedA { ... })`, where `A` is your struct.
//!
//! ## Specifying your own error types
//!
//! By default, the error type returned by `try_from` is `Vec<String>`, which also forces the
//! validator functions to return `Result<T, String>`.
//!
//! You can plug in your own error type using `#[validation_error(MyValidationError)]` attribute
//! annotation.
extern crate valibuk_derive;

pub use valibuk_derive::Validated;
