<!---
Copyright (C) 2020 Robin Krahl <robin.krahl@ireas.org>
SPDX-License-Identifier: CC0-1.0
-->

# merge-rs

The `merge` crate provides the `Merge` trait that can be used to merge multiple
values into one:

```rust
trait Merge {
    fn merge(&mut self, other: Self);
}
```

`Merge` is implemented for `Option` and can be derived for structs:

<!-- should be kept in sync with examples/user.rs -->
```rust
use merge::Merge;

#[derive(Merge)]
struct User {
    // Fields with the skip attribute are skipped by Merge
    #[merge(skip)]
    pub name: &'static str,

    // The Merge implementation for Option replaces its value if it is None
    pub location: Option<&'static str>,

    // The strategy attribute is used to customize the merge behavior
    #[merge(strategy = merge::vec::append)]
    pub groups: Vec<&'static str>,
}

let defaults = User {
    name: "",
    location: Some("Internet"),
    groups: vec!["rust"],
};
let mut ferris = User {
    name: "Ferris",
    location: None,
    groups: vec!["mascot"],
};
ferris.merge(defaults);

assert_eq!("Ferris", ferris.name);
assert_eq!(Some("Internet"), ferris.location);
assert_eq!(vec!["mascot", "rust"], ferris.groups);
```

A merge strategy is a function with the signature `fn merge<T>(left: &mut T,
right: T)` that merges `right` into `left`.  The `merge` crate provides
strategies for the most common types, but you can also define your own
strategies.

The trait can be used to merge configuration from different sources, for
example environment variables, multiple configuration files and command-line
arguments, see the `args.rs` example.

## Features

This crate has the following features:

- `derive` (default):  Enables the derive macro for the `Merge` trait using the
  `merge_derive` crate.
- `num` (default): Enables the merge strategies in the `num` module that
  require the `num_traits` crate.
- `std` (default): Enables the merge strategies in the `vec` module that
  require the standard library.  If this feature is not set, `merge` is a
  `no_std` library.

## Minimum Supported Rust Version

This crate supports Rust 1.36.0 or later.

## Contact

For bug reports, patches, feature requests and other messages, please send a
mail to [~ireas/public-inbox@lists.sr.ht][] using the `[merge-rs]` prefix in
the subject.

## License

This project is dual-licensed under the [Apache-2.0][] and [MIT][] licenses.
The documentation and configuration files contained in this repository are
licensed under the [Creative Commons Zero][CC0] license.  You can find a copy
of the license texts in the `LICENSES` directory.

`merge-rs` complies with [version 3.0 of the REUSE specification][reuse].

[~ireas/public-inbox@lists.sr.ht]: mailto:~ireas/public-inbox@lists.sr.ht
[Apache-2.0]: https://opensource.org/licenses/Apache-2.0
[MIT]: https://opensource.org/licenses/MIT
[CC0]: https://creativecommons.org/publicdomain/zero/1.0/
[reuse]: https://reuse.software/practices/3.0/
