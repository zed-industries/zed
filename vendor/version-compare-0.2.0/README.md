[![Build status on GitLab CI][gitlab-ci-master-badge]][gitlab-ci-link]
[![Crate version][crate-version-badge]][crate-link]
[![Documentation][docs-badge]][docs]
[![Download statistics][crate-download-badge]][crate-link]
[![Coverage status][coverage-badge]][coverage-link]
[![Dependencies][dependency-badge]][crate-link]
[![License][crate-license-badge]][crate-link]

[coverage-badge]: https://gitlab.com/timvisee/version-compare/badges/master/coverage.svg
[coverage-link]: https://coveralls.io/gitlab/timvisee/version-compare
[crate-download-badge]: https://img.shields.io/crates/d/version-compare.svg
[crate-license-badge]: https://img.shields.io/crates/l/version-compare.svg
[crate-link]: https://crates.io/crates/version-compare
[crate-version-badge]: https://img.shields.io/crates/v/version-compare.svg
[dependency-badge]: https://img.shields.io/badge/dependencies-none!-green.svg
[docs-badge]: https://img.shields.io/docsrs/version-compare
[docs]: https://docs.rs/version-compare
[gitlab-ci-link]: https://gitlab.com/timvisee/version-compare/pipelines
[gitlab-ci-master-badge]: https://gitlab.com/timvisee/version-compare/badges/master/pipeline.svg

# Rust library: version-compare

> Rust library to easily compare version numbers with no specific format, and test against various comparison operators.

Comparing version numbers is hard, especially with weird version number formats.

This library helps you to easily compare any kind of version number with no
specific format using a best-effort approach.
Two version numbers can be compared to each other to get a comparison operator
(`<`, `==`, `>`), or test them against a comparison operator.

Along with version comparison, the library provides various other tools for
working with version numbers.

Inspired by PHPs [version_compare()](http://php.net/manual/en/function.version-compare.php).

_Note: Still a work in progress. Configurability is currently very limited. Things will change._

### Formats

Version numbers that would parse successfully include:  
`1`, `3.10.4.1`, `1.2.alpha`, `1.2.dev.4`, ` `, ` .   -32 . 1`, `MyApp 3.2.0 / build 0932` ...

See a list of how version numbers compare [here](https://github.com/timvisee/version-compare/blob/411ed7135741ed7cf2fcf4919012fb5412dc122b/src/test.rs#L50-L103).

## Example

This library is very easy to use. Here's a basic usage example:

`Cargo.toml`:
```toml
[dependencies]
version-compare = "0.1"
```

[`example.rs`](examples/example.rs):
```rust
use version_compare::{compare, compare_to, Cmp, Version};

fn main() {
    let a = "1.2";
    let b = "1.5.1";

    // The following comparison operators are used:
    // - Cmp::Eq -> Equal
    // - Cmp::Ne -> Not equal
    // - Cmp::Lt -> Less than
    // - Cmp::Le -> Less than or equal
    // - Cmp::Ge -> Greater than or equal
    // - Cmp::Gt -> Greater than

    // Easily compare version strings
    assert_eq!(compare(a, b), Ok(Cmp::Lt));
    assert_eq!(compare_to(a, b, Cmp::Le), Ok(true));
    assert_eq!(compare_to(a, b, Cmp::Gt), Ok(false));

    // Parse and wrap version strings as a Version
    let a = Version::from(a).unwrap();
    let b = Version::from(b).unwrap();

    // The Version can easily be compared with
    assert_eq!(a < b, true);
    assert_eq!(a <= b, true);
    assert_eq!(a > b, false);
    assert_eq!(a != b, true);
    assert_eq!(a.compare(&b), Cmp::Lt);
    assert_eq!(a.compare_to(&b, Cmp::Lt), true);

    // Or match the comparison operators
    match a.compare(b) {
        Cmp::Lt => println!("Version a is less than b"),
        Cmp::Eq => println!("Version a is equal to b"),
        Cmp::Gt => println!("Version a is greater than b"),
        _ => unreachable!(),
    }
}
```

See the [`examples`](examples) directory for more.

## Features

* Compare version numbers, get: `<`, `==`, `>`
* Compare against a comparison operator
  (`<`, `<=`, `==`, `!=`, `>=`, `>`)
* Parse complex and unspecified formats
* Static, standalone methods to easily compare version strings in a single line
  of code

#### Future ideas

* Version ranges
* Support for [npm-style](https://semver.npmjs.com/) operators (e.g. `^1.0` or `~1.0`)
* Manifest: extend `Manifest` for to support a wide set of constraints
* Building blocks for building your own specific version number parser
* Batch comparisons

#### Semver

Version numbers using the [semver](http://semver.org/) format are compared
correctly with no additional configuration.

If your version number strings follow this exact format you may be better off
using the [`semver`](https://crates.io/crates/semver) crate for more format
specific features.

If that isn't certain however, `version-compare` makes comparing a breeze.

## Builds

This library is automatically build and tested every day and for each commit using CI services.

See the current status here: https://gitlab.com/timvisee/version-compare/-/pipelines

## License

This project is released under the MIT license. Check out the [LICENSE](LICENSE) file for more information.
