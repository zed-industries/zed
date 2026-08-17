# CPAL - Cross-Platform Audio Library

[![Actions Status](https://github.com/RustAudio/cpal/workflows/cpal/badge.svg)](https://github.com/RustAudio/cpal/actions)
[![Crates.io](https://img.shields.io/crates/v/cpal.svg)](https://crates.io/crates/cpal) [![docs.rs](https://docs.rs/cpal/badge.svg)](https://docs.rs/cpal/)

Low-level library for audio input and output in pure Rust.

## Minimum Supported Rust Version (MSRV)

The minimum Rust version required depends on which audio backend and features you're using, as each platform has different dependencies:

- **AAudio (Android):** Rust **1.82** (due to `ndk` crate requirements)
- **ALSA (Linux/BSD):** Rust **1.77** (due to `alsa-sys` crate requirements)
- **CoreAudio (macOS/iOS):** Rust **1.80** (due to `coreaudio-rs` crate requirements)
- **JACK (Linux/BSD/macOS/Windows):** Rust **1.82** (due to `jack` crate requirements)
- **WASAPI/ASIO (Windows):** Rust **1.82** (due to `windows` crate requirements)
- **WASM (`wasm32-unknown`):** Rust **1.82** (due to `gloo` crate requirements)
- **WASM (`wasm32-wasip1`):** Rust **1.78** (target stabilized in 1.78)
- **WASM (`audioworklet`):** Rust **nightly** (requires `-Zbuild-std` for atomics support)

## Supported Platforms

This library currently supports the following:

- Enumerate supported audio hosts.
- Enumerate all available audio devices.
- Get the current default input and output devices.
- Enumerate known supported input and output stream formats for a device.
- Get the current default input and output stream formats for a device.
- Build and run input and output PCM streams on a chosen device with a given stream format.

Currently, supported hosts include:

- Linux (via ALSA or JACK)
- Windows (via WASAPI by default, ASIO or JACK optionally)
- macOS (via CoreAudio or JACK)
- iOS (via CoreAudio)
- Android (via AAudio)
- Emscripten
- WebAssembly (via Web Audio API or Audio Worklet)

Note that on Linux, the ALSA development files are required for building (even when using JACK). These are provided as part of the `libasound2-dev` package on Debian and Ubuntu distributions and `alsa-lib-devel` on Fedora.

## Compiling for WebAssembly

If you are interested in using CPAL with WebAssembly, please see [this guide](https://github.com/RustAudio/cpal/wiki/Setting-up-a-new-CPAL-WASM-project) in our Wiki which walks through setting up a new project from scratch. Some of the examples in this repository also provide working configurations that you can use as reference.

## Optional Features

CPAL provides the following optional features:

### `asio`

**Platform:** Windows

Enables the ASIO (Audio Stream Input/Output) backend. ASIO provides low-latency audio I/O by bypassing the Windows audio stack.

**Requirements:**
- ASIO drivers for your audio device
- LLVM/Clang for build-time bindings generation

**Setup:** See the [ASIO setup guide](#asio-on-windows) below for detailed installation instructions.

### `jack`

**Platform:** Linux, DragonFly BSD, FreeBSD, NetBSD, macOS, Windows

Enables the JACK (JACK Audio Connection Kit) backend. JACK is an audio server providing low-latency connections between applications and audio hardware.

**Requirements:**
- JACK server and client libraries must be installed on the system

**Usage:** See the [beep example](examples/beep.rs) for selecting the JACK host at runtime.

**Note:** JACK is available as an alternative backend on all supported platforms. It provides an option for pro-audio users who need JACK's routing and inter-application audio connectivity. The native backends (ALSA for Linux/BSD, WASAPI/ASIO for Windows, CoreAudio for macOS) remain the default and recommended choice for most applications.

### `wasm-bindgen`

**Platform:** WebAssembly (wasm32-unknown-unknown)

Enables the Web Audio API backend for browser-based audio. This is the base feature required for any WebAssembly audio support.

**Requirements:**
- Target `wasm32-unknown-unknown`
- Web browser with Web Audio API support

**Usage:** See the `wasm-beep` example for basic WebAssembly audio setup.

### `audioworklet`

**Platform:** WebAssembly (wasm32-unknown-unknown)

Enables the Audio Worklet backend for lower-latency web audio processing compared to the default Web Audio API backend.

**Requirements:**
- The `wasm-bindgen` feature (automatically enabled)
- Build with atomics support: `RUSTFLAGS="-C target-feature=+atomics,+bulk-memory,+mutable-globals"`
- Web server must send Cross-Origin headers for SharedArrayBuffer support

**Setup:** See the `audioworklet-beep` example README for complete setup instructions.

**Note:** Audio Worklet provides better performance than the default Web Audio API by running audio processing on a separate thread.

### `custom`

**Platform:** All platforms

Enables support for user-defined custom host implementations, allowing integration with audio systems not natively supported by CPAL.

**Usage:** See `examples/custom.rs` for implementation details.

## ASIO on Windows

### Locating the ASIO SDK

The location of ASIO SDK is exposed to CPAL by setting the `CPAL_ASIO_DIR` environment variable.

The build script will try to find the ASIO SDK by following these steps in order:

1. Check if `CPAL_ASIO_DIR` is set and if so use the path to point to the SDK.
2. Check if the ASIO SDK is already installed in the temporary directory, if so use that and set the path of `CPAL_ASIO_DIR` to the output of `std::env::temp_dir().join("asio_sdk")`.
3. If the ASIO SDK is not already installed, download it from <https://www.steinberg.net/asiosdk> and install it in the temporary directory. The path of `CPAL_ASIO_DIR` will be set to the output of `std::env::temp_dir().join("asio_sdk")`.

In an ideal situation you don't need to worry about this step.

### Preparing the Build Environment

1. **Install LLVM/Clang**: `bindgen`, the library used to generate bindings to the C++ SDK, requires clang. Download and install LLVM from <http://releases.llvm.org/download.html> under the "Pre-Built Binaries" section.

2. **Set LIBCLANG_PATH**: Add the LLVM `bin` directory to a `LIBCLANG_PATH` environment variable. If you installed LLVM to the default directory, this should work in the command prompt:
   ```
   setx LIBCLANG_PATH "C:\Program Files\LLVM\bin"
   ```

3. **Install ASIO Drivers** (optional for testing): If you don't have any ASIO devices or drivers available, you can download and install ASIO4ALL from <http://www.asio4all.org/>. Be sure to enable the "offline" feature during installation.

4. **Visual Studio**: The build script assumes Microsoft Visual Studio is installed. It will try to find `vcvarsall.bat` and execute it with the right host and target architecture. If needed, you can manually execute it:
   ```
   "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvarsall.bat" amd64
   ```
   For more information see the [vcvarsall.bat documentation](https://docs.microsoft.com/en-us/cpp/build/building-on-the-command-line).

### Using ASIO in Your Application

1. **Enable the feature** in your `Cargo.toml`:
   ```toml
   cpal = { version = "*", features = ["asio"] }
   ```

2. **Select the ASIO host** in your code:
   ```rust
   let host = cpal::host_from_id(cpal::HostId::Asio)
       .expect("failed to initialise ASIO host");
   ```

### Troubleshooting

If you encounter compilation errors from `asio-sys` or `bindgen`:
- Verify `CPAL_ASIO_DIR` is set correctly
- Try running `cargo clean`
- Ensure LLVM/Clang is properly installed and `LIBCLANG_PATH` is set

### Cross-Compilation

When Windows is the host and target OS, the build script supports all cross-compilation targets supported by the MSVC compiler.

It is also possible to compile Windows applications with ASIO support on Linux and macOS using the MinGW-w64 toolchain.

**Requirements:**
- Include the MinGW-w64 include directory in your `CPLUS_INCLUDE_PATH` environment variable
- Include the LLVM include directory in your `CPLUS_INCLUDE_PATH` environment variable

**Example for macOS** (targeting `x86_64-pc-windows-gnu` with `mingw-w64` installed via brew):
```
export CPLUS_INCLUDE_PATH="$CPLUS_INCLUDE_PATH:/opt/homebrew/Cellar/mingw-w64/11.0.1/toolchain-x86_64/x86_64-w64-mingw32/include"
```

## Troubleshooting

### No Default Device Available

If you receive errors about no default input or output device:

- **Linux/ALSA:** Ensure your user is in the `audio` group and that ALSA is properly configured
- **Linux/PulseAudio:** Check that PulseAudio is running: `pulseaudio --check`
- **Windows:** Verify your audio device is enabled in Sound Settings
- **macOS:** Check System Preferences > Sound for available devices
- **Mobile (iOS/Android):** Ensure your app has microphone/audio permissions

### Buffer Size Issues

If you experience audio glitches or dropouts:

- Try `BufferSize::Default` first before requesting specific sizes
- When using `BufferSize::Fixed`, query `SupportedBufferSize` to find valid ranges
- Smaller buffers reduce latency but increase CPU load and risk dropouts
- Ensure your audio callback completes quickly and avoids blocking operations

### Build Errors

- **ASIO on Windows:** Verify `LIBCLANG_PATH` is set and LLVM is installed
- **ALSA on Linux:** Install development packages: `libasound2-dev` (Debian/Ubuntu) or `alsa-lib-devel` (Fedora)
- **JACK:** Install JACK development libraries before enabling the `jack` feature

## Examples

CPAL comes with several examples demonstrating various features:

- `beep` - Generate a simple sine wave tone
- `enumerate` - List all available audio devices and their capabilities
- `feedback` - Pass input audio directly to output (microphone loopback)
- `record_wav` - Record audio from the default input device to a WAV file
- `synth_tones` - Generate multiple tones simultaneously

Run an example with:
```bash
cargo run --example beep
```

For platform-specific features, enable the relevant features:
```bash
cargo run --example beep --features asio  # Windows ASIO
cargo run --example beep --features jack  # JACK backend
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Resources

- **Documentation:** [docs.rs/cpal](https://docs.rs/cpal)
- **Examples:** [examples/](examples/) directory in this repository
- **Discord:** Join the [#cpal channel](https://discord.gg/vPmmSgJSPV) for questions and discussion
- **GitHub:** [Report issues](https://github.com/RustAudio/cpal/issues) and [view source code](https://github.com/RustAudio/cpal)
- **RustAudio:** Part of the [RustAudio organization](https://github.com/RustAudio)

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
