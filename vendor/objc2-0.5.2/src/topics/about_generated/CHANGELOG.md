# Framework crate changelog

Changes to `objc2`'s framework crates will be documented in this file.

The history of these crates are a bit convoluted; the Foundation parts
originally existed as `objc2-foundation`, but later it was integrated into
`objc2` under `objc2::foundation`, later again moved into `icrate::Foundation`
as part of the larger `icrate`, and finally `icrate` was split back out into
`objc2-foundation` and other smaller crates - hence the confusing versioning.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased - YYYY-MM-DD


## 0.2.1 - 2024-05-21

### Added
* `NS_OPTIONS` enums are now `bitflags!`-style enums.

  This means that they have the usual `ops::BitAnd`, `ops::BitOr`, `ops::Not`,
  etc. implementations that you would expect from an option enum.
* Added optional support for a few methods depending on types from `libc`.
* Added new framework crates:
  - `CoreBluetooth` / `objc2-core-bluetooth`.
  - `Virtualization` / `objc2-virtualization`.
  - `Symbols` / `objc2-symbols`.
  - `UIKit` / `objc2-ui-kit`.
  - `Accounts` / `objc2-accounts`.
  - `AppTrackingTransparency` / `objc2-app-tracking-transparency`.
  - `MLCompute` / `objc2-ml-compute`.
  - `ContactsUI` / `objc2-contacts-ui`.
  - `NearbyInteraction` / `objc2-nearby-interaction`.
  - `ScreenCaptureKit` / `objc2-screen-capture-kit`.
  - `CoreImage` / `objc2-core-image`.
  - `CoreMotion` / `objc2-core-motion`.
  - `MultipeerConnectivity` / `objc2-multipeer-connectivity`.
  - `NaturalLanguage` / `objc2-natural-language`.
  - `CoreML` / `objc2-core-ml`.
  - `Vision` / `objc2-vision`.
  - `AVKit` / `objc2-av-kit`.
  - `NetworkExtension` / `objc2-network-extension`.
  - `SensitiveContentAnalysis` / `objc2-sensitive-content-analysis`.
  - `PhotosUI` / `objc2-photos-ui`.
  - `FinderSync` / `objc2-finder-sync`.
  - `Social` / `objc2-social`.
* `objc2-quartz-core`: Added `CAMetalDrawable` and `CAMetalLayer`.
* `objc2-app-kit`: Added methods to access `CALayer` from `NSView`.
* `objc2-metal`: Added `MTLCounterErrorValue`, `MTLCounterDontSample` and
  `MTLTextureSwizzleChannelsDefault` constants.

### Changed
* Updated SDK from Xcode 15.3 to 15.4.

  View the release notes to learn more details:
  - [15.4](https://developer.apple.com/documentation/xcode-release-notes/xcode-15_4-release-notes)

### Fixed
* `objc2-metal`: All protocols are now marked as `IsRetainable`.

  This may technically break your code if you had some custom object that
  implements the protocol, but was marked with `Mutability = Mutable`, but
  Metal protocols are assumed retainable by Objective-C code, so that'd have
  been unsound anyway.
* Add `#[cfg(target_os = "...")]` when a platform-specific dependency is used.

  This improves iOS support, as crates like `objc2-app-kit` are no longer
  enabled on this platform.
* Fix dependency feature flags (e.g. `block2`) not enabling the matching
  features in dependencies (e.g. not enabling `objc2-foundation/block2`).

### Removed
* `objc2-metal`: Removed internal `__MTLPackedFloat3` and made `MTLPackedFloat3` public.
* `objc2-core-data`: Removed broken GNUStep support.
* `objc2-quartz-core`: Removed broken GNUStep support.


## 0.2.0 - 2024-04-17

### Added
* Added `NSObject` categories, notably those used by key-value coding and
  observing.
* Added a few statics that were previously omitted (notably a few
  `NSWindowLevel` constants).

### Changed
* Updated SDK from Xcode 15.0.1 to 15.3.

  View the release notes to learn more details:
  - [15.1](https://developer.apple.com/documentation/xcode-release-notes/xcode-15_1-release-notes)
  - [15.2](https://developer.apple.com/documentation/xcode-release-notes/xcode-15_2-release-notes)
  - [15.3](https://developer.apple.com/documentation/xcode-release-notes/xcode-15_3-release-notes)
* **BREAKING**: Changed how categories are handled; now, when a library has
  defined methods on a class defined in a different framework, a helper trait
  is output with the methods, instead of the methods being implemented
  directly on the type.
* **BREAKING**: Changed how enums are handled; now a newtype is generated for
  each enum, with the enum variants as constants on that newtype, instead of
  cluttering the top-level namespace.
* **BREAKING**: Split `icrate` into multiple smaller crates all prefixed with
  `objc2-`, and changed how feature flags work.

  Feature flags are now based on the header file name where the item is
  defined, instead of being based on the class name itself.

  These two things should decrease compilation times by quite a lot, at the
  cost of having to specify more precisely what you need in your `Cargo.toml`.

  An example migration can be seen in the following:

  ```toml
  # Before
  [dependencies.icrate]
  version = "0.1.0"
  features = [
    "Foundation",
    "Foundation_NSNotification",
    "Foundation_NSString",
    "Foundation_NSThread",
    "Foundation_NSArray",
    "Foundation_NSMutableArray",
    "AppKit",
    "AppKit_NSApplication",
  ]

  # After
  # Moved to `objc2-foundation` and `objc2-app-kit` crates.
  [dependencies]
  # Removed `Foundation_NSMutableArray`, it is included via. `NSArray`.
  # Added `NSObject` as the `NSCopying` protocol comes from there.
  objc2-foundation = { version = "0.2", features = ["NSNotification", "NSString", "NSThread", "NSObject", "NSArray"] }
  # Added `NSResponder` as it's required by `NSApplication`.
  # Added `NSRunningApplication` as a lot of application constants come from here.
  objc2-app-kit = { version = "0.2", features = ["NSResponder", "NSApplication", "NSRunningApplication"] }
  ```
* Marked `NSView::isFlipped`, `NSView::convertRect_toView`,
  `NSWindow::convertRectToScreen` and `NSWindow::convertPointFromScreen` as
  safe.
* Renamed the `block` and `objective-c` feature flags to `block2` and `objc2`.
* **BREAKING**: Updated `block2` to `v0.5`.

### Deprecated
* Deprecated `Foundation::MainThreadMarker::run_on_main`, use the new
  free-standing function `objc2_foundation::run_on_main` instead.

### Removed
* Removed private functionality in the `Speech` framework. This was never
  intended to be exposed by Apple.

  This is technically a breaking change, but will be allowed in a minor
  version since it isn't supported by Apple.

### Fixed
* Fixed the `QuartzCore` and `Photos` frameworks not being loaded correctly.
* Fixed a few feature gates on methods showing up unnecessarily.
* **BREAKING**: Added `MainThreadMarker` parameter to a few methods where it
  was erroneously missing.
* **BREAKING**: Fix the type of `GKMatchProperties` and `NSWindow`'s
  `restorationClass`.
* **BREAKING**: Fixed feature flags not being emitted on typedefs, enums,
  functions and statics.


## icrate 0.1.2 - 2023-04-18

## Fixed
* Fixed "multiple applicable items in scope" error when upgrading `objc2` to `v0.5.1`.


## icrate 0.1.1 - 2023-04-17

### Deprecated
* Deprecated the `icrate` crate, it has been split into multiple smaller crates.


## icrate 0.1.0 - 2023-12-23

### Added
* Added `MainThreadMarker` `From` implementation for `MainThreadOnly` types.
* Added `Send` and `Sync` implementations for a bunch more types (same as the
  ones Swift marks as `@Sendable`).
* Made some common methods in `AppKit` safe.
* Added missing `NSCopying` and `NSMutableCopying` zone methods.
* Added `Eq` and `Ord` implementations for `NSNumber`, since its
  handling of floating point values allows it.
* Added `NS[Mutable]Dictionary::from_id_slice` and
  `NS[Mutable]Dictionary::from_slice`.
* Added `NSMutableDictionary::insert` and `NSMutableSet::insert` which can
  be more efficient than the previous insertion methods.

### Changed
* Updated SDK from Xcode 14.2 to 15.0.1.

  View the release notes to learn more details:
  - [14.3](https://developer.apple.com/documentation/xcode-release-notes/xcode-14_3-release-notes)
  - [14.3.1](https://developer.apple.com/documentation/xcode-release-notes/xcode-14_3_1-release-notes)
  - [15.0](https://developer.apple.com/documentation/xcode-release-notes/xcode-15-release-notes)
  - [15.0.1](https://developer.apple.com/documentation/xcode-release-notes/xcode-15_0_1-release-notes)

  Breaking changes are noted elsewhere in this changelog entry.
* Moved the `ns_string!` macro to `icrate::Foundation::ns_string`. The old
  location in the crate root is deprecated.
* **BREAKING**: The following two methods on
  `MTLAccelerationStructureCommandEncoder` now take a nullable scratch buffer:
  - `refitAccelerationStructure_descriptor_destination_scratchBuffer_scratchBufferOffset`
  - `refitAccelerationStructure_descriptor_destination_scratchBuffer_scratchBufferOffset_options`
* **BREAKING**: Marked UI-related classes as `MainThreadOnly`, and UI-related
  protocols as `IsMainThreadOnly`.

  This means that they can now only be constructed, retrieved and used on the
  main thread, meaning you usually have to aquire a `MainThreadMarker` first.

  ```rust, ignore
  // Before
  let app = unsafe { NSApplication::sharedApplication() };
  let view = unsafe { NSView::initWithFrame(NSView::alloc(), frame) };
  // Do something with `app` and `view`

  // After
  let mtm = MainThreadMarker::new().unwrap();
  let app = unsafe { NSApplication::sharedApplication(mtm) };
  let view = unsafe { NSView::initWithFrame(mtm.alloc(), frame) };
  // Do something with `app` and `view`
  ```
* **BREAKING**: Changed the `NSApp` static to be a function taking `MainThreadMarker`.
* **BREAKING**: Renamed `NS[Mutable]Dictionary::from_keys_and_objects` to
  `NS[Mutable]Dictionary::from_vec`.
* **BREAKING**: Renamed `NSMutableDictionary::insert` and
  `NSMutableSet::insert` to `insert_id`.
* **BREAKING**: `CWWiFiClient::interfaceNames` has been renamed to `CWWiFiClient::interfaceNames_class`.
* **BREAKING**: Updated `objc2` to `v0.5.0`.
* **BREAKING**: Updated `block2` to `v0.4.0`.

### Removed
* **BREAKING**: Removed the `MainThreadMarker` argument from the closure
  passed to `MainThreadBound::get_on_main`.
* **BREAKING**: Removed `Foundation::CopyHelper` since it is superseded by
  `objc2::mutability::CounterpartOrSelf`.
* **BREAKING**: Removed the following APIs, as they are no longer available in macOS 14 / iOS 17:
  - `NSFileProviderDomain::volumeUUID`
  - `CLBeaconIdentityConstraint::UUID`
  - `CLBeaconIdentityConstraint::major`
  - `CLBeaconIdentityConstraint::minor`
  - `ASIdentifierManager::clearAdvertisingIdentifier`
* Removed private `MetricKit::_MXSignpostMetricsSnapshot` function.

### Fixed
* **BREAKING**: Added `Eq + Hash` requirement on most `NSDictionary` and
  `NSSet` methods, thereby making sure that the types are actually correct
  to use in such hashing collections.
* **BREAKING**: Added `HasStableHash` requirement on `NSDictionary` and
  `NSSet` creation methods, fixing a long-standing soundess issue.
* Fixed the protocol names of `NSAccessibilityElementProtocol`,
  `NSTextAttachmentCellProtocol` and `NSFileProviderItemProtocol`.
* **BREAKING**: Generic types no longer strictly require `Message` (although
  most of their trait implementations still require that).
* **BREAKING**: Removed a workaround that made the `NSCopying` and
  `NSMutableCopying` protocols not act as regular protocols (many methods used
  `AnyObject` instead of the correct `ProtocolObject<dyn NSCopying>`).
* Update the minimum deployment target, which adds a few missing protocol
  implementations and methods for `NSPopover` and `NSLayoutAnchor`.
* **BREAKING**: `CKSystemSharingUIObserver` and `CKLocationSortDescriptor` are no longer marked thread safe.
* **BREAKING**: `NSColor::ignoresAlpha` now requires a main thread marker.


## icrate 0.0.4 - 2023-07-31

### Changed
* **BREAKING**: Updated `block2` to `v0.3.0`.

### Fixed
* Documentation on docs.rs.


## icrate 0.0.3 - 2023-06-20

### Added
* Added the following frameworks:
  - `HealthKit`
  - `MediaPlayer`
  - `MetricKit`
  - `PhotoKit`
* Added `NSCopying` and `NSMutableCopying` implementations for the classes
  that implement those protocols.
* Added the following methods:
  - `NSArray::get_retained`
  - `NSArray::first_retained`
  - `NSArray::last_retained`
  - `NSSet::get_retained`
  - `NSSet::to_vec`
  - `NSSet::to_vec_retained`
  - `NSDictionary::get_retained`
  - `NSDictionary::keys_retained`
  - `NSDictionary::values_retained`
* Added `MainThreadMarker::alloc` for allocating objects that need to be so on
  the main thread.
* Added automatically generated `new`/`init` methods for all types.
* Added `FromIterator` impls for various collection types.

### Changed
* **BREAKING**: Renamed the `from_slice` method on `NSArray`, `NSSet`,
  `NSMutableArray` and `NSMutableSet` to `from_id_slice`, and provided a new
  `from_slice` method that takes `&[&T]` instead.
* **BREAKING**: Changed `NSMutableArray::replace` to return an `Result` in
  case the index was out of bounds.
* **BREAKING**: Changed `NSMutableArray::remove` to return an `Option` in case
  the index was out of bounds.
* **BREAKING**: Removed ownership parameter from generic types, since the
  ownership/mutability information is now stored in `ClassType::Mutability`.
* **BREAKING**: Renamed `NSMutableCopying::mutable_copy` to `::mutableCopy`.
* **BREAKING**: The default value for `NSUUID` was changed from a nil UUID to
  a new random UUID.
* **BREAKING**: Changed how iteration works.

  Instead of the single `NSFastEnumerator`, we now have concrete types
  `array::Iter`, `array::IterMut`, `array::IterRetained` and
  `array::IntoIter`, which allows iterating over `NSArray` in different ways.

  Combined with proper `IntoIterator` implementations for collection types,
  you can now do:
  ```rust, ignore
  let mut array: Id<NSMutableArray<T>> = ...;

  for item in &array {
      // item: &T
  }

  // If T: IsMutable
  for item in &mut array {
      // item: &mut T
  }

  // If T: IsIdCloneable
  for item in array.iter_retained() {
      // item: Id<T>
  }

  for item in array {
      // item: Id<T>
  }
  ```

  (similar functionality exist for `NSSet` and `NSDictionary`).
* **BREAKING**: Renamed `NSDictionary` methods:
  - `keys` -> `keys_vec`.
  - `values` -> `values_vec`.
  - `values_mut` -> `values_vec_mut`.
  - `keys_and_objects` -> `to_vecs`.
  - `iter_keys` -> `keys`.
  - `iter_values` -> `values`.
* **BREAKING**: `NSDictionary::keys_retained` and
  `NSDictionary::values_retained` now return an iterator instead.
* **BREAKING**: Updated `objc2` to `v0.4.0`.
* **BREAKING**: Updated `block2` to `v0.2.0`.

### Removed
* **BREAKING**: Removed various redundant `NSProxy` methods.
* **BREAKING**: Removed `NSArray::to_shared_vec` and `NSArray::into_vec`, use
  `NSArray::to_vec` or `NSArray::to_vec_retained` instead.
* **BREAKING**: Removed associated types from `NSCopying` and
  `NSMutableCopying`, that information is now specified in
  `ClassType::Mutability` instead.
* **BREAKING**: Removed a few `init` methods on subclasses that were declared
  on categories on their superclass. These should be re-added at some point.

### Fixed
* Soundness issues with enumeration / iteration over collection types.


## icrate 0.0.2 - 2023-02-07

### Added
* Added the following frameworks:
  - `Accessibility`
  - `AdServices`
  - `AdSupport`
  - `AutomaticAssessmentConfiguration`
  - `Automator`
  - `BackgroundAssets`
  - `BackgroundTasks`
  - `BusinessChat`
  - `CallKit`
  - `ClassKit`
  - `CloudKit`
  - `Contacts`
  - `CoreLocation`
  - `DataDetection`
  - `DeviceCheck`
  - `EventKit`
  - `ExceptionHandling`
  - `ExtensionKit`
  - `ExternalAccessory`
  - `FileProvider`
  - `FileProviderUI`
  - `GameController`
  - `GameKit`
  - `IdentityLookup`
  - `InputMethodKit`
  - `LinkPresentation`
  - `LocalAuthentication`
  - `LocalAuthenticationEmbeddedUI`
  - `MailKit`
  - `MapKit`
  - `Metal`
  - `MetalFX`
  - `MetalKit`
  - `OSAKit`
  - `CoreAnimation` (also known as `QuartzCore`)
  - `SoundAnalysis`
  - `Speech`
  - `StoreKit`
  - `UniformTypeIdentifiers`
  - `UserNotifications`
  - `WebKit`
* Updated the SDK version from XCode `14.0.1` to `14.2`.
  - See differences [here](https://sdk.news/macOS-13.0/).
* Added `Foundation::MainThreadBound` helper struct for things that are only
  accesible from the main thread.
* Added `#[deprecated]` annotations to the autogenerated output.
* Added disambiguation for duplicated methods (e.g. `NSThread::threadPriority`
  vs. `NSThread::threadPriority_class`).
* Properly implemented protocols for defined classes.

### Changed
* Cfg-gated static `ns_string!` functionality behind the
  `unstable-static-nsstring` cargo feature.
* Autogenerated method parameters are now in snake-case, for better IDE
  support.
* **BREAKING**: Cfg-gate all classes, and everything that references said
  classes.

  This means that to use e.g. `Foundation::NSThread::name`, you have to enable
  the `Foundation_NSThread` and `Foundation_NSString` cargo features.
* **BREAKING**: Updated `objc2` to `v0.3.0-beta.5`.
* **BREAKING**: Updated `block2` to `v0.2.0-alpha.8`.

### Removed
* **BREAKING**: The optional `uuid` integration, since one might want to use
  `icrate` internally in that crate in the future, and that would break.
* **BREAKING**: Removed `NSNib::instantiateWithOwner_topLevelObjects`,
  `NSBundle::loadNibNamed_owner_topLevelObjects` and `NSFreeMapTable` since
  they had weird memory management.

### Fixed
* Ensure we never hit a memory management issue again.
* **BREAKING**: Fixed a few `*mut` pointers that should've been `*const`.
* **BREAKING**: Fixed a few generic ownership parameters that defaulted to
  `Shared`.
* Removed a few instances of `TodoProtocols`.
* Fixed type-encoding of a few `struct`s.
* Fixed `NSProxy` trait methods.
* **BREAKING**: Fixed type in methods that worked with out-parameters.


## icrate 0.0.1 - 2022-12-24

### Added
* Added `NSString::write_to_file`.
* Added `NSLock` class and `NSLocking` protocol.
* Added autogenerated implementations of the following frameworks:
  - `AppKit`
  - `AuthenticationServices`
  - `CoreData`
  - `Foundation`

### Changed
* **BREAKING**: Moved from `objc2::foundation` into `icrate::Foundation`.
* **BREAKING**: Changed the following methods:
  - `NSString`
    - `concat` -> `stringByAppendingString`
    - `join_path` -> `stringByAppendingPathComponent`
    - `has_prefix` -> `hasPrefix`
    - `has_suffix` -> `hasSuffix`
  - `NSMutableString`
    - `from_nsstring` -> `stringWithString`
    - `with_capacity` -> `stringWithCapacity`
    - `push_nsstring` -> `appendString`
    - `replace` -> `setString`
  - `NSAttributedString`
    - `init_with_attributes` -> `unsafe initWithString_attributes`
    - `init_with_string` -> `initWithString`
    - `new_with_attributes` -> `unsafe new_with_attributes`
    - `len_utf16` -> `length`
  - `NSMutableAttributedString`
    - `replace` -> `setAttributedString`
  - `NSBundle`
    - `main` -> `mainBundle`
    - `info` -> `infoDictionary`
  - `NSDictionary`
    - `keys_array` -> `allKeys`
    - `into_values_array` -> `allValues`
  - `NSMutableDictionary`
    - `clear` -> `removeAllObjects`
  - `NSMutableArray`
    - `clear` -> `removeAllObjects`
  - `NSMutableSet`
    - `clear` -> `removeAllObjects`
  - `NSError`
    - `user_info` -> `userInfo`
    - `localized_description` -> `localizedDescription`
  - `NSException`
    - `user_info` -> `userInfo`
  - `NSMutableData`
    - `from_data` -> `dataWithData`
    - `with_capacity` -> `dataWithCapacity`
    - `set_len` -> `setLength`
  - `NSUUID`
    - `new_v4` -> `UUID`
    - `string` -> `UUIDString`
  - `NSThread`
    - `current` -> `currentThread`
    - `main` -> `mainThread`
    - `is_main` -> `isMainThread`
  - `NSProcessInfo`
    - `process_info` -> `processInfo`
* **BREAKING**: Make `NSComparisonResult` work like all other enums.
* **BREAKING**: Changed `NSDictionary` to be `Shared` by default.
* **BREAKING** (TEMPORARY): Renamed `NSEnumerator`, `NSFastEnumeration` and
  `NSFastEnumerator` until the story around them are properly figured out.
* **BREAKING**: Make `NSArray::objects_in_range` return an `Option` (it was
  unsound before).

### Fixed
* Fixed `NSZone` not specifying a `#[repr(...)]`.


## objc2::foundation 0.3.0-beta.3 - 2022-09-01

### Added
* Added `NSSet`.
* Added `NSMutableSet`.
* Added `NSMutableDictionary`.
* Added `NSNotFound`.
* Added `NSBundle`.
* Added `NSTimeInterval`.
* Added `NSString::len_utf16` and `NSAttributedString::len_utf16`.
* Added `NSString::concat` and `NSString::join_path`.
* Added `CGSize`, `CGPoint` and `CGRect` (aliases to equivalent `NS`-types
  that helps readability).

### Changed
* **BREAKING**: `NSSize::new` no longer requires it's arguments to be
  non-negative. Use `NSSize::abs` or `NSRect::standardize` if the API you're
  binding to requires a non-negative size.


## objc2::foundation 0.3.0-beta.2 - 2022-08-28

### Added
* Added `NSNumber`.
* Added `NSError`.
* Implement `UnwindSafe` and `RefUnwindSafe` for all objects.
* Implemented `IntoIterator` for references to `NSArray`, `NSMutableArray`,
  `NSData` and `NSMutableData`.
* Implemented `Extend` for `NSMutableArray`.
* Add extra `Extend<&u8>` impl for `NSMutableData`.
* Added function `NSValue::contains_encoding` for determining if the encoding
  of the `NSValue` matches the encoding of the given type.
* Added functions `get_range`, `get_point`, `get_size` and `get_rect` to
  `NSValue` to help safely returning various types it will commonly contain.
* `NSArray` and `NSMutableArray` now have sensible defaults for the ownership
  of the objects they contain.

### Changed
* **BREAKING**: Moved from external crate `objc2_foundation` into
  `objc2::foundation`.
* **BREAKING**: Made `NSValue` not generic any more. While we loose some
  type-safety from this, it makes `NSValue` much more useful in the real
  world!
* **BREAKING**: Made `NSArray::new` generic over ownership.
* **BREAKING**: Made `NSObject::is_kind_of` take a generic `T: ClassType`
  instead of a `runtime::Class`.

### Fixed
* Made `Debug` impls for all objects print something useful.

### Removed
* `NSObject::hash_code`, `NSObject::is_equal` and `NSObject::description` in
  favour of having the trait implementations `Hash`, `PartiqalEq` and `Debug`.


## objc2-foundation 0.2.0-alpha.6 - 2022-07-19

### Added
* Added `MainThreadMarker` to help with designing APIs where a method is only
  safe to call on the main thread.
* Added `NSException` object.
* Added `extern_class!` macro to help with defining interfaces to classes.
  Further changelog for this can be found in `CHANGELOG.md`.
* Added `declare_class!` macro to help with declaring custom classes.
  Further changelog for this can be found in `CHANGELOG.md`.
* Expose the `objc2` version that this uses in the crate root.
* Added `NSZone`.

### Changed
* Changed a few `Debug` impls.


## objc2-foundation 0.2.0-alpha.5 - 2022-06-13

### Added
* Objects now `Deref` to their superclasses. E.g. `NSMutableArray` derefs to
  `NSArray`, which derefs to `NSObject`, which derefs to `Object`.

  This allows more ergonomic usage.
* Implement `PartialOrd` and `Ord` for `NSString` and `NSRange`.
* Added `NSString::has_prefix` and `NSString::has_suffix`.
* Added `NSRange` methods `new`, `is_empty`, `contains` and `end`.
* Added `NSThread` object.
* Added `is_multi_threaded` and `is_main_thread` helper functions.
* Added `NSProcessInfo` object.
* Added `NSMutableData` methods `from_data`, `with_capacity` and `push`.
* Added `io::Write` and `iter::Extend` implementation for `NSMutableData`.
* Added `NSUUID` object.
* Added `NSMutableString` object.
* Added basic `NSAttributedString` object.
* Added basic `NSMutableAttributedString` object.
* Added `NSInteger` and `NSUInteger` (type aliases to `isize` and `usize`).
* Added `CGFloat`.
* Added `NSPoint`.
* Added `NSSize`.
* Added `NSRect`.
* Implement `Borrow` and `BorrowMut` for all objects.
* Implement `ToOwned` for copyable types.

### Changed
* **BREAKING**: Removed the following helper traits in favor of inherent
  methods on the objects themselves:
  - `INSMutableArray`
  - `INSArray`
  - `INSMutableData`
  - `INSData`
  - `INSDictionary`
  - `INSString`
  - `INSValue`
  - `INSObject`

  This changed because objects now deref to their superclasses.
* **BREAKING**: Relaxed a lot of bounds from `INSObject` to `Message`. At some
  point in the future a new trait will be introduced which remedies this
  change.
* **BREAKING**: Removed the `I` prefix from:
  - `INSCopying` (now `NSCopying`)
  - `INSMutableCopying` (now `NSMutableCopying`)
  - `INSFastEnumeration` (now `NSFastEnumeration`)
* **BREAKING**: Renamed `NSMutableData::append` to `extend_from_slice`.


## 0.2.0-alpha.4 - 2022-01-03

### Added
* Implement `PartialOrd` and `Ord` for `NSComparisonResult` and `NSValue`.
* Implement `fmt::Display` for `NSValue`.
* Implement `DefaultId` for relevant objects.
* Implement `AsRef` and `Index` for `NSData` and `NSMutableData`.
* Implement `AsMut` and `IndexMut` for `NSMutableData`.

### Changed
* **BREAKING**: Renamed `INSFastEnumeration::enumerator` to
  `INSFastEnumeration::iter_fast`.

### Removed
* **BREAKING**: Removed `Deref` and `DerefMut` from `NSData` and
  `NSMutableData`, since these invoke a non-trivial amount of code, and could
  lead to hard-to-diagnose performance issues.


## objc2-foundation 0.2.0-alpha.3 - 2021-12-22

### Added
* **BREAKING**: Added associated `Ownership` type to `NSCopying`.
* **BREAKING**: Added associated `Ownership` type to `INSData`.
* **BREAKING**: Added associated `Ownership` type to `INSArray`.
* Added common trait impls (`PartialEq`, `Eq`, `Hash` and `Debug`) to
  `NSValue`, `NSDictionary`, `NSArray` and `NSMutableArray`.

### Changed
* **BREAKING**: Made some creation methods a bit less generic (e.g.
  `INSDictionary::from_keys_and_objects` now always returns `Id<_, Shared>`).
* Relax bounds on generic `INSObject` impls.

### Removed
* **BREAKING**: Removed associated `Ownership` type from `INSObject`; instead,
  it is present on the types that actually need it (for example `NSCopying`).
* **BREAKING**: Removed `Sized` bound on `INSObject`.

### Fixed
* Soundness issue with `NSValue`, `NSDictionary`, `NSArray` and
  `NSMutableArray` not specifying a `#[repr(...)]`.
* **BREAKING**: `NSObject` is no longer `Send` and `Sync` (because its
  subclasses may not be).


## objc2-foundation 0.2.0-alpha.2 - 2021-11-22

### Added
* **BREAKING**: Added associated `Ownership` type to `INSObject` to specify
  whether the type can be mutated or not. `NSString` is a prime example of a
  type that you may never get a `Owned/&mut` reference to, since it is very
  easy to create two `NSString`s with the same underlying allocation.
* Added helper `is_empty` methods.
* Added `INSArray::first_mut` and `INSArray::last_mut`.

### Changed
* **BREAKING**: Renamed a lot of methods to better match Rusts naming scheme:
  - `INSArray`
    - `count` -> `len`
    - `object_at` -> `get`
    - `mut_object_at` -> `get_mut`
    - `shared_object_at` -> `get_retained`
    - `first_object` -> `first`
    - `last_object` -> `last`
    - `object_enumerator` -> `iter`
  - `INSMutableArray`
    - `add_object` -> `push`
    - `insert_object_at` -> `insert`
    - `replace_object_at` -> `replace`
    - `remove_object_at` -> `remove`
    - `remove_last_object` -> `pop`
    - `remove_all_objects` -> `clear`
  - `INSDictionary`
    - `count` -> `len`
    - `object_for` -> `get`
    - `key_enumerator` -> `iter_keys`
    - `object_enumerator` -> `iter_values`
  - `INSValue`
    - `value` -> `get`
    - `from_value` -> `new`
  - `NSComparisonResult`
    - `from_ordering` -> `from`
    - `as_ordering` -> `into`
  - `NSRange`
    - `from_range` -> `from`
    - `as_range` -> `into`
* Use `SliceId` for better performance when creating arrays and dictionaries.

### Removed
* **BREAKING**: Removed the `object_struct!` macro. It may be re-added in
  another form in the future.
* **BREAKING**: Removed `NSMutableSharedArray<T>` and `NSSharedArray<T>` type
  aliases. Use `NSMutableArray<T, Shared>` and `NSArray<T, Shared>` instead.
* **BREAKING**: Removed `Any / 'static` bound on `INSObject`. This allows
  implementing it for objects that contain lifetimes from the outer scope.

### Fixed
* **BREAKING**: Marked `INS...` traits as `unsafe` to implement.
* **BREAKING**: Removed `new` method from `INSObject` since some classes don't
  want this called. It has been re-added to other `INS...` traits on a case by
  case basis (in particular not `NSValue`).
* **BREAKING**: `INSString::as_str` now takes an a reference to
  `objc2::rc::AutoreleasePool`. This ensure that the returned `&str` is only
  used while the current autorelease pool is valid.
* Fixed `NSData::from_vec` on GNUStep.


## objc2-foundation 0.2.0-alpha.1 - 2021-10-28

### Added
* Implement new `RefEncode` trait for objects.
* Implement `Encode` for `NSComparisonResult` and `NSFastEnumerationState`.
* Implement `RefEncode` for objects and `NSFastEnumerationState`.

### Changed
* **BREAKING**: Uses `Id` from `objc2::rc` module instead of `objc_id` crate.
* **BREAKING**: `INSValue::encoding` now returns `&str` instead of `Encoding`.

### Fixed
* Use proper `#[repr(C)]` structs to represent Objective-C objects.
* `INSString::from_str` on GNUStep (`UTF8_ENCODING` was the wrong type).


## objc2-foundation 0.2.0-alpha.0 - 2021-08-29

Note: This is the version that is, as of this writing, available on the
`master` branch in the original `objc-foundation` project.

### Added
* Implement `Display` for `NSString`.
* Make `INSObject::class` faster using the `objc::class!` macro.

### Changed
* **BREAKING**: Forked the project, the crate name is now `objc2-foundation`.

### Fixed
* Fixed types in various calls to `objc::msg_send!` for better verification.


## objc-foundation [0.1.1] - 2016-06-19

### Fixed
* An issue with passing functions (instead of function pointers) in
  `INSMutableArray::sort_by`.


## objc-foundation [0.1.0] - 2016-03-20

### Changed
* Update `objc` to `v0.2`.
* Update `objc_id` to `v0.1`.


## objc-foundation [0.0.4] - 2015-12-09

### Removed
* `libc` dependency.


## objc-foundation [0.0.3] - 2015-11-07

### Added
* `object_struct!` macro.

### Changed
* `libc` version can both be `0.1` and `0.2`.


## objc-foundation [0.0.2] - 2015-09-03

### Added
* `Any` bound on `INSObject`, because of a change in `objc` `v0.1.6`.


## objc-foundation [0.0.1] - 2015-06-13

Initial release.


[0.1.1]: https://github.com/madsmtm/objc2/compare/objc-foundation-0.1.0...objc-foundation-0.1.1
[0.1.0]: https://github.com/madsmtm/objc2/compare/objc-foundation-0.0.4...objc-foundation-0.1.0
[0.0.4]: https://github.com/madsmtm/objc2/compare/objc-foundation-0.0.3...objc-foundation-0.0.4
[0.0.3]: https://github.com/madsmtm/objc2/compare/objc-foundation-0.0.2...objc-foundation-0.0.3
[0.0.2]: https://github.com/madsmtm/objc2/compare/objc-foundation-0.0.1...objc-foundation-0.0.2
[0.0.1]: https://github.com/madsmtm/objc2/releases/tag/objc-foundation-0.0.1
