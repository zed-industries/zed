# unicode-categories
`unicode-categories` is a simple crate that adds many extensions
to the `char` type that allow for the querying of whether or not
a character is a member of a certain category of unicode characters.

```
use unicode_categories::UnicodeCategories;

assert!('a'.is_letter_lowercase());
assert!('A'.is_letter_uppercase());
assert!('\n'.is_other_control());
```

This crate can be obtained from crates.io and used as a dependency
like any other crate.

## Regenerating the tables
The script in `scripts/unicode.py` reads a file named `UnicodeData.txt`,
which is assumed to be in the current working directory. This file
can be obtained from ftp://ftp.unicode.org . The script outputs a rust
file to standard out, which can be piped to the desired location.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
