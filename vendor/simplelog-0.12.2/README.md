# simplelog [![Build Status](https://github.com/drakulix/simplelog.rs/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/Drakulix/simplelog.rs/actions) [![Coverage Status](https://coveralls.io/repos/github/Drakulix/simplelog.rs/badge.svg?branch=master)](https://coveralls.io/github/Drakulix/simplelog.rs?branch=master) [![Crates.io](https://img.shields.io/crates/v/simplelog.svg)](https://crates.io/crates/simplelog) [![Crates.io](https://img.shields.io/crates/l/simplelog.svg)](https://crates.io/crates/simplelog)
## A simple and easy-to-use logging facility for Rust's [log](https://crates.io/crates/log) crate

`simplelog` does not aim to provide a rich set of features, nor to provide the
best logging solution. It aims to be a maintainable, easy to integrate facility
for small to medium sized projects, that find [env_logger](https://crates.io/crates/env_logger)
to be somewhat lacking in features. In those cases `simplelog` should provide an
easy alternative.

## Concept
`simplelog` provides a series of logging facilities, that can be easily combined.

- `SimpleLogger` (very basic logger that logs to stderr/out, should never fail)
- `TermLogger` (advanced terminal logger, that splits to stderr/out and has color support) (can be excluded on unsupported platforms)
- `WriteLogger` (logs to a given struct implementing `Write`. e.g. a file)
- `CombinedLogger` (can be used to form combinations of the above loggers)

## Usage
```rust
#[macro_use] extern crate log;
extern crate simplelog;

use simplelog::*;

use std::fs::File;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");
}

```

### Results in
```
$ cargo run --example usage
   Compiling simplelog v0.12.2 (file:///home/drakulix/Projects/simplelog)
     Running `target/debug/examples/usage`
[ERROR] Bright red error
```
and my_rust_binary.log
```
11:13:03 [ERROR] usage: Bright red error
11:13:03 [INFO] usage: This only appears in the log file
```

## Getting Started

Just add
```
[dependencies]
simplelog = "^0.12.2"
```
to your `Cargo.toml`

## ANSI color and style support

This crate can internally depend on a [paris](https://github.com/0x20F/paris) crate to provide support for ANSI color and styles.
To use this feature you need to set a _paris_ feature, like this:
```
[dependencies]
simplelog = { version = "^0.12.2", features = ["paris"] }
```
in your `Cargo.toml`

After this you can use e.g. the following call:
```rust
info!("I can write <b>bold</b> text or use tags to <red>color it</>");
```

This will automatically generates terminal control sequences for desired styles.

More formatting info: [paris crate documentation](https://github.com/0x20F/paris)

## [Documentation](https://docs.rs/simplelog/)

## Contributing
If you wish to contribute your own logger to `simplelog` or advance/extend existing loggers,
feel free to create a pull request. Just don't blindly assume, that your logger will be accepted.
The rational about this library is, that it is simple to use. This mostly comes down to a small
and easy API, but also includes things like the amount of added dependencies. If you feel unsure
about your plans, feel free to create an issue to talk about your ideas.

### Happy Coding!
