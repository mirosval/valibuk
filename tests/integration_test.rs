use builder_validator::Validated;

fn is_positive(i: i32) -> Result<i32, String> {
    if i > 0 {
        Ok(i)
    } else {
        Err("wrong".to_string())
    }
}

#[test]
fn test_no_validated_fields() {
    #[derive(Validated)]
    struct A {
        a: i32,
    }
    let i: i32 = 1;
    let a = A::from_unvalidated(UnvalidatedA { a: i }).expect("valid instance");
    assert_eq!(a.a, i);
}

#[test]
fn test_one_validated_field() {
    #[derive(Validated)]
    struct A {
        #[validator(is_positive)]
        a: i32,
    }
    let a: i32 = 1;
    let instance = A::from_unvalidated(UnvalidatedA { a }).expect("valid instance");
    assert_eq!(instance.a, a);
}

#[test]
fn test_one_validated_one_not_validated_field() {
    #[derive(Validated)]
    struct A {
        #[validator(is_positive)]
        a: i32,
        b: u8,
    }
    let a: i32 = 1;
    let b: u8 = 2;
    let instance = A::from_unvalidated(UnvalidatedA { a, b }).expect("valid instance");
    assert_eq!(instance.a, a);
    assert_eq!(instance.b, b);
}
