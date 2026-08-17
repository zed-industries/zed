# xkeysyms

This crate provides constants representing all of the X11 keyboard symbols. It 
also provides utility functions for working with those symbols, and for
converting between keyboard codes and keyboard symbols. This crate does not 
depend on a particular implementation of the X11 protocol and can therefore be
used in any context where X11 keyboard symbols are needed.

In addition, this crate contains no unsafe code and is fully compatible with
`no_std` environments.

## MSRV Policy

The Minimum Safe Rust Version for this crate is **1.58.1**.

## Updating Headers

To update the automatically generated keyboard symbols in the
`automatically_generated.rs` file, install [Just] and run `just`. The process
creates a Debian Docker container in order to keep the files consistent, so make
sure Docker is installed first.

[Just]: https://github.com/casey/just

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or 
   http://opensource.org/licenses/MIT)
 * Zlib license ([LICENSE-ZLIB](LICENSE-ZLIB) or 
   https://opensource.org/licenses/Zlib)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
triple licensed as above, without any additional terms or conditions.

