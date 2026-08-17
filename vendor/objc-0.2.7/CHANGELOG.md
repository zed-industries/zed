## 0.2.7

### Fixed

* Uses of `msg_send!` will now correctly fail to compile if no return type
  can be inferred, instead of relying on an edge case of the compiler
  that will soon change and silently cause undefined behavior.

## 0.2.6

### Fixed

* Suppressed a deprecation warning in `sel!`, `msg_send!`, and `class!`.

## 0.2.5

### Added

* `autoreleasepool` returns the value returned by its body closure.

## 0.2.4

### Added

* Added an `rc` module with reference counting utilities:
  `StrongPtr`, `WeakPtr`, and `autoreleasepool`.

* Added some reference counting ABI foreign functions to the `runtime` module.

### Fixed

* Messaging nil under GNUstep now correctly returns zeroed results for all
  return types.

## 0.2.3

### Added

* Added a `class!` macro for getting statically-known classes. The result is
  non-optional (avoiding a need to unwrap) and cached so each usage will only
  look up the class once.

* Added caching to the `sel!` macro so that each usage will only register the
  selector once.

### Fixed

* Fixed the implementation of `objc::runtime` structs so there can't be unsound
  references to uninhabited types.

## 0.2.2

### Added

* Implemented `Sync` and `Send` for `Sel`.

## 0.2.1

### Added

* Added support for working with protocols with the `Protocol` struct.
  The protocols a class conforms to can be examined with the new
  `Class::adopted_protocols` and `Class::conforms_to` methods.

* Protocols can be declared using the new `ProtocolDecl` struct.

## 0.2.0

### Added

* Added verification for the types used when sending messages.
  This can be enabled for all messages with the `"verify_message"` feature,
  or you can test before sending specific messages with the
  `Message::verify_message` method. Verification errors are reported using the
  new `MessageError` struct.

* Added support for the GNUstep runtime!
  Operating systems besides OSX and iOS will fall back to the GNUstep runtime.

* Root classes can be declared by using the `ClassDecl::root` constructor.

### Changed

* C types are now used from `std::os::raw` rather than `libc`. This means
  `Encode` may not be implemented for `libc` types; switch them to the
  `std::os::raw` equivalents instead. This avoids an issue that would arise
  from simultaneously using different versions of the libc crate. 

* Dynamic messaging was moved into the `Message` trait; instead of
  `().send(obj, sel!(description))`, use
  `obj.send_message(sel!(description), ())`.

* Rearranged the parameters to `ClassDecl::new` for consistency; instead of
  `ClassDecl::new(superclass, "MyObject")`, use
  `ClassDecl::new("MyObject", superclass)`.

* Overhauled the `MethodImplementation` trait. Encodings are now accessed
  through the `MethodImplementation::Args` associated type. The `imp_for`
  method was replaced with `imp` and no longer takes a selector or returns an
  `UnequalArgsError`, although `ClassDecl::add_method` still validates the
  number of arguments.

* Updated the definition of `Imp` to not use the old dispatch prototypes.
  To invoke an `Imp`, it must first be transmuted to the correct type.

* Removed `objc_msgSend` functions from the `runtime` module; the availability
  of these functions varies and they shouldn't be called without trasmuting,
  so they are now hidden as an implementation detail of messaging.

### Fixed

* Corrected alignment of ivars in `ClassDecl`; declared classes may now have a
  smaller size. 

* With the `"exception"` or `"verify_message"` feature enabled, panics from
  `msg_send!` will now be triggered from the line and file where the macro is
  used, rather than from within the implementation of messaging.
