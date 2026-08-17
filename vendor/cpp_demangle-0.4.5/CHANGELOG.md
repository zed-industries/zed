# Unreleased

Released YYYY/MM/DD

## Added

* TODO (or remove section if none)

## Changed

* TODO (or remove section if none)

## Deprecated

* TODO (or remove section if none)

## Removed

* TODO (or remove section if none)

## Fixed

* TODO (or remove section if none)

## Security

* TODO (or remove section if none)

--------------------------------------------------------------------------------

# 0.4.5

Released 2025/9/23

## Fixed

* ABI tags are now accepted after all variants of qualified names.
  (#272)[https://github.com/gimli-rs/cpp_demangle/issues/272]

* Unqualified nested names without prefixes are accepted to match
  libiberty and LLVM.

## Added

* Fixed-point types from the N1169 draft of ISO/IEC DTR 18037 (_Accum, _Fract)
  are supported. (#298)[https://github.com/gimli-rs/cpp_demangle/pull/298]

* The proposed (and widely implemented) explicitly named object parameters are
  supported. (#299)[https://github.com/gimli-rs/cpp_demangle/pull/299]

--------------------------------------------------------------------------------

# 0.4.4

Released 2024/8/28

## Added

* C++23 extended floating-point types (_FloatN, Float_Nx, and std::bfloat16_t)
  and integer types (_BitInt) are supported.
  (#295)[https://github.com/gimli-rs/cpp_demangle/pull/295]

--------------------------------------------------------------------------------

# 0.4.3

Released 2023/8/18

## Fixed

* Inheriting constructions now correctly add themselves to the substitution table,
  fixing issues with demangling symbols that use them and then use a later
  substitution. (#286)[https://github.com/gimli-rs/cpp_demangle/issues/286]

--------------------------------------------------------------------------------

# 0.4.2

Released 2023/6/26

## Fixed

* Handling of recursion depth errors has been improved. Exceeding cpp_demangle's
  recursion limits will now return immediately, which significantly improves
  performance when attempting to demangle these symbols, and removes the possibility
  of returning an incorrect demangling if any productions in the symbol might be
  ambiguous. (#284)[https://github.com/gimli-rs/cpp_demangle/pull/284]

--------------------------------------------------------------------------------

# 0.4.1

Released 2023/4/13

## Fixed

* A case of runaway recursion caused by parsing ambiguous input in an incorrect
  order has been fixed. (#280)[https://github.com/gimli-rs/cpp_demangle/pull/280]

--------------------------------------------------------------------------------

# 0.4.0

Released 2022/10/20

## Removed

* The deprecated nightly and cppfilt features are gone.

## Changed

* If no-default-features is used the alloc feature must be explicitly specified.

--------------------------------------------------------------------------------

# 0.3.6

Released 2022/10/20

## Added

* The non-deprecated versions of noexcept are supported. #273

* Most of the subobject production that clang uses is supported. #273

## Changed

* Rust 2018 is now used. #251

* cppfilt now uses clap 4.0. #271

## Fixed

* no_std works. #251

* Inheriting constructors no longer produce substitutable values. #272

--------------------------------------------------------------------------------

# 0.3.5

Released 2021/12/02

## Changed

* The LLVM mangling for vector types with a dimension expression is now
  supported. The libiberty mangling which appears to be unused by gcc has
  been removed.

--------------------------------------------------------------------------------

# 0.3.4

Released 2021/11/21

## Added

* `DemangleOptions` now has a `hide_expression_literal_types` method that
  can make it easier to match user-provided template instance names. #230

--------------------------------------------------------------------------------

# 0.3.3

Released 2021/7/8

## Added

* The builtin `char8_t` type is now recognized #224

## Changed

* `glob` is no longer a build time dependency #220

--------------------------------------------------------------------------------

# 0.3.2

Released 2020/11/27

## Added

* `ParseOptions` is introduced, with new API variants `Symbol::new_with_options`
  and `Symbol::with_tail_and_options`. The existing APIs use the default parsing
  options.
* Recursion limits are now configurable via `ParseOptions` and `DemangleOptions`.
* Transaction clone symbols are supported #217

## Changed

* The default parsing recursion limit is now 96 (up from 64). The value was
  chosen to avoid pathological symbols overflowing the stack of a debug build.
  Users may be able to safely raise the limits substantially depending on their
  expected workload and tolerance for crashes.

--------------------------------------------------------------------------------

# 0.3.1

Released 2020/10/09

## Added

* Java Resource symbols are now supported #200

## Fixed

* C++ reference collapsing rules are honored.
* Misc style fixes.

## Changed

* DemangleOptions is now repr(C)

--------------------------------------------------------------------------------

# 0.3.0

Released 2020/06/11

## Changed

* The DemangleOptions API has changed to be more future-proof and the
  DemangleNodeType enum now has an __NonExhaustive variant to discourage
  pattern matching without a `_ => ()` arm.

--------------------------------------------------------------------------------

# 0.2.17

Released 2020/06/09

## Added

* Return types can now be elided from demangled symbols via
  DemangleOptions::no_return_typ. #202

* A vtable marker is now emitted for semantic consumers.

--------------------------------------------------------------------------------

# 0.2.16

Released 2020/05/13

## Added

* Block invocation symbols. #197

* The spaceship operator <=>. #198

--------------------------------------------------------------------------------

# 0.2.15

Released 2020/04/24

## Added

* A C API to cpp_demangle is now available. #191

* Additional AST markers are emitted for semantic consumers. #189

## Fixed

* Multiple clone suffixes are now supported. #194

--------------------------------------------------------------------------------

# 0.2.14

Released 2019/11/15

## Fixed

* Certain symbols can have cyclic back references, or at least very deep stacks
  of back references. Many of those symbols are valid! But as a practical
  implementation to avoid stack overflows and infinite loops, we now place a
  limit on the depth of back references we will follow. This is similar to the
  parse limit that we already had, but for a different phase of the
  demangling. [#186](https://github.com/gimli-rs/cpp_demangle/pull/186)

--------------------------------------------------------------------------------

# 0.2.13

Released 2019/07/30

## Fixed

* Fix parsing of outdated `sr` forms that prevented parsing other symbols. See
  #173 for details.

* Ensures a space is printed before a `&` or `&&` reference qualifier. #176

* Fixed placement of parentheses in symbols with function pointer arguments that
  have `const` qualifiers. #175

--------------------------------------------------------------------------------

# 0.2.12

Released 2018/08/09

## Fixed

* *Actually* fixed builds using `no-default-features = true` to not accidentally
  enable `no_std` mode, which requires nightly rust, and break builds on
  non-nightly channels. Enabling the `no_std` mode now requires disabling the
  `std` feature *and* enabling the `alloc` feature.

--------------------------------------------------------------------------------

# 0.2.11

Released 2018/08/09

## Fixed

* Fixed builds using `no-default-features = true` to not accidentally enable
  `no_std` mode, which requires nightly rust, and break builds on non-nightly
  channels. Enabling the `no_std` mode now requires disabling the `std` feature
  *and* enabling the `alloc` feature.

--------------------------------------------------------------------------------

# 0.2.10

Released 2018/08/08

## Added

* Added support for `no_std`! This currently requires nightly Rust's `alloc`
  feature to get access to `BTreeMap`. Enable `no_std` support by building
  without the on-by-default `std` feature. [#148][]

## Fixed

* Fixed formatting of some conversion operators. [#149][]
* Fixed parsing some tricky symbols with template argument packs that came out
  of boost. [#150][] [#152][]

[#148]: https://github.com/gimli-rs/cpp_demangle/pull/148
[#149]: https://github.com/gimli-rs/cpp_demangle/pull/149
[#150]: https://github.com/gimli-rs/cpp_demangle/pull/150
[#152]: https://github.com/gimli-rs/cpp_demangle/pull/152

--------------------------------------------------------------------------------

# 0.2.9

Released 2018/05/14

## Fixed

* Fixed a few issues with parentheticals.
* Should not force recompilation via build.rs for every compile anymore (bug
  introduced in 0.2.8 when trying to make the package that is distributed on
  crates.io smaller).

--------------------------------------------------------------------------------

# 0.2.8

Released 2018/05/11

Bug fixes, more `libiberty` tests passing, and we can now parse and demangle all
but one symbol from Firefox's `libxul`:

```
Total number of libxul symbols:                       274346
Number of libxul symbols parsed:                      274345 (100.00%)
Number of libxul symbols demangled:                   274345 (100.00%)
Number of libxul symbols demangled same as libiberty: 227259 (82.84%)
```

## Fixed

* AFL.rs fuzzing integration is fixed for the new AFL.rs releases.
* Fixed formatting of constructors and destructors.
* Fixed parsing of the `<function-param>` production.
* Fixed parsing of call expression productions.
* Parsing an operator's operands will only parse as many operands as the
  operator's arity, instead of as many as it can.

--------------------------------------------------------------------------------

# 0.2.7

Released 2017/11/27

Making lots of progress on symbols found in the wild! Here are our stats for
symbols from Firefox's `libxul`:

```
Total number of libxul symbols:                       274346
Number of libxul symbols parsed:                      274319 (99.99%)
Number of libxul symbols demangled:                   274319 (99.99%)
Number of libxul symbols demangled same as libiberty: 199928 (72.88%)
```

Additionally, the `libiberty` test threshold bumped up from 70 to 83 during this
release.

## Added

* Added support for GCC's "global constructors" and "global destructors"
  extensions.
* Added support for GCC's extensions to the `<special-name>` production:
  construction vtables, typeinfo functions, TLS initialization functions, TLS
  wrapper functions.
* Added support for the now-defunct `<local-source-name>` production. My
  understanding is that this is from an older version of the ABI standard. It
  isn't in the current version, but all the other demanglers support it, so we
  will too.

## Changed

* `cpp_demangle` is now part of the `gimli-rs` GitHub organization. The
  canonical repository is now https://github.com/gimli-rs/cpp_demangle
* Literals are now formatted how `libiberty` formats them. For example,
  `foo<true>()` rather than `foo<1>()`.
* Unary operators are now formatted with parentheses, matching `libiberty`.

## Fixed

* Nested array types and multi-dimensional arrays are now mangled correctly.
* Nested function types and their qualifiers are now mangled correctly.
* Arrays of function types and function types with array return and parameter
  types are now mangled correctly.
* The `new` operator is now correctly formatted as `operator new` rather than
  `operatornew`. Same for the `delete` operator.

--------------------------------------------------------------------------------

# 0.2.6

Released 2017/11/08

## Added

* Added support for vector types
* Added support for ABI tags
* Added support for `<unqualified-name> ::= <closure-type-name>` productions

## Fixed

* Fixed erroneous insertions into the substitutions table with prefixes and
  nested names
* Well known components were previously incorrectly not permitted to prefix
  template arguments
