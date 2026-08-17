# What `#[derive(DerefMut)]` generates

Deriving `Deref` only works for a single field of a struct, or a single field of
each variant of an enum.
Furthermore it requires that the type also implements `Deref`, so usually
`Deref` should also be derived.
The resulting implementation of `Deref` will allow you to mutably dereference
the struct its member directly.

1. Dereferencing to the field, i.e. like if your type was a reference type.
2. Doing a dereference on the field, for when the field itself is a reference
   type like `&mut` and `Box`.

With `#[deref_mut]` or `#[deref_mut(ignore)]` it's possible to indicate the
field that you want to derive `DerefMut` for.




## Example usage

```rust
# use derive_more::{Deref, DerefMut};
#
#[derive(Deref, DerefMut)]
struct Num {
    num: i32,
}

#[derive(Deref, DerefMut)]
enum Enum {
    V1(i32),
    V2 { num: i32 },
}

#[derive(Deref, DerefMut)]
#[deref(forward)]
#[deref_mut(forward)]
struct MyBoxedInt(Box<i32>);

#[derive(Deref, DerefMut)]
#[deref(forward)]
#[deref_mut(forward)]
enum MyBoxedIntEnum {
    V1(Box<i32>),
    V2 { num: Box<i32> },
}

// You can specify the field you want to derive DerefMut for
#[derive(Deref, DerefMut)]
struct CoolVec {
    cool: bool,
    #[deref]
    #[deref_mut]
    vec: Vec<i32>,
}

#[derive(Deref, DerefMut)]
enum CoolVecEnum {
    V1(Vec<i32>),
    V2 {
        cool: bool,
        #[deref]
        #[deref_mut]
        vec: Vec<i32>,
    },
}

let mut num = Num{num: 123};
let mut boxed = MyBoxedInt(Box::new(123));
let mut cool_vec = CoolVec{cool: true, vec: vec![123]};
*num += 123;
assert_eq!(246, *num);
*boxed += 1000;
assert_eq!(1123, *boxed);
cool_vec.push(456);
assert_eq!(vec![123, 456], *cool_vec);
```




## Structs

When deriving a non-forwarded `DerefMut` for a struct:

```rust
# use derive_more::{Deref, DerefMut};
#
#[derive(Deref, DerefMut)]
struct CoolVec {
    cool: bool,
    #[deref]
    #[deref_mut]
    vec: Vec<i32>,
}
```

Code like this will be generated:

```rust
# use ::core::ops::Deref;
# struct CoolVec {
#     cool: bool,
#     vec: Vec<i32>,
# }
# impl Deref for CoolVec {
#     type Target = Vec<i32>;
#     #[inline]
#     fn deref(&self) -> &Self::Target {
#         &self.vec
#     }
# }
impl derive_more::core::ops::DerefMut for CoolVec {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}
```

When deriving `DerefMut` for a tuple struct with one field:

```rust
# use derive_more::{Deref, DerefMut};
#
#[derive(Deref, DerefMut)]
#[deref(forward)]
#[deref_mut(forward)]
struct MyBoxedInt(Box<i32>);
```

When deriving a forwarded `DerefMut` for a struct:

```rust
# use ::core::ops::Deref;
# struct MyBoxedInt(Box<i32>);
# impl Deref for MyBoxedInt {
#     type Target = <Box<i32> as Deref>::Target;
#     #[inline]
#     fn deref(&self) -> &Self::Target {
#         <Box<i32> as Deref>::deref(&self.0)
#     }
# }
impl derive_more::core::ops::DerefMut for MyBoxedInt {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        <Box<i32> as derive_more::core::ops::DerefMut>::deref_mut(&mut self.0)
    }
}
```




## Enums

When deriving a non-forwarded `DerefMut` for an enum:

```rust
# use derive_more::{Deref, DerefMut};
#
#[derive(Deref, DerefMut)]
enum CoolVecEnum {
    V1(Vec<i32>),
    V2 {
        cool: bool,
        #[deref]
        #[deref_mut]
        vec: Vec<i32>,
    },
}
```

Code like this will be generated:

```rust
# use ::core::ops::Deref;
# enum CoolVecEnum {
#     V1(Vec<i32>),
#     V2 {
#         cool: bool,
#         vec: Vec<i32>,
#     },
# }
# impl Deref for CoolVecEnum {
#     type Target = Vec<i32>;
#     #[inline]
#     fn deref(&self) -> &Self::Target {
#         match self {
#             CoolVecEnum::V1(__0) => __0,
#             CoolVecEnum::V2 { cool: _, vec: __0 } => __0,
#         }
#     }
# }
impl derive_more::with_trait::DerefMut for CoolVecEnum {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            CoolVecEnum::V1(__0) => __0,
            CoolVecEnum::V2 { cool: _, vec: __0 } => __0,
        }
    }
}
```

When deriving a forwarded `DerefMut` for an enum:

```rust
# use derive_more::{Deref, DerefMut};
#
#[derive(Deref, DerefMut)]
#[deref(forward)]
#[deref_mut(forward)]
enum MyBoxedIntEnum {
    V1(Box<i32>),
    V2 { num: Box<i32> },
}
```

Code like this will be generated:

```rust
# use ::core::ops::Deref;
# enum MyBoxedIntEnum {
#     V1(Box<i32>),
#     V2 { num: Box<i32> },
# }
# impl Deref for MyBoxedIntEnum {
#     type Target = <Box<i32> as derive_more::with_trait::Deref>::Target;
#     #[inline]
#     fn deref(&self) -> &Self::Target {
#         match self {
#             MyBoxedIntEnum::V1(__0) =>
#                 <Box<i32> as derive_more::with_trait::Deref>::deref(__0),
#             MyBoxedIntEnum::V2 { num: __0 } =>
#                 <Box<i32> as derive_more::with_trait::Deref>::deref(__0),
#         }
#     }
# }
impl derive_more::with_trait::DerefMut for MyBoxedIntEnum {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MyBoxedIntEnum::V1(__0) => {
                <Box<i32> as derive_more::with_trait::DerefMut>::deref_mut(__0)
            }
            MyBoxedIntEnum::V2 { num: __0 } => {
                <Box<i32> as derive_more::with_trait::DerefMut>::deref_mut(__0)
            }
        }
    }
}
```
