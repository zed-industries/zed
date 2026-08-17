# Convert Case

Converts to and from various cases.

## Rust Library `convert_case`

Convert case was written in Rust and is ready to be used inline with your rust code as a library.
```{rust}
use convert_case::{Case, Casing};

assert_eq!("ronnieJamesDio", "Ronnie_James_dio".to_case(Case::Camel));
assert_eq!("io_stream", "IOStream".to_case(Case::Snake));
assert_eq!(
    "2020-04-16 My Cat Cali",
    "2020-04-16_my_cat_cali".from_case(Case::Snake).to_case(Case::Title)
);
```
You can read the API documentation on [docs.rs](https://docs.rs/convert_case/) for a list of all features and read lots of examples.

## Command Line Utility `ccase`

The [command line utility `ccase`](https://github.com/rutrum/ccase) was made to expose the tools of the `convert_case` library to the command line.
```
$ ccase -t title super_mario_64
Super Mario 64

$ ccase -f snake -t title 2020-04-15_my_cat_cali
2020-04-16 My Cat Cali

$ ccase -t camel "convert to camel"
convertToCamel
```

## Links

| | `convert_case` | `ccase` |
| --- | --- | --- |
| Repository | [github](https://github.com/rutrum/convert-case) | [github](https://github.com/rutrum/ccase) |
| Crate | [crates.io](https://crates.io/crates/convert_case) | [crates.io](https://crates.io/crates/ccase) |
| Documentation | [docs.rs](https://docs.rs/convert_case) | |

## Cases

This is list of cases that convert\_case supports.  Some cases are simply aliases of others.  The "Random" and "PseudoRandom" cases are provided in the `convert_case` library with the "random" feature.

| Case | Example |
| ---- | ------- |
| Snake | my\_variable\_name |
| Constant<br />UpperSnake | MY\_VARIABLE\_NAME |
| Ada | My\_Variable\_Name |
| Kebab | my-variable-name |
| Cobol<br />UpperKebab | MY-VARIABLE-NAME |
| Train | My-Variable-Name |
| Flat | myvariablename |
| UpperFlat | MYVARIABLENAME |
| Pascal<br />UpperCamel | MyVariableName |
| Camel | myVariableName |
| Upper | MY VARIABLE NAME |
| Lower | my variable name |
| Title | My Variable Name |
| Sentence | My variable name |
| Alternating | mY vArIaBlE nAmE |
| Toggle | mY vARIABLE nAME |
| Random | MY vaRiabLe nAME |
| PseudoRandom | mY VaRiAblE nAMe |

## Change Log

### 0.8.0: Pattern Overhaul, Custom Case

Pattern is no longer an enum.  It is now a type alias for `fn(&[&str]) -> Vec<String>`.  The variants of Pattern can now be referenced as functions inside the `pattern` module.  For upgrading this means changing `Pattern::Lowercase` to `pattern::lowercase`, and calling the function directly instead of invoking the `mutate` method on the enum.

Inside the pattern module is also the type alias `Pattern` itself.

Other breaking changes:
* Add `Case::Ada` (capital pattern with underscore delimiter.)
* Add `Case::Custom` variant.  It is a struct variant that takes three parameters:
    * pattern with type `Pattern`
    * delim with type `&static str`, and
    * boundaries with type `&'static [Boundary]`.
* Because of the new `pattern::noop` function, `Converter` attribute `pattern` is now of type `Pattern` and not `Option<Pattern>`
* `Case::deterministic_cases`, `Case::all_cases`, and `Case::random_cases` now return static arrays instead of vecs

Other changes:
* Added `Case::split`, `Case::mutate`, and `Case::join` which expose operations related to the boundaries, pattern, and delimiter of a case
* Is now `no_std` compatible

### 0.7.1

* Removed debug print statement.

### 0.7.0: Custom Boundaries

Boundary is no longer an enum.  It now is a struct, and each enum variant cooresponds to an associated constant.  For upgrading this just means changing `Boundary::LowerUpper` to just `Boundary::LOWER_UPPER`.

The benefit of this is that you can make your boundary conditions now, by instantiating the `Boundary` struct, or using `Boundary::from_delim()`.  Now you can split on newlines, periods, double-colons, emojis, or a more complex case like a symbol followed by a digit.  You also define which characters, if any, are removed during segmentation, and where the split happens.

Changes from this feature:
* Previous `Boundary::PascalName` enum variants now much refered to as `Boundary::CONSTANT_NAME` constants.
* All functions that returned groups of boundaries (such as `Boundary::defaults()`, `Boundary::digit_letter()`, etc) now are const and return fixed-sized arrays `[Boundary; N]`, not `Vec<Boundary>`.
* `Boundary::all()` was removed, since there's no longer a sense of "all" boundaries, since you can create your own.
* `Boundary::list_from()` has been renamed to `Boundary::defaults_from()` and no longer outputs `Boundary::UPPER_LOWER`, since this function now only checks default boundaries.
* Create custom delimiter boundaries using `Boundary::from_delim()`.

Other breaking changes:

* Rename `Case::ScreamingSnake` to `Case::Constant`.
* Add `Case::Sentence` (sentence pattern and space delimiter.)
* `Casing` trait implemented for `Arc<str>` and `Rc<str>` again.

Other changes:

* Remove most imports from doc comments.
* Remove loop over `str::chars` in favor of `graphemes` from `unicode-segmentation`.

## Other Projects

Github user [Wild-W](https://github.com/Wild-W) has built [nodejs bindings for convert_case](https://github.com/Wild-W/convert-case) that are up to date with 0.6.0.
