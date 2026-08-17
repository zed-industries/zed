<div align="center">

# mac-notification-sys

![platform](https://img.shields.io/badge/platform-macOS-lightgrey)
[![version](https://img.shields.io/crates/v/mac-notification-sys)](https://crates.io/crates/mac-notification-sys/)
[![license](https://img.shields.io/crates/l/mac-notification-sys)](https://crates.io/crates/mac-notification-sys/)
[![contributors](https://img.shields.io/github/contributors/h4llow3En/mac-notification-sys)](https://github.com/h4llow3En/mac-notification-sys/graphs/contributors)


[![build](https://img.shields.io/github/actions/workflow/status/h4llow3En/mac-notification-sys/ci.yml?branch=master)](https://github.com/h4llow3En/mac-notification-sys/actions?query=workflow%3A"Continuous+Integration")
![downloads](https://img.shields.io/crates/d/mac-notification-sys)
[![documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/mac-notification-sys/)

</div>

A simple wrapper to deliver or schedule macOS Notifications in Rust.

## Usage

```toml
#Cargo.toml
[dependencies]
mac-notification-sys = "0.6"
```

## Documentation

The documentation can be found [here](https://docs.rs/mac-notification-sys/latest/mac_notification_sys/)

## Example

```rust
use mac_notification_sys::*;

fn main() {
    let bundle = get_bundle_identifier_or_default("firefox");
    set_application(&bundle).unwrap();

    send_notification(
        "Danger",
        Some("Will Robinson"),
        "Run away as fast as you can",
        None,
    )
    .unwrap();

    send_notification(
        "NOW",
        None,
        "Without subtitle",
        Some(Notification::new().sound("Blow")),
    )
    .unwrap();
}

```

## TODO

- [ ] Add timeout option so notifications can be auto-closed
- [ ] Allow NSDictionary to hold various types (perhaps with a union?)
- [ ] Switch to UserNotification if possible

## Contributors

 Thanks goes to these wonderful people:
 <a href="https://github.com/h4llow3En/mac-notification-sys/graphs/contributors">
   <img src="https://contrib.rocks/image?repo=h4llow3En/mac-notification-sys" />
 </a>

Any help in form of descriptive and friendly [issues](https://github.com/h4llow3En/mac-notification-sys/issues) or comprehensive pull requests are welcome!

## license
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in mac-notification-sys by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

### Conventions
The Changelog of this library is generated from its commit log, there any commit message must conform with https://www.conventionalcommits.org/en/v1.0.0/. For simplicity you could make your commits with [convco](https://crates.io/crates/convco).
