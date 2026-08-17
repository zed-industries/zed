Derive macro available if borsh is built with `features = ["derive"]`.

# derive proc-macro for [`BorshSerialize`] trait

## Bounds

Generally, `BorshSerialize` adds `borsh::ser::BorshSerialize` bound to any type parameter
found in item's fields.

```rust
use borsh::BorshSerialize;

/// impl<U, V> borsh::ser::BorshSerialize for A<U, V>
/// where
///     U: borsh::ser::BorshSerialize,
///     V: borsh::ser::BorshSerialize,
#[derive(BorshSerialize)]
struct A<U, V> {
    x: U,
    y: V,
}
```

```rust
use borsh::BorshSerialize;

/// impl<U, V> borsh::ser::BorshSerialize for A<U, V>
/// where
///     U: borsh::ser::BorshSerialize,
#[derive(BorshSerialize)]
struct A<U, V> {
    x: U,
    #[borsh(skip)]
    y: V,
}
```

## Attributes

### 1. `#[borsh(crate = "path::to::borsh")]` (item level attribute)

###### syntax

Attribute takes literal string value, which is the syn's [Path](https://docs.rs/syn/2.0.92/syn/struct.Path.html) to `borsh` crate used.

###### usage

Attribute is optional.

1. If the attribute is not provided, [crate_name](https://docs.rs/proc-macro-crate/3.2.0/proc_macro_crate/fn.crate_name.html) is used to find a version of `borsh`
   in `[dependencies]` of the relevant `Cargo.toml`. If there is no match, a compilation error, similar to the following, is raised:

```bash
 1  error: proc-macro derive panicked
   --> path/to/file.rs:27:10
    |
 27 | #[derive(BorshSerialize, BorshDeserialize)]
    |          ^^^^^^^^^^^^^^
    |
    = help: message: called `Result::unwrap()` on an `Err` value: CrateNotFound { crate_name: "borsh", path: "/path/to/Cargo.toml" }
```

2. If the attribute is provided, the check for `borsh` in `[dependencies]` of the relevant `Cargo.toml` is skipped.

Examples of usage:

(example is not tested, as there's usually no `reexporter` crate during doc build)
```rust,ignore
use reexporter::borsh::BorshSerialize;

// specifying the attribute removes need for a direct import of `borsh` into `[dependencies]`
#[derive(BorshSerialize)]
#[borsh(crate = "reexporter::borsh")]
struct B {
    x: u64,
    y: i32,
    c: String,
}
```

```rust,ignore
use reexporter::borsh::{self, BorshSerialize};

// specifying the attribute removes need for a direct import of `borsh` into `[dependencies]`
#[derive(BorshSerialize)]
#[borsh(crate = "borsh")]
struct B {
    x: u64,
    y: i32,
    c: String,
}
```

### 2. `borsh(use_discriminant=<bool>)` (item level attribute)
This attribute is only applicable to enums.
`use_discriminant` allows to override the default behavior of serialization of enums with explicit discriminant.
`use_discriminant` is `false` behaves like version of borsh of 0.10.3.
You must specify `use_discriminant` for all enums with explicit discriminants in your project.

This is equivalent of borsh version 0.10.3 (explicit discriminant is ignored and this enum is equivalent to `A` without explicit discriminant):

```rust
use borsh::BorshSerialize;

#[derive(BorshSerialize)]
#[borsh(use_discriminant = false)]
enum A {
    A,
    B = 10,
}
```

To have explicit discriminant value serialized as is, you must specify `borsh(use_discriminant=true)` for enum.
```rust
use borsh::BorshSerialize;

#[derive(BorshSerialize)]
#[borsh(use_discriminant = true)]
enum B {
    A,
    B = 10,
}
```

###### borsh, expressions, evaluating to `isize`, as discriminant
This case is not supported:
```rust,compile_fail
const fn discrim() -> isize {
    0x14
}

#[derive(BorshSerialize)]
#[borsh(use_discriminant = true)]
enum X {
    A,
    B = discrim(), // expressions, evaluating to `isize`, which are allowed outside of `borsh` context
    C,
    D,
    E = 10,
    F,
}
```

###### borsh explicit discriminant does not support literal values outside of u8 range
This is not supported:
```rust,compile_fail
#[derive(BorshSerialize)]
#[borsh(use_discriminant = true)]
enum X {
    A,
    B = 0x100, // literal values outside of `u8` range
    C,
    D,
    E = 10,
    F,
}
```

### 3. `#[borsh(skip)]` (field level attribute)

`#[borsh(skip)]` makes derive skip serializing annotated field.

`#[borsh(skip)]` makes derive skip adding any type parameters, present in the field, to parameters bound by `borsh::ser::BorshSerialize`.

```rust
use borsh::BorshSerialize;

#[derive(BorshSerialize)]
struct A {
    x: u64,
    #[borsh(skip)]
    y: f32,
}
```

### 4. `#[borsh(bound(serialize = ...))]` (field level attribute)

###### syntax

Attribute takes literal string value, which is a comma-separated list of syn's [WherePredicate](https://docs.rs/syn/latest/syn/enum.WherePredicate.html)-s, which may be empty.

###### usage

Attribute adds possibility to override bounds for `BorshSerialize` in order to enable:

1. removal of bounds on type parameters from struct/enum definition itself and moving them to the trait's implementation block.
2. fixing complex cases, when derive hasn't figured out the right bounds on type parameters automatically.

```rust
use borsh::BorshSerialize;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;
/// additional bound `T: Ord` (required by `HashMap`) is injected into
/// derived trait implementation via attribute to avoid adding the bounds on the struct itself
#[cfg(any(feature = "hashbrown", feature = "std"))]
#[derive(BorshSerialize)]
struct A<T, U> {
    a: String,
    #[borsh(bound(serialize =
        "T: borsh::ser::BorshSerialize + Ord,
         U: borsh::ser::BorshSerialize"))]
    b: HashMap<T, U>,
}
```


```rust
use borsh::BorshSerialize;
trait TraitName {
    type Associated;
    fn method(&self);
}
/// derive here figures the bound erroneously as `T: borsh::ser::BorshSerialize`
#[derive(BorshSerialize)]
struct A<T, V>
where
    T: TraitName,
{
    #[borsh(bound(serialize = "<T as TraitName>::Associated: borsh::ser::BorshSerialize"))]
    field: <T as TraitName>::Associated,
    another: V,
}
```

###### interaction with `#[borsh(skip)]`

`#[borsh(bound(serialize = ...))]` replaces bounds, which are derived automatically,
irrelevant of whether `#[borsh(skip)]` attribute is present.

### 5. `#[borsh(serialize_with = ...)]` (field level attribute)

###### syntax

Attribute takes literal string value, which is a syn's [ExprPath](https://docs.rs/syn/latest/syn/struct.ExprPath.html).

###### usage

Attribute adds possibility to specify full path of function, optionally qualified with generics,
with which to serialize the annotated field.

It may be used when `BorshSerialize` cannot be implemented for field's type, if it's from foreign crate.

It may be used to override the implementation of serialization for some other reason.

```rust
use borsh::BorshSerialize;
use indexmap::IndexMap;

/// this a stub module, representing a 3rd party crate `indexmap`
mod indexmap {
    /// this a stub struct, representing a 3rd party `indexmap::IndexMap`
    /// or some local type we want to override trait implementation for
    pub struct IndexMap<K, V> {
        pub(crate) tuples: Vec<(K, V)>,
    }
    
}

mod index_map_impl {
    use super::indexmap::IndexMap;

    pub fn serialize_index_map<
        K: borsh::ser::BorshSerialize,
        V: borsh::ser::BorshSerialize,
        W: borsh::io::Write,
    >(
        obj: &IndexMap<K, V>,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::io::Error> {
        // the line of implementation for type from real `indexmap` crate
        // let key_value_tuples = obj.iter().collect::<Vec<_>>();
        let key_value_tuples = obj.tuples.iter().collect::<Vec<_>>();
        borsh::BorshSerialize::serialize(&key_value_tuples, writer)?;
        Ok(())
    }
}

#[derive(BorshSerialize)]
struct B<K, V> {
    #[borsh(
        serialize_with = "index_map_impl::serialize_index_map",
    )]
    x: IndexMap<K, V>,
    y: String,
}

# fn main() {
# }
```

###### usage (comprehensive example)

[borsh/examples/serde_json_value.rs](https://github.com/near/borsh-rs/blob/master/borsh/examples/serde_json_value.rs) is
a more complex example of how the attribute may be used.

###### interaction with `#[borsh(skip)]`

`#[borsh(serialize_with = ...)]` is not allowed to be used simultaneously with `#[borsh(skip)]`.

