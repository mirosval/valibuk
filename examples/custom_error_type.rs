use valibuk::Validated;

// Define a custom error type
#[derive(Debug, PartialEq)]
enum MyValidationError {
    ZeroError,
    NegativeError,
}

// Define a validator using String as error type
fn is_positive(i: i32) -> Result<i32, MyValidationError> {
    match i {
        i if i == 0 => Err(MyValidationError::ZeroError),
        i if i < 0 => Err(MyValidationError::NegativeError),
        i => Ok(i),
    }
}

#[derive(Validated, Debug, PartialEq)]
#[validation_error(MyValidationError)] // Derive using this custom error type
struct A {
    #[validator(is_positive)] // Use the String validator
    a: i32,
}

fn main() {
    {
        let a: i32 = 1;
        // This should fail, because validator error type does not match the defined custom error type
        let instance = A::try_from(UnvalidatedA { a }).expect("valid instance");
        assert_eq!(instance.a, a);
    }

    {
        let a: i32 = 0;
        let instance = A::try_from(UnvalidatedA { a });
        assert!(instance
            .expect_err("should fail")
            .contains(&MyValidationError::ZeroError))
    }
}
