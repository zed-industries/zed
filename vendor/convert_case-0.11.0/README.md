# `convert-case`

Rust library for converting between string cases.

```{rust}
use convert_case::ccase;

assert_eq!(
    ccase!(camel, "My_Var_Name"),
    "myVarName",
);
assert_eq!(
    ccase!(snake, "IOStream"),
    "io_stream",
);
assert_eq!(
    ccase!(snake -> title, "2020-04-16_family_photo"),
    "2020-04-16 Family Photo",
);
```

`convert-case` is highly customizable.  You can read the API documentation on [docs.rs](https://docs.rs/convert_case/) for a list of all features and read lots of examples.

## Cases

This is list of cases that `convert_case` provides out of the box.  You can always make your own custom case.

| Case | Example |
| ---- | ------- |
| Snake | `my_variable_name` |
| Constant<br />UpperSnake | `MY_VARIABLE_NAME` |
| Ada | `My_Variable_Name` |
| Kebab | `my-variable-name` |
| Cobol<br />UpperKebab | `MY-VARIABLE-NAME` |
| Train | `My-Variable-Name` |
| Flat | `myvariablename` |
| UpperFlat | `MYVARIABLENAME` |
| Pascal<br />UpperCamel | `MyVariableName` |
| Camel | `myVariableName` |
| Upper | `MY VARIABLE NAME` |
| Lower | `my variable name` |
| Title | `My Variable Name` |
| Sentence | `My variable name` |

## Additional utilities with `convert_case_extras`

Some cases and utilities that didn't feel appropriate in this library are made available in a distinct crate called [`convert_case_extras`](https://github.com/rutrum/convert-case-extras).  This crate is a demonstration of what can be built on top of the `convert_case` API.

## Command Line Utility `ccase`

The [command line utility `ccase`](https://github.com/rutrum/ccase) was made to expose the tools of the `convert_case` library to the command line.
```
$ ccase -t title super_mario_64
Super Mario 64

$ ccase -f snake -t title 2020-04-15_my_cat
2020-04-16 My Cat

$ ccase -t camel "convert to camel"
convertToCamel
```

## Links

| | `convert_case` | `convert_case_extras` | `ccase` |
| --- | --- | --- | --- |
| Repository | [github](https://github.com/rutrum/convert-case) | [github](https://github.com/rutrum/convert-case-extras) | [github](https://github.com/rutrum/ccase) |
| Crate | [crates.io](https://crates.io/crates/convert_case) | [crates.io](https://crates.io/crates/convert_case_extras) | [crates.io](https://crates.io/crates/ccase) |
| Documentation | [docs.rs](https://docs.rs/convert_case) | [docs.rs](https://docs.rs/convert_case_extras) | |

## Change Log

### 0.11.0: Multiple Patterns

The headline change permits trailing, leading, and duplicate delimiters to persist (default) or be removed.  Instead of applying a single pattern, `Converter` supports multiple patterns and will apply them in order.  This comes with another pattern `Pattern::RemoveEmpty` that drops empty words.   This allows for opting into what was default behavior in 0.8.0 and before.

New features:
* `Converter` now supports multiple patterns via `Converter.patterns: Vec<Pattern>`. Patterns are applied in sequence, allowing for composition.
* `Pattern::RemoveEmpty` filters out empty words. This is useful when splitting produces empty words from leading, trailing, or duplicate delimiters (e.g., `"--a--b"` split on hyphens).
* `Casing::remove_empty()` is a convenience method that adds `Pattern::RemoveEmpty` to the `patterns` vector.
```rust
"--leading-delims".remove_empty().to_case(Case::Camel)
// "leadingDelims"
```

Breaking changes:
* `delim_boundary` macro is now `separator`
* Removed `Casing::is_case`.  Similar functionality is now implemented in `convert-case-extras`.
* Instances of `delim` have been renamed to `delimiter`:
    * `Converter.delim` -> `Converter.delimiter` 
    * `Converter::set_delim` -> `Converter::set_delimiter`
    * `Case::Custom.delim` -> `Case::Custom.delimiter` 
    * `Case::delim` -> `Case::delimiter`
* `Converter.pattern` is now `Converter.patterns` with type `Vec<Pattern>`
    * New pattern methods on `Converter` just like boundaries: `set_patterns`, `add_pattern`, `add_patterns`, `remove_pattern`, `remove_patterns`.
    * `Pattern::Noop` removed. An empty patterns vector represents no mutation.
* `Boundary::Custom` and `Pattern::Custom` variants are no longer comparable. 
  * Implementations of `PartialEq`, `Eq`, and `Hash` are no longer derived on `Boundary` and `Pattern`, and have been manually implemented.
  * Custom variants always return `false` for equality checks because function pointers cannot be reliably compared. This has been true for all of rust, but the compiler warning is relatively new to the author.  This means `remove_boundary` and `remove_pattern` will not work for custom variants.
  * Further, all `Custom` variants now hash to the same value.
    
Other changes:
* Much improved documentation: fixed typos, better examples.
* More comprehensive testing using `rstest`.

### 0.10.0: More clean up to prepare for 1.0.0

Since the library is so extensible with its new API, there is no longer a need for some niche or fun transformations to be made available in this library.  Some of the features that are removed are now in a new library `convert_case_extras`.  That library will have a lower threshold on what is included (i.e. more features), and will also serve as a demonstration of what's capable with the `convert_case` API.

Removed:
* `Case::Toggle` and `Pattern::Toggle`
* `Case::Alternating` and `Pattern::Alternating`
* `Case::Random` and `Pattern::Random`
* `Case::PseudoRandom` and `Pattern::PseudoRandom`
* `random` feature is removed.  The library no longer has any features.
* `Case::deterministic_cases` is removed
* `Case::random_cases` is removed

Other breaking changes:
* `Boundary::Custom` has lost the `arg` parameter and `Boundary::Custom.condition` is more simply `fn(&[&str]) -> bool`.
  * `arg` was originally used for building boundaries from delimiters with the `Boundary::from_delim` function, which is also removed because
  * `delim_boundary!` macro has replaced `Boundary::from_delim` functionality, without the need of the `arg` parameters
* `Casing::with_boundaries` is now `Casing::set_boundaries` and `Casing::without_boundaries` is now `Casing::remove_boundaries` to align with `Converter`

### 0.9.0: Back to enums, but keep the customization

This release is trying to finalize the API for a 1.0.0 release.  In hindsight, making `Pattern` a function and `Boundary` a struct were poor decisions.  While trying to add customized behavior, I made it harder to use.  Luckily, by following the convention that was done with `Case` in 0.8.0, which was providing a struct-variant `Case::Custom`, I can keep the new customization options while reverting back to the enum pattern the library has had since the beginning.

If you are updating from before 0.7.0, I don't think any changes are necessary.  If you're coming from 0.7.0 or higher,
* Change boundaries from constants to enum variants: `Boundary::UPPER_SNAKE` into `Boundary::UpperSnake`
* Change patterns from functions to enum variants on `Pattern`: `pattern::lowercase` into `Pattern::Lowercase`

Most excitingly, I've introduced a brand new way to use the crate for 99% of use cases, which is the default behavior out of the box to just convert a string to a case, with no modifications to behavior.  Instead of
```
use convert_case::{Case, Casing};
"MyVarName".to_case(Case::Snake);
"my-var name".from_case(Case::Title).to_case(Case::Snake);
```
You can use the `ccase!` macro:
```
use convert_case::ccase;
ccase!(snake, "MyVarName");
ccase!(title -> snake, "my-var name");
```
This means only one import and its invocation is brief and easy to read.  This is now the first thing you see when viewing the docs, which themselves have gotten a huge overhaul and many improvements in this version.

New features:
* `ccase!` macro that performs case conversion on a string _without needing to import `Case` or `Casing`_.  It has two forms:
    * `ccase!(snake, "string")` is equivalent to `"string".to_case(Case::Snake)`
    * `ccase!(kebab -> snake, "string")` is equivalent to `"string".from_case(Case::Kebab).to_case(Case::Snake)`
* While not intended to be used directly, the new `case!` macro returns a `Case` variant from the snake case version of the variant.  For instance, `case!(snake)` is substituted for `Case::Snake` and `case!(upper_flat)` for `Case::UpperFlat`.

Other breaking changes:
* Leading, trailing, and duplicate delimiters are no longer removed and are returned as is.
* `Boundary` is reverted back to being an enum, but with a `Custom` variant that gives all the same flexibility that `Boundary` had as a struct.  This aligns with the `Case::Custom` pattern.
    * `Boundary.match` will return true if a set of graphemes matches the boundary condition
    * `Boundary` methods for `start` and `len` describe how enum variants consume letters when matched
* `Pattern` is reverted back to being an enum, but with a `Custom` variant that allowed you to pass your own `fn (&[&str]) -> Vec<String>` as input.
    * `Pattern.mutate` will perform the associated mutation function
* `Converter.with_boundaries` has been renamed to `Converter.set_boundaries`.
* Removed `Converter.remove_delim` and `Converter.remove_pattern`.  You can use `set_delim("")` and `set_pattern(Pattern::Noop)` instead.
* `ToString` is no longer a required trait for using `is_case`
* `word_pattern` module has been removed

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

Boundary is no longer an enum.  It now is a struct, and each enum variant corresponds to an associated constant.  For upgrading this just means changing `Boundary::LowerUpper` to just `Boundary::LOWER_UPPER`.

The benefit of this is that you can make your boundary conditions now, by instantiating the `Boundary` struct, or using `Boundary::from_delim()`.  Now you can split on newlines, periods, double-colons, emojis, or a more complex case like a symbol followed by a digit.  You also define which characters, if any, are removed during segmentation, and where the split happens.

Changes from this feature:
* Previous `Boundary::PascalName` enum variants now much referred to as `Boundary::CONSTANT_NAME` constants.
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
