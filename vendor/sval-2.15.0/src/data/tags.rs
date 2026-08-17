/*!
Built-in tags for fundamental types.
*/

use super::Tag;

/**
A tag for a value that represents the `Some` variant of a Rust `Option`.

# Valid datatypes

- `tagged`
*/
pub const RUST_OPTION_SOME: Tag = Tag::new("RUST_OPTION_SOME");

/**
A tag for a value that represents the `None` variant of a Rust `Option`.

# Valid datatypes

- `tag`
*/
pub const RUST_OPTION_NONE: Tag = Tag::new("RUST_OPTION_NONE");

/**
A tag for Rust's `()` type.

# Valid datatypes

- `tag`
*/
pub const RUST_UNIT: Tag = Tag::new("RUST_UNIT");

/**
A tag for arbitrary-precision decimal numbers.

# Valid datatypes

- `text`

# Grammar

```text
^(?<sign>[+-])?(?<integral>[0-9]+)(?:\.(?<fractional>[0-9]+))?(?:[eE](?<expsign>[+-])?(?<exp>[0-9]+))?$
```

Examples of valid numbers include:

- `1`
- `+1`
- `-1`
- `001`
- `123.456`
- `1e2`
- `123.456e789`
- `123.456e+789`
- `123.456e-789`
*/
pub const NUMBER: Tag = Tag::new("NUMBER");

/**
A tag for values that have a constant size.

# Valid datatypes

Any datatype that accepts a size hint.

- `text`
- `binary`
- `map`
- `seq`
- `record`
- `tuple`
*/
pub const CONSTANT_SIZE: Tag = Tag::new("CONSTANT_SIZE");

/**
A tag for labels that are valid Rust identifiers.

`sval` uses this tag by default for labels on record values and enum variants.

# Valid datatypes

- `label`
 */
pub const VALUE_IDENT: Tag = Tag::new("VALUE_IDENT");

/**
A tag for indexes that are zero-based non-negative offsets into a larger structure.

`sval` uses this tag by default for indexes on tuple values and enum variants.

# Valid datatypes

- `index`
*/
pub const VALUE_OFFSET: Tag = Tag::new("VALUE_OFFSET");
