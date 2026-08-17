# What `#[derive(AddAssign)]` generates

> **NOTE**: `SubAssign`, `BitAndAssign`, `BitOrAssign` and `BitXorAssign` derives
>           are fully equivalent to the `AddAssign` derive described below.

Deriving `AddAssign` works very similar to deriving `Add`. The difference is that
it mutates the existing value in-place instead of creating a new one.




## Structs

The derived `AddAssign` implementation will allow two structs of the same type to
be added together, mutating the first one in-place. This is done by `AddAssign`ing
their respective fields.

```rust
# use derive_more::AddAssign;
#
#[derive(AddAssign)]
struct MyInts(i32, i32);

#[derive(AddAssign)]
struct Point2D {
    x: i32,
    y: i32,
}
```
This generates code equivalent to:
```rust
# use std::ops::AddAssign;
#
# struct MyInts(i32, i32);
#
# struct Point2D {
#     x: i32,
#     y: i32,
# }
#
impl AddAssign for MyInts {
    fn add_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self(self_0, self_1), Self(rhs_0, rhs_1)) => {
                AddAssign::add_assign(self_0, rhs_0);
                AddAssign::add_assign(self_1, rhs_1);
            }
        }
    }
}

impl AddAssign for Point2D {
    fn add_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self { x: self_0, y: self_1 },
             Self { x: rhs_0, y: rhs_1 }) => {
                AddAssign::add_assign(self_0, rhs_0);
                AddAssign::add_assign(self_1, rhs_1);
            }
        }
    }
}
```

The behaviour is similar with more or less fields.


### Ignoring

Sometimes a struct needs to hold a field (most commonly `PhantomData`) that doesn't
participate in `AddAssign` implementation. Such field could be ignored using the
`#[add_assign(skip)]` attribute.

```rust
# use core::marker::PhantomData;
# use derive_more::AddAssign;
#
#[derive(AddAssign)]
struct TupleWithZst<T>(i32, #[add_assign(skip)] PhantomData<T>);

#[derive(AddAssign)]
struct StructWithZst<T> {
    x: i32,
    #[add_assign(skip)] // or #[add_assign(ignore)]
    _marker: PhantomData<T>,
}
```




## Enums

Deriving `AddAssign` is not (yet) supported for enums.

This is mostly due to the fact that it is not trivial convert the `Add`
derivation code, because that returns a `Result<EnumType>` instead of an `EnumType`.
Handling the case where it errors would be hard and maybe impossible.
