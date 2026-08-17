# Changelog

## 0.2.12 - 2025-10-24

* Update litrs dependency to 1.0.0

## 0.2.11 - 2025-02-16

* Collect defaults recursively (#19)

## 0.2.10 - 2024-07-12

* Revert parsing of multi-lines string while parsing features
  (Keep parsing of multi-lines string when detecting if we need to use Cargo.toml.orig)

## 0.2.9 - 2024-07-11

* Fix parsing of multi-lines string (#25)
* Fix `out_of_scope_macro_calls` compatibility warning
* Fix documentation having too many `#` (#22)

## 0.2.8 - 2023-12-29

* Remove `\n` between features (#17)
* Don't throw an error when there is no features in Cargo.toml (#20)

## 0.2.7 - 2022-12-21

* Fix parsing of Cargo.toml with multi-line array of array (#16)

## 0.2.6 - 2022-09-24

* Fix parsing of escaped string literal in the macro arguments

## 0.2.5 - 2022-09-17

* Allow customization of the output with the `feature_label=` parameter

## 0.2.4 - 2022-09-14

* Fix dependencies or features written with quotes

## 0.2.3 - 2022-08-15

* Fix parsing of table with `#` within strings (#10)

## 0.2.2 - 2022-07-25

* Fix parsing of dependencies or feature spanning multiple lines (#9)

## 0.2.1 - 2022-02-12

* Fix indentation of multi-lines feature comments (#5)

## 0.2.0 - 2022-02-11

* Added ability to document optional features. (This is a breaking change in the
  sense that previously ignored comments may now result in errors)

## 0.1.0 - 2022-02-01

Initial release
