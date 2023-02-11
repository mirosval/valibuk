use builder_validator::Validated;

#[test]
fn test_no_validated_fields() {
    #[derive(Validated)]
    struct A {
        a: i32,
    }
    let a = A::from_unvalidated(UnvalidatedA { a: 0 }).expect("valid instance");
    assert_eq!(a.a, 0);
}
