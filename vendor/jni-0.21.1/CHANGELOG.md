# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

<!-- Use the following sections from the spec: http://keepachangelog.com/en/1.0.0/
  - Added for new features.
  - Changed for changes in existing functionality.
  - Deprecated for soon-to-be removed features.
  - Removed for now removed features.
  - Fixed for any bug fixes.
  - Security in case of vulnerabilities. -->

## [Unreleased]


## [0.21.1] — 2023-03-08

### Fixes
- Compilation is fixed for architectures with a C ABI that has unsigned `char` types. ([#419](https://github.com/jni-rs/jni-rs/pull/419))

## [0.21.0] — 2023-02-13

This release makes extensive breaking changes in order to improve safety. Most projects that use this library will need to be changed. Please see [the migration guide](docs/0.21-MIGRATION.md).

### Added
- `JavaStr::into_raw()` which drops the `JavaStr` and releases ownership of the raw string pointer ([#374](https://github.com/jni-rs/jni-rs/pull/374))
- `JavaStr::from_raw()` which takes ownership of a raw string pointer to create a `JavaStr` ([#374](https://github.com/jni-rs/jni-rs/pull/374))
- `JNIEnv::get_string_unchecked` is a cheaper, `unsafe` alternative to `get_string` that doesn't check the given object is a `java.lang.String` instance. ([#328](https://github.com/jni-rs/jni-rs/issues/328))
- `WeakRef` and `JNIEnv#new_weak_ref`. ([#304](https://github.com/jni-rs/jni-rs/pull/304))
- `define_class_bytearray` method that takes an `AutoElements<jbyte>` rather than a `&[u8]` ([#244](https://github.com/jni-rs/jni-rs/pull/244))
- `JObject` now has an `as_raw` method that borrows the `JObject` instead of taking ownership like `into_raw`. Needed because `JObject` no longer has the `Copy` trait. ([#392](https://github.com/jni-rs/jni-rs/issues/392))
- `JavaVM::destroy()` (unsafe) as a way to try and unload a `JavaVM` on supported platforms ([#391](https://github.com/jni-rs/jni-rs/issues/391))
- `JavaVM::detach_current_thread()` (unsafe) as a way to explicitly detach a thread (normally this is automatic on thread exit). Needed to detach daemon threads manually if using `JavaVM::destroy()` ([#391](https://github.com/jni-rs/jni-rs/issues/391))
- `JPrimitiveArray<T: TypeArray>` and type-specific aliases like `JByteArray`, `JIntArray` etc now provide safe, reference wrappers for the `sys` types `jarray` and `jbyteArray` etc with a lifetime like `JObject` ([#400](https://github.com/jni-rs/jni-rs/pull/400))
- `JObjectArray` provides a reference wrapper for a `jobjectArray` with a lifetime like `JObject`. ([#400](https://github.com/jni-rs/jni-rs/pull/400))
- `AutoElements` and `AutoElementsCritical` (previously `AutoArray`/`AutoPrimitiveArray`) implement `Deref<Target=[T]>` and `DerefMut` so array elements can be accessed via slices without needing additional `unsafe` code. ([#400](https://github.com/jni-rs/jni-rs/pull/400))
- `AsJArrayRaw` trait which enables `JNIEnv::get_array_length()` to work with `JPrimitiveArray` or `JObjectArray` types ([#400](https://github.com/jni-rs/jni-rs/pull/400))
- `InitArgsBuilder` now has `try_option` and `option_encoded` methods. ([#414](https://github.com/jni-rs/jni-rs/pull/414))

### Changed
- `JNIEnv::get_string` checks that the given object is a `java.lang.String` instance to avoid undefined behaviour from the JNI implementation potentially aborting the program. ([#328](https://github.com/jni-rs/jni-rs/issues/328))
- `JNIEnv::call_*method_unchecked` was marked `unsafe`, as passing improper argument types, or a bad number of arguments, can cause a JVM crash. ([#385](https://github.com/jni-rs/jni-rs/issues/385))
- The `JNIEnv::new_object_unchecked` function now takes arguments as `&[jni::sys::jvalue]` to avoid allocating, putting it inline with changes to `JniEnv::call_*_unchecked` from 0.20.0 ([#382](https://github.com/jni-rs/jni-rs/pull/382))
- The `get_superclass` function now returns an Option instead of a null pointer if the class has no superclass ([#151](https://github.com/jni-rs/jni-rs/issues/151))
- The `invocation` feature now locates the JVM implementation dynamically at runtime (via the `java-locator` crate by default) instead of linking with the JVM at build time ([#293](https://github.com/jni-rs/jni-rs/pull/293))
- Most `JNIEnv` methods now require `&mut self`. This improves safety by preventing `JObject`s from getting an invalid lifetime. Most native method implementations (that is, `#[no_mangle] extern "system" fn`s) must now make the `JNIEnv` parameter `mut`. See the example on the crate documentation. ([#392](https://github.com/jni-rs/jni-rs/issues/392))
- `JByteBuffer`, `JClass`, `JNIEnv`, `JObject`, `JString`, and `JThrowable` no longer have the `Clone` or `Copy` traits. This improves safety by preventing object references from being used after the JVM deletes them. Most functions that take one of these types as a parameter (except `extern fn`s that are directly called by the JVM) should now borrow it instead, e.g. `&JObject` instead of `JObject`. ([#392](https://github.com/jni-rs/jni-rs/issues/392))
- `AutoLocal` is now generic in the type of object reference (`JString`, etc). ([#392](https://github.com/jni-rs/jni-rs/issues/392))
- The closure passed to `JNIEnv::with_local_frame` must now take a `&mut JNIEnv` parameter, which has a different lifetime. This improves safety by preventing local references from escaping the closure, which would cause a use-after-free bug. `Executor::with_attached` and `Executor::with_attached_capacity` have been similarly changed. ([#392](https://github.com/jni-rs/jni-rs/issues/392))
- The closure passed to `JNIEnv::with_local_frame` can now return a generic `Result<T, E>` so long as the error implements `From<jni::errors::Error>` ([#399](https://github.com/jni-rs/jni-rs/issues/399))
- `JNIEnv::with_local_frame` now returns the same type that the given closure returns ([#399](https://github.com/jni-rs/jni-rs/issues/399))
- `JNIEnv::with_local_frame` no longer supports returning a local reference directly to the calling scope (see `with_local_frame_returning_local`) ([#399](https://github.com/jni-rs/jni-rs/issues/399))
- `Executor::with_attached` and `Executor::with_attached_capacity` have been changed in the same way as `JNIEnv::with_local_frame` (they are thin wrappers) ([#399](https://github.com/jni-rs/jni-rs/issues/399))
- `Desc`, `JNIEnv::pop_local_frame`, and `TypeArray` are now `unsafe`. ([#392](https://github.com/jni-rs/jni-rs/issues/392))
- The `Desc` trait now has an associated type `Output`. Many implementations now return `AutoLocal`, so if you call `Desc::lookup` yourself and then call `as_raw` on the returned object, make sure the `AutoLocal` isn't dropped too soon (see the `Desc::lookup` documentation for examples). ([#392](https://github.com/jni-rs/jni-rs/issues/392))
- The `Desc<JClass>` trait is no longer implemented for `JObject` or `&JObject`. The previous implementation that called `.get_object_class()` was surprising and a simpler cast would make it easy to mistakenly pass instances where a class is required. ([#118](https://github.com/jni-rs/jni-rs/issues/118))
- Named lifetimes in the documentation have more descriptive names (like `'local` instead of `'a`). The new naming convention is explained in the `JNIEnv` documentation. ([#392](https://github.com/jni-rs/jni-rs/issues/392))
- Object reference types (`JObject`, `JClass`, `AutoLocal`, `GlobalRef`, etc) now implement `AsRef<JObject>` and `Deref<Target = JObject>`. Typed wrappers like `JClass` also implement `Into<JObject>`, but `GlobalRef` does not. ([#392](https://github.com/jni-rs/jni-rs/issues/392))
- Most `JList` and `JMap` methods now require a `&mut JNIEnv` parameter. `JListIter` and `JMapIter` no longer implement `Iterator`, and instead have a `next` method that requires a `&mut JNIEnv` parameter (use `while let` loops instead of `for`). ([#392](https://github.com/jni-rs/jni-rs/issues/392))
- `JValue` has been changed in several ways: ([#392](https://github.com/jni-rs/jni-rs/issues/392))
    - It is now a generic type named `JValueGen`. `JValue` is now a type alias for `JValueGen<&JObject>`, that is, it borrows an object reference. `JValueOwned` is a type alias for `JValueGen<JObject>`, that is, it owns an object reference.
    - `JValueOwned` does not have the `Copy` trait.
    - The `to_jni` method is now named `as_jni`, and it borrows the `JValueGen` instead of taking ownership.
    - `JObject` can no longer be converted directly to `JValue`, which was commonly done when calling Java methods or constructors. Instead of `obj.into()`, use `(&obj).into()`.
- All `JNIEnv` array APIs now work in terms of `JPrimitiveArray` and `JObjectArray` (reference wrappers with a lifetime) instead of `sys` types like `jarray` and `jbyteArray` ([#400](https://github.com/jni-rs/jni-rs/pull/400))
- `AutoArray` and `AutoPrimitiveArray` have been renamed `AutoElements` and `AutoElementsCritical` to show their connection and differentiate from new `JPrimitiveArray` API ([#400](https://github.com/jni-rs/jni-rs/pull/400))
- `get_primitive_array_critical` is now `unsafe` and has been renamed to `get_array_elements_critical` (consistent with the rename of `AutoPrimitiveArray`) with more detailed safety documentation ([#400](https://github.com/jni-rs/jni-rs/pull/400))
- `get_array_elements` is now also `unsafe` (for many of the same reasons as `get_array_elements_critical`) and has detailed safety documentation ([#400](https://github.com/jni-rs/jni-rs/pull/400))
- `AutoArray/AutoArrayCritical::size()` has been replaced with `.len()` which can't fail and returns a `usize` ([#400](https://github.com/jni-rs/jni-rs/pull/400))
- The `TypeArray` trait is now a private / sealed trait, that is considered to be an implementation detail for the `AutoArray` API.
- `JvmError` has several more variants and is now `non_exhaustive`. ([#414](https://github.com/jni-rs/jni-rs/pull/414))
- `InitArgsBuilder::option` raises an error on Windows if the string is too long. The limit is currently 1048576 bytes. ([#414](https://github.com/jni-rs/jni-rs/pull/414))

### Fixed
- Trying to use an object reference after it has been deleted now causes a compile error instead of undefined behavior. As a result, it is now safe to use `AutoLocal`, `JNIEnv::delete_local_ref`, and `JNIEnv::with_local_frame`. (Most of the limitations added in #392, listed above, were needed to make this work.) ([#381](https://github.com/jni-rs/jni-rs/issues/381), [#392](https://github.com/jni-rs/jni-rs/issues/392))
- Class lookups via the `Desc` trait now return `AutoLocal`s, which prevents them from leaking. ([#109](https://github.com/jni-rs/jni-rs/issues/109), [#392](https://github.com/jni-rs/jni-rs/issues/392))
- `InitArgsBuilder::option` properly encodes non-ASCII characters on Windows. ([#414](https://github.com/jni-rs/jni-rs/pull/414))

### Removed
- `get_string_utf_chars` and `release_string_utf_chars` from `JNIEnv` (See `JavaStr::into_raw()` and `JavaStr::from_raw()` instead) ([#372](https://github.com/jni-rs/jni-rs/pull/372))
- All `JNIEnv::get_<type>_array_elements()` methods have been removed as redundant since they would all be equivalent to `get_array_elements()` with the introduction of `JPrimitiveArray` ([#400](https://github.com/jni-rs/jni-rs/pull/400))

## [0.20.0] — 2022-10-17

### Added
- `Default` trait implemented for `JObject`, `JString`, `JClass`, and `JByteBuffer` ([#199](https://github.com/jni-rs/jni-rs/issues/199))
- `Debug` trait implemented for `JavaVM`, `GlobalRef`, `GlobalRefGuard`, `JStaticMethodID` and `ReleaseMode` ([#345](https://github.com/jni-rs/jni-rs/pull/345))
- `ReturnType` for specifying object return types without a String allocation. ([#329](https://github.com/jni-rs/jni-rs/issues/329))

### Changed
- The `release_string_utf_chars` function has been marked as unsafe. ([#334](https://github.com/jni-rs/jni-rs/pull/334))
- Mark `JNIEnv::new_direct_byte_buffer` as `unsafe` ([#320](https://github.com/jni-rs/jni-rs/pull/320))
- `JNIEnv::new_direct_byte_buffer` now takes a raw pointer and size instead of a slice ([#351](https://github.com/jni-rs/jni-rs/pull/351) and [#364](https://github.com/jni-rs/jni-rs/pull/364))
- `JNIEnv::direct_buffer_address` returns a raw pointer instead of a slice ([#364](https://github.com/jni-rs/jni-rs/pull/364))
- The lifetime of `AutoArray` is no longer tied to the lifetime of a particular `JNIEnv` reference. ([#302](https://github.com/jni-rs/jni-rs/issues/302))
- Relaxed lifetime restrictions on `JNIEnv::new_local_ref`. Now it can be used to create a local
  reference from a global reference. ([#301](https://github.com/jni-rs/jni-rs/issues/301) / [#319](https://github.com/jni-rs/jni-rs/pull/319))
- `JMethodID` and `JStaticMethodID` implement `Send` + `Sync` and no longer has a lifetime parameter, making method
  IDs cacheable (with a documented 'Safety' note about ensuring they remain valid). ([#346](https://github.com/jni-rs/jni-rs/pull/346))
- `JFieldID` and `JStaticFieldID` implement `Send` + `Sync` and no longer has a lifetime parameter, making field
  IDs cacheable (with a documented 'Safety' note about ensuring they remain valid). ([#346](https://github.com/jni-rs/jni-rs/pull/365))
- The `call_*_method_unchecked` functions now take `jni:sys::jvalue` arguments to avoid allocating
  a `Vec` on each call to map + collect `JValue`s as `sys:jvalue`s ([#329](https://github.com/jni-rs/jni-rs/issues/329))
- The `From` trait implementations converting `jni_sys` types like `jobject` to `JObject` have been replaced
  with `unsafe` `::from_raw` functions and corresponding `::into_raw` methods. Existing `::into_inner` APIs were
  renamed `::into_raw` for symmetry. ([#197](https://github.com/jni-rs/jni-rs/issues/197))
- The APIs `JNIEnv::set_rust_field`, `JNIEnv::get_rust_field` and `JNIEnv::take_rust_field` have been marked as `unsafe` ([#219](https://github.com/jni-rs/jni-rs/issues/219))

## [0.19.0] — 2021-01-24

### Added
- `AutoArray` and generic `get_array_elements()`, along with `get_<type>_array_elements` helpers. (#287)
- `size()` method to `AutoArray` and `AutoPrimitiveArray`. (#278 / #287)
- `discard()` method to `AutoArray` and `AutoPrimitiveArray`. (#275 / #287)

### Changed
- Removed AutoPrimitiveArray::commit(). (#290)
- `AutoByte/PrimitiveArray.commit()` now returns `Result`. (#275)
- Removed methods get/release/commit_byte/primitive_array_{elements|critical}. (#281)
- Renamed methods get_auto_byte/long/primitive_array_{elements|critical} to
	get_byte/long/primitive_array_{elements|critical}. (#281)

## [0.18.0] — 2020-09-23

### Added
- `JNIEnv#define_unnamed_class` function that allows loading a class without
  specifying its name. The name is inferred from the class data. (#246)
- `SetStatic<type>Field`. (#248)
- `TryFrom<JValue>` for types inside JValue variants (#264).
- Implemented Copy for JNIEnv (#255).
- `repr(transparent)` attribute to JavaVM struct (#259)

### Changed
- Switch from `error-chain` to `thiserror`, making all errors `Send`. Also, support all JNI errors
  in the `jni_error_code_to_result` function and add more information to the `InvalidArgList`
  error. ([#242](https://github.com/jni-rs/jni-rs/pull/242))

## [0.17.0] — 2020-06-30

### Added
- Get/ReleaseByteArrayElements, and Get/ReleasePrimitiveArrayCritical. (#237)

## [0.16.0] — 2020-02-28

### Fixed
- Java VM instantiation with some MacOS configurations. (#220, #229, #230).

## [0.15.0] — 2020-02-28

### Added
- Ability to pass object wrappers that are convertible to `JObject` as arguments to the majority
 of JNIEnv methods without explicit conversion. (#213)
- `JNIEnv#is_same_object` implementation. (#213)
- `JNIEnv#register_native_methods`. (#214)
- Conversion from `Into<JObject>` to `JValue::Object`.

### Fixed
- Passing `null` as class loader to `define_class` method now allowed according
  to the JNI specification. (#225)

## [0.14.0] — 2019-10-31

### Changed
- Relaxed some lifetime restrictions in JNIEnv to support the case when
  method, field ids; and global references to classes
  have a different (larger) lifetime than JNIEnv. (#209)

## [0.13.1] — 2019-08-22

### Changed
- Various documentation improvements.

## [0.13.0] — 2019-07-05

0.13 brings major improvements in thread management, allowing to attach the native threads
permanently and safely; `Executor` for extra convenience and safety; and other
improvements and fixes.

:warning: If your code attaches native threads — make sure to check the updated documentation
of [JavaVM](https://docs.rs/jni/0.13.0/jni/struct.JavaVM.html) to learn about the new features!

### Added
- `JavaVM::attach_current_thread_permanently` method, which attaches the current
  thread and detaches it when the thread finishes. Daemon threads attached
  with `JavaVM::attach_current_thread_as_daemon` also automatically detach themselves
  when finished. The number of currently attached threads may be acquired using
  `JavaVM::threads_attached` method. (#179, #180)
- `Executor` — a simple thread attachment manager which helps to safely
  execute a closure in attached thread context and to automatically free
  created local references at closure exit. (#186)

### Changed
- The default JNI API version in `InitArgsBuilder` from V1 to V8. (#178)
- Extended the lifetimes of `AutoLocal` to make it more flexible. (#190)
- Default exception type from checked `java.lang.Exception` to unchecked `java.lang.RuntimeException`.
  It is used implicitly when `JNIEnv#throw` is invoked with exception message:
  `env.throw("Exception message")`; however, for efficiency reasons, it is recommended
  to specify the exception type explicitly *and* use `throw_new`:
  `env.throw_new(exception_type, "Exception message")`. (#194)

### Fixed
- Native threads attached with `JavaVM::attach_current_thread_as_daemon` now automatically detach
  themselves on exit, preventing Java Thread leaks. (#179)
- Local reference leaks in `JList`, `JMap` and `JMapIter`. (#190, #191)

## [0.12.3]

### Added
- `From<jboolean>` implementation for `JValue` (#173)
- `Debug` trait for InitArgsBuilder. (#175)
- `InitArgsBuilder#options` returning the collected JVM options. (#177)

## [0.12.2]

### Changed
- Updated documentation of GetXArrayRegion methods (#169)
- Improved ABI compatibility on various platforms (#170)

## [0.12.1]

This release does not bring code changes.

### Changed
- Updated project documentation.

## [0.12.0]

### Changed
- `JString`, `JMap` and `JavaStr` and their respective iterators now require an extra lifetime so
  that they can now work with `&'b JNIEnv<'a>`, where `'a: 'b`.

## [0.11.0]

### Highlights
This release brings various improvements and fixes, outlined below. The most notable changes are:
- `null` is no longer represented as an `Err` with error kind `NullPtr` if it is a value of some
  nullable Java reference (not an indication of an error). Related issues: #136, #148, #163.
- `unsafe` methods, providing a low-level API similar to JNI, has been marked safe and renamed
  to have `_unchecked` suffix. Such methods can be used to implement caching of class references
  and method IDs to improve performance in loops and frequently called Java callbacks.
  If you have such, check out [the docs][unchecked-docs] and [one of early usages][cache-exonum]
  of this feature.

[unchecked-docs]: https://docs.rs/jni/0.11.0/jni/struct.JNIEnv.html
[cache-exonum]: https://github.com/exonum/exonum-java-binding/blob/affa85c026c1870b502725b291822c00f199745d/exonum-java-binding/core/rust/src/utils/jni_cache.rs#L40

### Added
- Invocation API support on Windows and AppVeyor CI (#149)

### Changed
- `push_local_frame`, `delete_global_ref` and `release_string_utf_chars`
no longer check for exceptions as they are
[safe](https://docs.oracle.com/javase/10/docs/specs/jni/design.html#exception-handling)
to call if there is a pending exception (#124):
  - `push_local_frame` will now work in case of pending exceptions — as
  the spec requires; and fail in case of allocation errors
  - `delete_global_ref` and `release_string_utf_chars` won't print incorrect
  log messages

- Rename some macros to better express their intent (see #123):
  - Rename `jni_call` to `jni_non_null_call` as it checks the return value
  to be non-null.
  - Rename `jni_non_null_call` (which may return nulls) to `jni_non_void_call`.

- A lot of public methods of `JNIEnv` have been marked as safe, all unsafe code
  has been isolated inside internal macros. Methods with `_unsafe` suffixes have
  been renamed and now have `_unchecked` suffixes (#140)

- `from_str` method of the `JavaType` has been replaced by the `FromStr`
  implementation

- Implemented Sync for GlobalRef (#102).

- Improvements in macro usage for JNI methods calls (#136):
  - `call_static_method_unchecked` and `get_static_field_unchecked` methods are
  allowed to return NULL object
  - Added checking for pending exception to the `call_static_method_unchecked`
  method (eliminated WARNING messages in log)

- Further improvements in macro usage for JNI method calls (#150):
  - The new_global_ref() and new_local_ref() functions are allowed to work with NULL objects according to specification.
  - Fixed the family of functions new_direct_byte_buffer(), get_direct_buffer_address() and get_direct_buffer_capacity()
   by adding checking for null and error codes.
  - Increased tests coverage for JNIEnv functions.

- Implemented Clone for JNIEnv (#147).

- The get_superclass(), get_field_unchecked() and get_object_array_element() are allowed to return NULL object according
 to the specification (#163).

### Fixed
- The issue with early detaching of a thread by nested AttachGuard. (#139)

## [0.10.2]

### Added
- `JavaVM#get_java_vm_pointer` to retrieve a JavaVM pointer (#98)
- This changelog and other project documents (#106)

### Changed
- The project is moved to an organization (#104)
- Updated versions of dependencies (#105)
- Improved project documents (#107)

### Fixed
- Crate type of a shared library with native methods
  must be `cdylib` (#100)

## [0.10.1]
- No changes has been made to the Changelog until this release.

[Unreleased]: https://github.com/jni-rs/jni-rs/compare/v0.21.1...HEAD
[0.21.1]: https://github.com/jni-rs/jni-rs/compare/v0.21.0...v0.21.1
[0.21.0]: https://github.com/jni-rs/jni-rs/compare/v0.20.0...v0.21.0
[0.20.0]: https://github.com/jni-rs/jni-rs/compare/v0.19.0...v0.20.0
[0.19.0]: https://github.com/jni-rs/jni-rs/compare/v0.18.0...v0.19.0
[0.18.0]: https://github.com/jni-rs/jni-rs/compare/v0.17.0...v0.18.0
[0.17.0]: https://github.com/jni-rs/jni-rs/compare/v0.16.0...v0.17.0
[0.16.0]: https://github.com/jni-rs/jni-rs/compare/v0.15.0...v0.16.0
[0.15.0]: https://github.com/jni-rs/jni-rs/compare/v0.14.0...v0.15.0
[0.14.0]: https://github.com/jni-rs/jni-rs/compare/v0.13.1...v0.14.0
[0.13.1]: https://github.com/jni-rs/jni-rs/compare/v0.13.0...v0.13.1
[0.13.0]: https://github.com/jni-rs/jni-rs/compare/v0.12.3...v0.13.0
[0.12.3]: https://github.com/jni-rs/jni-rs/compare/v0.12.2...v0.12.3
[0.12.2]: https://github.com/jni-rs/jni-rs/compare/v0.12.1...v0.12.2
[0.12.1]: https://github.com/jni-rs/jni-rs/compare/v0.12.0...v0.12.1
[0.12.0]: https://github.com/jni-rs/jni-rs/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/jni-rs/jni-rs/compare/v0.10.2...v0.11.0
[0.10.2]: https://github.com/jni-rs/jni-rs/compare/v0.10.1...v0.10.2
[0.10.1]: https://github.com/jni-rs/jni-rs/compare/v0.1...v0.10.1
