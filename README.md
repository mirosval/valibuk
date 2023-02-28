# Valibuk

Valibuk is a library and a set of macros implementing the correct-by-construction pattern.

Correct-by-construction is a pattern that leverages the type system to guard against bugs that can come from improperly validating inputs. It does so by having an "unvalidated" type and a "validated" type. The only way of obtaining an instance of the validated type is to run all the defined validations on the unvalidated type. Then the correctness is achieved by using the correct type.

## A small example

https://github.com/mirosval/valibuk/blob/c78d7578bac0d46874496fe9fd4bdb4b88b06220/examples/minimal.rs

See more examples in `tests` and `examples`.

## TODO

- [x] Move validator registrations into macro annotations
- [x] Support fields without validating
- [x] Add UI tests using trybuild
- [x] Support structs with lifetime params
- [x] Support structs with generics
- [ ] Support global validators (take the whole struct)
- [ ] Add validator combinators
