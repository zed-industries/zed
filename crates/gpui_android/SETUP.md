# Building a GPUI app for Android

This guide takes you from zero to an APK that boots, renders frames on the
device's GPU and accepts touch input — using `gpui` + `gpui_android` from
this workspace. The runnable example referenced throughout is
`crates/gpui/examples/hello_android.rs`, packaged by the Gradle harness at
`crates/gpui/examples/android-host/`.

Each command in this guide is the **exact one we use** to produce the
existing APK; nothing here is aspirational.

---

## 1. One-time prerequisites

You need four things on the host:

| What | Tested version | Notes |
|---|---|---|
| Android NDK | 29.0.14206865 | 26+ should work. Install via Android Studio's SDK Manager, or `sdkmanager "ndk;29.0.14206865"` |
| Rust toolchain (workspace pins it) | 1.95.0 | The repo's `rust-toolchain.toml` selects it; rustup auto-installs |
| Rustup Android targets | `aarch64-linux-android`, `x86_64-linux-android` (optional `armv7-linux-androideabi`) | `rustup target add aarch64-linux-android x86_64-linux-android` |
| JDK 21 with `javac` | OpenJDK 21 / Android Studio's JBR | Ubuntu's `openjdk-21-jre` is **JRE only** — Gradle needs `javac`. The bundled JBR at `/snap/android-studio/current/jbr` works. |

**Android SDK** — install at least one platform JAR (`platforms/android-34`
in our case). The `compileSdk` value in
`crates/gpui/examples/android-host/app/build.gradle.kts` decides which JAR
Gradle expects; bump that line if you only have a different platform
installed.

**Gradle** — the example's `gradlew` is a wrapper that downloads Gradle 8.10.2
on first invocation. If you don't have the wrapper yet, generate it once:

```bash
cd crates/gpui/examples/android-host
JAVA_HOME=/snap/android-studio/current/jbr \
PATH=$JAVA_HOME/bin:$PATH \
gradle wrapper --gradle-version=8.10.2
```

If you don't have a system `gradle` either, [download the
distribution](https://services.gradle.org/distributions/gradle-8.10.2-bin.zip),
unzip it, and use its `bin/gradle` for that single bootstrap step.

---

## 2. Build the example end-to-end

The repo ships two scripts:

- `script/build-gpui-android` — runs `cargo` with the right NDK env vars set.
- `script/build-gpui-android-apk` — chains the cargo build + Gradle assembly.

A debug APK in one command:

```bash
JAVA_HOME=/snap/android-studio/current/jbr \
PATH=$JAVA_HOME/bin:$PATH \
ANDROID_HOME=$HOME/Android/Sdk \
script/build-gpui-android-apk debug
```

Output: `crates/gpui/examples/android-host/app/build/outputs/apk/debug/app-debug.apk`

Release builds: `script/build-gpui-android-apk release`. The release variant
runs R8 over `proguard-rules.pro`, which preserves the JNI entry-points
(`-keepclasseswithmembernames class * { native <methods>; }`) and the
`GameActivity` glue.

To install + launch + watch logs:

```bash
adb install -r crates/gpui/examples/android-host/app/build/outputs/apk/debug/app-debug.apk
adb shell am start -n dev.zed.gpui.gallery/.GalleryActivity
adb logcat | grep -E "RustStdoutStderr|RustPanic|hello_android|gpui_android|gpui_wgpu"
```

You should see `Loaded NNN Android system fonts`, then GPU adapter selection
(`Selected GPU adapter: "<name>" (Vulkan)`), then `the first frame is
available`. The phone shows the dark gallery with a counter, theme toggle,
five accent swatches and a device-info panel.

---

## 3. Use `gpui_android` in a new app

The example sits inside this workspace; for a stand-alone app you need:

### 3.1 Cargo skeleton

```toml
# Cargo.toml
[package]
name = "my_android_app"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[target.'cfg(target_os = "android")'.dependencies]
gpui          = { git = "https://github.com/zed-industries/zed", branch = "feat-android-support" }
gpui_platform = { git = "https://github.com/zed-industries/zed", branch = "feat-android-support", default-features = false, features = ["android"] }
gpui_android  = { git = "https://github.com/zed-industries/zed", branch = "feat-android-support" }
log            = "0.4"
android_logger = "0.14"
```

> **Note the `features = ["android"]` on `gpui_platform`.** The Android
> backend is opt-in — without that feature, `gpui_platform` compiles fine
> for `target_os = "android"` (handy for library crates that want to be
> Android-portable without pulling in `wgpu`/`cosmic-text`/JNI deps), but
> `gpui_platform::current_platform()` panics with a clear message at
> runtime if you try to actually use it. Apps that ship to a device
> always need the feature on; library crates can leave it off and let the
> end binary opt in.

> Today every `gpui_*` crate uses workspace-internal crates (notably
> `collections`, `gpui_macros`, etc.). Until those are factored out into
> their own published crates, **the only way to consume `gpui` outside this
> repo is via a git dependency** pointing at this branch. Updating the
> story for `cargo add gpui` is tracked separately — see the **Publishing**
> section at the end.

### 3.2 `src/lib.rs`

```rust
#![cfg(target_os = "android")]

use gpui::{App, Bounds, Context, Render, Window, WindowOptions, div, prelude::*, px, rgb};
use gpui_platform::application;

struct Hello;

impl Render for Hello {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x202020))
            .text_color(rgb(0xffffff))
            .text_3xl()
            .flex()
            .items_center()
            .justify_center()
            .child("Hello, Android!")
    }
}

#[unsafe(no_mangle)]
fn android_main(app: gpui_android::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    gpui_android::set_android_app(app);
    application().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Hello)).unwrap();
        cx.activate(true);
    });
}
```

### 3.3 Cross-compile

```bash
# The minimum env-var set the cargo build needs:
NDK=$HOME/Android/Sdk/ndk/29.0.14206865/toolchains/llvm/prebuilt/linux-x86_64
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$NDK/bin/aarch64-linux-android26-clang
export CC_aarch64_linux_android=$NDK/bin/aarch64-linux-android26-clang
export AR_aarch64_linux_android=$NDK/bin/llvm-ar

cargo build --release --target aarch64-linux-android
# → target/aarch64-linux-android/release/libmy_android_app.so
```

### 3.4 Gradle harness

The fastest way to get a working harness is to copy
`crates/gpui/examples/android-host/` and rename a few identifiers:

| File | Replace |
|---|---|
| `app/build.gradle.kts` | `dev.zed.gpui.gallery` → your app id; `keepDebugSymbols ... libhello_android.so` → your `.so` filename |
| `app/src/main/AndroidManifest.xml` | `lib_name` value, activity name, `@style/Theme.GpuiGallery` |
| `app/src/main/java/.../GalleryActivity.java` | package, class name, `System.loadLibrary("…")` |
| `app/src/main/res/values/styles.xml` | theme name (the manifest must reference it) |
| `app/proguard-rules.pro` | `dev.zed.gpui.gallery.**` → your package |

Then drop the `.so` into `app/src/main/jniLibs/arm64-v8a/lib<your_lib>.so`
and `./gradlew :app:assembleDebug`.

---

## 4. The non-obvious gotchas

These all blew up the example at some point. Each is a one-line fix once
you know it; the order below is roughly the order you'll hit them.

### Touch drag → `ScrollWheel` synthesis

GPUI's `overflow_scroll` only reacts to `PlatformInput::ScrollWheel`
events, not to `MouseMove`. Touchscreens have no wheel, so
`gpui_android` synthesises a companion `ScrollWheel` for every touch
drag inside `AndroidWindow::dispatch_input`: phase `Started` on
`MouseDown`, `Moved` with `delta = -(current - previous)` finger
position on each `MouseMove`, and `Ended` on `MouseUp`. The negation
makes a finger-up swipe scroll content *up* (i.e. reveal what's below) —
the natural touch convention. Quick taps still fire click handlers
because their hit-test only requires the down/up pair to land on the
same element. Containers you want to scroll need an `id(...)` and
`overflow_y_scroll()` (or `overflow_scroll`) just like on every other
platform.

### `request_frame` has to be driven continuously

GPUI marks the active entity dirty whenever `cx.notify()` runs, but it
relies on the platform calling its `request_frame` callback on every
vsync to actually re-render. On the web that's `requestAnimationFrame`;
on Wayland it's the surface frame callback. **`gpui_android` drives it
from inside the `app.poll_events` loop** (every ~16 ms). Without that, a
click handler fires, state mutates, `cx.notify()` runs — and the screen
silently never updates, so the app looks frozen ("touch isn't working")
even though touch is fine. If you re-implement the run loop, make sure
the equivalent `dispatch_request_frame` call happens every iteration.

### Theme

GameActivity extends `AppCompatActivity`, which throws
`IllegalStateException: You need to use a Theme.AppCompat theme (or
descendant)` if your activity's theme is the legacy framework
`@android:style/Theme.NoTitleBar.Fullscreen`. Use a theme that descends from
`Theme.AppCompat.*` (see `styles.xml` in the example).

### androidx.appcompat

`GameActivity` references `androidx.appcompat.app.AppCompatActivity`, so
even if your Java doesn't use any AppCompat widgets you still need
`implementation("androidx.appcompat:appcompat:1.7.0")` in
`app/build.gradle.kts`. Without it: `cannot access AppCompatActivity` at
the `compileDebugJavaWithJavac` step.

### Kotlin stdlib jdk7/jdk8 collision

`androidx.games:games-activity:3.0.5` pulls in `kotlin-stdlib-jdk7:1.6.21`
and `kotlin-stdlib-jdk8:1.6.21`. `androidx.appcompat:1.7.0` brings
`kotlin-stdlib:1.8.22` which already bundles those classes → R8 sees
duplicate classes. Fix with constraints (already in
`crates/gpui/examples/android-host/app/build.gradle.kts`):

```kotlin
constraints {
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk7:1.8.22")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8:1.8.22")
}
```

### Android system fonts

`fontdb 0.23` (the version `cosmic-text 0.17` depends on) has cfg branches
for Windows, Linux and macOS only. On Android its `load_system_fonts()`
**does nothing**, so even `Roboto` doesn't resolve. `gpui_android`'s
`AndroidPlatform::new` walks `/system/fonts`, `/data/fonts` and
`/product/fonts` manually and feeds every `.ttf`/`.ttc`/`.otf` into
`cosmic-text` — there's nothing to do on the consumer side, but be aware
that startup logs `Loaded NNN Android system fonts` and that's normal.

### Surface lifecycle

Android can revoke the `ANativeWindow` at any moment. `gpui_android` drops
the `wgpu::Surface` synchronously inside `MainEvent::TerminateWindow` and
gates every draw on a `surface_alive: AtomicBool`. **Don't** cache a raw
`NativeWindow` pointer of your own — call
`gpui_android::android_app().unwrap().native_window()` whenever you need
one.

### `on_finish_launching` waits for the surface

GPUI's `Application::run` synchronously calls your launch closure, which
calls `cx.open_window`, which expects the window's `sprite_atlas` to be
real (not a no-op). On Android we don't have a `NativeWindow` until
`MainEvent::InitWindow` arrives. `AndroidPlatform::run` therefore **defers
your `on_finish_launching` closure** until the first surface has been
published — your launch code may run a few hundred milliseconds after
`Platform::run` is called.

### Re-entrant `RefCell`s

Every `dispatch_*` / `update_*` method on `AndroidWindow` uses
`self.state.borrow_mut()` to update its own state, then invokes a user
callback. The callback frequently re-enters `PlatformWindow` methods (`scale_factor`,
`bounds`, …) that also borrow `self.state`. The crate's helpers scope the
borrow to a block that ends *before* the callback fires; do the same in
any code you add.

### `#[unsafe(no_mangle)]`

Edition 2024 requires the `unsafe(...)` wrapper. Without it `rustc` rejects
the attribute and your `android_main` symbol never lands in the `.so`.

### Java version

Gradle 8.10.2 supports JDK up to 23. Ubuntu's `openjdk-21-jre` is **a JRE
only** — Gradle's compile step needs `javac` and aborts with
`Toolchain installation … does not provide the required capabilities:
[JAVA_COMPILER]`. Use Android Studio's bundled JBR
(`/snap/android-studio/current/jbr` on Linux) or install `openjdk-21-jdk`.

### `compileSdk` matching the platform JAR

If `compileSdk = 35` and only `platforms/android-34` is installed under
your SDK, AGP errors out at resource processing. Either install the
matching platform or lower `compileSdk` (we use 34 because that's what
ships with Android Studio on this host). `targetSdk` is independent of
`compileSdk` and is what Play Store cares about.

### R8 + JNI

R8 strips unused symbols in release builds, including `native <methods>`
that are only called from C/Rust. The `proguard-rules.pro` keep rule is
mandatory for release builds; debug skips R8 entirely so it doesn't bite
there.

### Edge-to-edge

`targetSdk = 35` enforces edge-to-edge. The example's theme already opts in
(`windowDrawsSystemBarBackgrounds`, transparent system bar colors). Your
content must reserve room for the status bar (typically ~80 logical pixels
top inset on a phone) and the gesture/nav bar at the bottom. The example
hard-codes 80px / 48px padding; the `MainEvent::InsetsChanged` /
`ContentRectChanged` events give you the real numbers in
`AndroidWindow::update_content_rect`.

### "Soft keyboard doesn't appear when I tap the counter"

The counter in the gallery is a **read-only number** — the `−` and `+`
buttons mutate it. It's not a text input, so Android won't show a
keyboard on tap (and `gpui_android` won't ask it to via
`AndroidApp::show_soft_input`). The IME-bridge plumbing is in place
(`AndroidWindow::set_input_handler` calls `set_ime_editor_info` +
`show_soft_input(false)` when GPUI registers a real
`PlatformInputHandler`); when you write a real text input element on
top of GPUI's `PlatformInputHandler` API, the keyboard will pop up
automatically.

For a working reference, `gpui_android::widgets::TextField` is a
single-line text field that handles the focus → IME plumbing for you.
Drop it into your view as `cx.new(|cx| TextField::new(cx, "placeholder",
FieldKind::Text))` and the keyboard pops on tap.

### Soft keyboard covers the focused input

With edge-to-edge enabled (the default on `targetSdk >= 35`), Android no
longer auto-resizes the GameActivity surface when the IME slides up,
even with `windowSoftInputMode="adjustResize"`. The window stays full
screen and the IME draws on top.

`gpui_android` queries `WindowInsets.Type.ime()` over JNI on every
`MainEvent::InsetsChanged` event (and again every iteration of the run
loop while the keyboard is up, to track the slide-up animation), and
shrinks the bounds it reports to GPUI by the IME's bottom inset. As
long as your root container uses `overflow_y_scroll()` (or any
flex/scroll layout that respects the window's `content_size`), the
focused input stays above the keyboard. The query only runs while the
keyboard is alive, so idle frames stay cheap.

Pre-API-30 devices don't expose the typed `ime()` inset; on those the
inset reads as `0` and the IME overlaps content. There's no good
fallback because `getSystemWindowInsets()` doesn't separate the IME
from the nav bar.

---

## 5. Build matrix that works today

Confirmed working on this branch:

| | aarch64-linux-android | x86_64-linux-android | armv7-linux-androideabi |
|---|---|---|---|
| `cargo check -p gpui_android --target …` | ✅ | ✅ | ✅ |
| `cargo build --release --example hello_android --target …` | ✅ | needs jniLibs/x86_64 in APK | not in APK matrix |
| `:app:assembleDebug` APK | ✅ | ✅ (if `.so` is present) | n/a |

The `app/build.gradle.kts` `abiFilters` line lists `arm64-v8a` and
`x86_64`. Add `"armeabi-v7a"` if you also want 32-bit ARM (not recommended
for new apps).

---

## 6. Publishing — what stops `cargo add gpui` today

`gpui_android` and `gpui_platform` aren't on crates.io, and `gpui` (which
is) doesn't yet ship the changes in this branch. So `cargo add gpui`
won't pull Android support until either:

1. **A point release of the `gpui` crate** that brings in `gpui_android` as
   an optional dependency under `cfg(target_os = "android")`, **or**
2. **A meta-crate** (e.g. `gpui-mobile` / `gpui-android`) that re-exports
   `gpui` plus the Android platform glue, published independently.

Until then, the git-dependency form in **§3.1** is the supported path.
We'll update this guide and announce on the repo as soon as either of the
above lands.
