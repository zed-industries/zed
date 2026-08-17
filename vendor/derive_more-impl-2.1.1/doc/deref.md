# Using `#[derive(Deref)]`

Deriving `Deref` only works for a single field of a struct, or a single field of each variant of an enum.
It's possible to use it in two ways:

1. Dereferencing to the field, i.e. like if your type was a reference type.
2. Doing a dereference on the field, for when the field itself is a reference type like `&` and `Box`.

With `#[deref]` or `#[deref(ignore)]` it's possible to indicate the field that
you want to derive `Deref` for.




## Example usage

```rust
# use derive_more::Deref;
#
#[derive(Deref)]
struct Num {
    num: i32,
}

#[derive(Deref)]
enum Enum {
    V1(i32),
    V2 { num: i32 },
}

#[derive(Deref)]
#[deref(forward)]
struct MyBoxedInt(Box<i32>);

#[derive(Deref)]
#[deref(forward)]
enum MyBoxedIntEnum {
    V1(Box<i32>),
    V2 { num: Box<i32> },
}

// You can specify the field you want to derive `Deref` for.
#[derive(Deref)]
struct CoolVec {
    cool: bool,
    #[deref]
    vec: Vec<i32>,
}

#[derive(Deref)]
enum CoolVecEnum {
    V1(Vec<i32>),
    V2 { cool: bool, #[deref] vec: Vec<i32> },
}

let num = Num{num: 123};
let boxed = MyBoxedInt(Box::new(123));
let cool_vec = CoolVec{cool: true, vec: vec![123]};
assert_eq!(123, *num);
assert_eq!(123, *boxed);
assert_eq!(vec![123], *cool_vec);

let num_v2 = Enum::V2{num: 123};
let boxed_v1 = MyBoxedIntEnum::V1(Box::new(123));
let cool_vec_v2 = CoolVecEnum::V2{cool: true, vec: vec![123]};
assert_eq!(123, *num_v2);
assert_eq!(123, *boxed_v1);
assert_eq!(vec![123], *cool_vec_v2);
```




## Structs

When deriving a non-forwarded `Deref` for a struct:

```rust
# use derive_more::Deref;
#
#[derive(Deref)]
struct CoolVec {
    cool: bool,
    #[deref]
    vec: Vec<i32>,
}
```

Code like this will be generated:

```rust
# struct CoolVec {
#     cool: bool,
#     vec: Vec<i32>,
# }
impl derive_more::core::ops::Deref for CoolVec {
    type Target = Vec<i32>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
```

When deriving a forwarded `Deref` for a struct:

```rust
# use derive_more::Deref;
#
#[derive(Deref)]
#[deref(forward)]
struct MyBoxedInt(Box<i32>);
```

Code like this will be generated:

```rust
# struct MyBoxedInt(Box<i32>);
impl derive_more::core::ops::Deref for MyBoxedInt {
    type Target = <Box<i32> as derive_more::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        <Box<i32> as derive_more::core::ops::Deref>::deref(&self.0)
    }
}
```




## Enums

When deriving a non-forwarded `Deref` for an enum:

```rust
# use derive_more::Deref;
#
#[derive(Deref)]
enum CoolVecEnum {
    V1(Vec<i32>),
    V2 { cool: bool, #[deref] vec: Vec<i32> },
}
```

Code like this will be generated:

```rust
# enum CoolVecEnum {
#     V1(Vec<i32>),
#     V2 { cool: bool, vec: Vec<i32> },
# }
impl derive_more::with_trait::Deref for CoolVecEnum {
    type Target = Vec<i32>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            CoolVecEnum::V1(__0) => __0,
            CoolVecEnum::V2 { cool: _, vec: __0 } => __0,
        }
    }
}
```

When deriving a forwarded `Deref` for an enum:

```rust
# use derive_more::Deref;
#
#[derive(Deref)]
#[deref(forward)]
enum MyBoxedIntEnum {
    V1(Box<i32>),
    V2 { num: Box<i32> },
}
```

Code like this will be generated:

```rust
# enum MyBoxedIntEnum {
#     V1(Box<i32>),
#     V2 { num: Box<i32> },
# }
impl derive_more::with_trait::Deref for MyBoxedIntEnum {
    type Target = <Box<i32> as derive_more::with_trait::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            MyBoxedIntEnum::V1(__0) =>
                <Box<i32> as derive_more::with_trait::Deref>::deref(__0),
            MyBoxedIntEnum::V2 { num: __0 } =>
                <Box<i32> as derive_more::with_trait::Deref>::deref(__0),
        }
    }
}
```
