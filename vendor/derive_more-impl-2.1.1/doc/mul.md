# What `#[derive(Mul)]` generates

> **NOTE**: `Div`, `Rem`, `Shr` and `Shl` derives are fully equivalent
>           to the `Mul` derive described below.

In contrast with deriving `Add`, there are two ways to derive `Mul`: scalar and structural.




## Scalar implementation

By default, deriving `Mul` is quite different from deriving `Add`. It is not used
to multiply two structs together. Instead, it will normally multiply a struct,
which can have multiple fields, with a single primitive type (e.g. a `u64`).
A new struct is then created with all the fields from the previous struct
multiplied by this other value.

A simple way of explaining the reasoning behind this difference between `Add`
and `Mul` deriving, is looking at arithmetic on meters. One meter can be added
to one meter, to get two meters. Also, one meter times two would be two meters,
but one meter times one meter would be one square meter. As this second case
clearly requires more knowledge about the meaning of the type in question
deriving for this is not implemented.


### Structs

Deriving `Mul` for a struct with a single field multiplies its field with
anything `Mul`tiplicable, producing the resulting struct.


```rust
# use derive_more::Mul;
#
#[derive(Mul)]
struct MyInt(i32);

#[derive(Mul)]
struct Point1D {
    x: i32,
}
```
This generates code equivalent to:
```rust
# use std::ops::Mul;
#
# struct MyInt(i32);
#
# struct Point1D {
#     x: i32,
# }
#
impl<Rhs> Mul<Rhs> for MyInt
where
    i32: Mul<Rhs, Output = i32>
{
    type Output = Self;

    fn mul(self, rhs: Rhs) -> Self::Output {
        match self {
            Self(self_0) => Self(Mul::mul(self_0, rhs)),
        }
    }
}

impl<Rhs> Mul<Rhs> for Point1D
where
    i32: Mul<Rhs, Output = i32>
{
    type Output = Self;

    fn mul(self, rhs: Rhs) -> Self::Output {
        match self {
            Self { x: self_0 } => Self { x: Mul::mul(self_0, rhs) },
        }
    }
}
```

The behaviour is slightly different for multiple fields, since the right hand
side of the multiplication now needs the `Copy` trait.

```rust
# use derive_more::Mul;
#
#[derive(Mul)]
struct MyInts(i32, i32);

#[derive(Mul)]
struct Point2D {
    x: i32,
    y: i32,
}
```
This generates code equivalent to:
```rust
# use std::ops::Mul;
#
# struct MyInts(i32, i32);
#
# struct Point2D {
#     x: i32,
#     y: i32,
# }
#
impl<Rhs: Copy> Mul<Rhs> for MyInts
where
    i32: Mul<Rhs, Output = i32>
{
    type Output = Self;

    fn mul(self, rhs: Rhs) -> Self::Output {
        match self {
            Self(self_0, self_1) => {
                Self(Mul::mul(self_0, rhs), Mul::mul(self_1, rhs))
            }
        }
    }
}

impl<Rhs: Copy> Mul<Rhs> for Point2D
where
    i32: Mul<Rhs, Output = i32>
{
    type Output = Self;

    fn mul(self, rhs: Rhs) -> Self::Output {
        match self {
            Self { x: self_0, y: self_1 } => Self {
                x: Mul::mul(self_0, rhs),
                y: Mul::mul(self_1, rhs),
            },
        }
    }
}
```

#### Ignoring

Sometimes a struct needs to hold a field (most commonly `PhantomData`) that doesn't
participate in a scalar `Mul` implementation. Such field could be ignored using
the `#[mul(skip)]` attribute.

```rust
# use core::marker::PhantomData;
# use derive_more::Mul;
#
#[derive(Mul)]
struct TupleWithZst<T>(i32, #[mul(skip)] PhantomData<T>);

#[derive(Mul)]
struct StructWithZst<T> {
    x: i32,
    #[mul(skip)] // or #[mul(ignore)]
    _marker: PhantomData<T>,
}
```




### Enums

Deriving scalar `Mul` implementation for enums is not (yet) supported.

Although it shouldn't be impossible no effort has been put into this yet.




## Structural implementation

Specifying the `#[mul(forward)]` attribute generates a structural `Mul`
implementation with the same semantics as `Add`: `Mul`tiplying respective
fields together and producing a new value with these fields.


### Structs

```rust
# use derive_more::Mul;
#
#[derive(Mul)]
#[mul(forward)]
struct MyInts(i32, i32);

#[derive(Mul)]
#[mul(forward)]
struct Point2D {
    x: i32,
    y: i32,
}
```
This generates code equivalent to:
```rust
# use std::ops::Mul;
#
# struct MyInts(i32, i32);
#
# struct Point2D {
#     x: i32,
#     y: i32,
# }
#
impl Mul for MyInts {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self(self_0, self_1), Self(rhs_0, rhs_1)) => {
                Self(Mul::mul(self_0, rhs_0), Mul::mul(self_1, rhs_1))
            }
        }
    }
}

impl Mul for Point2D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self { x: self_0, y: self_1 }, Self { x: rhs_0, y: rhs_1 }) => {
                Self { x: Mul::mul(self_0, rhs_0), y: Mul::mul(self_1, rhs_1) }
            }
        }
    }
}
```
The behaviour is similar with more or less fields.

#### Ignoring

Sometimes a struct needs to hold a field (most commonly `PhantomData`) that doesn't
participate in a structural `Mul` implementation. Such field could be ignored using
the `#[mul(skip)]` attribute.

```rust
# use core::marker::PhantomData;
# use derive_more::Mul;
#
#[derive(Mul)]
#[mul(forward)]
struct TupleWithZst<T>(i32, #[mul(skip)] PhantomData<T>);

#[derive(Mul)]
#[mul(forward)]
struct StructWithZst<T> {
    x: i32,
    #[mul(skip)] // or #[mul(ignore)]
    _marker: PhantomData<T>,
}
```


### Enums

For enums each variant can be `Mul`tiplied structurally in a similar way to another
instance of the same variant. There's one big difference however: it returns
a `Result<EnumType>`, because an error is returned when two different variants are
`Mul`tiplied together.

```rust
# use derive_more::Mul;
#
#[derive(Mul)]
#[mul(forward)]
enum MixedInts {
    BigInt(i64),
    NamedSmallInts { x: i32, y: i32 },
    Unit,
}
```
This generates code equivalent to:
```rust
# use std::ops::Mul;
#
# enum MixedInts {
#     BigInt(i64),
#     NamedSmallInts { x: i32, y: i32 },
#     Unit,
# }
#
impl Mul for MixedInts {
    type Output = Result<Self, derive_more::BinaryError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::BigInt(self_0), Self::BigInt(rhs_0)) => {
                Ok(Self::BigInt(Mul::mul(self_0, rhs_0)))
            }
            (Self::NamedSmallInts { x: self_0, y: self_1 },
             Self::NamedSmallInts { x: rhs_0, y: rhs_1 }) => {
                Ok(Self::NamedSmallInts {
                    x: Mul::mul(self_0, rhs_0),
                    y: Mul::mul(self_1, rhs_1),
                })
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

Also note the `Unit` variant that throws a `derive_more::UnitError` when `Mul`tiplying it
to itself.

#### Ignoring

Similarly to structs, enum fields could be ignored using the `#[add(skip)]` attribute.

```rust
# use derive_more::Mul;
#
struct I32(i32); // doesn't implement `Mul`

#[derive(Mul)]
#[mul(forward)]
enum MixedInts {
    TwoSmallInts(i32, #[mul(skip)] I32),
    NamedSmallInts {
        #[mul(skip)] // or #[mul(ignore)]
        x: I32,
        y: i32,
    },
}
```

> **NOTE**: Ignoring all the fields of a variant or ignoring the variant itself is not allowed
>           (results in a compilation error).
