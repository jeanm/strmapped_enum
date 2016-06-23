# String-mapped Enum

This crate exports the macro `strmapped_enum!` that builds and enum and automatically implements `std::str::FromStr` and `std::fmt::Display` for its variants.

## Usage

```rust
#[macro_use]
extern crate strmapped_enum;

use std::str::FromStr;

strmapped_enum!(
pub enum Mark {
    Exclamation = "!",
    Question = "?",
}
);

assert_eq!(Mark::Exclamation.to_str(), "!");
assert_eq!(Mark::from_str("?").unwrap(), Mark::Question);
```

## License

This crate is distributed under the terms of both the MIT license and the Apache License (Version 2.0).
