# Valibuk

Valibuk is a library and a set of macros implementing the correct-by-construction pattern.

Correct-by-construction is a pattern that leverages the type system to guard against bugs that can come from improperly validating inputs. It does so by having an "unvalidated" type and a "validated" type. The only way of obtaining an instance of the validated type is to run all the defined validations on the unvalidated type. Then the correctness is achieved by using the correct type.

## A small example

```rust
// 1. Having a T -> Result<T, E> validator
fn is_positive(i: i32) -> Result<i32, String> {
  if i > 0 {
    Ok(i)
  } else {
    Err("wrong".to_string())
  }
}

// 3. Derive (1) the `unvalidated` type and a `from_unvalidated` function
#[derive(Validated)]
// 2. And a struct 
struct A {
  a: i32,
}
let i: i32 = 1;
// 4. Construct the instance of the original type from the unvalidated version
let a = A::from_unvalidated(UnvalidatedA { a: i }).expect("valid instance");
assert_eq!(a.a, i);
```

## TODO

- [x] Move validator registrations into macro annotations
- [x] Support fields without validating
- [x] Add UI tests using trybuild
- [x] Support structs with lifetime params
- [x] Support structs with generics
- [ ] Support global validators (take the whole struct)
- [ ] Add validator combinators
