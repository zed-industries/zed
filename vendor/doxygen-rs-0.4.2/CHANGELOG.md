# Changelog

## Version 0.4.2

- Improve group parsing
- Trim strings where required ([rust3ds/ctru-rs#125](https://github.com/rust3ds/ctru-rs/pull/125#issuecomment-1568644343))

## Version 0.4.1

- Fix missing parentheses in text ([#7](https://github.com/Techie-Pi/doxygen-rs/pull/7))

## Version 0.4.0

- Rewrote _all_ the internals to improve speed and developer experience.
- Only exposes the `doxygen_rs::transform` function and the `doxygen_rs::generator::rustdoc` function.

## Version 0.3.0

- Improve support for sublists and multiline strings [#5](https://github.com/Techie-Pi/doxygen-rs/pull/5)
- Improve documentation