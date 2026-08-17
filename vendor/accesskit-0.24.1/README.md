# AccessKit

This is the shared cross-platform crate for [AccessKit](https://accesskit.dev/). It defines the data structures that represent an accessibility tree, and the trait for handling action requests from assistive technologies.

To use AccessKit in your application or toolkit, you will also need a platform adapter. The following platform adapters are currently available:

* [accesskit_windows](https://crates.io/crates/accesskit_windows): exposes an AccessKit tree on Windows using the UI Automation API
* [accesskit_macos](https://crates.io/crates/accesskit_macos): exposes an AccessKit tree on MacOS through the Cocoa `NSAccessibility` protocol
* [accesskit_unix](https://crates.io/crates/accesskit_unix): exposes an AccessKit tree on Linux and Unix systems through the AT-SPI protocol
* [accesskit_android](https://crates.io/crates/accesskit_android): exposes an AccessKit tree on Android through the Java-based Android accessibility API
* [accesskit_winit](https://crates.io/crates/accesskit_winit): wraps other platform adapters for use with the [winit](https://crates.io/crates/winit) windowing library

Some platform adapters include simple examples.
