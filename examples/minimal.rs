use valibuk::Validated;

// 1. Having a T -> Result<T, E> validator
fn is_positive(i: i32) -> Result<i32, String> {
    if i > 0 {
        Ok(i)
    } else {
        Err("wrong".to_string())
    }
}

// 3. Derive (1) the `unvalidated` type and a `std::convert::TryFrom` trait
#[derive(Validated)]
// 2. And a struct
struct A {
    #[validator(is_positive)] // Apply the function from (1) as validator
    a: i32,
}

fn main() {
    let i: i32 = 1;
    // 4. Construct the instance of the original type from the unvalidated version
    let a = A::try_from(UnvalidatedA { a: i }).expect("valid instance");
    assert_eq!(a.a, i);
}
