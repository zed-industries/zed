Derive macro available if borsh is built with `features = ["derive"]`.

# derive proc-macro for [`BorshDeserialize`] trait

## Bounds

Generally, `BorshDeserialize` adds `borsh::de::BorshDeserialize` bound to any type parameter
found in item's fields and `core::default::Default` bound to any type parameter found
in item's skipped fields.

```rust
use borsh::BorshDeserialize;

/// impl<U, V> borsh::de::BorshDeserialize for A<U, V>
/// where
///     U: borsh::de::BorshDeserialize,
///     V: borsh::de::BorshDeserialize,
#[derive(BorshDeserialize)]
struct A<U, V> {
    x: U,
    y: V,
}
```

```rust
use borsh::BorshDeserialize;

/// impl<U, V> borsh::de::BorshDeserialize for A<U, V>
/// where
///     U: borsh::de::BorshDeserialize,
///     V: core::default::Default,
#[derive(BorshDeserialize)]
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
 27 | #[derive(BorshDeserialize, BorshSerialize)]
    |          ^^^^^^^^^^^^^^^^
    |
    = help: message: called `Result::unwrap()` on an `Err` value: CrateNotFound { crate_name: "borsh", path: "/path/to/Cargo.toml" }
```

2. If the attribute is provided, the check for `borsh` in `[dependencies]` of the relevant `Cargo.toml` is skipped.

Examples of usage:

(example is not tested, as there's usually no `reexporter` crate during doc build)
```rust,ignore
use reexporter::borsh::BorshDeserialize;

// specifying the attribute removes need for a direct import of `borsh` into `[dependencies]`
#[derive(BorshDeserialize)]
#[borsh(crate = "reexporter::borsh")]
struct B {
    x: u64,
    y: i32,
    c: String,
}
```

```rust,ignore
use reexporter::borsh::{self, BorshDeserialize};

// specifying the attribute removes need for a direct import of `borsh` into `[dependencies]`
#[derive(BorshDeserialize)]
#[borsh(crate = "borsh")]
struct B {
    x: u64,
    y: i32,
    c: String,
}
```

### 2. `#[borsh(init=...)]` (item level attribute)

###### syntax

Attribute's value is syn's [Path](https://docs.rs/syn/2.0.92/syn/struct.Path.html)-s, passed to borsh top level meta attribute as value of `init` argument.

###### usage

`#[borsh(init=...)]` allows to automatically run an initialization function right after deserialization.
This adds a lot of convenience for objects that are architectured to be used as strictly immutable.

```rust
type CryptoHash = String;

use borsh::BorshDeserialize;
#[derive(BorshDeserialize)]
#[borsh(init=init)]
struct Message {
    message: String,
    timestamp: u64,
    hash: CryptoHash,
}

impl Message {
    pub fn init(&mut self) {
        self.hash = {
            let mut hash = CryptoHash::new();
            hash.push_str(&self.message);
            hash.push_str(&format!("{}", self.timestamp));
            hash
        };
    }
}
```

### 3. `borsh(use_discriminant=<bool>)` (item level attribute)

This attribute is only applicable to enums.
`use_discriminant` allows to override the default behavior of serialization of enums with explicit discriminant.
`use_discriminant` is `false` behaves like version of borsh of 0.10.3.
It's useful for backward compatibility and you can set this value to `false` to deserialise data serialised by older version of `borsh`.
You must specify `use_discriminant` for all enums with explicit discriminants in your project.

This is equivalent of borsh version 0.10.3 (explicit discriminant is ignored and this enum is equivalent to `A` without explicit discriminant):
```rust
use borsh::BorshDeserialize;
#[derive(BorshDeserialize)]
#[borsh(use_discriminant = false)]
enum A {
    A,
    B = 10,
}
```

To have explicit discriminant value serialized as is, you must specify `borsh(use_discriminant=true)` for enum.
```rust
use borsh::BorshDeserialize;
#[derive(BorshDeserialize)]
#[borsh(use_discriminant = true)]
enum B {
    A,
    B = 10,
}
```


###### borsh, expressions, evaluating to `isize`, as discriminant
This case is not supported:

```rust,compile_fail
use borsh::BorshDeserialize;
const fn discrim() -> isize {
    0x14
}

#[derive(BorshDeserialize)]
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


###### borsh explicit discriminant does not support literal values outside of u8 range.
This is not supported:

```rust,compile_fail
#[derive(BorshDeserialize)]
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


### 4. `#[borsh(skip)]` (field level attribute)

`#[borsh(skip)]` makes derive skip deserializing annotated field.

`#[borsh(skip)]` makes derive skip adding any type parameters, present in the field, to parameters bound by `borsh::de::BorshDeserialize`.

It adds `core::default::Default` bound to any
parameters encountered in annotated field.


```rust
use borsh::BorshDeserialize;
#[derive(BorshDeserialize)]
struct A {
    x: u64,
    #[borsh(skip)]
    y: f32,
}
```


### 5. `#[borsh(bound(deserialize = ...))]` (field level attribute)

###### syntax

Attribute takes literal string value, which is a comma-separated list of syn's [WherePredicate](https://docs.rs/syn/latest/syn/enum.WherePredicate.html)-s, which may be empty.


###### usage

Attribute adds possibility to override bounds for `BorshDeserialize` in order to enable:

1. removal of bounds on type parameters from struct/enum definition itself and moving them to the trait's implementation block.
2. fixing complex cases, when derive hasn't figured out the right bounds on type parameters automatically.

```rust
use borsh::BorshDeserialize;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;
use core::hash::Hash;
/// additional bounds `T: Ord + Hash + Eq` (required by `HashMap`) are injected into
/// derived trait implementation via attribute to avoid adding the bounds on the struct itself
#[cfg(any(feature = "hashbrown", feature = "std"))]
#[derive(BorshDeserialize)]
struct A<T, U> {
    a: String,
    #[borsh(bound(
        deserialize =
        "T: Ord + Hash + Eq + borsh::de::BorshDeserialize,
         U: borsh::de::BorshDeserialize"
    ))]
    b: HashMap<T, U>,
}
```


```rust
use borsh::BorshDeserialize;
trait TraitName {
    type Associated;
    fn method(&self);
}
// derive here figures the bound erroneously as `T: borsh::de::BorshDeserialize,`
#[derive(BorshDeserialize)]
struct A<T, V>
where
    T: TraitName,
{
    #[borsh(bound(deserialize = "<T as TraitName>::Associated: borsh::de::BorshDeserialize"))]
    field: <T as TraitName>::Associated,
    another: V,
}
```

###### interaction with `#[borsh(skip)]`

`#[borsh(bound(deserialize = ...))]` replaces bounds, which are derived automatically,
irrelevant of whether `#[borsh(skip)]` attribute is present.

```rust
use borsh::BorshDeserialize;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;
/// implicit derived `core::default::Default` bounds on `K` and `V` type parameters are removed by
/// empty bound specified, as `HashMap` has its own `Default` implementation
#[cfg(any(feature = "hashbrown", feature = "std"))]
#[derive(BorshDeserialize)]
struct A<K, V, U>(
    #[borsh(skip, bound(deserialize = ""))]
    HashMap<K, V>,
    U,
);
```

### 6. `#[borsh(deserialize_with = ...)]` (field level attribute)

###### syntax

Attribute takes literal string value, which is a syn's [ExprPath](https://docs.rs/syn/latest/syn/struct.ExprPath.html).

###### usage

Attribute adds possibility to specify full path of function, optionally qualified with generics,
with which to deserialize the annotated field.

It may be used when `BorshDeserialize` cannot be implemented for field's type, if it's from foreign crate.

It may be used to override the implementation of deserialization for some other reason.

```rust
use borsh::BorshDeserialize;
use indexmap::IndexMap;
use core::hash::Hash;

/// this a stub module, representing a 3rd party crate `indexmap`
mod indexmap {
    /// this a stub struct, representing a 3rd party `indexmap::IndexMap`
    /// or some local type we want to override trait implementation for
    pub struct IndexMap<K, V> {
        pub(crate) tuples: Vec<(K, V)>,
    }
    
}

mod index_map_impl {
    use super::IndexMap;
    use core::hash::Hash;

    pub fn deserialize_index_map<
        R: borsh::io::Read,
        K: borsh::de::BorshDeserialize + Hash + Eq,
        V: borsh::de::BorshDeserialize,
    >(
        reader: &mut R,
    ) -> ::core::result::Result<IndexMap<K, V>, borsh::io::Error> {
        let vec: Vec<(K, V)> = borsh::BorshDeserialize::deserialize_reader(reader)?;
        // the line of implementation for type from real `indexmap` crate
        // let result: IndexMap<K, V> = vec.into_iter().collect();
        let result = IndexMap {
            tuples: vec,
        };
        Ok(result)
    }
}

#[derive(BorshDeserialize)]
struct B<K: Hash + Eq, V> {
    #[borsh(
        deserialize_with = "index_map_impl::deserialize_index_map",
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

`#[borsh(deserialize_with = ...)]` is not allowed to be used simultaneously with `#[borsh(skip)]`.

