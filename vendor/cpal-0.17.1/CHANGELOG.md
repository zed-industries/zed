# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.17.1] - 2026-01-04

### Added

- **ALSA**: `Default` implementation for `Device` (returns the ALSA "default" device).
- **CI**: Checks default/no-default/all feature sets with platform-dependent MSRV for JACK.

### Fixed

- **ALSA**: Device enumeration now includes both hints and physical cards.
- **JACK**: No longer builds on iOS.
- **WASM**: WasmBindgen no longer crashes (regression from 0.17.0).

### Changed

- **ALSA**: Devices now report direction from hint metadata and physical hardware probing.

## [0.17.0] - 2025-12-20

### Added

- `DeviceTrait::id` method that returns a stable audio device ID.
- `HostTrait::device_by_id` to select a device by its stable ID.
- `Display` and `FromStr` implementations for `HostId`.
- Support for custom `Host`s, `Device`s, and `Stream`s.
- `Sample::bits_per_sample` method.
- `Copy` implementation to `InputCallbackInfo` and `OutputCallbackInfo`.
- `StreamError::StreamInvalidated` variant for when stream must be rebuilt.
- `StreamError::BufferUnderrun` variant for buffer underrun/overrun notifications.
- `Hash` implementation to `Device` for all backends.
- **AAudio**: `Send` and `Sync` implementations to `Stream`.
- **AAudio**: Support for 12 and 24 kHz sample rates.
- **ALSA**: `I24` and `U24` sample format support (24-bit samples stored in 4 bytes).
- **ALSA**: Support for 12, 24, 352.8, 384, 705.6, and 768 kHz sample rates.
- **ALSA**: `Eq` and `PartialEq` implementations to `Device`.
- **CI**: Native ARM64 Linux support in GitHub Actions.
- **CoreAudio**: `i8`, `i32` and `I24` sample format support (24-bit samples stored in 4 bytes).
- **CoreAudio**: Support for loopback recording (recording system audio output) on macOS > 14.6.
- **CoreAudio**: `Send` implementation to `Stream`.
- **Emscripten**: `BufferSize::Fixed` validation against supported range.
- **iOS**: Complete AVAudioSession integration for device enumeration and buffer size control.
- **JACK**: Support for macOS and Windows platforms.
- **JACK**: `BufferSize::Fixed` validation to reject requests that don't match server buffer size.
- **WASAPI**: Expose `IMMDevice` from WASAPI host Device.
- **WASAPI**: `I24` and `U24` sample format support (24-bit samples stored in 4 bytes).
- **WASAPI**: `Send` and `Sync` implementations to `Stream`.
- **WebAudio**: `Send` and `Sync` implementations to `Stream`.
- **WebAudio**: `BufferSize::Fixed` validation against supported range.

### Changed

- MSRV depends on the platform and at minimum 1.77.
- Set examples to Rust 2021.
- `SampleRate` from struct to `u32` type alias.
- Update `audio_thread_priority` to 0.34.
- Migrate CHANGELOG to [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format.
- **AAudio**: Configure buffer to ensure consistent callback buffer sizes.
- **AAudio**: Buffer size range detection to query the AudioService property correctly.
- **ALSA**: Improve `BufferSize::Fixed` precision and audio callback performance.
- **ALSA**: `BufferSize::Default` to use the device defaults.
- **ALSA**: Card enumeration to work like `aplay -L` does.
- **ALSA**: Update `alsa` to 0.10.
- **ALSA**: Pass `silent=true` to `PCM.try_recover`, so it doesn't write to stderr.
- **ALSA**: Report buffer underruns/overruns via `StreamError::BufferUnderrun`.
- **ASIO**: Share `sys::Asio` instance across all `Host` instances.
- **CI**: Fix cargo publish to trigger on GitHub releases instead of every master commit.
- **CI**: Replace cargo install commands with cached tool installation for faster builds.
- **CI**: Update actions to latest versions (checkout@v5, rust-cache@v2).
- **CI**: Verify compatibility with windows crates since v0.59.
- **CI**: Test platforms on appropriate MSRV per backend.
- **CI**: Fix `cargo update` syntax for compatibility with Cargo 1.70 (use `-p` flag instead of positional argument).
- **CoreAudio**: `Device::supported_configs` to return a single element containing the available sample rate range when all elements have the same `mMinimum` and `mMaximum` values.
- **CoreAudio**: Default audio device detection to be lazy when building a stream, instead of during device enumeration.
- **CoreAudio**: Configure device buffer to ensure predictable callback buffer sizes.
- **CoreAudio**: Remove `Clone` implementation from `Stream`.
- **JACK**: Use `StreamError::StreamInvalidated` for JACK server sample rate changes.
- **JACK**: Report buffer underruns/overruns via `StreamError::BufferUnderrun`.
- **WASAPI**: Update `windows` to >= 0.59, <= 0.62.

### Fixed

- **ALSA**: Format selection to probe hardware endianness instead of assuming native byte order.
- **ALSA**: Data race in stream shutdown.
- **ASIO**: Handling for `kAsioResetRequest` message to prevent driver UI becoming unresponsive.
- **ASIO**: Buffer silencing logic to work with non-conformant drivers (e.g., FL Studio ASIO).
- **CoreAudio**: Timestamp accuracy.
- **CoreAudio**: Segfaults when enumerating devices.
- **CoreAudio**: Undefined behavior related to null pointers and aligned reads.
- **CoreAudio**: Unnecessary microphone permission requests when using output devices only.
- **iOS**: Example by properly activating audio session.

### Removed

- **WebAudio**: Optional `wee-alloc` feature for security reasons.

## [0.16.0] - 2025-06-07

### Added

- Optional `supports_input`/`output` methods to `DeviceTrait`.
- 384000Hz to `COMMON_SAMPLE_RATES`.
- Constructors for `InputCallbackInfo`, `OutputCallbackInfo` and `StreamInstant`.
- `Default` impl for `Host`.
- `PartialOrd`, `Ord` and `Hash` implementations for `SampleFormat`.
- `Clone`, `PartialEq`, `Eq` and `Hash` implementations for all error enums.
- **ASIO**: Support for int24.

### Changed

- **AAudio**: Migrate from `oboe` to `ndk::audio`. **NOTE:** This raises the minimum Android API version to 26 (Android 8/Oreo).
- **AAudio**: Improve device names.
- **ALSA**: Set realtime priority for stream threads.
- **ALSA**: Improved card enumeration.
- **CoreAudio**: Update `coreaudio-rs` dependency to 0.13.
- **JACK**: Update `jack` dependency to 0.13.
- **WASAPI**: Set realtime priority for stream threads.

### Fixed

- **ALSA**: Don't panic when handling invalid stream timestamps.
- **ALSA**: Fix infinite loop on broken pipes.
- **ASIO**: Fix build failure on Windows.
- **CoreAudio**: Fix callback being called after dropping the stream.
- **CoreAudio**: Fix non-default audio output.
- **CoreAudio**: Fix handling of integer input formats.
- **WASAPI**: Fixed memory leak.
- **WASAPI**: Remove usage of `eval`.

## [0.15.3] - 2024-03-04

### Added

- `try_with_sample_rate`, a non-panicking variant of `with_sample_rate`.
- `#[must_use]` attribute to struct `platform::Stream`.
- `Copy` implementation to enum `SupportedBufferSize` and struct `SupportedStreamConfigRange`.
- `Clone` implementation to `platform::Device`.

### Changed

- **AAudio**: Update `jni` dependency to 0.21.
- **AAudio**: Update `oboe` dependency to 0.6.
- **AAudio**: Update `ndk` dependency to 0.8 and disable `default-features`.
- **ALSA**: Update `alsa` dependency to 0.9.
- **CI**: Update actions, use Android 30 API level in CI, remove `asmjs-unknown-emscripten` target.
- **Examples**: Migrate wasm example to `trunk`, improve syth-thones example.
- **WASAPI**: Update `windows` dependency to v0.54.
- **WebAudio**: Update `wasm-bindgen` to 0.2.89.

### Fixed

- **WebAudio**: Crash on web/wasm when `atomics` flag is enabled.

### Removed

- `parking_lot` dependency in favor of the std library.

## [0.15.2] - 2023-03-30

### Added

- **WebAudio**: Support for multichannel output streams.

### Changed

- **WASAPI**: Update `windows` dependency.

### Fixed

- **WASAPI**: Fix some thread panics.

## [0.15.1] - 2023-03-14

### Added

- **AAudio**: Feature `oboe-shared-stdcxx` to enable `shared-stdcxx` on `oboe` for Android support.

### Changed

- **CoreAudio**: Switch `mach` dependency to `mach2`.

### Removed

- `thiserror` dependency.

## [0.15.0] - 2023-01-29

### Added

- **CoreAudio**: Disconnection detection on Mac OS.

### Changed

- Switch to the `dasp_sample` crate for the sample trait.
- Adopt edition 2021.
- **AAudio**: Update `oboe` dependency.
- **AAudio**: Update `alsa` dependency.
- **CoreAudio**: Update `coreaudio-sys` dependency.
- **Emscripten**: Switch to `web-sys` on the emscripten target.
- **JACK**: Update `jack` dependency.
- **WASAPI**: Update `windows-rs` dependency.

## [0.14.2] - 2022-12-02

### Removed

- `nix` dependency.

## [0.14.1] - 2022-10-23

### Added

- **ALSA**: Support for the 0.6.1 release of `alsa-rs`.
- **NetBSD**: Platform support.

### Changed

- **CI**: Various improvements.

### Fixed

- **ASIO**: Feature broken in 0.14.0.

## [0.14.0] - 2022-08-22

### Changed

- Update `parking_lot` and `once_cell` dependencies.
- **AAudio**: Turn `ndk-glue` into a dev-dependency and use `ndk-context` instead.
- **AAudio**: Update `ndk` and `ndk-glue` dependencies.
- **JACK**: Update `jack` dependency.
- **WASAPI**: Switch to `windows-rs` crate.

## [0.13.5] - 2022-01-28

### Changed

- Faster sample format conversion.
- **AAudio**: Update `ndk`, `oboe`, and `ndk-glue` dependencies.
- **ALSA**: Update `alsa` and `nix` dependencies.
- **JACK**: Update `jack` dependency.

## [0.13.4] - 2021-08-08

### Changed

- **AAudio**: Update `jni` dependency.
- **ALSA**: Improve stream setup parameters.
- **CoreAudio**: Update `core-foundation-sys` dependency.
- **JACK**: Update `rust-jack` dependency.
- **WASAPI**: Allow both threading models and switch the default to STA.

## [0.13.3] - 2021-03-29

### Added

- Give each thread a unique name.

### Fixed

- **ALSA**: Fix distortion regression on some configs.

## [0.13.2] - 2021-03-16

### Changed

- **AAudio**: Update `ndk`, `nix`, `oboe`, and `jni` dependencies.

## [0.13.1] - 2020-11-08

### Changed

- Update `parking_lot` dependency.

### Fixed

- **WASAPI**: Don't panic when device is plugged out on Windows.

## [0.13.0] - 2020-10-28

### Added

- **AAudio**: Android support via `oboe-rs`.
- **CI**: Android APK build and CI job.

## [0.12.1] - 2020-07-23

### Fixed

- **ASIO**: Bugfix release to get the asio feature working again.

## [0.12.0] - 2020-07-09

### Added

- `build_input/output_stream_raw` methods allowing for dynamically handling sample format type.
- `InputCallbackInfo` and `OutputCallbackInfo` types and update expected user data callback function signature to provide these.
- **DragonFly BSD**: Platform support.

### Changed

- Large refactor removing the blocking EventLoop API.
- Rename many `Format` types to `StreamConfig`:
  - `Format` type's `data_type` field renamed to `sample_format`.
  - `Shape` -> `StreamConfig` - The configuration input required to build a stream.
  - `Format` -> `SupportedStreamConfig` - Describes a single supported stream configuration.
  - `SupportedFormat` -> `SupportedStreamConfigRange` - Describes a range of supported configurations.
  - `Device::default_input/output_format` -> `Device::default_input/output_config`.
  - `Device::supported_input/output_formats` -> `Device::supported_input/output_configs`.
  - `Device::SupportedInput/OutputFormats` -> `Device::SupportedInput/OutputConfigs`.
  - `SupportedFormatsError` -> `SupportedStreamConfigsError`.
  - `DefaultFormatError` -> `DefaultStreamConfigError`.
  - `BuildStreamError::FormatNotSupported` -> `BuildStreamError::StreamConfigNotSupported`.
- **WASAPI**: Address deprecated use of `mem::uninitialized`.

### Removed

- `UnknownTypeBuffer` in favour of specifying sample type.

## [0.11.0] - 2019-12-11

### Added

- Name to `HostId`.
- **WASAPI**: `winbase` winapi feature to solve windows compile error issues.

### Changed

- Remove many uses of `std::mem::uninitialized`.
- Panic on stream ID overflow rather than returning an error.
- Move errors into a separate module.
- Switch from `failure` to `thiserror` for error handling.
- **ALSA**: Use `snd_pcm_hw_params_set_buffer_time_near` rather than `set_buffer_time_max`.
- **CI**: Lots of improvements.
- **Examples**: Use `ringbuffer` crate in feedback example.

### Fixed

- **ALSA**: Fix some underruns that could occur.
- **WASAPI**: Fix capture logic.

## [0.10.0] - 2019-07-05

### Added

- New Host API, adding support for alternative audio APIs.
- `StreamEvent` type to allow users to handle stream callback errors.
- **ASIO**: ASIO host, available under Windows.

### Changed

- Remove sleep loop on macOS in favour of using a `Condvar`.
- Overhaul error handling throughout the crate.
- Remove `panic!` from OutputBuffer Deref impl as it is no longer necessary.
- **ALSA**: Remove unnecessary Mutex in favour of channels.
- **CoreAudio**: Update `core-foundation-sys` and `coreaudio-rs` dependencies.
- **WASAPI**: Remove unnecessary Mutex in favour of channels.

## [0.9.0] - 2019-06-06

### Added

- **ALSA**: Error handling for unknown device errors.
- **Emscripten**: `default_output_format` implementation.

### Changed

- Better buffer handling.

### Fixed

- Logic error in frame/sample size.
- **WASAPI**: Fix resuming a paused stream.

## [0.8.2] - 2018-07-03

### Added

- `Display` and `Error` implementations for `DefaultFormatError`.

### Changed

- Upgrade `lazy_static` dependency.

## [0.8.1] - 2018-04-01

### Fixed

- **CoreAudio**: Handling of non-default sample rates for input streams.

## [0.8.0] - 2018-02-15

### Added

- `Device::supported_{input/output}_formats` methods.
- `Device::default_{input/output}_format` methods.
- `default_{input/output}_device` functions.
- `StreamData` type for handling either input or output streams in `EventLoop::run` callback.
- **ALSA**: Input stream support.
- **CoreAudio**: Input stream support.
- **Examples**: `record_wav.rs` example that records 3 seconds to `$CARGO_MANIFEST_DIR/recorded.wav` using default input device.
- **WASAPI**: Input stream support.

### Changed

- Replace usage of `Voice` with `Stream` throughout the crate.
- **Examples**: Update `enumerate.rs` example to display default input/output devices and formats.

### Removed

- `Endpoint` in favour of `Device` for supporting both input and output streams.

## [0.7.0] - 2018-02-04

### Added

- **CoreAudio**: `Endpoint` and `Format` enumeration for macOS.
- **CoreAudio**: Format handling for `build_voice` method.

### Changed

- Rename `ChannelsCount` to `ChannelCount`.
- Rename `SamplesRate` to `SampleRate`.
- Rename the `min_samples_rate` field of `SupportedFormat` to `min_sample_rate`.
- Rename the `with_max_samples_rate()` method of `SupportedFormat` to `with_max_sample_rate()`.
- Rename the `samples_rate` field of `Format` to `sample_rate`.
- Changed the type of the `channels` field of the `SupportedFormat` struct from `Vec<ChannelPosition>` to `ChannelCount` (an alias to `u16`).

### Removed

- Unused ChannelPosition API.

## [0.6.0] - 2017-12-11

### Added

- Improvements to the crate documentation.
- **ALSA**: `pause` and `play` support.

### Changed

- **CoreAudio**: Reduced the number of allocations.
- **Emscripten**: Backend to consume less CPU.

### Fixed

- **CoreAudio**: Fixes for macOS build (#186, #189).

## [0.5.1] - 2017-10-21

### Added

- `Sample::to_i16()`, `Sample::to_u16()` and `Sample::from` methods.

## [0.5.0] - 2017-10-21

### Added

- `EventLoop::build_voice`, `EventLoop::destroy_voice`, `EventLoop::play`, and `EventLoop::pause` methods.
- `VoiceId` struct that is now used to identify a voice owned by an `EventLoop`.

### Changed

- `EventLoop::run()` to take a callback that is called whenever a voice requires sound data.
- `supported_formats()` to produce a list of `SupportedFormat` instead of `Format`. A `SupportedFormat` must then be turned into a `Format` in order to build a voice.

### Removed

- Dependency on the `futures` library.
- `Voice` and `SamplesStream` types.

## [0.4.6] - 2017-10-11

### Added

- **iOS**: Minimal support.

### Changed

- Run rustfmt on the code.

### Fixed

- **BSD**: Fixes for *BSDs.

### Removed

- `get_` prefix of methods.

## [0.4.5] - 2017-04-29

### Changed

- Simplify the Cargo.toml.
- **ALSA**: Bump alsa-sys version number.
- **ALSA**: Mark alsa-sys as linking to alsa.

### Fixed

- **CoreAudio**: SampleStream also holds on to the AudioUnit so it is not dropped.
- **CoreAudio**: Fix for loop in EventLoop::run being optimised out in a release build on macOS.

### Removed

- Stop publishing on gh-pages.

## [0.4.4] - 2017-02-04

### Fixed

- **ALSA**: Pass period instead of buffer to snd_pcm_sw_params_set_avail_min.

## [0.4.3] - 2017-02-01

### Fixed

- **ALSA**: Set sw_params_set_avail_min based on get_params buffer size.

## [0.4.2] - 2017-01-19

### Added

- **CoreAudio**: coreaudio-rs dependency for i686-apple-darwin.

### Deprecated

- Mark deprecated functions as deprecated.

## [0.4.1] - 2016-11-16

### Added

- **ALSA**: Implement play and pause.
- **CoreAudio**: Implement play/pause.

### Fixed

- **WASAPI**: Fix compilation on windows.

## [0.4.0] - 2016-10-01

### Changed

- **ALSA**: Update to futures 0.1.1.
- **WASAPI**: Update to futures 0.1.1.

### Fixed

- **CoreAudio**: Do not lock inner twice. Fixes bug in osx futures 0.1.1 update.
- **CoreAudio**: Try fix the OSX code with futures.

## [0.3.1] - 2016-08-20

### Added

- **WASAPI**: Some documentation to the winapi implementation.

### Changed

- **ALSA**: Bump alsa-sys to 0.1.

### Fixed

- Fix #126.
- Fix most warnings.

## [0.3.0] - 2016-08-12

### Added

- **CoreAudio**: Update backend to new futures-rs oriented design.

### Changed

- Update documentation.
- Use a max buffer size in order to avoid problems.
- Update deps.
- **ALSA**: Make it work on Linux.
- **Null**: Update the null implementation.

## [0.2.12] - 2016-07-10

### Added

- **CoreAudio**: Add get_period to Voice.

### Changed

- **CoreAudio**: Update to coreaudio-rs 0.5.0.

### Fixed

- **CoreAudio**: Correct implementation of get_pending_samples.
- **CoreAudio**: Return correct Voice period.

## [0.2.11] - 2016-04-25

### Fixed

- Be more relaxed with c_void.

## [0.2.10] - 2016-04-22

### Added

- **ALSA**: Add pollfd.

### Fixed

- **ALSA**: Fix underflow detection.
- **Android**: Fix the android build.
- **Android**: Add ARM target.

## [0.2.9] - 2016-01-28

### Added

- **CoreAudio**: Add support for U16/I16 PCM formats.
- **CoreAudio**: Implement some missing functions.

### Changed

- Handle channels positionning.
- Update Cargo.toml after the previous changes.
- Allow for building for mipsel targets.
- **ALSA**: Use correct ALSA channels.
- **CoreAudio**: Implementation cleanup.
- **CoreAudio**: Make Voice Send/Sync.
- **CoreAudio**: Set sample rate to 44100.
- **Examples**: Update the beep example.

### Fixed

- **ALSA**: Fix underflow bug on linux.
- **CI**: Fix for travis build.
- **CoreAudio**: Fix compilation on OSX with the new API for coreaudio-rs.
- **CoreAudio**: Return correct length of buffer, stub unimpl funcs.
- **CoreAudio**: Restore CoreAudio support after API overhaul.
- **Examples**: Add some sane error messages.
- **Examples**: Improve error reporting in beep example.

### Removed

- Do not use a wildcard version number.
- **CoreAudio**: Revert "Add support for U16/I16 PCM formats" (was causing issues).

## [0.2.8] - 2015-11-10

### Changed

- Libc 0.2.
- **WASAPI**: Update winapi.

### Fixed

- **WASAPI**: Catch another 'device not found' error code.

## [0.2.7] - 2015-09-27

### Added

- `Voice::get_period()` method.

## [0.2.6] - 2015-09-22

### Fixed

- **ALSA**: Make sure that all writes succeed.
- **ALSA**: Make the implementation more robust by recovering from underruns.

## [0.2.5] - 2015-09-22

### Added

- `Voice::get_pending_samples` method.

## [0.2.4] - 2015-09-22

### Added

- `endpoint::get_name()` method.
- **Examples**: An enumerate example.
- **WASAPI**: Device name support.

### Changed

- **ALSA**: Correctly enumerate supported formats.

### Fixed

- **ALSA**: Various fixes.
- **ALSA**: Use the correct format.
- **ALSA**: Use the correct device name when enumerating formats.
- **WASAPI**: Fix bug and filter out devices that are not "Output".

## [0.2.3] - 2015-09-22

### Added

- `#[inline]` attributes.
- `underflow()` method to Voice.

### Changed

- Store the format in the public `Voice` struct.
- **WASAPI**: General cleanup.
- **WASAPI**: Update winapi dependency.

### Fixed

- **WASAPI**: Fix the hack in the implementation.

### Removed

- Unused extern crate libc.

## [0.2.2] - 2015-09-11

### Added

- `UnknownBufferType::len()` method.

### Fixed

- **Null**: Restore the null implementation and compile it every time.

## [0.2.1] - 2015-09-10

### Changed

- Handle channels positionning.
- Update Cargo.toml after the previous changes.
- **Examples**: Update the beep example.

### Fixed

- **ALSA**: Fix compilation.

## [0.2.0] - 2015-09-01

### Added

- Proper error handling.
- Supported formats enumeration.
- `endpoint::get_name()` method.

### Changed

- **ALSA**: Make it compile again.
- **WASAPI**: Enable 32bits samples.
- **WASAPI**: Better error handling in format detection.
- **WASAPI**: Now decoding the format from the WAVEFORMAT returned by the winapi.
- **WASAPI**: Handle F32 formats in Voice::new.
- **WASAPI**: Use the format passed as parameter in Voice::new.
- **WASAPI**: Correctly enumerate audio devices (core + wasapi).

### Fixed

- Fix doctests.
- Add more detailed message to panic.

### Removed

- Conversion system.
- Use of box syntax.

## [0.1.2] - 2015-07-22

### Fixed

- **ALSA**: Correct reported sample format.
- **WASAPI**: Fix samples signs on win32.

## [0.1.1] - 2015-07-20

### Fixed

- Fix the version in the README.
- **WASAPI**: Fix the win32 build.

## [0.1.0] - 2015-07-11

### Added

- Bump to 0.1.0.

## [0.0.23] - 2015-07-04

### Fixed

- **WASAPI**: Fix platform-specific dependencies with MSVC.

## [0.0.22] - 2015-06-24

### Fixed

- **ALSA**: Calls to a single ALSA channel are not thread safe.

## [0.0.21] - 2015-06-05

### Changed

- **Examples**: Simplify beep example.
- **WASAPI**: Use shiny new COM.

## [0.0.20] - 2015-04-20

### Changed

- Rustup and version bumps.
- **ALSA**: Remove integer suffixes in alsa-sys.

## [0.0.19] - 2015-04-04

### Changed

- Update for Rustc 1.0.0 beta.

## [0.0.18] - 2015-03-30

### Changed

- Update for change in rustc and winapi.

## [0.0.17] - 2015-03-26

### Changed

- Rustup.
- **ALSA**: Publish alsa-sys before cpal.

## [0.0.16] - 2015-03-25

### Added

- **CoreAudio**: OSX support via the Apple Core Audio, Audio Unit C API. Only supports f32 so far.
- **CoreAudio**: Coreaudio bindings.

### Changed

- Rustup.

### Fixed

- **CI**: Fix travis build.
- **CoreAudio**: Fixed callback to send proper buffersize, removed code in lib where sampleformat affected buffersize.
- **CoreAudio**: Properly shutdown the AudioUnit on drop.

### Removed

- **CoreAudio**: Removed core_audio-sys local bindings in favour of new coreaudio-rs crate.

## [0.0.15] - 2015-02-22

### Changed

- Bump version.
- Update for rustc.

## [0.0.14] - 2015-02-19

### Changed

- Update for rustc.
- **ALSA**: Bump alsa-sys version.
- **ALSA**: Clean up alsa-sys.
- **CI**: Automatically publish on crates.io on successful builds.
- **CI**: Publish alsa-sys too.

## [0.0.13] - 2015-02-12

### Changed

- Update with libc version.

## [0.0.12] - 2015-01-29

### Changed

- Bump version number.

## [0.0.11] - 2015-01-29

### Changed

- Update for rustc.

## [0.0.10] - 2015-01-20

### Added

- **Null**: "null" implementation for platforms that aren't supported.

### Changed

- Changed integer suffix from 'u' to 'us'.
- Bump version number (multiple times).
- Update for rust-1.0 alpha.
- **WASAPI**: Update for winapi.

## [0.0.8] - 2015-01-08

### Changed

- Bump version number.
- Update for Rustc.

## [0.0.7] - 2015-01-05

### Changed

- Update for rustc.

## [0.0.6] - 2014-12-30

### Added

- `#[must_use]` marker for Buffer.

### Changed

- Bump version number.
- Update for changes in rustc.

## [0.0.5] - 2014-12-23

### Added

- `play()` and `pause()` functions.
- Implement f32 to i16 and f32 to u16 conversions.
- Tests for convert_samples_rate.
- Tests for convert_channels.

### Changed

- Cleanup convert_samples_rate.
- Cleanup convert_channels.

### Fixed

- **CI**: Fix the appveyor build.

## [0.0.4] - 2014-12-20

### Changed

- Update for rustc.

## [0.0.3] - 2014-12-17

### Added

- Link to documentation.
- Some documentation.
- Fixes and tests for samples conversions.
- All samples formats.
- Samples formats conversions.
- **CI**: Automatic gh-pages deployment in travis.

### Changed

- Bump version number.
- Improve documentation.
- Use Cow for formats conversions to avoid an allocation and copy.

### Fixed

- Minor README update.
- **CI**: Remove old section from travis.yml.
- **CI**: Fix travis.yml.

### Removed

- Rename `Channel` to `Voice`.

## [0.0.2] - 2014-12-17

### Added

- Basic API.
- Some documentation.
- Keywords.
- Some formats-related functions.
- Some basic data conversion.
- Some samples rate conversions.
- Variable input format system.
- Samples iterator.
- **ALSA**: Basic implementation.
- **ALSA**: alsa-sys library.
- **CI**: Appveyor file.
- **CI**: Config for rust-ci in travis.
- **Examples**: Draft for example music playing.
- **Examples**: Semi-working WASAPI example.
- **WASAPI**: Destructor for wasapi::Channel.

### Changed

- Bump version number.
- Buffer now always has the u8 format.
- Modify API to use a "samples" iterator.
- Change player architecture to avoid data losses.
- Minor nitpicking.
- Update for rustc.
- **ALSA**: Use the official winapi crate.
- **ALSA**: More tweaks for alsa-sys.
- **ALSA**: Minor tweaks in Cargo.toml files.
- **Examples**: Replace example by a smaller one.
- **Examples**: Replace example by a sinusoid generator.
- **Examples**: Rename example to "beep".

### Fixed

- Fix warnings.
- Fix PCM formats conversions not working.
- Fix issue when calling `buffer.samples()` multiple times with the same buffer.
- Sound output now works correctly.
- Minor fixes.
- **WASAPI**: Revert "Switch to retep998/winapi".

### Removed

- Switch back to using buffers.
- Remove old code.

## [0.0.1] - 2014-12-11

### Added

- Initial commit.

[0.17.1]: https://github.com/RustAudio/cpal/compare/v0.17.0...v0.17.1
[0.17.0]: https://github.com/RustAudio/cpal/compare/v0.16.0...v0.17.0
[0.16.0]: https://github.com/RustAudio/cpal/compare/v0.15.3...v0.16.0
[0.15.3]: https://github.com/RustAudio/cpal/compare/v0.15.2...v0.15.3
[0.15.2]: https://github.com/RustAudio/cpal/compare/v0.15.1...v0.15.2
[0.15.1]: https://github.com/RustAudio/cpal/compare/v0.15.0...v0.15.1
[0.15.0]: https://github.com/RustAudio/cpal/compare/v0.14.2...v0.15.0
[0.14.2]: https://github.com/RustAudio/cpal/compare/v0.14.1...v0.14.2
[0.14.1]: https://github.com/RustAudio/cpal/compare/v0.14.0...v0.14.1
[0.14.0]: https://github.com/RustAudio/cpal/compare/v0.13.5...v0.14.0
[0.13.5]: https://github.com/RustAudio/cpal/compare/v0.13.4...v0.13.5
[0.13.4]: https://github.com/RustAudio/cpal/compare/v0.13.3...v0.13.4
[0.13.3]: https://github.com/RustAudio/cpal/compare/v0.13.2...v0.13.3
[0.13.2]: https://github.com/RustAudio/cpal/compare/v0.13.1...v0.13.2
[0.13.1]: https://github.com/RustAudio/cpal/compare/v0.13.0...v0.13.1
[0.13.0]: https://github.com/RustAudio/cpal/compare/v0.12.1...v0.13.0
[0.12.1]: https://github.com/RustAudio/cpal/compare/v0.12.0...v0.12.1
[0.12.0]: https://github.com/RustAudio/cpal/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/RustAudio/cpal/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/RustAudio/cpal/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/RustAudio/cpal/compare/v0.8.2...v0.9.0
[0.8.2]: https://github.com/RustAudio/cpal/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/RustAudio/cpal/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/RustAudio/cpal/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/RustAudio/cpal/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/RustAudio/cpal/compare/v0.5.1...v0.6.0
[0.5.1]: https://github.com/RustAudio/cpal/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/RustAudio/cpal/compare/v0.4.6...v0.5.0
[0.4.6]: https://github.com/RustAudio/cpal/compare/v0.4.5...v0.4.6
[0.4.5]: https://github.com/RustAudio/cpal/compare/v0.4.4...v0.4.5
[0.4.4]: https://github.com/RustAudio/cpal/compare/v0.4.3...v0.4.4
[0.4.3]: https://github.com/RustAudio/cpal/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/RustAudio/cpal/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/RustAudio/cpal/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/RustAudio/cpal/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/RustAudio/cpal/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/RustAudio/cpal/compare/v0.2.12...v0.3.0
[0.2.12]: https://github.com/RustAudio/cpal/compare/v0.2.11...v0.2.12
[0.2.11]: https://github.com/RustAudio/cpal/compare/v0.2.10...v0.2.11
[0.2.10]: https://github.com/RustAudio/cpal/compare/v0.2.9...v0.2.10
[0.2.9]: https://github.com/RustAudio/cpal/compare/v0.2.8...v0.2.9
[0.2.8]: https://github.com/RustAudio/cpal/compare/v0.2.7...v0.2.8
[0.2.7]: https://github.com/RustAudio/cpal/compare/v0.2.6...v0.2.7
[0.2.6]: https://github.com/RustAudio/cpal/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/RustAudio/cpal/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/RustAudio/cpal/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/RustAudio/cpal/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/RustAudio/cpal/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/RustAudio/cpal/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/RustAudio/cpal/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/RustAudio/cpal/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/RustAudio/cpal/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/RustAudio/cpal/compare/v0.0.23...v0.1.0
[0.0.23]: https://github.com/RustAudio/cpal/compare/v0.0.22...v0.0.23
[0.0.22]: https://github.com/RustAudio/cpal/compare/v0.0.21...v0.0.22
[0.0.21]: https://github.com/RustAudio/cpal/compare/v0.0.20...v0.0.21
[0.0.20]: https://github.com/RustAudio/cpal/compare/v0.0.19...v0.0.20
[0.0.19]: https://github.com/RustAudio/cpal/compare/v0.0.18...v0.0.19
[0.0.18]: https://github.com/RustAudio/cpal/compare/v0.0.17...v0.0.18
[0.0.17]: https://github.com/RustAudio/cpal/compare/v0.0.16...v0.0.17
[0.0.16]: https://github.com/RustAudio/cpal/compare/v0.0.15...v0.0.16
[0.0.15]: https://github.com/RustAudio/cpal/compare/v0.0.14...v0.0.15
[0.0.14]: https://github.com/RustAudio/cpal/compare/v0.0.13...v0.0.14
[0.0.13]: https://github.com/RustAudio/cpal/compare/v0.0.12...v0.0.13
[0.0.12]: https://github.com/RustAudio/cpal/compare/v0.0.11...v0.0.12
[0.0.11]: https://github.com/RustAudio/cpal/compare/v0.0.10...v0.0.11
[0.0.10]: https://github.com/RustAudio/cpal/compare/v0.0.8...v0.0.10
[0.0.8]: https://github.com/RustAudio/cpal/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/RustAudio/cpal/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/RustAudio/cpal/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/RustAudio/cpal/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/RustAudio/cpal/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/RustAudio/cpal/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/RustAudio/cpal/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/RustAudio/cpal/releases/tag/v0.0.1
