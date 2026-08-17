# Unreleased

# 0.9.0 (2024-04-26)

- Move `MediaFormat` from `media::media_codec` to its own `media::media_format` module. (#442)
- media_format: Expose `MediaFormat::copy()` and `MediaFormat::clear()` from API level 29. (#449)
- **Breaking:** media_format: Mark all `fn set_*()` and `fn str()` as taking `self` by `&mut`. (#452)
- **Breaking:** Require all `dyn Fn*` types to implement `Send` when the FFI implementation invokes them on a separate thread: (#455)
  - `audio::AudioStreamDataCallback`;
  - `audio::AudioStreamErrorCallback`;
  - `media::image_reader::BufferRemovedListener`;
  - `media::image_reader::ImageListener`;
  - `media::media_codec::ErrorCallback`;
  - `media::media_codec::FormatChangedCallback`;
  - `media::media_codec::InputAvailableCallback`;
  - `media::media_codec::OutputAvailableCallback`.
- Drop previous `Box`ed callbacks _after_ registering new ones, instead of before. (#455)
- input_queue: Add `from_java()` constructor, available since API level 33. (#456)
- event: Add `from_java()` constructors to `KeyEvent` and `MotionEvent`, available since API level 31. (#456)
- **Breaking:** image_reader: Special-case return statuses in `Image`-acquire functions. (#457)
- **Breaking:** image_reader: Mark `ImageReader::acquire_latest_image_async()` `unsafe` to match the safety requirements on `ImageReader::acquire_next_image_async()`. (#457)
- event: Implement `SourceClass` `bitflag` and provide `Source::class()` getter. (#458)
- Ensure all `bitflags` implementations consider all (including unknown) bits in negation and `all()`. (#458)
- **Breaking:** Mark all enums as `non_exhaustive` and fix `repr` types. (#459)
- **Breaking:** native_window: Remove redundant `TRANSFORM_` prefix from `NativeWindowTransform` variants. (#460)
- **Breaking:** hardware_buffer: Convert `HardwareBufferUsage` to `bitflags`. (#461)
- bitmap: Guard `BitmapCompressError` behind missing `api-level-30` feature. (#462)
- native_window: Require linking against `libnativewindow` for most API >= 26 functions. (#465)
- **Breaking:** audio: Merge `AudioResult` variant enum into `AudioError`. (#467)
- data_space: Add missing `DataSpaceRange::Unspecified` variant. (#468)
- **Breaking:** looper: Require `Send` marker when adding fd event callbacks on `ForeignLooper`. (#469)
- **Breaking:** Upgrade to [`ndk-sys 0.6.0`](../ndk-sys/CHANGELOG.md#060-2024-04-26). (#472)

# 0.8.0 (2023-10-15)

- event: Add `tool_type` getter for `Pointer`. (#323)
- input_queue: Allow any non-zero return code from `pre_dispatch()` again, as per documentation. (#325)
- asset: Use entire asset length when mapping buffer. (#387)
- Bump MSRV to 1.66 for `raw-window-handle 0.5.1`, `num_enum`'s `catch_all` with arbitrary enum discriminants. (#388, #431)
- Bump optional `jni` dependency for doctest example from `0.19` to `0.21`. (#390)
- **Breaking:** Upgrade to [`ndk-sys 0.5.0`](../ndk-sys/CHANGELOG.md#050-2023-10-15). (#370)
- **Breaking:** Upgrade `bitflags` crate from `1` to `2`. (#394)
- bitmap: Add `try_format()` to `AndroidBitmapInfo` to handle unexpected formats without panicking. (#395)
- Add `Font` bindings. (#397)
- **Breaking:** Upgrade `num_enum` crate from `0.5.1` to `0.7`. (#398, #419)
- **Breaking:** Renamed, moved and flattened "`media`" error types and helpers to a new `media_error` module. (#399, #432)
- **Breaking:** media_codec: Wrap common dequeued-buffer status codes in enum. (#401)
- **Breaking:** media_codec: Return `MaybeUninit` bytes in `buffer_mut()`. (#403)
- native_window: Add `lock()` to blit raw pixel data. (#404)
- hardware_buffer_format: Add `YCbCr_P010` and `R8_UNORM` variants. (#405)
- **Breaking:** hardware_buffer_format: Add catch-all variant. (#407)
- asset: Add missing `is_allocated()` and `open_file_descriptor()` methods. (#409)
- **Breaking:** media_codec: Add support for asynchronous notification callbacks. (#410)
- Add panic guards to callbacks. (#412)
- looper: Add `remove_fd()` to unregister events/callbacks for a file descriptor. (#416)
- **Breaking:** Use `BorrowedFd` and `OwnedFd` to clarify possible ownership transitions. (#417)
- **Breaking:** Upgrade to [`ndk-sys 0.5.0`](../ndk-sys/CHANGELOG.md#050-2023-10-15). (#420)
- Add bindings for `sync.h`. (#423)
- **Breaking:** bitmap: Provide detailed implementation for `AndroidBitmapInfoFlags`. (#424)
- native_window: Add `set_buffers_transform()`, `try_allocate_buffers()` and `set_frame_rate*()`. (#425)
- Add bindings for `ASharedMemory`. (#427)
- hardware_buffer: Add `id()` to retrieve a system-wide unique identifier for a `HardwareBuffer`. (#428)
- **Breaking:** bitmap: Strip `Android` prefix from structs and enums, and `Bitmap` from `Result`. (#430)
- **Breaking:** `raw-window-handle 0.5` support is now behind an _optional_ `rwh_05` crate feature and `raw-window-handle` `0.4` and `0.6` support is provided via the new `rwh_04` and (default-enabled) `rwh_06` crate features. (#434)
- **Breaking:** looper: Provide `event` value to file descriptor poll callback. (#435)
- **Breaking:** `HardwareBufferFormat` is no longer exported from `hardware_buffer` and `native_window`, and can only be reached through the `hardware_buffer_format` module. (#436)
- **Breaking:** `get_` prefixes have been removed from all public functions in light of the [C-GETTER](https://rust-lang.github.io/api-guidelines/naming.html#getter-names-follow-rust-convention-c-getter) convention. (#437)
- Add `DataSpace` type and relevant functions on `Bitmap` and `NativeWindow`. (#438)
- bitmap: Add `Bitmap::compress()` and `Bitmap::compress_raw()` functions. (#440)
- **Breaking:** Turn `BitmapError` into a `non_exhaustive` `enum`. (#440)
- **Breaking:** audio: Rename `AudioErrorResult` to `AudioResult` and turn into a `non_exhaustive` `enum`. (#441)

# 0.7.0 (2022-07-24)

- hardware_buffer: Make `HardwareBuffer::as_ptr()` public for interop with Vulkan. (#213)
- **Breaking:** `Configuration::country()` now returns `None` when the country is unset (akin to `Configuration::language()`). (#220)
- Add `MediaCodec` and `MediaFormat` bindings. (#216)
- **Breaking:** Upgrade to [`ndk-sys 0.4.0`](../ndk-sys/CHANGELOG.md#040-2022-07-24) and use new `enum` newtype wrappers. (#245)
- native_window: Use `release`/`acquire` for `Drop` and `Clone` respectively. (#207)
- **Breaking:** audio: Rename from `aaudio` to `audio` and drop `A` prefix. (#273)
- Implement `HasRawWindowHandle` directly on `NativeWindow`. (#274, #319)
- **Breaking:** native_activity: Replace `CStr` return types with `Path`. (#279)
- native_window: Add `format()` getter and `set_buffers_geometry()` setter. (#276)
- native_activity: Add `set_window_format()` setter. (#277)
- native_activity: Add `set_window_flags()` to change window behavior. (#278)
- Add `SurfaceTexture` bindings. (#267)
- Improve library and structure documentation, linking back to the NDK docs more rigorously. (#290)
- **Breaking:** input_queue: `get_event()` now returns a `Result` with `std::io::Error`; `InputQueueError` has been removed. (#292)
- **Breaking:** input_queue: `has_events()` now returns a `bool` directly without being wrapped in `Result`. (#294)
- **Breaking:** hardware_buffer: `HardwareBufferError` has been removed and replaced with `std::io::Error` in return types. (#295)
- Fixed `HardwareBuffer` leak on buffers returned from `AndroidBitmap::get_hardware_buffer()`. (#296)
- Bump optional `jni` dependency for doctest example from `0.18` to `0.19`. (#300)
- hardware_buffer: Made `HardwareBufferDesc` fields `pub`. (#313)
- **Breaking:** Remove `hardware_buffer` and `trace` features in favour of using `api-level-26` or `api-level-23` directly. (#320)

# 0.6.0 (2022-01-05)

- **Breaking:** Upgrade to [`ndk-sys 0.3.0`](../ndk-sys/CHANGELOG.md#030-2022-01-05) and migrate to `jni-sys` types that it now directly uses in its bindings. (#209 / #214)

# 0.5.0 (2021-11-22)

- **Breaking:** Replace `add_fd_with_callback` `ident` with constant value `ALOOPER_POLL_CALLBACK`,
  as per <https://developer.android.com/ndk/reference/group/looper#alooper_addfd>.
- **Breaking:** Accept unboxed closure in `add_fd_with_callback`.
- aaudio: Replace "Added in" comments with missing `#[cfg(feature)]`.
- aaudio: Add missing `fn get_allowed_capture_policy()`.
- configuration: Add missing `api-level-30` feature to `fn screen_round()`.

# 0.4.0 (2021-08-02)

- **Breaking:** Model looper file descriptor events integer as `bitflags`.

# 0.3.0 (2021-01-30)

- **Breaking:** Looper `ident` not passed in `data` pointer anymore.
  `attach_looper` now only sets the `ident` field when attaching an
  `InputQueue` to a `ForeignLooper`.
  If you are relying on `Poll::Event::data` to tell event fd and
  input queue apart, please use `Poll::Event::ident` and the new
  constants introduced in `ndk-glue`!

# 0.2.1 (2020-10-15)

- Fix documentation build on docs.rs

# 0.2.0 (2020-09-15)

- **Breaking:** Updated to use [ndk-sys 0.2.0](../ndk-sys/CHANGELOG.md#020-2020-09-15)
- Added `media` bindings
- Added `bitmap` and `hardware_buffer` bindings
- Added `aaudio` bindings
- Fixed assets directory path to be relative to the manifest
- Added `trace` feature for native tracing

# 0.1.0 (2020-04-22)

- Initial release! 🎉
