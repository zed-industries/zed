use alloc::ffi::CString;
use core::ptr::NonNull;

use crate::__macro_helpers::defined_ivars::get_initialized_ivar_ptr;
use crate::encode::RefEncode;
use crate::rc::{Allocated, Retained};
use crate::runtime::{AnyClass, AnyProtocol, ProtocolObject};
use crate::MainThreadMarker;

/// Types that can be sent Objective-C messages.
///
/// Implementing this provides [`MessageReceiver`] implementations for common
/// pointer types and references to the type, which allows using them as the
/// receiver (first argument) in the [`msg_send!`][`crate::msg_send`] macro.
///
/// This trait also allows the object to be used in [`Retained`].
///
/// This is a subtrait of [`RefEncode`], meaning the type must also implement
/// that, almost always with [`RefEncode::ENCODING_REF`] being
/// [`Encoding::Object`].
///
/// This can be implemented for unsized (`!Sized`) types, but the intention is
/// not to support dynamically sized types like slices, only `extern type`s
/// (which is currently unstable).
///
/// [`MessageReceiver`]: crate::runtime::MessageReceiver
/// [`Encoding::Object`]: crate::Encoding::Object
///
///
/// # `Drop` interaction
///
/// If the inner type implements [`Drop`], that implementation will very
/// likely not be called, since there is no way to ensure that the Objective-C
/// runtime will do so. If you need to run some code when the object is
/// destroyed, implement the `dealloc` method instead.
///
/// The [`define_class!`] macro does this for you, but the [`extern_class!`]
/// macro fundamentally cannot.
///
/// [`define_class!`]: crate::define_class
/// [`extern_class!`]: crate::extern_class
///
///
/// # Safety
///
/// The type must represent an Objective-C object, meaning it:
/// - Must be valid to reinterpret as [`AnyObject`].
/// - Must be able to be the receiver of an Objective-C message sent with
///   [`objc_msgSend`] or similar.
/// - Must respond to the standard memory management `retain`, `release` and
///   `autorelease` messages.
/// - Must support weak references. (In the future we should probably make a
///   new trait for this, for example `NSTextView` only supports weak
///   references on macOS 10.12 or above).
///
/// [`AnyObject`]: crate::runtime::AnyObject
/// [`objc_msgSend`]: https://developer.apple.com/documentation/objectivec/1456712-objc_msgsend
///
///
/// # Example
///
/// ```
/// use objc2::runtime::NSObject;
/// use objc2::{Encoding, Message, RefEncode};
///
/// #[repr(C)]
/// struct MyObject {
///     // This has the exact same layout as `NSObject`
///     inner: NSObject
/// }
///
/// unsafe impl RefEncode for MyObject {
///     const ENCODING_REF: Encoding = Encoding::Object;
/// }
///
/// unsafe impl Message for MyObject {}
///
/// // `*mut MyObject` and other pointer/reference types to the object can
/// // now be used in `msg_send!`
/// //
/// // And `Retained<MyObject>` can now be constructed.
/// ```
///
/// Implement the trait manually for a class with a lifetime parameter.
///
/// ```
#[doc = include_str!("../examples/class_with_lifetime.rs")]
/// ```
pub unsafe trait Message: RefEncode {
    /// Increment the reference count of the receiver.
    ///
    /// This extends the duration in which the receiver is alive by detaching
    /// it from the lifetime information carried by the reference.
    ///
    /// This is similar to using [`Clone` on `Retained<Self>`][clone-id], with
    /// the addition that it can be used on a plain reference.
    ///
    /// If your type may have come from a mutable type like `NSMutableString`,
    /// you should consider using `NSCopying::copy` instead to avoid carrying
    /// around a mutable string when you did not intend to.
    ///
    /// [clone-id]: crate::rc::Retained#impl-Clone-for-Retained<T>
    #[inline]
    #[doc(alias = "objc_retain")]
    fn retain(&self) -> Retained<Self>
    where
        Self: Sized, // Temporary
    {
        let ptr: *const Self = self;
        let ptr: *mut Self = ptr as _;
        // SAFETY:
        // - The pointer is valid since it came from `&self`.
        // - The lifetime of the pointer itself is extended, but any lifetime
        //   that the object may carry is still kept within the type itself.
        let obj = unsafe { Retained::retain(ptr) };
        // SAFETY: The pointer came from `&self`, which is always non-null,
        // and objc_retain always returns the same value.
        unsafe { obj.unwrap_unchecked() }
    }
}

/// Marks types that represent specific classes.
///
/// Sometimes it is enough to generically know that a type is messageable,
/// e.g. [`Retained`] works with any type that implements the [`Message`]
/// trait. But often, you have an object that you know represents a specific
/// Objective-C class - this trait allows you to communicate that, as well as
/// a few properties of the class to the rest of the type-system.
///
/// This is implemented for your type by the
/// [`define_class!`][crate::define_class] and
/// [`extern_class!`][crate::extern_class] macros.
///
///
/// # Safety
///
/// This is meant to be a sealed trait, and should not be implemented outside
/// of the aforementioned macros. See those for safety preconditions.
///
///
/// # Examples
///
/// Use the trait to access the [`AnyClass`] of an object.
///
/// ```
/// use objc2::{ClassType, msg_send};
/// use objc2::rc::Retained;
/// # use objc2::runtime::{NSObject as MyObject};
///
/// // Get the class of the object.
/// let cls = <MyObject as ClassType>::class();
/// // Or, since the trait is in scope.
/// let cls = MyObject::class();
///
/// // We can now access properties of the class.
/// assert_eq!(cls.name().to_str().unwrap(), MyObject::NAME);
///
/// // And we can send messages to the class.
/// //
/// // SAFETY:
/// // - The class is `MyObject`, which can safely be initialized with `new`.
/// // - The return type is correctly specified.
/// let obj: Retained<MyObject> = unsafe { msg_send![cls, new] };
/// ```
///
/// Use the trait to allocate a new instance of an object.
///
/// ```
/// use objc2::{msg_send, AnyThread};
/// use objc2::rc::Retained;
/// # use objc2::runtime::{NSObject as MyObject};
///
/// let obj = MyObject::alloc();
///
/// // Now we can call initializers on this newly allocated object.
/// //
/// // SAFETY: `MyObject` can safely be initialized with `init`.
/// let obj: Retained<MyObject> = unsafe { msg_send![obj, init] };
/// ```
///
/// Use the [`extern_class!`][crate::extern_class] macro to implement this
/// trait for a type.
///
/// ```
/// use objc2::runtime::NSObject;
/// use objc2::{extern_class, ClassType, AnyThread};
///
/// extern_class!(
///     // SAFETY: The superclass is correctly specified, and the class can be
///     // safely used from any thread.
///     #[unsafe(super(NSObject))]
///     # // For testing purposes
///     # #[name = "NSObject"]
///     struct MyClass;
/// );
///
/// let cls = MyClass::class();
/// let obj = MyClass::alloc();
/// ```
//
// Actual safety preconditions:
//
// 1. The type must represent a specific class.
// 2. [`Self::Super`] must be a superclass of the class (or something that
//    represents any object, like [`AnyObject`][crate::runtime::AnyObject]).
// 3. [`Self::ThreadKind`] must be correct. It is safe to default to the
//    super class' thread kind, `<Self::Super as ClassType>::ThreadKind`.
// 4. [`Self::NAME`] must be the name of the class that this type represents.
// 5. The class returned by [`Self::class`] must be the class that this type
//    represents.
pub unsafe trait ClassType: Message {
    /// The superclass of this class.
    ///
    /// If you have implemented [`Deref`] for your type, it is highly
    /// recommended that this is equal to [`Deref::Target`].
    ///
    /// This may be [`AnyObject`] if the class is a root class.
    ///
    /// [`Deref`]: std::ops::Deref
    /// [`Deref::Target`]: std::ops::Deref::Target
    /// [`AnyObject`]: crate::runtime::AnyObject
    type Super: Message;

    /// Whether the type can be used from any thread, or from only the main
    /// thread.
    ///
    /// One of [`dyn AnyThread`] or [`dyn MainThreadOnly`].
    ///
    /// Setting this makes `ClassType` provide an implementation of either
    /// [`AnyThread`] or [`MainThreadOnly`].
    ///
    /// [`dyn AnyThread`]: AnyThread
    /// [`dyn MainThreadOnly`]: MainThreadOnly
    type ThreadKind: ?Sized + ThreadKind;

    /// The name of the Objective-C class that this type represents.
    ///
    /// `T::NAME` is the `const` version of `T::class().name()`.
    ///
    /// This must not contain any NUL bytes.
    //
    // TODO: Convert this to CStr next time we do big changes to ClassType.
    const NAME: &'static str;

    /// Get a reference to the Objective-C class that this type represents.
    ///
    /// May register the class with the runtime if it wasn't already.
    ///
    ///
    /// # Panics
    ///
    /// This may panic if something went wrong with getting or creating the
    /// class, e.g. if the program is not properly linked to the framework
    /// that defines the class.
    fn class() -> &'static AnyClass;

    /// Get an immutable reference to the superclass.
    // Note: It'd be safe to provide a default impl using transmute here if
    // we wanted to!
    fn as_super(&self) -> &Self::Super;

    #[doc(hidden)]
    const __INNER: ();

    /// Inner type to use when subclassing with `define_class!`.
    ///
    /// This is used by NSObject to control which auto traits are set for
    /// defined subclasses. Set to `= Self` in all other cases.
    #[doc(hidden)]
    type __SubclassingType: ?Sized;
}

/// Marks class types whose implementation is defined in Rust.
///
/// This is used in [`define_class!`], and allows access to the instance
/// variables that a given type declares, see that macro for details.
///
/// [`define_class!`]: crate::define_class
//
// Note: We mark this trait as not `unsafe` for better documentation, since
// implementing it inside `define_class!` is not `unsafe`.
//
// Safety is ensured by `__UNSAFE_OFFSETS_CORRECT`.
pub trait DefinedClass: ClassType {
    /// A type representing the instance variables that this class carries.
    type Ivars: Sized;

    // TODO: Add `ivars_ptr(this: NonNull<Self>) -> NonNull<Self::Ivars>`?

    /// Get a reference to the instance variable data that this object
    /// carries.
    #[inline]
    #[track_caller]
    fn ivars(&self) -> &Self::Ivars
    where
        Self: Sized, // Required because of MSRV
    {
        let ptr: NonNull<Self> = NonNull::from(self);
        // SAFETY: The pointer is valid and initialized.
        let ivars = unsafe { get_initialized_ivar_ptr(ptr) };
        // SAFETY: The lifetime of the instance variable is tied to the object.
        unsafe { ivars.as_ref() }
    }

    #[doc(hidden)]
    fn __ivars_offset() -> isize;

    #[doc(hidden)]
    fn __drop_flag_offset() -> isize;

    /// # Safety
    ///
    /// The ivar offset and drop flag offsets must be implemented correctly.
    #[doc(hidden)]
    const __UNSAFE_OFFSETS_CORRECT: ();
}

/// Marks types that represent specific protocols.
///
/// This is the protocol equivalent of [`ClassType`].
///
/// This is implemented automatically by the [`extern_protocol!`] macro for
/// `dyn T`, where `T` is the protocol.
///
/// [`ClassType`]: crate::ClassType
/// [`extern_protocol!`]: crate::extern_protocol
///
///
/// # Safety
///
/// This is meant to be a sealed trait, and should not be implemented outside
/// of the [`extern_protocol!`] macro.
///
///
/// # Examples
///
/// Use the trait to access the [`AnyProtocol`] of different objects.
///
/// ```
/// use objc2::ProtocolType;
/// use objc2::runtime::NSObjectProtocol;
/// // Get a protocol object representing the `NSObject` protocol
/// let protocol = <dyn NSObjectProtocol>::protocol().expect("NSObject to have a protocol");
/// assert_eq!(<dyn NSObjectProtocol>::NAME, protocol.name().to_str().unwrap());
/// # // Ensure Foundation links on GNUStep
/// # let _cls = objc2::class!(NSObject);
/// ```
///
/// Use the [`extern_protocol!`] macro to implement and use this trait.
///
/// ```no_run
/// use objc2::{extern_protocol, ProtocolType};
///
/// extern_protocol!(
///     unsafe trait MyProtocol {}
/// );
///
/// let protocol = <dyn MyProtocol>::protocol();
/// ```
pub unsafe trait ProtocolType {
    /// The name of the Objective-C protocol that this type represents.
    ///
    /// This must not contain any NUL bytes.
    //
    // TODO: Convert this to CStr next time we do big changes to ProtocolType.
    const NAME: &'static str;

    /// Get a reference to the Objective-C protocol object that this type
    /// represents.
    ///
    /// May register the protocol with the runtime if it wasn't already.
    ///
    /// Note that some protocols [are not registered with the runtime][p-obj],
    /// depending on various factors. In those cases, this function may return
    /// `None`.
    ///
    /// [p-obj]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocProtocols.html#//apple_ref/doc/uid/TP30001163-CH15-TPXREF149
    ///
    ///
    /// # Panics
    ///
    /// This may panic if something went wrong with getting or creating the
    /// protocol, e.g. if the program is not properly linked to the framework
    /// that defines the protocol.
    fn protocol() -> Option<&'static AnyProtocol> {
        get_protocol(Self::NAME)
    }

    #[doc(hidden)]
    const __INNER: ();
}

// Outlined to reduce code size
fn get_protocol(name: &str) -> Option<&'static AnyProtocol> {
    let name = CString::new(name).expect("protocol name must be UTF-8");
    AnyProtocol::get(&name)
}

// Split into separate traits for better diagnostics
mod private {
    pub trait SealedAnyThread {}
    pub trait SealedMainThreadOnly {}
    pub trait SealedThreadKind {}
}

/// Marker trait for classes (and protocols) that are usable from any thread,
/// i.e. the opposite of [`MainThreadOnly`].
///
/// This is mostly an implementation detail to expose the [`alloc`] method
/// with different signatures depending on whether a class is main thread only
/// or not. You can safely assume that things are safe to use from any thread,
/// _unless_ they implement [`MainThreadOnly`], not only if they implement
/// this trait.
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented; it is
/// implemented automatically when you implement [`ClassType`].
//
// NOTE: Ideally this would be an auto trait that had a negative impl for
// `MainThreadOnly`, something like:
//
//     pub unsafe auto trait AnyThread {}
//     pub unsafe trait MainThreadOnly {}
//     impl<T: ?Sized + MainThreadOnly> !AnyThread for T {}
//
// This isn't possible in current Rust though, so we'll have to hack it.
pub unsafe trait AnyThread: private::SealedAnyThread {
    /// Allocate a new instance of the class.
    ///
    /// The return value can be used directly inside [`msg_send!`] to
    /// initialize the object.
    ///
    /// [`msg_send!`]: crate::msg_send
    #[inline]
    fn alloc() -> Allocated<Self>
    where
        Self: Sized + ClassType,
    {
        // SAFETY:
        // - It is always safe to (attempt to) allocate an object.
        // - The object is of the correct type, since we've used the class
        //   from `Self::class`.
        // - The object is safe to `dealloc` on the current thread (due to the
        //   `AnyThread` bound which guarantees it is not `MainThreadOnly`).
        //
        // While Xcode's Main Thread Checker doesn't report `alloc` and
        // `dealloc` as unsafe from other threads, things like `NSView` and
        // `NSWindow` still do a non-trivial amount of stuff on `dealloc`,
        // even if the object is freshly `alloc`'d - which is why we disallow
        // this.
        //
        // This also has the nice property that `Allocated<T>` is guaranteed
        // to be allowed to `init` on the current thread.
        //
        // See also `MainThreadMarker::alloc`.
        unsafe { Allocated::alloc(Self::class()) }
    }
}

// The impl here is a bit bad for diagnostics, but required to prevent users
// implementing the trait themselves.
impl<'a, T: ?Sized + ClassType<ThreadKind = dyn AnyThread + 'a>> private::SealedAnyThread for T {}
unsafe impl<'a, T: ?Sized + ClassType<ThreadKind = dyn AnyThread + 'a>> AnyThread for T {}

impl<P: ?Sized> private::SealedAnyThread for ProtocolObject<P> {}
unsafe impl<P: ?Sized + AnyThread> AnyThread for ProtocolObject<P> {}

/// Marker trait for classes and protocols that are only safe to use on the
/// main thread.
///
/// This is commonly used in GUI code like `AppKit` and `UIKit`, e.g.
/// `UIWindow` is only usable from the application's main thread because it
/// accesses global statics like the `UIApplication`.
///
/// See [`MainThreadMarker`] for a few more details on this.
///
///
/// # Safety
///
/// It is unsound to implement [`Send`] or [`Sync`] together with this.
///
/// This is a sealed trait, and should not need to be implemented; it is
/// implemented automatically when you implement [`ClassType`].
#[doc(alias = "@MainActor")]
pub unsafe trait MainThreadOnly: private::SealedMainThreadOnly {
    /// Get a [`MainThreadMarker`] from the main-thread-only object.
    ///
    /// This function exists purely in the type-system, and will succeed at
    /// runtime (with a safety check when debug assertions are enabled).
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn mtm(&self) -> MainThreadMarker {
        #[cfg(debug_assertions)]
        assert!(
            MainThreadMarker::new().is_some(),
            "the main-thread-only object that we tried to fetch a MainThreadMarker from was somehow not on the main thread",
        );

        // SAFETY: Objects which are `MainThreadOnly` are guaranteed
        // `!Send + !Sync` and are only constructible on the main thread.
        //
        // Since we hold `&self`, i.e. a reference to such an object, and we
        // know it cannot possibly be on another thread than the main, we know
        // that the current thread is the main thread.
        unsafe { MainThreadMarker::new_unchecked() }
    }

    /// Allocate a new instance of the class on the main thread.
    ///
    ///
    /// # Example
    ///
    /// Create a view on the main thread.
    ///
    /// ```
    /// use objc2::{MainThreadOnly, MainThreadMarker};
    /// # #[cfg(available_in_app_kit)]
    /// use objc2_app_kit::NSView;
    /// use objc2_core_foundation::CGRect;
    /// #
    /// # use objc2::rc::{Allocated, Retained};
    /// #
    /// # objc2::extern_class!(
    /// #     #[unsafe(super(objc2::runtime::NSObject))]
    /// #     #[thread_kind = MainThreadOnly]
    /// #     #[name = "NSObject"] // For example
    /// #     struct NSView;
    /// # );
    /// #
    /// # impl NSView {
    /// #     fn initWithFrame(this: Allocated<Self>, _frame: CGRect) -> Retained<Self> {
    /// #         // Don't use frame, this is NSObject
    /// #         unsafe { objc2::msg_send![this, init] }
    /// #     }
    /// # }
    ///
    /// # #[cfg(doctests_not_always_run_on_main_thread)]
    /// let mtm = MainThreadMarker::new().expect("must be on the main thread");
    /// # let mtm = unsafe { MainThreadMarker::new_unchecked() };
    ///
    /// let frame = CGRect::default();
    /// let view = NSView::initWithFrame(NSView::alloc(mtm), frame);
    /// ```
    #[inline]
    fn alloc(mtm: MainThreadMarker) -> Allocated<Self>
    where
        Self: Sized + ClassType,
    {
        let _ = mtm;
        // SAFETY: We hold `MainThreadMarker`, and the class is safe to
        // allocate on the main thread.
        unsafe { Allocated::alloc(Self::class()) }
    }
}

impl<'a, T: ?Sized + ClassType<ThreadKind = dyn MainThreadOnly + 'a>> private::SealedMainThreadOnly
    for T
{
}
unsafe impl<'a, T: ?Sized + ClassType<ThreadKind = dyn MainThreadOnly + 'a>> MainThreadOnly for T {}

impl<P: ?Sized> private::SealedMainThreadOnly for ProtocolObject<P> {}
unsafe impl<P: ?Sized + MainThreadOnly> MainThreadOnly for ProtocolObject<P> {}

/// The allowed values in [`ClassType::ThreadKind`].
///
/// One of [`dyn AnyThread`] or [`dyn MainThreadOnly`].
///
/// [`dyn AnyThread`]: AnyThread
/// [`dyn MainThreadOnly`]: MainThreadOnly
pub trait ThreadKind: private::SealedThreadKind {
    // To mark `ThreadKind` as dyn-incompatible for now.
    #[doc(hidden)]
    const __DYN_INCOMPATIBLE: ();
}

impl private::SealedThreadKind for dyn AnyThread + '_ {}
impl ThreadKind for dyn AnyThread + '_ {
    const __DYN_INCOMPATIBLE: () = ();
}

impl private::SealedThreadKind for dyn MainThreadOnly + '_ {}
impl ThreadKind for dyn MainThreadOnly + '_ {
    const __DYN_INCOMPATIBLE: () = ();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    fn dyn_compatible(_: &dyn AnyThread, _: &dyn MainThreadOnly) {}
}
