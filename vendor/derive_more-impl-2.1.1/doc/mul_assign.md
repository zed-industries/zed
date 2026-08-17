# What `#[derive(MulAssign)]` generates

> **NOTE**: `DivAssign`, `RemAssign`, `ShrAssign` and `ShlAssign` derives are fully
>           equivalent to the `MulAssign` derive described below.

Deriving `MulAssign` is very similar to deriving `Mul`. The difference is that it
mutates the existing instance instead of creating a new one.




## Scalar implementation


### Structs

Deriving `MulAssign` for a struct with multiple fields multiplies its fields with
anything multiplicable, mutating them in-place.

```rust
# use derive_more::MulAssign;
#
#[derive(MulAssign)]
struct MyInts(i32, i32);

#[derive(MulAssign)]
struct Point2D {
    x: i32,
    y: i32,
}
```
This generates code equivalent to:
```rust
# use std::ops::MulAssign;
#
# struct MyInts(i32, i32);
#
# struct Point2D {
#     x: i32,
#     y: i32,
# }
#
impl<Rhs: Copy> MulAssign<Rhs> for MyInts
where
    i32: MulAssign<Rhs>
{
    fn mul_assign(&mut self, rhs: Rhs) {
        match self {
            Self(self_0, self_1) => {
                MulAssign::mul_assign(self_0, rhs);
                MulAssign::mul_assign(self_1, rhs);
            }
        }
    }
}

impl<Rhs: Copy> MulAssign<Rhs> for Point2D
where
    i32: MulAssign<Rhs>
{
    fn mul_assign(&mut self, rhs: Rhs) {
        match self {
            Self { x: self_0, y: self_1 } => {
                MulAssign::mul_assign(self_0, rhs);
                MulAssign::mul_assign(self_1, rhs);
            },
        }
    }
}
```

Note, that `Copy`is not required for `Rhs` when the struct has only a single field.

#### Ignoring

Sometimes a struct needs to hold a field (most commonly `PhantomData`) that doesn't
participate in a scalar `MulAssign` implementation. Such field could be ignored using
the `#[mul_assign(skip)]` attribute.

```rust
# use core::marker::PhantomData;
# use derive_more::MulAssign;
#
#[derive(MulAssign)]
struct TupleWithZst<T>(i32, #[mul_assign(skip)] PhantomData<T>);

#[derive(MulAssign)]
struct StructWithZst<T> {
    x: i32,
    #[mul_assign(skip)] // or #[mul_assign(ignore)]
    _marker: PhantomData<T>,
}
```


### Enums

Deriving scalar `MulAssign` implementation for enums is not (yet) supported (in the same manner as deriving `Mul`).

Although it shouldn't be impossible no effort has been put into this yet.




## Structural implementation

Specifying the `#[mul_assign(forward)]` attribute generates a structural `MulAssign`
implementation with the same semantics as `AddAssign`: `MulAssign`ing the respective
fields, mutating them in-place.


### Structs

```rust
# use derive_more::MulAssign;
#
#[derive(MulAssign)]
#[mul_assign(forward)]
struct MyInts(i32, i32);

#[derive(MulAssign)]
#[mul_assign(forward)]
struct Point2D {
    x: i32,
    y: i32,
}
```
This generates code equivalent to:
```rust
# use std::ops::MulAssign;
#
# struct MyInts(i32, i32);
#
# struct Point2D {
#     x: i32,
#     y: i32,
# }
#
impl MulAssign for MyInts {
    fn mul_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self(self_0, self_1), Self(rhs_0, rhs_1)) => {
                MulAssign::mul_assign(self_0, rhs_0);
                MulAssign::mul_assign(self_1, rhs_1);
            }
        }
    }
}

impl MulAssign for Point2D {
    fn mul_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self { x: self_0, y: self_1 }, Self { x: rhs_0, y: rhs_1 }) => {
                MulAssign::mul_assign(self_0, rhs_0);
                MulAssign::mul_assign(self_1, rhs_1);
            }
        }
    }
}
```
The behaviour is similar with more or less fields.

#### Ignoring

Sometimes a struct needs to hold a field (most commonly `PhantomData`) that doesn't
participate in a structural `MulAssign` implementation. Such field could be ignored using
the `#[mul_assign(skip)]` attribute.

```rust
# use core::marker::PhantomData;
# use derive_more::MulAssign;
#
#[derive(MulAssign)]
#[mul_assign(forward)]
struct TupleWithZst<T>(i32, #[mul_assign(skip)] PhantomData<T>);

#[derive(MulAssign)]
#[mul_assign(forward)]
struct StructWithZst<T> {
    x: i32,
    #[mul_assign(skip)] // or #[mul_assign(ignore)]
    _marker: PhantomData<T>,
}
```


### Enums

Deriving `AddAssign` structurally is not (yet) supported for enums.

This is mostly due to the fact that it is not trivial convert the `Mul`
derivation code, because that returns a `Result<EnumType>` instead of an `EnumType`.
Handling the case where it errors would be hard and maybe impossible.
