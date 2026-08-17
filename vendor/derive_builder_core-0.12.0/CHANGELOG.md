# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## 0.3.0 - 2018-10-22

- Updated all dependencies #138

## 0.2.0 - 2017-12-16

### Fixed
- `unused_mut` lint (variable does not need to be mutable) #104

## 0.1.7 - 2017-04-29

### Fixed
- for generic structs, apply the `T: Clone` type bound in builder impl
  instead of struct definition #91
- only emit the `T: Clone` type bound when it is actually needed, i.e.
  mutable/immutable pattern, but not owned pattern.

## 0.1.6 - 2017-04-26

### Added
- pre-build validation

## 0.1.5 - 2017-04-25

### Added
- derive traits on builder struct

## 0.1.4 - 2017-04-12

### Added
- try_setters

## 0.1.3 - 2017-04-11

### Fixed
- `setter(skip)` honors struct-inherited and explicit defaults #68

## 0.1.2 - 2017-04-10
### Added
- Bindings to abstract over libstd/libcore

### Changed
- Use `bindings: Bindings` instead of `no_std: bool`

### Fixed
- support generic references in structs #55
- no_std support #63

## 0.1.1 - 2017-04-08
### Added
- struct default

## 0.1 - 2017-03-25
### Added
- helper crate `derive_builder_core`:
  Allow `derive_builder` to use its own code generation technique.
- helper structs implementing `quote::ToTokens`:
  Allow unit tests on code generation items.
