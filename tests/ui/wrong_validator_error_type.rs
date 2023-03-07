use valibuk::Validated;

#[derive(Validated)]
struct A {
    #[validator(|a| a > 0, "&str instead of String")] // Use inline boolean validator
    a: i32,
}

fn main() {
    let a: i32 = 1;
    // This should fail, because the error in the validator is &str, not String
    let instance = A::try_from(UnvalidatedA { a }).expect("valid instance");
    assert_eq!(instance.a, a);
}
