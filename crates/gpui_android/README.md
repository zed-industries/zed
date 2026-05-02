# gpui_android

[GPUI](https://gpui.rs)'s Android platform back-end.

It implements GPUI's `Platform`, `PlatformWindow`, `PlatformDispatcher`,
`PlatformDisplay`, `PlatformAtlas`, `PlatformKeyboardLayout` and
`PlatformTextSystem` traits on top of:

- [`android-activity`](https://crates.io/crates/android-activity) (the
  `GameActivity` backend) for the JVM ⇄ native event pump
- [`wgpu`](https://crates.io/crates/wgpu) over Vulkan for rendering
- [`cosmic-text`](https://crates.io/crates/cosmic-text) for shaping/layout,
  fed from `/system/fonts` (we re-implement what `fontdb` 0.23 forgot to
  do on Android)
- raw [`jni`](https://crates.io/crates/jni) calls for clipboard, intents,
  AndroidKeyStore credentials, system-bell — no Java/Kotlin shim required

## Status

- Phase 1 of the original plan is **shipped end-to-end**: the
  `crates/gpui/examples/hello_android` example builds, installs, and renders
  an interactive gallery on a real device (Mali-G57 / Samsung).
- Phase 2 (custom JVM `View` for first-class IME + AccessKit a11y) is
  blocked on upstream `rust-mobile/android-view` stabilising. The platform's
  trait surface is shaped to plug it in without breaking changes.

## Quick start

```rust
#![cfg(target_os = "android")]

use gpui::{App, WindowOptions};
use gpui_platform::application;

#[unsafe(no_mangle)]
fn android_main(app: gpui_android::AndroidApp) {
    android_logger::init_once(android_logger::Config::default());
    gpui_android::set_android_app(app);
    application().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| MyRoot::new())).unwrap();
        cx.activate(true);
    });
}
```

`Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]

[target.'cfg(target_os = "android")'.dependencies]
gpui          = { git = "https://github.com/zed-industries/zed", branch = "feat-android-support" }
gpui_platform = { git = "https://github.com/zed-industries/zed", branch = "feat-android-support", default-features = false }
gpui_android  = { git = "https://github.com/zed-industries/zed", branch = "feat-android-support" }
log            = "0.4"
android_logger = "0.14"
```

## Full setup guide

`gpui_android` doesn't ship a Java side-car or a build wrapper; you need a
small `GameActivity` subclass + a Gradle harness around your `.so`.

**See [`SETUP.md`](./SETUP.md)** for:

- exact NDK / JDK / Gradle versions we tested with
- a copy-paste-ready Gradle module
- every Android-specific gotcha we hit (theme, kotlin-stdlib collision,
  font-db, surface lifecycle, RefCell re-entry, edge-to-edge, …)
- the working build matrix
- the publishing roadmap (today: git dep; next: `cargo add gpui`)

The runnable reference implementation lives at
`crates/gpui/examples/hello_android.rs` with its Gradle harness at
`crates/gpui/examples/android-host/`.
