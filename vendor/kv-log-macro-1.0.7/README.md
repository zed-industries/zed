# kv-log-macro
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Log macro for log's kv-unstable backend.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
```rust
use kv_log_macro::info;

fn main() {
    femme::start(log::LevelFilter::Info).unwrap();
    info!("hello");
    info!("hello",);
    info!("hello {}", "cats");
    info!("hello {}", "cats",);
    info!("hello {}", "cats", {
        cat_1: "chashu",
        cat_2: "nori",
    });
}
```

## Installation
```sh
$ cargo add kv-log-macro
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
None.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/kv-log-macro.svg?style=flat-square
[2]: https://crates.io/crates/kv-log-macro
[3]: https://img.shields.io/travis/yoshuawuyts/kv-log-macro/master.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/kv-log-macro
[5]: https://img.shields.io/crates/d/kv-log-macro.svg?style=flat-square
[6]: https://crates.io/crates/kv-log-macro
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/kv-log-macro

[releases]: https://github.com/yoshuawuyts/kv-log-macro/releases
[contributing]: https://github.com/yoshuawuyts/kv-log-macro/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/kv-log-macro/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/kv-log-macro/labels/help%20wanted
