use valibuk::Validated;

// Define a validator using String as error type
fn is_positive(i: i32) -> Result<i32, String> {
    if i > 0 {
        Ok(i)
    } else {
        Err("wrong".to_string())
    }
}

// Define a custom error type
#[derive(Debug)]
struct E;

#[derive(Validated)]
#[validation_error(E)] // Derive using this custom error type
struct A {
    #[validator(is_positive)] // Use the String validator
    a: i32,
}

fn main() {
    let a: i32 = 1;
    // This should fail, because validator error type does not match the defined custom error type
    let instance = A::from_unvalidated(UnvalidatedA { a }).expect("valid instance");
    assert_eq!(instance.a, a);
}
