# What `#[derive(Add)]` generates

> **NOTE**: `Sub`, `BitAnd`, `BitOr` and `BitXor` derives are fully equivalent
>           to the `Add` derive described below.

Deriving `Add` works by adding two values structurally (field by field, according
to their type structure).




## Structs

The derived `Add` implementation will allow two structs of the same type to be
added together. This is done by `Add`ing their respective fields together and
creating a new struct with those values.

```rust
# use derive_more::Add;
#
#[derive(Add)]
struct MyInts(i32, i32);

#[derive(Add)]
struct Point2D {
    x: i32,
    y: i32,
}
```
This generates code equivalent to:
```rust
# use std::ops::Add;
#
# struct MyInts(i32, i32);
#
# struct Point2D {
#     x: i32,
#     y: i32,
# }
#
impl Add for MyInts {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self(self_0, self_1), Self(rhs_0, rhs_1)) => {
                Self(Add::add(self_0, rhs_0), Add::add(self_1, rhs_1))
            }
        }
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self { x: self_0, y: self_1 }, Self { x: rhs_0, y: rhs_1 }) => {
                Self { x: Add::add(self_0, rhs_0), y: Add::add(self_1, rhs_1) }
            }
        }
    }
}
```

The behaviour is similar with more or less fields.


### Ignoring

Sometimes a struct needs to hold a field (most commonly `PhantomData`) that doesn't
participate in `Add` implementation. Such field could be ignored using the `#[add(skip)]`
attribute.

```rust
# use core::marker::PhantomData;
# use derive_more::Add;
#
#[derive(Add)]
struct TupleWithZst<T>(i32, #[add(skip)] PhantomData<T>);

#[derive(Add)]
struct StructWithZst<T> {
    x: i32,
    #[add(skip)] // or #[add(ignore)]
    _marker: PhantomData<T>,
}
```




## Enums

For enums each variant can be `Add`ed in a similar way to another instance of the
same variant. There's one big difference however: it returns a `Result<EnumType>`,
because an error is returned when two different variants are `Add`ed together.

```rust
# use derive_more::Add;
#
#[derive(Add)]
enum MixedInts {
    SmallInt(i32),
    BigInt(i64),
    TwoSmallInts(i32, i32),
    NamedSmallInts { x: i32, y: i32 },
    UnsignedOne(u32),
    UnsignedTwo(u32),
    Unit,
}
```
This generates code equivalent to:
```rust
# use std::ops::Add;
#
# enum MixedInts {
#     SmallInt(i32),
#     BigInt(i64),
#     TwoSmallInts(i32, i32),
#     NamedSmallInts { x: i32, y: i32 },
#     UnsignedOne(u32),
#     UnsignedTwo(u32),
#     Unit,
# }
#
impl Add for MixedInts {
    type Output = Result<Self, derive_more::BinaryError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::SmallInt(self_0), Self::SmallInt(rhs_0)) => {
                Ok(Self::SmallInt(Add::add(self_0, rhs_0)))
            }
            (Self::BigInt(self_0), Self::BigInt(rhs_0)) => {
                Ok(Self::BigInt(Add::add(self_0, rhs_0)))
            }
            (Self::TwoSmallInts(self_0, self_1), Self::TwoSmallInts(rhs_0, rhs_1)) => {
                Ok(Self::TwoSmallInts(Add::add(self_0, rhs_0), Add::add(self_1, rhs_1)))
            }
            (Self::NamedSmallInts { x: self_0, y: self_1 },
             Self::NamedSmallInts { x: rhs_0, y: rhs_1 }) => {
                Ok(Self::NamedSmallInts {
                    x: Add::add(self_0, rhs_0),
                    y: Add::add(self_1, rhs_1),
                })
            }
            (Self::UnsignedOne(self_0), Self::UnsignedOne(rhs_0)) => {
                Ok(Self::UnsignedOne(Add::add(self_0, rhs_0)))
            }
            (Self::UnsignedTwo(self_0), Self::UnsignedTwo(rhs_0)) => {
                Ok(Self::UnsignedTwo(Add::add(self_0, rhs_0)))
            }
            (Self::Unit, Self::Unit) => Err(derive_more::BinaryError::Unit(
                derive_more::UnitError::new("add"),
            )),
            _ => Err(derive_more::BinaryError::Mismatch(
                derive_more::WrongVariantError::new("add"),
            )),
        }
    }
}
```

Also note the `Unit` variant that throws a `derive_more::UnitError` when `Add`ing it to itself.


### Ignoring

Similarly to structs, enum fields could be ignored using the `#[add(skip)]` attribute.

```rust
# use derive_more::Add;
#
struct I32(i32); // doesn't implement `Add`

#[derive(Add)]
enum MixedInts {
    TwoSmallInts(i32, #[add(skip)] I32),
    NamedSmallInts {
        #[add(skip)] // or #[add(ignore)]
        x: I32,
        y: i32,
    },
}
```

> **NOTE**: Ignoring all the fields of a variant or ignoring the variant itself is not allowed
>           (results in a compilation error).
