# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html),
with the exception that 0.x versions can break between minor versions.

## [0.16.2] - 2025-09-19
### Added
- Add an "oniguruma mode" flag to control whether `\<` and `\>` are treated as literals or word-boundary assertions. (#186)
- Add support for const-size backrefs in lookbehinds. (#182)
### Changed
- A few small internal changes which allowed us to expose a web-based playground for fancy-regex. (#181)
### Fixed
- Fix behavior of repetition on an empty match. (#179)
- Always return the original input pattern when `as_str` is called. (#185)
- Fix panic when matching non-ascii text as part of a case insensitive backref. (#189)
- The `toy` example wasn't always showing the correct analysis or compiled program when optimizations were applied. (#181)

## [0.16.1] - 2025-08-02
### Fixed
- Fixed a bug whereby sometimes a backreference to a non-existing capture group would compile successfully
  when it should fail, causing a panic in the VM when trying to match the regex. (#174)

## [0.16.0] - 2025-08-01
### Added
- Add an optimization step after the pattern is parsed but before it is analyzed.
  Currently it only optimizes one specific use-case - where the expression is easy except
  for a trailing positive lookahead whose contents are also easy. The optimization is to delegate to the regex crate instead of using the backtracking VM. (#171)
### Changed
- Patterns which are anchored to the start of the text (i.e. with `^` when not in multiline mode) should now fail faster when there is no match, because `fancy-regex` no longer tries to match at other positions. (#170)
- The `CompileError` for an invalid (numbered) backref has been updated to mention which backref was invalid (#170)
- Removed dependency on derivative (#169)
### Fixed
- Fixed a bug whereby sometimes a capture group containing a backref to itself would cause a compile error, when it is valid - this fixes a few Oniguruma test cases (#170)

## [0.15.0] - 2025-07-06
### Added
- Support `\Z` - anchor to the end of the text before any trailing newlines. (#148)
- Support `\O` - any character including newlines. (#158)
- The parser can now parse subroutine calls and relative backreferences (but execution is still unsupported). This is preparation for future work. Some new error variants have been added for features which can be parsed but are still otherwise unsupported.
- Backreferences can now be case insensitive. (#160)
- `RegexBuilder`: Add options for `multi_line`, `ignore_whitespace`, `dot_matches_new_line` (#165)
### Fixed
- Fix infinite loop when backtracking limit is hit (#153)
- Fix `RegexBuilder.case_insensitive` not always applying when it should. (#163)
- The `toy` example has had various bugfixes, and unit tests added. (#152, #159)

## [0.14.0] - 2024-10-24
### Added
- Add `split`, `splitn` methods to `Regex` to split a string into substrings (#140)
- Add `case_insensitive` method to `RegexBuilder` to force case-insensitive mode (#132)
### Changed
- Bump bit-set dependency to 0.8 (#139)

## [0.13.0] - 2023-12-22
### Added
- Support for relative backreferences using `\k<-1>` (-1 references the
  previous group) (#121)
- Add `try_replacen` to `Regex` which returns a `Result` instead of panicking
  when matching errors (#130)
### Changed
- Switch from regex crate to regex-automata and regex-syntax (lower level APIs)
  to simplify internals (#121)
- **Note:** Due to above change, more backtracking is done in fancy-regex itself
  instead of regex-automata, and you might get a `BacktrackLimitExceeded` with
  some patterns that you didn't get before. You can increase the backtrack limit
  using `RegexBuilder::backtrack_limit` to help with that.
- Allow escaping some letters in character classes, e.g. `[\A]` used to error
  but now matches the same as `[A]` (for compatibility with Oniguruma)
- MSRV (minimum supported Rust version) is now 1.66.1 (from 1.61.0)
### Fixed
- Fix index out of bounds panic when parsing unclosed `(?(` (#125)

## [0.12.0] - 2023-11-11
### Added
- Support for `no_std` (the `std` feature is enabled by default but can be
  disabled if desired) (#111)
- `TryFrom` `&str` and `String` impl for `Regex` (#115)
### Changed
- `Error` and its components are now `Clone` (#116)
- MSRV (minimum supported Rust version) is now 1.61.0 (from 1.42.0)

## [0.11.0] - 2023-01-12
### Added
- Support for [conditionals](https://www.regular-expressions.info/conditional.html): using a regex like
  `(?<test>a)?b(?(test)c|d)` will try to match `c` after `b` if `a` matched in the capture group named
  `test`, otherwise `d` after `b` if `a` wasn't captured into the `test` group.
### Changed
- Updated parse errors to show the position they occurred at.
### Fixed
- Fix panic when backref is used within referenced group itself and
  group end index is not known yet (#103)

## [0.10.0] - 2022-04-28
### Added
- Support for `\G` ([anchor to end of previous match](https://www.regular-expressions.info/continue.html)): Using a regex
  like `\G\w` will match each letter of `foo` in `foo bar` but
  nothing else.

## [0.9.0] - 2022-04-21
### Added
- Support for `\K` ([keep out](https://www.regular-expressions.info/keep.html)): Using a regex like `@\K\w+` will match
  things like `@foo` but the resulting match text will only include
  `foo`, keeping out the `@`.

## [0.8.0] - 2022-02-22
### Added
- Allow users to disable any of the `unicode` and `perf-*` features of
  the regex crate. Disabling these features can reduce compile time
  and/or binary size for use cases where these features are not needed.
  (All features remain enabled by default.)
### Changed
- MSRV (minimum supported Rust version) is now 1.42.0 (from 1.41.1)

## [0.7.1] - 2021-07-29
### Fixed
- Fix panic on incomplete escape sequences in input regexes
- Disallow quantifers on lookarounds and other zero-width assertion
  expressions, e.g. the `+` in `(?=hello)+`

## [0.7.0] - 2021-07-12
### Added
- `Regex` now has replace methods like the regex crate:
  - `replace` - single replacement
  - `replace_all` - replace all non-overlapping matches
  - `replacen` - configurable number of replacements

## [0.6.0] - 2021-05-17
### Added
- `Regex` now implements `Clone`, `Display`, `FromStr`
- `Captures` now implements `Index<usize>` to access captures by number
  and `Index<&str>` to access by name

## [0.5.0] - 2021-02-15
### Added
- Methods `find_iter` and `captures_iter` to iterate over all
  non-overlapping matches for a string
- Method `find_from_pos` to `find` starting from a specific position
### Changed
- MSRV (minimum supported Rust version) is now 1.41.1 (from 1.32.0)

## [0.4.1] - 2020-11-09
### Added
- `escape` function to escape special characters in a string so that it
  matches literally

## [0.4.0] - 2020-09-27
### Added
- Support for named groups and backrefs:
  - Capture with `(?<name>...)` or `(?P<name>...)`
  - Backref with `\k<name>` or `(?P=name)`
  - `Captures::name` to get matched group by name
  - `Regex::capture_names` to get capture names in regex
- Support for expanding matches using a replacement template string
  - `Captures::expand` for regex crate compatible syntax
  - See `Expander` for python-compatible syntax and advanced usage
- `Match::range` and some `From` impls for convenience

## [0.3.5] - 2020-04-28
### Changed
- Include string snippet in errors for unknown group and invalid escape
  to make it easier to identify the problem.

## [0.3.4] - 2020-04-28
### Added
- Support comments using `(?# comment)` syntax
- Support unicode escapes like `\u21D2` and `\U0001F60A`

## [0.3.3] - 2020-02-28
### Changed
- Optimization: Delegate const-sized suffixes in more cases
- Optimization: Use `captures_read_at` when delegating to regex crate

## [0.3.2] - 2020-02-05
### Fixed
- Some regexes with fancy parts in the beginning/middle didn't match
  when they should have, e.g. `((?!x)(a|ab))c` didn't match `abc`.

## [0.3.1] - 2019-12-09
### Added
- Add `delegate_size_limit` and `delegate_dfa_size_limit` to
  `RegexBuilder` to allow configuring these limits for regex crate.

## [0.3.0] - 2019-11-27
### Added
- Add limit for backtracking so that execution errors instead of running
  for a long time in case of catastrophic backtracking.
- Add `RegexBuilder` with `backtrack_limit` to configure the new
  backtrack limit per regex.
- `Error` now implements `std::error::Error` trait
### Fixed
- Fix panic in backref matching with multibyte chars

## [0.2.0] - 2019-10-19
### Added
- More documentation and examples
- Support character class nesting and intersections (implemented in
  regex crate)
- Support atomic groups, both the the `(?>foo)` group syntax and the
  `a++`, `a*+` and `a?+` possessive syntax
- Support `\b`, `\f`, `\t`, `\n`, `\r`, `\v`
- Support look-behind with variable sized alternative
- Implement `Debug` for `Regex`
- More test coverage including running one of Oniguruma's test suites
### Changed
- Change `find` to return a `Match` struct (breaking change)
- Change `Captures` API (breaking change):
  - Replace `at` and `pos` with `get` that returns a `Match` struct
  - Remove `is_empty` (use `len`)
- Allow unescaped `]` and `}` as literals
- Allow unescaped `{` as literal when not after atom
- Allow escapes such as `\<` or `\e` inside character classes
- Allow up to 8 characters in `\x{...}` escape
- Allow escaping of space to make literal space
- Allow `(a|)`
- Reject invalid backreferences
### Fixed
- Multiple fixes for alternatives in look-arounds
- Fix hex escape to not include letters after "F"
- Fix handling of unescaped `]` in character classes
- Fix case insensitive character classes and other escapes
- Don't ignore spaces in character classes even with "comment mode"

## [0.1.0] - 2017-02-06
### Added
- Initial release


[0.16.2]: https://github.com/fancy-regex/fancy-regex/compare/0.16.1...0.16.2
[0.16.1]: https://github.com/fancy-regex/fancy-regex/compare/0.16.0...0.16.1
[0.16.0]: https://github.com/fancy-regex/fancy-regex/compare/0.15.0...0.16.0
[0.15.0]: https://github.com/fancy-regex/fancy-regex/compare/0.14.0...0.15.0
[0.14.0]: https://github.com/fancy-regex/fancy-regex/compare/0.13.0...0.14.0
[0.13.0]: https://github.com/fancy-regex/fancy-regex/compare/0.12.0...0.13.0
[0.12.0]: https://github.com/fancy-regex/fancy-regex/compare/0.11.0...0.12.0
[0.11.0]: https://github.com/fancy-regex/fancy-regex/compare/0.10.0...0.11.0
[0.10.0]: https://github.com/fancy-regex/fancy-regex/compare/0.9.0...0.10.0
[0.9.0]: https://github.com/fancy-regex/fancy-regex/compare/0.8.0...0.9.0
[0.8.0]: https://github.com/fancy-regex/fancy-regex/compare/0.7.1...0.8.0
[0.7.1]: https://github.com/fancy-regex/fancy-regex/compare/0.7.0...0.7.1
[0.7.0]: https://github.com/fancy-regex/fancy-regex/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/fancy-regex/fancy-regex/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/fancy-regex/fancy-regex/compare/0.4.1...0.5.0
[0.4.1]: https://github.com/fancy-regex/fancy-regex/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/fancy-regex/fancy-regex/compare/0.3.5...0.4.0
[0.3.5]: https://github.com/fancy-regex/fancy-regex/compare/0.3.4...0.3.5
[0.3.4]: https://github.com/fancy-regex/fancy-regex/compare/0.3.3...0.3.4
[0.3.3]: https://github.com/fancy-regex/fancy-regex/compare/0.3.2...0.3.3
[0.3.2]: https://github.com/fancy-regex/fancy-regex/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/fancy-regex/fancy-regex/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/fancy-regex/fancy-regex/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/fancy-regex/fancy-regex/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/fancy-regex/fancy-regex/commits/0.1.0
