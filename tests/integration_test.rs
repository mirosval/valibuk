use valibuk::Validated;

fn is_positive(i: i32) -> Result<i32, String> {
    if i > 0 {
        Ok(i)
    } else {
        Err("wrong".to_string())
    }
}

fn is_at_least<'a>(n: usize) -> impl Fn(&'a str) -> Result<&'a str, String> {
    move |s| {
        if s.len() >= n {
            Ok(s)
        } else {
            Err("wrong".to_string())
        }
    }
}

fn is_at_least_3<'a>(a: &'a str) -> Result<&str, String> {
    is_at_least::<'a>(3)(a)
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

#[test]
fn test_no_validated_fields() {
    #[derive(Validated)]
    struct A {
        a: i32,
    }
    let i: i32 = 1;
    let a = A::try_from(UnvalidatedA { a: i }).expect("valid instance");
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
    let instance = A::try_from(UnvalidatedA { a }).expect("valid instance");
    assert_eq!(instance.a, a);
}

#[test]
fn test_one_validated_one_not_validated_field() {
    #[derive(Validated)]
    struct A {
        #[validator(is_positive)]
        a: i32,
        _b: u8, // for whatever reason if this is just `b`, I get a warning
                // about unused variables?
    }
    let a: i32 = 1;
    let b: u8 = 2;
    let instance = A::try_from(UnvalidatedA { a, _b: b }).expect("valid instance");
    assert_eq!(instance.a, a);
    assert_eq!(instance._b, b);
}

#[test]
fn test_lifetime() {
    #[derive(Validated)]
    struct A<'a> {
        #[validator(is_at_least_3)]
        a: &'a str,
    }
    let a: &str = "aaa";
    let instance = A::try_from(UnvalidatedA { a }).expect("valid instance");
    assert_eq!(instance.a, a);
}

#[test]
fn test_generics() {
    #[derive(Validated)]
    struct A<T> {
        #[validator(is_positive)]
        a: i32,
        _b: T,
    }
    let a: i32 = 1;
    let b: u8 = 2;
    let instance = A::try_from(UnvalidatedA { a, _b: b }).expect("valid instance");
    assert_eq!(instance.a, a);
    assert_eq!(instance._b, b);
}

#[test]
fn test_inline_validator() {
    #[derive(Validated, Debug, PartialEq)]
    struct A {
        #[validator(|a| if a > 0 { Ok(a) } else { Err("validation err".to_string()) })]
        a: i32,
    }
    {
        // positive case
        let a: i32 = 1;
        let instance = A::try_from(UnvalidatedA { a }).expect("valid instance");
        assert_eq!(instance.a, a);
    }
    {
        // negative case
        let a: i32 = -1;
        let instance: Result<A, Vec<String>> = A::try_from(UnvalidatedA { a });
        let expected = Err(vec!["validation err".to_string()]);
        assert_eq!(instance, expected);
    }
}

#[test]
fn test_boolean_inline_validator() {
    #[derive(Validated, Debug, PartialEq)]
    struct A {
        #[validator(|a| a > 0, "Error: a should be positive".to_string())]
        a: i32,
    }
    {
        // positive case
        let a: i32 = 1;
        let instance = A::try_from(UnvalidatedA { a }).expect("valid instance");
        assert_eq!(instance.a, a);
    }
    {
        // negative case
        let a: i32 = -1;
        let instance = A::try_from(UnvalidatedA { a });
        let expected = Err(vec!["Error: a should be positive".to_string()]);
        assert_eq!(instance, expected);
    }
}

#[test]
fn test_boolean_inline_validator_custom_error() {
    #[derive(Debug, PartialEq)]
    struct E;
    #[derive(Validated, Debug, PartialEq)]
    #[validation_error(E)]
    struct A {
        #[validator(|a| a > 0, E)]
        a: i32,
    }
    {
        // negative case
        let a: i32 = -1;
        let instance = A::try_from(UnvalidatedA { a });
        let expected = Err(vec![E]);
        assert_eq!(instance, expected);
    }
}
