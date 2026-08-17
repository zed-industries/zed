<div align="center">

# notify-rust

[![build](https://img.shields.io/github/actions/workflow/status/hoodie/notify-rust/build-platforms.yml?branch=main)](https://github.com/hoodie/notify-rust/actions?query=workflow%3A"Continuous+Integration")
[![Crates.io](https://img.shields.io/crates/d/notify-rust)](https://crates.io/crates/notify-rust)
[![contributors](https://img.shields.io/github/contributors/hoodie/notify-rust)](https://github.com/hoodie/notify-rust/graphs/contributors)
![maintenance](https://img.shields.io/maintenance/yes/2027)

[![version](https://img.shields.io/crates/v/notify-rust)](https://crates.io/crates/notify-rust/)
[![documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/notify-rust/)
[![license](https://img.shields.io/crates/l/notify-rust.svg?style=flat)](https://crates.io/crates/notify-rust/)

</div>

A not so well-named library for displaying desktop notifications.

```toml
[dependencies]
notify-rust = "4"
```

## Usage & Documentation

Please see the [documentation](https://docs.rs/crate/notify-rust/) for current examples.

### Simple Notification

```rust
use notify_rust::Notification;
Notification::new()
    .summary("Firefox News")
    .body("This will almost look like a real firefox notification.")
    .icon("firefox")
    .show()?;
```

### Persistent Notification

```rust
use notify_rust::{Notification, Hint};
Notification::new()
    .summary("Category:email")
    .body("This has nothing to do with emails.\nIt should not go away until you acknowledge it.")
    .icon("thunderbird")
    .appname("thunderbird")
    .hint(Hint::Category("email".to_owned()))
    .hint(Hint::Resident(true)) // this is not supported by all implementations
    .timeout(0) // this however is
    .show()?;
```

## Linux/BSD support

The main audience of this library are Linux/BSD based desktop environments that follow the XDG specification (see [gnome dev docs](http://web.archive.org/web/20200606201408/https://developer.gnome.org/notification-spec/) or [libnotify docs](https://www.galago-project.org/specs/notification/0.9/index.html)). These include KDE, Gnome, XFCE, LXDC, Mate (and probably also most others that I haven't personally tested).

### Features

#### `images`

Enables sending of images with notifications via the `image_data()` / pixel-buffer API. This is only supported on XDG and requires the [**image** crate](https://lib.rs/image) as well as [**lazy_static**](https://lib.rs/lazy_static) to determine the supported spec-version on startup.

> **Note on `image_path()`:** Passing an image by file path via `image_path()` does **not** require the `images` feature on any platform. On Linux/BSD it maps to the `image-path` XDG hint, on macOS it maps to `content_image` in `mac-notification-sys`, and on Windows it is passed directly to `winrt-notification` — all without any additional dependencies.

#### `d`

Enables the usage of [**dbus-rs**](https://lib.rs/dbus) instead of [**zbus**](https://lib.rs/zbus) (also XDG only).
This is functionally identical to the default feature set.
**As long as you still compile with default-features this will only enable dbus-rs usage, but not default to it!** In order to use the dbus-rs implementation set the environment variable **`DBUSRS `** or compile notify-rust with **`--no-default-features`**.

## macOS support

This library shines on linux and bsd, which is its original target platform.
Lately it gained support for macOS thanks to [mac-notification-sys](https://crates.io/crates/mac-notification-sys).
However this only includes a small subset of the current functionality, since [`NSNotification`](https://developer.apple.com/reference/foundation/nsnotification)s don't have as many features.

**call for participation:** You are a versed macOS UI developer with mad Objective-C skillz? <abbr title="pull request sil vous plait">PRSV</abbr>.

## Windows support

Similar to macOS we support windows via the help of [winrt-notification](https://crates.io/crates/winrt-notification).

## Commandline tool

Checkout [toastify](https://github.com/hoodie/toastify), it exposes most of the functionality of the lib to the commandline.

## Contributing

Any help in form of descriptive and friendly [issues](https://github.com/hoodie/notify-rust/issues) or comprehensive pull requests are welcome!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in notify-rust by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

### Conventions

The Changelog of this library is generated from its commit log, there any commit message must conform with https://www.conventionalcommits.org/en/v1.0.0/. For simplicity you could make your commits with [convco](https://crates.io/crates/convco).

### Recognition

Thanks goes to these wonderful people:

 <a href="https://github.com/hoodie/notify-rust/graphs/contributors">
   <img src="https://contrib.rocks/image?repo=hoodie/notify-rust" />
 </a>

### License

<sup>
`notify-rust` is licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
