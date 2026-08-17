# Changelog

Notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased - YYYY-MM-DD


## 0.5.2 - 2024-05-21

### Added
* Added `Retained::autorelease_ptr`.
* Added the feature flag `"relax-sign-encoding"`, which when enabled, allows
  using e.g. `NSInteger` in places where you would otherwise have to use
  `NSUInteger`.

### Changed
* Renamed `Id` to `Retained`, to better reflect what it represents.

  The old name is kept as a soft-deprecated type-alias (will be fully
  deprecated in `v0.6.0`).

  The same is done for:
  - `rc::WeakId` to `rc::Weak`.
  - `rc::DefaultId` to `rc::DefaultRetained`.
  - `rc::IdFromIterator` to `rc::RetainedFromIterator`.
  - `rc::IdIntoIterator` to `rc::RetainedIntoIterator`.

### Deprecated
* Deprecated the `apple` Cargo feature flag, it is assumed by default on Apple
  platforms.


## 0.5.1 - 2024-04-17

### Added
* Made the following runtime methods available without the `"malloc"` feature
  flag:
  - `Method::return_type`.
  - `Method::argument_type`.
  - `AnyClass::classes`.
  - `AnyClass::instance_methods`.
  - `AnyClass::adopted_protocols`.
  - `AnyClass::instance_variables`.
  - `AnyProtocol::protocols`.
  - `AnyProtocol::adopted_protocols`.
* Added `Id::into_raw` as the oppositve of `Id::from_raw`.
* Added the following missing methods on `NSObjectProtocol`:
  - `isEqual`.
  - `hash`.
  - `isKindOfClass`.
  - `isMemberOfClass`.
  - `respondsToSelector`.
  - `conformsToProtocol`.
  - `description`.
  - `debugDescription`.
  - `isProxy`.
  - `retainCount`.
* Added `NSObject::init`.
* Added `NSObject::doesNotRecognizeSelector`.

### Changed
* Moved `ClassBuilder` and `ProtocolBuilder` from the `declare` module to the
  `runtime` module. The old locations are deprecated.
* Enabled the `"verify"` feature flag's functionality when debug assertions are
  enabled.
* Renamed `Id::new` to `Id::from_raw`. The previous name is kept as a
  deprecated alias.


## 0.5.0 - 2023-12-03

### Added
* Added the following traits to the `mutability` module (see the documentation
  for motivation and usage info):
  - `HasStableHash`.
  - `IsAllowedMutable`.
  - `IsMainThreadOnly`.
  - `CounterpartOrSelf`.
* Added new `encode` traits `EncodeReturn`, `EncodeArgument` and
  `EncodeArguments`.
* Added methods `as_ptr` and `as_mut_ptr` to `Allocated`.
* Added optimization for converting `msg_send_id![cls, alloc]` to a call to
  the faster runtime function `objc_alloc`.
* Added `DeclaredClass`, which represents classes that are declared in Rust.
* Added `Allocated::set_ivars`, which sets the instance variables of an
  object, and returns the new `rc::PartialInit`.
* Added the ability for `msg_send_id!` to call `super` methods.
* Implement `Send` and `Sync` for `ProtocolObject` if the underlying protocol
  implements it.
* Added ability to create `Send` and `Sync` versions of
  `ProtocolObject<dyn NSObjectProtocol>`.

### Changed
* **BREAKING**: Changed how instance variables work in `declare_class!`.

  Previously, instance variables had to implement `Encode`, and you had to
  initialize them properly, which was difficult to ensure.

  Now, you implement the new `DeclaredClass` trait instead, which helps to
  ensure all of this for you.

  ```rust
  // Before
  declare_class!(
      struct MyObject {
          object: IvarDrop<Id<NSObject>, "_object">,
          data: IvarDrop<Option<Box<MyData>>, "_data">,
      }

      mod ivars;

      unsafe impl ClassType for MyObject {
          type Super = NSObject;
          type Mutability = InteriorMutable;
          const NAME: &'static str = "MyObject";
      }

      unsafe impl MyObject {
          #[method(init)]
          unsafe fn init(this: *mut Self) -> Option<NonNull<Self>> {
              let this: Option<&mut Self> = msg_send![super(this), init];
              this.map(|this| {
                  Ivar::write(&mut this.object, NSObject::new());
                  Ivar::write(&mut this.data, Box::new(MyData::new()));
                  NonNull::from(this)
              })
          }
      }
  );

  extern_methods!(
      unsafe impl MyObject {
          #[method_id(new)]
          pub fn new() -> Id<Self>;
      }
  );

  fn main() {
      let obj = MyObject::new();
      println!("{:?}", obj.object);
  }

  // After
  struct MyIvars {
      object: Id<NSObject>,
      data: Option<Box<MyData>>,
  }

  declare_class!(
      struct MyObject;

      unsafe impl ClassType for MyObject {
          type Super = NSObject;
          type Mutability = InteriorMutable;
          const NAME: &'static str = "MyObject";
      }

      impl DeclaredClass for MyObject {
          type Ivars = MyIvars;
      }

      unsafe impl MyObject {
          #[method_id(init)]
          pub fn init(this: Allocated<Self>) -> Option<Id<Self>> {
              let this = this.set_ivars(MyIvars {
                  object: NSObject::new(),
                  data: MyData::new(),
              });
              unsafe { msg_send_id![super(this), init] }
          }
      }
  );

  extern_methods!(
      unsafe impl MyObject {
          #[method_id(new)]
          pub fn new() -> Id<Self>;
      }
  );

  fn main() {
      let obj = MyObject::new();
      println!("{:?}", obj.ivars().object);
  }
  ```
* **BREAKING**: `AnyClass::verify_sel` now take more well-defined types
  `EncodeArguments` and  `EncodeReturn`.
* **BREAKING**: Changed how the `mutability` traits work; these no longer have
  `ClassType` as a super trait, allowing them to work for `ProtocolObject` as
  well.

  This effectively means you can now `copy` a `ProtocolObject<dyn NSCopying>`.
* **BREAKING**: Allow implementing `DefaultId` for any type, not just those
  who are `IsAllocableAnyThread`.
* **BREAKING**: Moved the `MethodImplementation` trait from the `declare`
  module to the `runtime` module.
* **BREAKING**: Moved the `MessageReceiver` trait to the `runtime` module.
* **BREAKING**: Make the `MessageReceiver` trait no longer implemented for
  references to `Id`. Dereference the `Id` yourself.

  Note: Passing `&Id` in `msg_send!` is still supported.
* **BREAKING**: `MessageReceiver::send_message` and
  `MessageReceiver::send_super_message` now take `EncodeArguments` and return
  `EncodeReturn`, instead of internal traits.

  This is done to make `MessageReceiver` more straightforward to understand,
  although it now also has slightly less functionality than `msg_send!`.

  In particular automatic conversion of `bool` is not supported in
  `MessageReceiver`.
* Relaxed the requirements for receivers in `MethodImplementation`; now,
  anything that implements `MessageReceiver` can be used as the receiver of
  a method.
* **BREAKING**: Renamed the associated types `Ret` and `Args` on
  `MethodImplementation` to `Return` and `Arguments`.
* **BREAKING**: Make `rc::Allocated` allowed to be `NULL` internally, such
  that uses of `Option<Allocated<T>>` is now simply `Allocated<T>`.
* `AnyObject::class` now returns a `'static` reference to the class.
* Relaxed `ProtocolType` requirement on `ProtocolObject`.
* **BREAKING**: Updated `encode` types to those from `objc2-encode v4.0.0`.

### Deprecated
* Soft deprecated using `msg_send!` without a comma between arguments (i.e.
  deprecated when the `"unstable-msg-send-always-comma"` feature is enabled).

  See the following for an example of how to upgrade:
  ```rust
  // Before
  let _: NSInteger = msg_send![
      obj,
      addTrackingRect:rect
      owner:obj
      userData:ptr::null_mut::<c_void>()
      assumeInside:Bool::NO
  ];
  // After
  let _: NSInteger = msg_send![
      obj,
      addTrackingRect: rect,
      owner: obj,
      userData: ptr::null_mut::<c_void>(),
      assumeInside: false,
  ];
  ```

### Fixed
* Fixed the name of the protocol that `NSObjectProtocol` references.
* Allow cloning `Id<AnyObject>`.
* **BREAKING**: Restrict message sending to `&mut` references to things that
  implement `IsAllowedMutable`.
* Disallow the ability to use non-`Self`-like types as the receiver in
  `declare_class!`.
* Allow adding instance variables with the same name on Apple platforms.
* **BREAKING**: Make loading instance variables robust and sound in the face
  of instance variables with the same name.

  To read or write the instance variable for an object, you should now use the
  `load`, `load_ptr` and `load_mut` methods on `Ivar`, instead of the `ivar`,
  `ivar_ptr` and `ivar_mut` methods on `AnyObject`.

  This _is_ more verbose, but it also ensures that the class for the instance
  variable you're loading is the same as the one the instance variable you
  want to access is defined on.

  ```rust
  // Before
  let number = unsafe { *obj.ivar::<u32>("number") };

  // After
  let ivar = cls.instance_variable("number").unwrap();
  let number = unsafe { *ivar.load::<u32>(&obj) };
  ```
* Implement `RefEncode` normally for `c_void`. This makes `AtomicPtr<c_void>`
  implement `Encode`.

### Removed
* **BREAKING**: Removed `ProtocolType` implementation for `NSObject`.
  Use the more precise `NSObjectProtocol` trait instead!
* **BREAKING**: Removed the `MessageArguments` trait.
* **BREAKING**: Removed the following items from the `declare` module: `Ivar`,
  `IvarEncode`, `IvarBool`, `IvarDrop`, `IvarType` and `InnerIvarType`.

  Ivar functionality is available in a different form now, see above under
  "Changed".
* **BREAKING**: Removed `ClassBuilder::add_static_ivar`.


## 0.4.1 - 2023-07-31

### Added
* Allow using `MainThreadMarker` in `extern_methods!`.
* Added the feature flag `"relax-void-encoding"`, which when enabled, allows
  using `*mut c_void` in a few places where you would otherwise have to
  specify the encoding precisely.

### Changed
* Renamed `runtime` types:
  - `Object` to `AnyObject`.
  - `Class` to `AnyClass`.
  - `Protocol` to `AnyProtocol`.

  To better fit with Swift's naming scheme. The types are still available
  under the old names as deprecated aliases.

### Fixed
* **BREAKING**: Updated `encode` types to those from `objc2-encode v3.0.0`.

  This is technically a breaking change, but it should allow this crate to be
  compiled together with pre-release versions of it, meaning that in practice
  strictly more code out there will compile because of this. Hence it was
  deemed the better trade-off.


## 0.4.0 - 2023-06-20

### Added
* Added `objc2::rc::autoreleasepool_leaking`, and improve performance of
  objects `Debug` impls.
* **BREAKING**: Added associated type `ClassType::Mutability`, which replaces
  the ownership type on `Id`, and must be specified for all class types.

  An example:
  ```rust
  // Before
  use objc2::runtime::NSObject;
  use objc2::{declare_class, ClassType};

  declare_class!(
      struct MyDelegate;

      unsafe impl ClassType for MyDelegate {
          type Super = NSObject;
      }

      // ... methods
  );

  // After
  use objc2::runtime::NSObject;
  use objc2::mutability::InteriorMutable;
  use objc2::{declare_class, ClassType};

  declare_class!(
      struct MyDelegate;

      unsafe impl ClassType for MyDelegate {
          type Super = NSObject;
          type Mutability = InteriorMutable; // Added
      }

      // ... methods
  );
  ```
* Added `ClassType::retain`, which is a safe way to go from a reference `&T`
  to an `Id<T>`.
* Added `mutability` module, containing various types that can be specified
  for the above.
* Preliminary support for specifying `where` bounds on methods inside
  `extern_protocol!` and `extern_methods!`.
* Allow arbitary expressions in `const NAME` in `extern_class!`,
  `extern_protocol!` and `declare_class!`.
* Added `rc::IdIntoIterator` helper trait and forwarding `IntoIterator`
  implementations for `rc::Id`.
* Added `rc::IdFromIterator` helper trait for implementing `IntoIterator`
  for `rc::Id`.
* Added `Display` impl for `runtime::Class`, `runtime::Sel` and
  `runtime::Protocol`.
* Added `Debug` impl for `runtime::Method` and `runtime::Ivar`.
* Added `Method::set_implementation`.
* Added `Method::exchange_implementation`.
* Added `Object::set_class`.

### Changed
* **BREAKING**: `objc2::rc::AutoreleasePool` is now a zero-sized `Copy` type
  with a lifetime parameter, instead of the lifetime parameter being the
  reference it was behind.
* **BREAKING**: Made `Id::autorelease` and `Id::autorelease_return` be
  associated functions instead of methods. This means they now have to be
  called as `Id::autorelease(obj, pool)` instead of `obj.autorelease(pool)`.

  Additionally, rename the mutable version to `Id::autorelease_mut`.
* **BREAKING**: Moved `VerificationError`, `ProtocolObject` and
  `ImplementedBy` into the `runtime` module.
* Relaxed a `fmt::Debug` bound on `WeakId`'s own `fmt::Debug` impl.
* Changed `Debug` impl for `runtime::Class`, `runtime::Sel` and
  `runtime::Protocol` to give more information.
* **BREAKING**: Updated `encode` module to `objc2-encode v2.0.0`.

### Fixed
* Fixed using autorelease pools on 32bit macOS and older macOS versions.
* Fixed memory leaks in and improved performance of `exception::catch`.

### Removed
* **BREAKING**: Removed `rc::SliceId`, since it is implementable outside
  `objc2` from the layout guarantees of `rc::Id`.
* **BREAKING**: Removed `Ownership` type parameter from `Id`, as well as
  `rc::Ownership`, `rc::Owned`, `rc::Shared`, `Id::from_shared` and
  `Id::into_shared`. This functionality has been moved from being at the
  "usage-level", to being moved to the "type-level" in the associated type
  `ClassType::Mutability`.

  While being slightly more restrictive, it should vastly help you avoid
  making mistakes around mutability (e.g. it is usually a mistake to make a
  mutable reference `&mut` to an Objective-C object).

  An example:
  ```rust
  // Before
  use objc2::rc::{Id, Shared};
  use objc2::runtime::NSObject;
  use objc2::msg_send_id;

  let obj: Id<NSObject, Shared> = unsafe { msg_send_id![NSObject::class(), new] };

  // After
  use objc2::rc::Id;
  use objc2::runtime::NSObject;
  use objc2::msg_send_id;

  let obj: Id<NSObject> = unsafe { msg_send_id![NSObject::class(), new] };
  ```
* **BREAKING**: Removed `impl<T> TryFrom<WeakId<T>> for Id<T>` impl since it
  did not have a proper error type, making it less useful than `WeakId::load`.
* **BREAKING**: Removed forwarding `Iterator` implementation for `Id`, since
  it conflicts with the `IntoIterator` implementation that it now has instead.


## 0.3.0-beta.5 - 2023-02-07

### Added
* Support `#[cfg(...)]` attributes in `extern_class!` macro.
* Added support for selectors with multiple colons like `abc::` in the `sel!`,
  `extern_class!`, `extern_protocol!` and `declare_class!` macros.
* Added ability to use `#[method_id(mySelector:)]` inside `declare_class!`,
  like you would do in `extern_methods!`.
* Added 16-fold impls for `EncodeArguments`, `MessageArguments`, and `MethodImplementation`.
* Added `NSObjectProtocol` trait for allowing `ProtocolObject` to implement
  `Debug`, `Hash`, `PartialEq` and `Eq`.
* Support running `Drop` impls on `dealloc` in `declare_class!`.
* Added `declare::IvarEncode` and `declare::IvarBool` types.
* **BREAKING**: Moved the `objc2_encode` traits to `objc2::encode`.

  This includes removing the `EncodeConvert` and `EncodeArguments` traits.
* Added support for out-parameters like `&mut Id<_, _>` in `msg_send!`,
  `msg_send_id!` and `extern_methods!`.

### Changed
* **BREAKING**: Using the automatic `NSError**`-to-`Result` functionality in
  `extern_methods!` now requires a trailing underscore (so now it's
  `#[method(myMethod:error:_)]` instead of `#[method(myMethod:error:)]`).
* **BREAKING**: Fundamentally changed how protocols work. Instead of being
  structs with inherent methods, they're now traits. This means that you can
  use their methods more naturally from your Objective-C objects.

  An example:
  ```rust
  // Before
  extern_protocol!(
      struct MyProtocol;

      unsafe impl ProtocolType for MyProtocol {
          #[method(myMethod)]
          fn myMethod(&self);
      }
  );

  let obj: &SomeObjectThatImplementsTheProtocol = ...;
  let proto: &MyProtocol = obj.as_protocol();
  proto.myMethod();

  // After
  extern_protocol!(
      unsafe trait MyProtocol {
          #[method(myMethod)]
          fn myMethod(&self);
      }

      unsafe impl ProtocolType for dyn MyProtocol {}
  );

  let obj: &SomeObjectThatImplementsTheProtocol = ...;
  obj.myMethod();
  // Or
  let proto: &ProtocolObject<dyn MyProtocol> = ProtocolObject::from_ref(obj);
  proto.myMethod();
  ```

  The `ConformsTo` trait has similarly been removed, and the `ImplementedBy`
  trait and `ProtocolObject` struct has been introduced instead.
* **BREAKING**: Moved `NSObject::is_kind_of` to the new `NSObjectProtocol`.
* **BREAKING**: Removed support for custom `dealloc` impls in
  `declare_class!`. Implement `Drop` for the type instead.
* **BREAKING**: Changed how the `declare_class!` macro works.

  Now, you must explicitly specify the "kind" of ivar you want, as well as the
  ivar name. Additionally, the class name must be explicitly specified.

  This change is done to make it easier to see what's going on beneath the
  hood (the name will be available in the final binary, which is important to
  be aware of)!

  An example:
  ```rust
  // Before
  declare_class!(
      struct MyClass {
          pub ivar: u8,
          another_ivar: bool,
          box_ivar: IvarDrop<Box<i32>>,
      }

      unsafe impl ClassType for MyClass {
          type Super = NSObject;
      }
  );

  // After
  declare_class!(
      struct MyClass {
          pub ivar: IvarEncode<u8, "_ivar">,
          another_ivar: IvarBool<"_another_ivar">,
          box_ivar: IvarDrop<Box<i32>, "_box_ivar">,
      }

      // Helper types for ivars will be put in here
      mod ivars;

      unsafe impl ClassType for MyClass {
          type Super = NSObject;
          const NAME: &'static str = "MyClass";
      }
  );

  ```
* Updated `ffi` module to `objc-sys v0.3.0`.
* **BREAKING**: Updated `encode` module to `objc2-encode v2.0.0-pre.4`.

### Fixed
* Allow empty structs in `declare_class!` macro.
* Allow using `extern_methods!` without the `ClassType` trait in scope.
* Fixed a few small issues with `declare_class!`.
* Fixed `()` being possible in argument position in `msg_send!`.


## 0.3.0-beta.4 - 2022-12-24

### Added
* Allow directly specifying class name in `extern_class!` macro.
* Added `ClassType::alloc`.

  This means you can now simplify your code as follows:
  ```rust
  // Before
  let obj: Id<NSObject, Shared> = unsafe {
      msg_send_id![msg_send_id![NSObject::class(), alloc], init]
  };

  // After
  let obj: Id<NSObject, Shared> = unsafe {
      msg_send_id![NSObject::alloc(), init]
  };
  ```
* Added `Class::class_method`.
* Added the ability to specify `error: _`, `somethingReturningError: _` and
  so on at the end of `msg_send!`/`msg_send_id!`, and have it automatically
  return a `Result<..., Id<NSError, Shared>>`.
* Added the ability to specify an extra parameter at the end of the selector
  in methods declared with `extern_methods!`, and let that be the `NSError**`
  parameter.
* Added `#[method_id(...)]` attribute to `extern_methods!`.
* Added `"verify"` feature as a replacement for the `"verify_message"`
  feature.
* Added `extern_protocol!` macro and `ProtocolType` trait.
* Added `ConformsTo` trait for marking that a type conforms to a specific
  protocol.
* Added `Encode` impl for `Option<Sel>`.
* Added `objc2::runtime::NSObject` - before, this was only available under
  `objc2::foundation::NSObject`.
* Added `objc2::runtime::NSZone` - before, this was only available under
  `objc2::foundation::NSZone`.

### Changed
* Allow other types than `&Class` as the receiver in `msg_send_id!` methods
  of the `new` family.
* **BREAKING**: Changed the `Allocated` struct to be used as `Allocated<T>`
  instead of `Id<Allocated<T>, O>`.
* Verify the message signature of overriden methods when declaring classes if
  the `verify` feature is enabled.
* Verify in `declare_class!` that protocols are implemented correctly.
* **BREAKING**: Changed the name of the attribute macro in `extern_methods`
  from `#[sel(...)]` to `#[method(...)]`.
* **BREAKING**: Changed `extern_methods!` and `declare_class!` such that
  associated functions whoose first parameter is called `this`, is treated as
  instance methods instead of class methods.
* **BREAKING**: Message verification is now enabled by default. Your message
  sends might panic with `debug_assertions` enabled if they are detected to
  be invalid. Test your code to see if that is the case!
* **BREAKING**: `declare_class!` uses `ConformsTo<...>` instead of the
  temporary `Protocol<...>` syntax.
* Require implementors of `Message` to support weak references.
* **BREAKING**: Moved `objc2::foundation` into `icrate::Foundation`.
* **BREAKING**: Moved `objc2::ns_string` into `icrate::ns_string`.
* Updated `ffi` module to `objc-sys v0.2.0-beta.3`.
* **BREAKING**: Updated `encode` module `objc2-encode v2.0.0-pre.3`.

### Fixed
* Fixed duplicate selector extraction in `extern_methods!`.
* Fixed using `deprecated` attributes in `declare_class!`.
* Fixed `cfg` attributes on methods and implementations in `declare_class!`.

### Removed
* **BREAKING**: Removed `"verify_message"` feature. It has been mostly
  replaced by `debug_assertions` and the `"verify"` feature.


## 0.3.0-beta.3 - 2022-09-01

### Added
* Added `Ivar::write`, `Ivar::as_ptr` and `Ivar::as_mut_ptr` for safely
  querying and modifying instance variables inside `init` methods.
* Added `IvarDrop<T>` to allow storing complex `Drop` values in ivars
  (currently `rc::Id<T, O>`, `Box<T>`, `Option<rc::Id<T, O>>` or
  `Option<Box<T>>`).
* **BREAKING**: Added required `ClassType::NAME` constant for statically
  determining the name of a specific class.
* Allow directly specifying class name in `declare_class!` macro.

### Removed
* **BREAKING**: `MaybeUninit` no longer implements `IvarType` directly; use
  `Ivar::write` instead.


## 0.3.0-beta.2 - 2022-08-28

### Added
* Added the `"unstable-static-class"` and `"unstable-static-class-inlined"`
  feature flags to make the `class!` macro zero cost.
* Moved the external crate `objc2_foundation` into `objc2::foundation` under
  (default) feature flag `"foundation"`.
* Added `declare_class!`, `extern_class!` and `ns_string!` macros from
  `objc2-foundation`.
* Added helper method `ClassBuilder::add_static_ivar`.
* **BREAKING**: Added `ClassType` trait, and moved the associated `class`
  methods that `extern_class!` and `declare_class!` generated to that. This
  means you'll have to `use objc2::ClassType` whenever you want to use e.g.
  `NSData::class()`.
* Added `Id::into_super`.
* Added `extern_methods!` macro.
* Added ability to call `msg_send![super(obj), ...]` without explicitly
  specifying the superclass.
* Added automatic conversion of `bool` to/from the Objective-C `BOOL` in
  `msg_send!`, `msg_send_id!`, `extern_methods!` and `declare_class!`.

  Example:
  ```rust
  // Before
  use objc2::{msg_send, msg_send_bool};
  use objc2::rc::{Id, Shared};
  use objc2::runtime::{Bool, Object};

  let obj: Id<Object, Shared>;
  let _: () = unsafe { msg_send![&obj, setArg: Bool::YES] };
  let is_equal = unsafe { msg_send_bool![&obj, isEqual: &*obj] };

  // After
  use objc2::msg_send;
  use objc2::rc::{Id, Shared};
  use objc2::runtime::Object;

  let obj: Id<Object, Shared>;
  let _: () = unsafe { msg_send![&obj, setArg: true] };
  let is_equal: bool = unsafe { msg_send![&obj, isEqual: &*obj] };
  ```

### Changed
* **BREAKING**: Change syntax in `extern_class!` macro to be more Rust-like.
* **BREAKING**: Change syntax in `declare_class!` macro to be more Rust-like.
* **BREAKING**: Renamed `Id::from_owned` to `Id::into_shared`.
* **BREAKING**: The return type of `msg_send_id!` is now more generic; it can
  now either be `Option<Id<_, _>>` or `Id<_, _>` (if the latter, it'll panic
  if the method returned `NULL`).

  Example:
  ```rust
  // Before
  let obj: Id<Object, Shared> = unsafe {
      msg_send_id![msg_send_id![class!(MyObject), alloc], init].unwrap()
  };

  // After
  let obj: Id<Object, Shared> = unsafe {
      msg_send_id![msg_send_id![class!(MyObject), alloc], init]
  };
  ```
* Updated `ffi` module to `objc-sys v0.2.0-beta.2`.
* **BREAKING**: Updated `encode` module `objc2-encode v2.0.0-pre.2`.

  In particular, `Encoding` no longer has a lifetime parameter:
  ```rust
  // Before
  #[repr(C)]
  pub struct NSRange {
      pub location: usize,
      pub length: usize,
  }
  unsafe impl Encode for NSRange {
      const ENCODING: Encoding<'static> = Encoding::Struct(
          "_NSRange", // This is how the struct is defined in C header files
          &[usize::ENCODING, usize::ENCODING]
      );
  }
  unsafe impl RefEncode for NSRange {
      const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
  }

  // After
  #[repr(C)]
  pub struct NSRange {
      pub location: usize,
      pub length: usize,
  }
  unsafe impl Encode for NSRange {
      const ENCODING: Encoding = Encoding::Struct(
          "_NSRange", // This is how the struct is defined in C header files
          &[usize::ENCODING, usize::ENCODING]
      );
  }
  unsafe impl RefEncode for NSRange {
      const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
  }
  ```

### Deprecated
* Depreacted `msg_send_bool!` in favour of new functionality on `msg_send!`
  that allows seamlessly handling `bool`.


## 0.3.0-beta.1 - 2022-07-19

### Added
* Added `msg_send_id!` to help with following Objective-C's memory management
  rules. **It is highly recommended that you use this instead of doing memory
  management yourself!**

  Example:
  ```rust
  // Before
  let obj: Id<Object, Shared> = unsafe {
      let obj: *mut Object = msg_send![class!(MyObject), alloc];
      let obj: *mut Object = msg_send![obj, init];
      Id::new(obj).unwrap()
  };

  // After
  let obj: Id<Object, Shared> = unsafe {
      msg_send_id![msg_send_id![class!(MyObject), alloc], init].unwrap()
  };
  ```
* Added the `"unstable-static-sel"` and `"unstable-static-sel-inlined"`
  feature flags to make the `sel!` macro (and by extension, the `msg_send!`
  macros) faster.
* Added `"unstable-c-unwind"` feature.
* Added unsafe function `Id::cast` for converting between different types of
  objects.
* Added `Object::ivar_ptr` to allow direct access to instance variables
  through `&Object`.
* Added `VerificationError` as more specific return type from
  `Class::verify_sel`.
* Added `rc::Allocated` struct which is used within `msg_send_id!`.
* Added `Class::responds_to`.
* Added `exception::Exception` object to improve error messages from caught
  exceptions.
* Added `declare::Ivar<T>` helper struct. This is useful for building safe
  abstractions that access instance variables.
* Added `Id::from_owned` helper function.

### Changed
* **BREAKING**: `Sel` is now required to be non-null, which means that you
  have to ensure that any selectors you receive from method calls are
  non-null before using them.
* **BREAKING**: `ClassBuilder::root` is now generic over the function pointer,
  meaning you will have to coerce initializer functions to pointers like in
  `ClassBuilder::add_method` before you can use it.
* **BREAKING**: Moved `MessageReceiver::verify_message` to `Class::verify_sel`
  and changed return type.
* Improved debug output with `verify_message` feature enabled.
* **BREAKING**: Changed `MessageReceiver::send_message` to panic instead of
  returning an error.
* **BREAKING**: Renamed `catch_all` feature to `catch-all`.
* **BREAKING**: Made passing the function pointer argument to
  `ClassBuilder::add_method`, `ClassBuilder::add_class_method` and similar
  more ergonomic.

  Let's say you have the following code:
  ```rust
  // Before
  let init: extern "C" fn(&mut Object, Sel) -> *mut Object = init;
  builder.add_method(sel!(init), init);
  ```

  Unfortunately, you will now encounter a very confusing error:
  ```text
    |
  2 | builder.add_method(sel!(init), init);
    |         ^^^^^^^^^^ implementation of `MethodImplementation` is not general enough
    |
     = note: `MethodImplementation` would have to be implemented for the type `for<'r> extern "C" fn(&'r mut Object, Sel) -> *mut Object`
     = note: ...but `MethodImplementation` is actually implemented for the type `extern "C" fn(&'0 mut Object, Sel) -> *mut Object`, for some specific lifetime `'0`
  ```

  To fix this, let the compiler infer the argument and return types:
  ```rust
  // After
  let init: extern "C" fn(_, _) -> _ = init;
  builder.add_method(sel!(init), init);
  ```
* Updated `ffi` module to `objc-sys v0.2.0-beta.1`.
* **BREAKING**: Updated `encode` module `objc2-encode v2.0.0-pre.1`.

### Fixed
* **BREAKING**: Disallow throwing `nil` exceptions in `exception::throw`.

### Removed
* **BREAKING**: Removed the `Sel::from_ptr` method.
* **BREAKING**: Removed `MessageError`.


## 0.3.0-beta.0 - 2022-06-13

### Added
* Added deprecated `Object::get_ivar` and `Object::get_mut_ivar` to make
  upgrading easier.
* Allow using `From`/`TryFrom` to convert between `rc::Id` and `rc::WeakId`.
* Added `Bool::as_bool` (more descriptive name than `Bool::is_true`).
* Added convenience method `Id::as_ptr` and `Id::as_mut_ptr`.
* The `objc2-encode` dependency is now exposed as `objc2::encode`.
* Added `Id::retain_autoreleased` to allow following Cocoas memory management
  rules more efficiently.
* Consistently allow trailing commas in `msg_send!`.
* Added `msg_send_bool!`, a less error-prone version of `msg_send!` for
  Objective-C methods that return `BOOL`.
* Implemented `MethodImplementation` for `unsafe` function pointers.

### Changed
* **BREAKING**: Changed signature of `Id::new` and `Id::retain` from
  `fn(NonNull<T>) -> Id<T>` to `fn(*mut T) -> Option<Id<T>>`.

  Concretely, you will have to change your code as follows.
  ```rust
  // Before
  let obj: *mut Object = unsafe { msg_send![class!(NSObject), new] };
  let obj = NonNull::new(obj).expect("Failed to allocate object.");
  let obj = unsafe { Id::new(obj) };
  // After
  let obj: *mut Object = unsafe { msg_send![class!(NSObject), new] };
  let obj = unsafe { Id::new(obj) }.expect("Failed to allocate object.");
  ```
* Allow specifying any receiver `T: Message` for methods added with
  `ClassBuilder::add_method`.
* Renamed `ClassDecl` and `ProtocolDecl` to `ClassBuilder` and
  `ProtocolBuilder`. The old names are kept as deprecated aliases.
* **BREAKING**: Changed how `msg_send!` works wrt. capturing its arguments.

  This will require changes to your code wherever you used `Id`, for example:
  ```rust
  // Before
  let obj: Id<Object, Owned> = ...;
  let p: i32 = unsafe { msg_send![obj, parameter] };
  let _: () = unsafe { msg_send![obj, setParameter: p + 1] };
  // After
  let mut obj: Id<Object, Owned> = ...;
  let p: i32 = unsafe { msg_send![&obj, parameter] };
  let _: () = unsafe { msg_send![&mut obj, setParameter: p + 1] };
  ```

  Notice that we now clearly pass `obj` by reference, and therein also
  communicate the mutability of the object (in the first case, immutable, and
  in the second, mutable).

  If you previously used `*mut Object` or `&Object` as the receiver, message
  sending should work exactly as before.
* **BREAKING**: `Class` no longer implements `Message` (but it can still be
  used as the receiver in `msg_send!`, so this is unlikely to break anything
  in practice).
* **BREAKING**: Sealed the `MethodImplementation` trait, and made its `imp`
  method privat.
* **BREAKING**: Updated `ffi` module to `objc-sys v0.2.0-beta.0`.
* **BREAKING**: Updated `objc2-encode` (`Encoding`, `Encode`, `RefEncode` and
  `EncodeArguments`) to `v2.0.0-pre.0`.

### Fixed
* Properly sealed the `MessageArguments` trait (it already had a hidden
  method, so this is not really a breaking change).

### Removed
* **BREAKING**: `ManuallyDrop` no longer implements `Message` directly.
* **BREAKING**: `MessageReceiver::as_raw_receiver` is no longer public.


## 0.3.0-alpha.6 - 2022-01-03

### Added
* Implement `Hash` for `Sel`, `Ivar`, `Class`, `Method` and `MessageError`.
* Implement `PartialEq` and `Eq` for `Ivar`, `Method` and `MessageError`.
* Implement `fmt::Pointer` for `Sel` and `rc::AutoreleasePool`.
* Implement `fmt::Debug` for `ClassDecl`, `ProtocolDecl` and `rc::AutoreleasePool`.

### Changed
* **BREAKING**: Renamed:
  - `Object::get_ivar` -> `Object::ivar`
  - `Object::get_mut_ivar` -> `Object::ivar_mut`
* Vastly improved documentation.
* **BREAKING**: Updated `ffi` module to `objc-sys v0.2.0-alpha.1`.
* **BREAKING**: Updated `objc2-encode` (`Encoding`, `Encode`, `RefEncode` and
  `EncodeArguments`) to `v2.0.0-beta.2`.


## 0.3.0-alpha.5 - 2021-12-22

### Added
* Export `objc-sys` as `ffi` module.
* Added common trait impls on `rc::Owned` and `rc::Shared` (useful in generic
  contexts).
* Implement `RefEncode` for `runtime::Protocol`.
* Added `Message` and `MessageReceiver` implementation for `ManuallyDrop<T>`
  (where `T` is appropriately bound). This allows patterns like:
  ```rust
  let obj = Id::new(msg_send![class!(MyObject), alloc]);
  let obj = ManuallyDrop::new(obj);
  // `init` takes ownership and possibly returns a new object.
  let obj = Id::new(msg_send![obj, init]);
  ```
* New cargo feature `"malloc"`, which allows cutting down on dependencies,
  most crates don't need the introspection features that this provides.

### Changed
* Deprecated `runtime::BOOL`, `runtime::YES` and `runtime::NO`. Use the
  newtype `Bool` instead, or low-level `ffi::BOOL`, `ffi::YES` and `ffi::NO`.
* **BREAKING**: The following methods now require the new `"malloc"` feature
  flag to be enabled:
  - `MessageReceiver::verify_message` (temporarily)
  - `Method::return_type`
  - `Method::argument_type`
  - `Class::classes`
  - `Class::instance_methods`
  - `Class::adopted_protocols`
  - `Class::instance_variables`
  - `Protocol::protocols`
  - `Protocol::adopted_protocols`
* Relaxed `Sized` bound on `rc::Id` and `rc::WeakId` to prepare for
  `extern type` support.
* **BREAKING**: Relaxed `Sized` bound on `rc::SliceId` and `rc::DefaultId`.
* **BREAKING**: Updated `objc-sys` to `v0.2.0-alpha.0`.
* Updated `objc2-encode` (`Encoding`, `Encode`, `RefEncode` and
  `EncodeArguments`) to `v2.0.0-beta.1`.

### Removed
* **BREAKING**: Removed the raw FFI functions from the `runtime` module. These
  are available in the new `ffi` module instead.

### Fixed
* An issue with inlining various `rc` methods.
* Most types (e.g. `Class` and `Method`) are now `Send`, `Sync`, `UnwindSafe`
  and `RefUnwindSafe` again.
  Notable exception is `Object`, because that depends on the specific
  subclass.


## 0.3.0-alpha.4 - 2021-11-22

Note: To use this version, specify `objc2-encode = "=2.0.0-beta.0"` in your
`Cargo.toml` as well.

### Added
* **BREAKING**: GNUStep users must depend on, and specify the appropriate
  feature flag on `objc-sys` for the version they're using.
* Moved `objc_exception` crate into `exception` module (under feature flag).
* Added support for `_Complex` types.
* Added `rc::SliceId`, `rc::SliceIdMut` and `rc::DefaultId` helper traits for
  extra functionality on `rc::Id`.

### Changed
* **BREAKING**: The `exception` feature now only enables the `exception`
  module, for general use. Use the new `catch_all` feature to wrap all message
  sends in a `@try/@catch`.
* **BREAKING**: Updated `objc-sys` to `v0.1.0`.
* **BREAKING**: Updated `objc2-encode` (`Encoding`, `Encode`, `RefEncode` and
  `EncodeArguments`) to `v2.0.0-beta.0`.


## 0.3.0-alpha.3 - 2021-09-05

Note: To use this version, specify `objc2-encode = "=2.0.0-alpha.1"` in your
`Cargo.toml` as well.

### Added
* Now uses the `objc-sys` (`v0.0.1`) crate for possibly better
  interoperability with other crates that link to `libobjc`.
* Added newtype `runtime::Bool` to fix soundness issues with using
  `runtime::BOOL` or `bool`.
* Moved `objc_id` crate into `rc` module. Notable changes:
  - Vastly improved documentation
  - Added `Id::autorelease`
  - Added `Id::from_shared`
  - Added a lot of forwarding implementations on `Id` for easier use
  - `Id` and `WeakId` are now able to use the null-pointer optimization
  - **BREAKING**: Added `T: Message` bounds on `Id`
  - **BREAKING**: Remove `ShareId` type alias
  - **BREAKING**: `Id` no longer have a default `Ownership`, you must specify
    it everywhere as either `Id<T, Shared>` or `Id<T, Owned>`
  - **BREAKING**: Sealed the `Ownership` trait
  - **BREAKING**: Renamed `Id::from_ptr` to `Id::retain`
  - **BREAKING**: Renamed `Id::from_retained_ptr` to `Id::new`
  - **BREAKING**: Changed `Id::share` to a `From` implementation (usage of
    `obj.share()` can be changed to `obj.into()`)
  - **BREAKING**: Fixed soundness issues with missing `Send` and `Sync` bounds
    on `Id` and `WeakId`
* Added sealed (for now) trait `MessageReceiver` to specify types that can
  be used as the receiver of a message (instead of only allowing pointer
  types).
* Add `MessageReceiver::send_super_message` method for dynamic selectors.

### Changed
* **BREAKING**: Change types of parameters to FFI functions exported in the
  `runtime` module.
* **BREAKING**: Most types are now `!UnwindSafe`, to discourage trying to use
  them after an unwind. This restriction may be lifted in the future.
* **BREAKING**: Most types are now `!Send` and `!Sync`. This was an oversight
  that is fixed in a later version.
* A lot of smaller things.
* **BREAKING**: Dynamic message sending with `Message::send_message` is moved
  to `MessageReceiver`.
* **BREAKING** Make `MessageArguments` a subtrait of `EncodeArguments`.
* Allow an optional comma after each argument to `msg_send!`.

### Removed
* **BREAKING**: Removed `rc::StrongPtr`. Use `Option<rc::Id<Object, Shared>>`
  instead (beware: This has stronger safety invariants!).
* **BREAKING**: Removed `rc::WeakPtr`. Use `rc::WeakId<Object>` instead.

### Fixed
* **BREAKING**: Stop unsafely dereferencing `msg_send!`s first argument. The
  `MessageReceiver` trait was introduced to avoid most breakage from this
  change.


## 0.3.0-alpha.2 - 2021-09-05

### Added
* Added `rc::AutoreleasePool` and `rc::AutoreleaseSafe` to make accessing
  autoreleased objects safe, by binding references to it using the
  `ptr_as_ref` and `ptr_as_mut` methods.

### Changed
* **BREAKING**: The closure in `rc::autoreleasepool` now takes an argument
  `&rc::AutoreleasePool`. This reference can be given to functions like
  `INSString::as_str` so that it knows which lifetime to bound the returned
  `&str` with.

  ```rust
  // Before
  autoreleasepool(|| {
      // Some code that autoreleases objects
  });

  // After
  autoreleasepool(|_pool| {
      // Some code that autoreleases objects
  });
  ```

### Fixed
* The encoding of `BOOL` on `GNUStep`.


## 0.3.0-alpha.1 - 2021-09-02

### Added
* More documentation of safety requirements, and in general.

### Changed
* **BREAKING**: Change `objc-encode` dependency to `objc2-encode` version
  `2.0.0-alpha.1`, and re-export the new `RefEncode` trait from that.
* **BREAKING**: Require that the receiver, arguments and return types of
  messages always implement `Encode`. This helps ensuring that only types made
  to go across the FFI boundary (`repr(C)`, ...) may. These requirements were
  already present when the `verify_message` feature was enabled.

  This is a very _disruptive change_, since libraries are now required to
  implement `Encode` and `RefEncode` for all types intended to go across the
  FFI-boundary to Objective-C. The change is justified because it helps
  ensuring that users only pass valid types to `msg_send!` (for example, this
  prevents users from accidentally passing `Drop` types to `msg_send`).

  See the following examples for how to implement these traits, and otherwise
  refer to the documentation of `objc2-encode` (`v2.0.0-alpha.1` or above).
  ```rust
  use objc2::{Encode, Encoding, RefEncode};

  /// Example struct.
  #[repr(C)]
  pub struct NSRange {
      pub location: usize,
      pub length: usize,
  }
  unsafe impl Encode for NSRange {
      const ENCODING: Encoding<'static> = Encoding::Struct(
          "_NSRange", // This is how the struct is defined in C header files
          &[usize::ENCODING, usize::ENCODING]
      );
  }
  unsafe impl RefEncode for NSRange {
      const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
  }

  /// Example object.
  #[repr(C)]
  pub struct __CFString(c_void);

  pub type CFStringRef = *const __CFString;

  unsafe impl RefEncode for __CFString {
      const ENCODING_REF: Encoding<'static> = Encoding::Object;
  }
  ```
* Temporarily disabled iOS tests.

### Fixed
* Statically find the correct `objc_msgSend[_X]` function to use based on the
  `Encode` implementation of the return type. This fixes using functions that
  return e.g. `type CGFloat = f32 / f64;`.
* Documentation links.


## 0.3.0-alpha.0 - 2021-08-29

Note: This is the version that is, as of this writing, available on the
`master` branch in the original `objc` project.

### Added
* Improve macro hygiene.
  ```rust
  // You can now do
  use objc2::{sel, class, msg_send};
  // Instead of
  #[macro_use]
  extern crate objc2;
  ```
* Update to Rust 2018.
* Other internal improvements.

### Changed
* **BREAKING**: Forked the project, so the crate name is now `objc2`.
* **BREAKING**: Updated encoding utilities to use `objc-encode`. See that for
  how to use the updated type `Encoding` and trait `Encode`.

  In short, you will likely need to change your implementations of `Encode`
  like this:
  ```rust
  use objc2::{Encode, Encoding};

  pub type CGFloat = ...; // Varies based on target_pointer_width

  #[repr(C)]
  pub struct NSPoint {
      pub x: CGFloat,
      pub y: CGFloat,
  }

  // Before
  unsafe impl Encode for NSPoint {
      fn encode() -> Encoding {
          let encoding = format!(
              "{{CGPoint={}{}}}",
              CGFloat::encode().as_str(),
              CGFloat::encode().as_str(),
          );
          unsafe { Encoding::from_str(&encoding) }
      }
  }

  // After
  unsafe impl Encode for NSPoint {
      const ENCODING: Encoding<'static> = Encoding::Struct(
          "CGPoint",
          &[CGFloat::ENCODING, CGFloat::ENCODING]
      );
  }
  ```
* **BREAKING**: Updated public dependency `malloc_buf` to `1.0`.
* **BREAKING**: `Method::return_type` and `Method::argument_type` now return
  `Malloc<str>` instead of `Encoding`.
* **BREAKING**: `Ivar::type_encoding` now return `&str` instead of `Encoding`.

### Removed
* **BREAKING**: Removed hidden `sel_impl!` macro.


## [0.2.7] (`objc` crate) - 2019-10-19

### Fixed
* **BREAKING**: Uses of `msg_send!` will now correctly fail to compile if no
  return type can be inferred, instead of relying on an edge case of the
  compiler that will soon change and silently cause undefined behavior.


## [0.2.6] (`objc` crate) - 2019-03-25

### Fixed
* Suppressed a deprecation warning in `sel!`, `msg_send!`, and `class!`.


## [0.2.5] (`objc` crate) - 2018-07-24

### Added
* **BREAKING**: `autoreleasepool` returns the value returned by its body
  closure.


## [0.2.4] (`objc` crate) - 2018-07-22

### Added
* Added an `rc` module with reference counting utilities:
  `StrongPtr`, `WeakPtr`, and `autoreleasepool`.
* Added some reference counting ABI foreign functions to the `runtime` module.

### Fixed
* Messaging nil under GNUstep now correctly returns zeroed results for all
  return types.


## [0.2.3] (`objc` crate) - 2018-07-07

### Added
* Added a `class!` macro for getting statically-known classes. The result is
  non-optional (avoiding a need to unwrap) and cached so each usage will only
  look up the class once.
* Added caching to the `sel!` macro so that each usage will only register the
  selector once.

### Fixed
* Implementation of `objc2::runtime` structs so there can't be unsound
  references to uninhabited types.


## [0.2.2] (`objc` crate) - 2016-10-30

### Added
* Implemented `Sync` and `Send` for `Sel`.


## [0.2.1] (`objc` crate) - 2016-04-23

### Added
* Added support for working with protocols with the `Protocol` struct.
  The protocols a class conforms to can be examined with the new
  `Class::adopted_protocols` and `Class::conforms_to` methods.
* Protocols can be declared using the new `ProtocolDecl` struct.


## [0.2.0] (`objc` crate) - 2016-03-20

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
* **BREAKING**: C types are now used from `std::os::raw` rather than `libc`.
  This means `Encode` may not be implemented for `libc` types; switch them to
  the `std::os::raw` equivalents instead. This avoids an issue that would
  arise from simultaneously using different versions of the libc crate.
* **BREAKING**: Dynamic messaging was moved into the `Message` trait; instead
  of `().send(obj, sel!(description))`, use
  `obj.send_message(sel!(description), ())`.
* **BREAKING**: Rearranged the parameters to `ClassDecl::new` for consistency;
  instead of `ClassDecl::new(superclass, "MyObject")`, use
  `ClassDecl::new("MyObject", superclass)`.
* **BREAKING**: Overhauled the `MethodImplementation` trait. Encodings are now
  accessed through the `MethodImplementation::Args` associated type. The
  `imp_for` method was replaced with `imp` and no longer takes a selector or
  returns an `UnequalArgsError`, although `ClassDecl::add_method` still
  validates the number of arguments.
* **BREAKING**: Updated the definition of `Imp` to not use the old dispatch
  prototypes. To invoke an `Imp`, it must first be transmuted to the correct
  type.

### Removed
  **BREAKING**: `objc_msgSend` functions from the `runtime` module; the
  availability of these functions varies and they shouldn't be called without
  trasmuting, so they are now hidden as an implementation detail of messaging.

### Fixed
* Corrected alignment of ivars in `ClassDecl`; declared classes may now have a
  smaller size.
* With the `"exception"` or `"verify_message"` feature enabled, panics from
  `msg_send!` will now be triggered from the line and file where the macro is
  used, rather than from within the implementation of messaging.

## [0.1.8] (`objc` crate) - 2015-11-06

### Changed
* Updated `libc` dependency.


## [0.1.7] (`objc` crate) - 2015-09-23

### Fixed
* `improper_ctypes` warning.


## [0.1.6] (`objc` crate) - 2015-08-08

### Added
* Added `"exception"` feature which catches Objective-C exceptions and turns
  them into Rust panics.
* Added support for `ARM`, `ARM64` and `x86` architectures.
* **BREAKING**: Added `Any` bound on message return types. In practice this
  probably won't break anything.
* Start testing on iOS.


## [0.1.5] (`objc` crate) - 2015-05-02

### Changed
* **BREAKING**: Renamed `IntoMethodImp` to `MethodImplementation`.
* **BREAKING**: Renamed `MethodImplementation::into_imp` to `::imp_for`.
* **BREAKING**: Relaxed `Sized` bounds on `Encode` and `Message`. In practice
  this probably won't break anything.

### Removed
* **BREAKING**: Removed `Id`, `Owned`, `Ownership`, `Shared`, `ShareId` and
  `WeakId`. Use them from the `objc_id` crate instead.
* **BREAKING**: Removed `Method::set_implementation` and
  `Method::exchange_implementation`.


## [0.1.4] (`objc` crate) - 2015-04-17

### Removed
* **BREAKING**: Removed `block` module. Use them from the `block` crate
  instead.


## [0.1.3] (`objc` crate) - 2015-04-11

### Added
* Implement `fmt::Pointer` for `Id`.

### Fixed
* Odd lifetime bug.


## [0.1.2] (`objc` crate) - 2015-04-04

### Fixed
* **BREAKING**: Replace uses of `PhantomFn` with `Sized`.


## [0.1.1] (`objc` crate) - 2015-03-27

### Added
* Implement `Error` for `UnequalArgsError`.

### Removed
* **BREAKING**: Move `objc::foundation` into new crate `objc_foundation`.


[0.2.7]: https://github.com/madsmtm/objc2/compare/objc-0.2.6...objc-0.2.7
[0.2.6]: https://github.com/madsmtm/objc2/compare/objc-0.2.5...objc-0.2.6
[0.2.5]: https://github.com/madsmtm/objc2/compare/objc-0.2.4...objc-0.2.5
[0.2.4]: https://github.com/madsmtm/objc2/compare/objc-0.2.3...objc-0.2.4
[0.2.3]: https://github.com/madsmtm/objc2/compare/objc-0.2.2...objc-0.2.3
[0.2.2]: https://github.com/madsmtm/objc2/compare/objc-0.2.1...objc-0.2.2
[0.2.1]: https://github.com/madsmtm/objc2/compare/objc-0.2.0...objc-0.2.1
[0.2.0]: https://github.com/madsmtm/objc2/compare/objc-0.1.8...objc-0.2.0
[0.1.8]: https://github.com/madsmtm/objc2/compare/objc-0.1.7...objc-0.1.8
[0.1.7]: https://github.com/madsmtm/objc2/compare/objc-0.1.6...objc-0.1.7
[0.1.6]: https://github.com/madsmtm/objc2/compare/objc-0.1.5...objc-0.1.6
[0.1.5]: https://github.com/madsmtm/objc2/compare/objc-0.1.4...objc-0.1.5
[0.1.4]: https://github.com/madsmtm/objc2/compare/objc-0.1.3...objc-0.1.4
[0.1.3]: https://github.com/madsmtm/objc2/compare/objc-0.1.2...objc-0.1.3
[0.1.2]: https://github.com/madsmtm/objc2/compare/objc-0.1.1...objc-0.1.2
[0.1.1]: https://github.com/madsmtm/objc2/compare/objc-0.1.0...objc-0.1.1
