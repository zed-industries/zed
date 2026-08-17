Derive macro available if borsh is built with `features = ["unstable__schema"]`.

# derive proc-macro for [`BorshSchema`] trait

## Bounds

Generally, `BorshSchema` adds `borsh::BorshSchema` bound to any type parameter
found in item's fields.

```rust
use borsh::BorshSchema;
/// impl<U, V> borsh::BorshSchema for A<U, V>
/// where
///     U: borsh::BorshSchema,
///     V: borsh::BorshSchema,
#[derive(BorshSchema)]
struct A<U, V> {
    x: U,
    y: V,
}
```

```rust
use borsh::BorshSchema;
/// impl<U, V> borsh::BorshSchema for A<U, V>
/// where
///     U: borsh::BorshSchema,
#[derive(BorshSchema)]
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
 27 | #[derive(BorshSchema, BorshSerialize)]
    |          ^^^^^^^^^^^
    |
    = help: message: called `Result::unwrap()` on an `Err` value: CrateNotFound { crate_name: "borsh", path: "/path/to/Cargo.toml" }
```

2. If the attribute is provided, the check for `borsh` in `[dependencies]` of the relevant `Cargo.toml` is skipped.

Examples of usage:

(example is not tested, as there's usually no `reexporter` crate during doc build)
```rust,ignore
use reexporter::borsh::BorshSchema;

// specifying the attribute removes need for a direct import of `borsh` into `[dependencies]`
#[derive(BorshSchema)]
#[borsh(crate = "reexporter::borsh")]
struct B {
    x: u64,
    y: i32,
    c: String,
}
```

```rust,ignore
use reexporter::borsh::{self, BorshSchema};

// specifying the attribute removes need for a direct import of `borsh` into `[dependencies]`
#[derive(BorshSchema)]
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
use borsh::BorshSchema;
#[derive(BorshSchema)]
#[borsh(use_discriminant = false)]
enum A {
    A,
    B = 10,
}
```

To have explicit discriminant value serialized as is, you must specify `borsh(use_discriminant=true)` for enum.
```rust
use borsh::BorshSchema;
#[derive(BorshSchema)]
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

#[derive(BorshSchema)]
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
#[derive(BorshSchema)]
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

`#[borsh(skip)]` makes derive skip including schema from annotated field into schema's implementation.

`#[borsh(skip)]` makes derive skip adding any type parameters, present in the field, to parameters bound by `borsh::BorshSchema`.

```rust
use borsh::BorshSchema;
#[derive(BorshSchema)]
struct A {
    x: u64,
    #[borsh(skip)]
    y: f32,
}
```

### 4. `#[borsh(schema(params = ...))]` (field level attribute)

###### syntax

Attribute takes literal string value, which is a comma-separated list of `ParameterOverride`-s, which may be empty.

###### usage
It may be used in order to:

1. fix complex cases, when derive hasn't figured out the right bounds on type parameters and
   declaration parameters automatically.
2. remove parameters, which do not take part in serialization/deserialization, from bounded ones and from declaration parameters.

`ParameterOverride` describes an entry like `order_param => override_type`,

e.g. `K => <K as TraitName>::Associated`.

Such an entry instructs `BorshSchema` derive to:

1. add `override_type` to types, bounded by `borsh::BorshSchema` in implementation block.
2. add `<override_type>::declaration()` to parameters vector in `fn declaration()` method of `BorshSchema` trait that is being derived.
3. the `order_param` is required to establish the same order in parameters vector (2.) as that of type parameters in generics of type, that `BorshSchema` is derived for.
4. entries, specified for a field, together replace whatever would've been derived automatically for 1. and 2. .


```rust
use borsh::BorshSchema;
trait TraitName {
    type Associated;
    fn method(&self);
}
// derive here figures the bound erroneously as `T: borsh::BorshSchema` .
// attribute replaces it with <T as TraitName>::Associated: borsh::BorshSchema`
#[derive(BorshSchema)]
struct A<V, T>
where
    T: TraitName,
{
    #[borsh(schema(params = "T => <T as TraitName>::Associated"))]
    field: <T as TraitName>::Associated,
    another: V,
}
```

```rust
use borsh::BorshSchema;
use core::marker::PhantomData;

trait EntityRef {
    fn key_property(&self) -> u64;
}
// K in PrimaryMap isn't stored during serialization / read during deserialization.
// thus, it's not a parameter, relevant for `BorshSchema`
// ...
// impl<K: EntityRef, V> borsh::BorshSchema for A<K, V>
// where
//     V: borsh::BorshSchema,
#[derive(BorshSchema)]
struct A<K: EntityRef, V> {
    #[borsh(
        schema(
            params = "V => V"
        )
    )]
    x: PrimaryMap<K, V>,
    y: String,
}

#[derive(BorshSchema)]
pub struct PrimaryMap<K, V>
where
    K: EntityRef,
{
    elems: Vec<V>,
    unused: PhantomData<K>,
}
```

###### interaction with `#[borsh(skip)]`

`#[borsh(schema(params = ...))]` is not allowed to be used simultaneously with `#[borsh(skip)]`.

### 5. `#[borsh(schema(with_funcs(declaration = ..., definitions = ...)))]` (field level attribute)

###### syntax

Each of `declaration` and `definitions` nested sub-attributes takes literal string value, which is a syn's [ExprPath](https://docs.rs/syn/latest/syn/struct.ExprPath.html).

Currently both `declaration` and `definitions` are required to be specified at the same time.

###### usage

Attribute adds possibility to specify full path of 2 functions, optionally qualified with generics,
with which to generate borsh schema for annotated field.

It may be used when `BorshSchema` cannot be implemented for field's type, if it's from foreign crate.

It may be used to override the implementation of schema for some other reason.

```rust
use borsh::BorshSchema;
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
    pub mod schema {
        use std::collections::BTreeMap;

        use borsh::{
            schema::{Declaration, Definition, self},
            BorshSchema,
        };

        pub fn declaration<K: BorshSchema, V: BorshSchema>() -> Declaration {
            let params = vec![<K>::declaration(), <V>::declaration()];
            format!(r#"{}<{}>"#, "IndexMap", params.join(", "))
        }

        pub fn add_definitions_recursively<K: BorshSchema, V: BorshSchema>(
            definitions: &mut BTreeMap<Declaration, Definition>,
        ) {
            let definition = Definition::Sequence {
                elements: <(K, V)>::declaration(),
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
            };
            let no_recursion_flag = definitions.get(&declaration::<K, V>()).is_none();
            schema::add_definition(declaration::<K, V>(), definition, definitions);
            if no_recursion_flag {
                <(K, V)>::add_definitions_recursively(definitions);
            }
        }
    }
}

#[derive(BorshSchema)]
struct B<K, V> {
    #[borsh(
        schema(
            with_funcs(
                declaration = "index_map_impl::schema::declaration::<K, V>",
                definitions = "index_map_impl::schema::add_definitions_recursively::<K, V>"
            ),
        )
    )]
    x: IndexMap<K, V>,
    y: String,
}
# fn main() {
# }
```

###### interaction with `#[borsh(skip)]`

`#[borsh(schema(with_funcs(declaration = ..., definitions = ...)))]` is not allowed to be used simultaneously with `#[borsh(skip)]`.

