# num_enum

Procedural macros to make inter-operation between primitives and enums easier.
This crate is no_std compatible.

[![crates.io](https://img.shields.io/crates/v/num_enum.svg)](https://crates.io/crates/num_enum)
[![Documentation](https://docs.rs/num_enum/badge.svg)](https://docs.rs/num_enum)
[![Build Status](https://travis-ci.org/illicitonion/num_enum.svg?branch=master)](https://travis-ci.org/illicitonion/num_enum)

## Turning an enum into a primitive

```rust
use num_enum::IntoPrimitive;

#[derive(IntoPrimitive)]
#[repr(u8)]
enum Number {
    Zero,
    One,
}

fn main() {
    let zero: u8 = Number::Zero.into();
    assert_eq!(zero, 0u8);
}
```

`num_enum`'s `IntoPrimitive` is more type-safe than using `as`, because `as` will silently truncate - `num_enum` only derives `From` for exactly the discriminant type of the enum.

## Attempting to turn a primitive into an enum with try_from

```rust
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum Number {
    Zero,
    One,
}

fn main() {
    let zero = Number::try_from(0u8);
    assert_eq!(zero, Ok(Number::Zero));

    let three = Number::try_from(3u8);
    assert_eq!(
        three.unwrap_err().to_string(),
        "No discriminant in enum `Number` matches the value `3`",
    );
}
```

### Variant alternatives

Sometimes a single enum variant might be representable by multiple numeric values.

The `#[num_enum(alternatives = [..])]` attribute allows you to define additional value alternatives for individual variants.

(The behavior of `IntoPrimitive` is unaffected by this attribute, it will always return the canonical value.)

```rust
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum Number {
    Zero = 0,
    #[num_enum(alternatives = [2])]
    OneOrTwo = 1,
}

fn main() {
    let zero = Number::try_from(0u8);
    assert_eq!(zero, Ok(Number::Zero));

    let one = Number::try_from(1u8);
    assert_eq!(one, Ok(Number::OneOrTwo));

    let two = Number::try_from(2u8);
    assert_eq!(two, Ok(Number::OneOrTwo));

    let three = Number::try_from(3u8);
    assert_eq!(
        three.unwrap_err().to_string(),
        "No discriminant in enum `Number` matches the value `3`",
    );
}
```

Range expressions are also supported for alternatives, but this requires enabling the `complex-expressions` feature:

```rust
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum Number {
    Zero = 0,
    #[num_enum(alternatives = [2..16])]
    Some = 1,
    #[num_enum(alternatives = [17, 18..=255])]
    Many = 16,
}

fn main() {
    let zero = Number::try_from(0u8);
    assert_eq!(zero, Ok(Number::Zero));

    let some = Number::try_from(15u8);
    assert_eq!(some, Ok(Number::Some));

    let many = Number::try_from(255u8);
    assert_eq!(many, Ok(Number::Many));
}
```

### Custom error types

`TryFromPrimitive` by default will use `num_enum::TryFromPrimitiveError` as its `Error` type.

If you want to use a different type, you can use an annotation for this:

```rust
use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = CustomError, constructor = CustomError::new))]
#[repr(u8)]
enum FirstNumber {
    Zero,
    One,
    Two,
}

struct CustomError {}

impl CustomError {
    fn new(value: u8) -> CustomError {
        CustomError {}
    }
}
```

## Safely turning a primitive into an exhaustive enum with from_primitive

If your enum has all possible primitive values covered, you can derive `FromPrimitive` for it (which auto-implement stdlib's `From`):

You can cover all possible values by:
* Having variants for every possible value
* Having a variant marked `#[num_enum(default)]`
* Having a variant marked `#[num_enum(catch_all)]`
* Having `#[num_enum(alternatives = [...])`s covering values not covered by a variant.

```rust
use num_enum::FromPrimitive;

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
enum Number {
    Zero,
    #[num_enum(default)]
    NonZero,
}

fn main() {
    assert_eq!(
        Number::Zero,
        Number::from(0_u8),
    );
    assert_eq!(
        Number::NonZero,
        Number::from(1_u8),
    );
}
```

### Default variant

Sometimes it is desirable to have an `Other` variant in an enum that acts as a kind of a wildcard matching all the value not yet covered by other variants.

The `#[num_enum(default)]` attribute (or the stdlib `#[default]` attribute) allows you to mark variant as the default.

(The behavior of `IntoPrimitive` is unaffected by this attribute, it will always return the canonical value.)

```rust
use num_enum::FromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
enum Number {
    Zero = 0,
    #[num_enum(default)]
    NonZero = 1,
}

fn main() {
    let zero = Number::from(0u8);
    assert_eq!(zero, Number::Zero);

    let one = Number::from(1u8);
    assert_eq!(one, Number::NonZero);

    let two = Number::from(2u8);
    assert_eq!(two, Number::NonZero);
}
```

Only `FromPrimitive` pays attention to `default` attributes, `TryFromPrimitive` ignores them.

### Catch-all variant

Sometimes it is desirable to have an `Other` variant which holds the otherwise un-matched value as a field.

The `#[num_enum(catch_all)]` attribute allows you to mark at most one variant for this purpose. The variant it's applied to must be a tuple variant with exactly one field matching the `repr` type.

```rust
use num_enum::FromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
enum Number {
    Zero = 0,
    #[num_enum(catch_all)]
    NonZero(u8),
}

fn main() {
    let zero = Number::from(0u8);
    assert_eq!(zero, Number::Zero);

    let one = Number::from(1u8);
    assert_eq!(one, Number::NonZero(1_u8));

    let two = Number::from(2u8);
    assert_eq!(two, Number::NonZero(2_u8));
}
```

As this is naturally exhaustive, this is only supported for `FromPrimitive`, not also `TryFromPrimitive`.

## Unsafely turning a primitive into an enum with unchecked_transmute_from

If you're really certain a conversion will succeed (and have not made use of `#[num_enum(default)]` or `#[num_enum(alternatives = [..])]`
for any of its variants), and want to avoid a small amount of overhead, you can use unsafe code to do this conversion.
Unless you have data showing that the match statement generated in the `try_from` above is a bottleneck for you,
you should avoid doing this, as the unsafe code has potential to cause serious memory issues in your program.

```rust
use num_enum::UnsafeFromPrimitive;

#[derive(Debug, Eq, PartialEq, UnsafeFromPrimitive)]
#[repr(u8)]
enum Number {
    Zero,
    One,
}

fn main() {
    assert_eq!(
        unsafe { Number::unchecked_transmute_from(0_u8) },
        Number::Zero,
    );
    assert_eq!(
        unsafe { Number::unchecked_transmute_from(1_u8) },
        Number::One,
    );
}

unsafe fn undefined_behavior() {
    let _ = Number::unchecked_transmute_from(2); // 2 is not a valid discriminant!
}
```

Note that this derive ignores any `default`, `catch_all`, and `alternatives` attributes on the enum.
If you need support for conversions from these values, you should use `TryFromPrimitive` or `FromPrimitive`.

This means, for instance, that the following is undefined behaviour:

```rust,no_run
use num_enum::UnsafeFromPrimitive;

#[derive(UnsafeFromPrimitive)]
#[repr(u8)]
enum Number {
    Zero = 0,

    // Same for `#[num_enum(catch_all)]`, and `#[num_enum(alternatives = [2, ...])]`
    #[num_enum(default)]
    One = 1,
}
let _undefined_behavior = unsafe { Number::unchecked_transmute_from(2) };
```

## Optional features

Some enum values may be composed of complex expressions, for example:

```rust
enum Number {
    Zero = (0, 1).0,
    One = (0, 1).1,
}
```

To cut down on compile time, these are not supported by default, but if you enable the `complex-expressions`
feature of your dependency on `num_enum`, these should start working.

## License

num_enum may be used under your choice of the BSD 3-clause, Apache 2, or MIT license.
