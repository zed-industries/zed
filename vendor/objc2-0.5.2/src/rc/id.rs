use core::fmt;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::{self, NonNull};

use super::AutoreleasePool;
use crate::mutability::{IsIdCloneable, IsMutable};
use crate::runtime::{objc_release_fast, objc_retain_fast};
use crate::{ffi, ClassType, Message};

/// A reference counted pointer type for Objective-C objects.
///
/// [`Retained`] strongly references or "retains" the given object `T`, and
/// decrements the retain count or "releases" it again when dropped, thereby
/// ensuring it will be deallocated at the right time.
///
/// The type `T` inside `Retained<T>` can be anything that implements
/// [`Message`].
///
/// `T`'s [`ClassType`] implementation (if any) determines whether it is
/// mutable, and by extension whether `Retained<T>` is mutable.
///
/// This can usually be gotten from one of the methods in the framework
/// crates, but can be created manually with the [`msg_send_id!`] macro (or
/// even more manually, with the [`Retained::retain`], [`Retained::from_raw`]
/// or [`Retained::retain_autoreleased`] methods).
///
/// [`msg_send_id!`]: crate::msg_send_id
///
///
/// # Comparison to `std` types
///
/// `Retained<T>` can be thought of as kind of a weird combination of [`Arc`]
/// and [`Box`]:
///
/// If `T` implements [`IsMutable`] (like it does on `NSMutableString` and
/// `NSMutableArray<_>`), `Retained<T>` acts like `Box<T>`, and allows mutable
/// / unique access to the type.
///
/// Otherwise, which is the most common case, `Retained<T>` acts like
/// `Arc<T>`, and allows cloning by bumping the reference count.
///
/// [`Arc`]: alloc::sync::Arc
/// [`Box`]: alloc::boxed::Box
///
///
/// # Forwarding implementations
///
/// Since `Retained<T>` is a smart pointer, it [`Deref`]s to `T`, and
/// similarly implements [`DerefMut`] when mutable.
///
/// It also forwards the implementation of a bunch of standard library traits
/// such as [`PartialEq`], [`AsRef`], and so on, so that it becomes possible
/// to use e.g. `Retained<NSString>` as if it was `NSString`. (Having
/// `NSString` directly is not possible since Objective-C objects cannot live
/// on the stack, but instead must reside on the heap).
///
/// Note that because of current limitations in the Rust trait system, some
/// traits like [`Default`], [`IntoIterator`], [`FromIterator`], [`From`] and
/// [`Into`] are not directly implementable on `NSString`; for that use-case,
/// we instead provide the [`DefaultRetained`], [`RetainedIntoIterator`] and
/// [`RetainedFromIterator`] traits, which make some of the the aforementioned
/// traits implementable on `Retained`.
///
/// [`DefaultRetained`]: crate::rc::DefaultRetained
/// [`RetainedIntoIterator`]: crate::rc::RetainedIntoIterator
/// [`RetainedFromIterator`]: crate::rc::RetainedFromIterator
///
///
/// # Memory layout
///
/// This is guaranteed to have the same size and alignment as a pointer to the
/// object, `*const T`.
///
/// Additionally, it participates in the null-pointer optimization, that is,
/// `Option<Retained<T>>` is guaranteed to have the same size as
/// `Retained<T>`.
///
///
/// # Example
///
/// Various usage of `Retained` on an immutable object.
///
/// ```
/// # #[cfg(not_available)]
/// use objc2_foundation::{NSObject, NSString};
/// # use objc2::runtime::NSObject;
/// use objc2::rc::Retained;
/// use objc2::{ClassType, msg_send_id};
/// #
/// # objc2::extern_class!(
/// #     pub struct NSString;
/// #
/// #     unsafe impl ClassType for NSString {
/// #         type Super = NSObject;
/// #         // This is wrong, but let's do it for the example
/// #         type Mutability = objc2::mutability::Immutable;
/// #     }
/// # );
///
/// // Use `msg_send_id!` to create an `Retained` with correct memory management
/// //
/// // SAFETY: The types are correct, and it is safe to call the `new`
/// // selector on `NSString`.
/// let string: Retained<NSString> = unsafe { msg_send_id![NSString::class(), new] };
/// // Or:
/// // let string = NSString::new();
///
/// // Methods on `NSString` is usable via. `Deref`
/// #[cfg(not_available)]
/// assert_eq!(string.len(), 0);
///
/// // Bump the reference count of the object (possible because the object is
/// // immutable, would not be possible for `NSMutableString`).
/// let another_ref: Retained<NSString> = string.clone();
///
/// // Convert one of the references to a reference to `NSObject` instead
/// let obj: Retained<NSObject> = Retained::into_super(string);
///
/// // And use the `Debug` impl from that
/// assert_eq!(format!("{obj:?}"), "");
///
/// // Finally, the `Retained`s go out of scope, the reference counts are
/// // decreased, and the string will deallocate
/// ```
#[repr(transparent)]
#[doc(alias = "id")]
#[doc(alias = "Id")]
#[doc(alias = "StrongPtr")]
// TODO: Add `ptr::Thin` bound on `T` to allow for only extern types
pub struct Retained<T: ?Sized> {
    /// A pointer to the contained object. The pointer is always retained.
    ///
    /// It is important that this is `NonNull`, since we want to dereference
    /// it later, and be able to use the null-pointer optimization.
    ///
    /// Additionally, covariance is correct because we're either the unique
    /// owner of `T`, or `T` is immutable.
    ptr: NonNull<T>,
    /// Necessary for dropck even though we never actually run T's destructor,
    /// because it might have a `dealloc` that assumes that contained
    /// references outlive the type.
    ///
    /// See <https://doc.rust-lang.org/nightly/nomicon/phantom-data.html>
    item: PhantomData<T>,
    /// Marks the type as !UnwindSafe. Later on we'll re-enable this.
    ///
    /// See <https://github.com/rust-lang/rust/issues/93367> for why this is
    /// required.
    notunwindsafe: PhantomData<&'static mut ()>,
}

/// Short type-alias to [`Retained`].
///
/// This type is soft-deprecated, and will be fully deprecated in `v0.6.0`,
/// see [#617].
///
/// [`Retained<T>`]: Retained
/// [#617]: https://github.com/madsmtm/objc2/issues/617
pub type Id<T> = Retained<T>;

impl<T: ?Sized> Retained<T> {
    #[inline]
    pub(crate) unsafe fn new_nonnull(ptr: NonNull<T>) -> Self {
        Self {
            ptr,
            item: PhantomData,
            notunwindsafe: PhantomData,
        }
    }
}

impl<T: ?Sized + Message> Retained<T> {
    /// Construct an [`Retained`] from a pointer that already has +1 retain count.
    ///
    /// Returns `None` if the pointer was NULL.
    ///
    /// This is useful when you have a retain count that has been handed off
    /// from somewhere else, usually Objective-C methods like `init`, `alloc`,
    /// `new`, `copy`, or methods with the `ns_returns_retained` attribute.
    ///
    /// If you do not have +1 retain count, such as if your object was
    /// retrieved from other methods than the ones noted above, use
    /// [`Retained::retain`] instead.
    ///
    ///
    /// # Safety
    ///
    /// You must uphold the same requirements as described in [`Retained::retain`].
    ///
    /// Additionally, you must ensure the given object pointer has +1 retain
    /// count.
    ///
    ///
    /// # Example
    ///
    /// Comparing different ways of creating a new `NSObject`.
    ///
    /// ```
    /// use objc2::rc::Retained;
    /// use objc2::runtime::NSObject;
    /// use objc2::{msg_send, msg_send_id, ClassType};
    ///
    /// // Manually using `msg_send!` and `Retained::from_raw`
    /// let obj: *mut NSObject = unsafe { msg_send![NSObject::class(), alloc] };
    /// let obj: *mut NSObject = unsafe { msg_send![obj, init] };
    /// // SAFETY: `-[NSObject init]` returns +1 retain count
    /// let obj: Retained<NSObject> = unsafe { Retained::from_raw(obj).unwrap() };
    ///
    /// // Or with `msg_send_id!`
    /// let obj: Retained<NSObject> = unsafe { msg_send_id![NSObject::alloc(), init] };
    ///
    /// // Or using the `NSObject::new` method
    /// let obj = NSObject::new();
    /// ```
    #[inline]
    // Note: We don't take a reference as a parameter since it would be too
    // easy to accidentally create two aliasing mutable references.
    pub unsafe fn from_raw(ptr: *mut T) -> Option<Self> {
        // Should optimize down to a noop.
        // SAFETY: Upheld by the caller
        NonNull::new(ptr).map(|ptr| unsafe { Retained::new_nonnull(ptr) })
    }

    /// Deprecated alias for [`Retained::from_raw`], see that for details.
    ///
    ///
    /// # Safety
    ///
    /// Same as [`Retained::from_raw`].
    #[deprecated = "use the more descriptive name `Retained::from_raw` instead"]
    #[inline]
    pub unsafe fn new(ptr: *mut T) -> Option<Self> {
        // SAFETY: Upheld by caller
        unsafe { Self::from_raw(ptr) }
    }

    /// Consumes the `Retained`, returning a raw pointer with +1 retain count.
    ///
    /// After calling this function, the caller is responsible for the memory
    /// previously managed by the `Retained`.
    ///
    /// This is effectively the opposite of [`Retained::from_raw`], see that for
    /// more details on when this function is useful.
    ///
    ///
    /// # Examples
    ///
    /// Converting an `Retained` to a pointer and back.
    ///
    /// ```
    /// use objc2::rc::Retained;
    /// use objc2::runtime::NSObject;
    ///
    /// let obj = NSObject::new();
    /// let ptr = Retained::into_raw(obj);
    /// // SAFETY: The pointer is valid, and has +1 retain count from above.
    /// let obj = unsafe { Retained::from_raw(ptr) }.unwrap();
    /// ```
    #[inline]
    pub fn into_raw(this: Self) -> *mut T {
        ManuallyDrop::new(this).ptr.as_ptr()
    }

    /// Returns a raw pointer to the object.
    ///
    /// The pointer is valid for at least as long as the `Retained` is held.
    ///
    /// See [`Retained::as_mut_ptr`] for the mutable equivalent.
    ///
    /// This is an associated method, and must be called as `Retained::as_ptr(obj)`.
    #[inline]
    pub fn as_ptr(this: &Self) -> *const T {
        this.ptr.as_ptr()
    }

    /// Returns a raw mutable pointer to the object.
    ///
    /// The pointer is valid for at least as long as the `Retained` is held.
    ///
    /// See [`Retained::as_ptr`] for the immutable equivalent.
    ///
    /// This is an associated method, and must be called as
    /// `Retained::as_mut_ptr(obj)`.
    #[inline]
    #[allow(unknown_lints)] // New lint below
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn as_mut_ptr(this: &mut Self) -> *mut T
    where
        T: IsMutable,
    {
        this.ptr.as_ptr()
    }

    #[inline]
    pub(crate) fn consume_as_ptr_option(this: Option<Self>) -> *mut T
    where
        T: Sized,
    {
        this.map(|this| Retained::into_raw(this))
            .unwrap_or_else(ptr::null_mut)
    }
}

// TODO: Add ?Sized bound
impl<T: Message> Retained<T> {
    /// Convert the type of the given object to another.
    ///
    /// This is equivalent to a `cast` between two pointers.
    ///
    /// See [`Retained::into_super`] and [`ProtocolObject::from_retained`] for
    /// safe alternatives.
    ///
    /// This is common to do when you know that an object is a subclass of
    /// a specific class (e.g. casting an instance of `NSString` to `NSObject`
    /// is safe because `NSString` is a subclass of `NSObject`).
    ///
    /// All `'static` objects can safely be cast to [`AnyObject`], since that
    /// assumes no specific class.
    ///
    /// [`AnyObject`]: crate::runtime::AnyObject
    /// [`ProtocolObject::from_retained`]: crate::runtime::ProtocolObject::from_retained
    ///
    ///
    /// # Safety
    ///
    /// You must ensure that the object can be reinterpreted as the given
    /// type.
    ///
    /// If `T` is not `'static`, you must ensure that `U` ensures that the
    /// data contained by `T` is kept alive for as long as `U` lives.
    ///
    /// Additionally, you must ensure that any safety invariants that the new
    /// type has are upheld.
    ///
    /// Note that it is not in general safe to cast e.g. `Retained<NSString>` to
    /// `Retained<NSMutableString>`, even if you've checked at runtime that the
    /// object is an instance of `NSMutableString`! This is because
    /// `Retained<NSMutableString>` assumes the string is unique, whereas it may
    /// have been cloned while being an `Retained<NSString>`.
    #[inline]
    pub unsafe fn cast<U: Message>(this: Self) -> Retained<U> {
        let ptr = ManuallyDrop::new(this).ptr.cast();
        // SAFETY: The object is forgotten, so we have +1 retain count.
        //
        // Caller verifies that the returned object is of the correct type.
        unsafe { Retained::new_nonnull(ptr) }
    }

    /// Retain the pointer and construct an [`Retained`] from it.
    ///
    /// Returns `None` if the pointer was NULL.
    ///
    /// This is useful when you have been given a pointer to an object from
    /// some API, and you would like to ensure that the object stays around
    /// while you work on it.
    ///
    /// For normal Objective-C methods, you may want to use
    /// [`Retained::retain_autoreleased`] instead, as that is usually more
    /// performant.
    ///
    /// See [`ClassType::retain`] for a safe alternative.
    ///
    ///
    /// # Safety
    ///
    /// If the object is mutable, the caller must ensure that there are no
    /// other pointers or references to the object, such that the returned
    /// `Retained` pointer is unique.
    ///
    /// Additionally, the pointer must be valid as a reference (aligned,
    /// dereferencable and initialized, see the [`std::ptr`] module for more
    /// information) or NULL.
    ///
    /// Finally, you must ensure that any data that `T` may reference lives
    /// for at least as long as `T`.
    ///
    /// [`std::ptr`]: core::ptr
    #[doc(alias = "objc_retain")]
    #[inline]
    pub unsafe fn retain(ptr: *mut T) -> Option<Retained<T>> {
        // SAFETY: The caller upholds that the pointer is valid
        let res: *mut T = unsafe { objc_retain_fast(ptr.cast()) }.cast();
        debug_assert_eq!(res, ptr, "objc_retain did not return the same pointer");
        // SAFETY: We just retained the object, so it has +1 retain count
        unsafe { Self::from_raw(res) }
    }

    /// Retains a previously autoreleased object pointer.
    ///
    /// This is useful when calling Objective-C methods that return
    /// autoreleased objects, see [Cocoa's Memory Management Policy][mmRules].
    ///
    /// This has exactly the same semantics as [`Retained::retain`], except it can
    /// sometimes avoid putting the object into the autorelease pool, possibly
    /// yielding increased speed and reducing memory pressure.
    ///
    /// Note: This relies heavily on being inlined right after [`msg_send!`],
    /// be careful to not accidentally require instructions between these.
    ///
    /// [mmRules]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmRules.html
    /// [`msg_send!`]: crate::msg_send
    ///
    ///
    /// # Safety
    ///
    /// Same as [`Retained::retain`].
    #[doc(alias = "objc_retainAutoreleasedReturnValue")]
    #[inline]
    pub unsafe fn retain_autoreleased(ptr: *mut T) -> Option<Retained<T>> {
        // Add magic nop instruction to participate in the fast autorelease
        // scheme.
        //
        // See `callerAcceptsOptimizedReturn` in `objc-object.h`:
        // https://github.com/apple-oss-distributions/objc4/blob/objc4-838/runtime/objc-object.h#L1209-L1377
        //
        // We will unconditionally emit these instructions, even if they end
        // up being unused (for example because we're unlucky with inlining,
        // some other work is done between the objc_msgSend and this, or the
        // runtime version is too old to support it).
        //
        // It may seem like there should be a better way to do this, but
        // emitting raw assembly is exactly what Clang and Swift does:
        // swiftc: https://github.com/apple/swift/blob/swift-5.5.3-RELEASE/lib/IRGen/GenObjC.cpp#L148-L173
        // Clang: https://github.com/llvm/llvm-project/blob/889317d47b7f046cf0e68746da8f7f264582fb5b/clang/lib/CodeGen/CGObjC.cpp#L2339-L2373
        //
        // Note that LLVM may sometimes insert extra instructions between the
        // assembly and the `objc_retainAutoreleasedReturnValue` call,
        // especially when doing tail calls and it needs to clean up the
        // function frame. Unsure how to avoid this in a performant manner?
        // Maybe force not doing tail calls by inserting assembly to do the
        // call manually?
        //
        // Resources:
        // - https://www.mikeash.com/pyblog/friday-qa-2011-09-30-automatic-reference-counting.html
        // - https://www.galloway.me.uk/2012/02/how-does-objc_retainautoreleasedreturnvalue-work/
        // - https://github.com/gfx-rs/metal-rs/issues/222
        // - https://news.ycombinator.com/item?id=29311736
        // - https://stackoverflow.com/a/23765612
        //
        // SAFETY:
        // Based on https://doc.rust-lang.org/stable/reference/inline-assembly.html#rules-for-inline-assembly
        //
        // We don't care about the value of the register (so it's okay to be
        // undefined), and its value is preserved.
        //
        // nomem: No reads or writes to memory are performed (this `mov`
        //   operates entirely on registers).
        // preserves_flags: `mov` doesn't modify any flags.
        // nostack: We don't touch the stack.

        // Only worth doing on the Apple runtime.
        // Not supported on TARGET_OS_WIN32.
        #[cfg(target_vendor = "apple")]
        {
            // Supported since macOS 10.7.
            #[cfg(target_arch = "x86_64")]
            {
                // x86_64 looks at the next call instruction.
                //
                // This is expected to be a PLT entry - if the user specifies
                // `-Zplt=no`, a GOT entry will be created instead, and this
                // will not work.
            }

            // Supported since macOS 10.8.
            #[cfg(target_arch = "arm")]
            unsafe {
                core::arch::asm!("mov r7, r7", options(nomem, preserves_flags, nostack))
            };

            // Supported since macOS 10.10.
            //
            // On macOS 13.0 / iOS 16.0 / tvOS 16.0 / watchOS 9.0, the runtime
            // instead checks the return pointer address, so we no longer need
            // to emit these extra instructions, see this video from WWDC22:
            // https://developer.apple.com/videos/play/wwdc2022/110363/
            #[cfg(all(target_arch = "aarch64", not(feature = "unstable-apple-new")))]
            unsafe {
                // Same as `mov x29, x29`.
                core::arch::asm!("mov fp, fp", options(nomem, preserves_flags, nostack))
            };

            // Supported since macOS 10.12.
            #[cfg(target_arch = "x86")]
            unsafe {
                core::arch::asm!("mov ebp, ebp", options(nomem, preserves_flags, nostack))
            };
        }

        // SAFETY: Same as `Retained::retain`, this is just an optimization.
        let res: *mut T = unsafe { ffi::objc_retainAutoreleasedReturnValue(ptr.cast()) }.cast();

        // Ideally, we'd be able to specify that the above call should never
        // be tail-call optimized (become a `jmp` instruction instead of a
        // `call`); Rust doesn't really have a way of doing this currently, so
        // we emit a `nop` to make such tail-call optimizations less likely to
        // occur.
        //
        // This is brittle! We should find a better solution!
        #[cfg(all(target_vendor = "apple", target_arch = "x86_64"))]
        {
            // SAFETY: Similar to above.
            unsafe { core::arch::asm!("nop", options(nomem, preserves_flags, nostack)) };
            // TODO: Possibly more efficient alternative? Also consider PLT.
            // #![feature(asm_sym)]
            // core::arch::asm!(
            //     "mov rdi, rax",
            //     "call {}",
            //     sym objc2::ffi::objc_retainAutoreleasedReturnValue,
            //     inout("rax") obj,
            //     clobber_abi("C"),
            // );
        }

        debug_assert_eq!(
            res, ptr,
            "objc_retainAutoreleasedReturnValue did not return the same pointer"
        );

        // SAFETY: Same as `Retained::retain`.
        unsafe { Self::from_raw(res) }
    }

    /// Autoreleases the [`Retained`], returning a pointer.
    ///
    /// The object is not immediately released, but will be when the innermost
    /// / current autorelease pool (given as a parameter) is drained.
    ///
    /// This is useful when defining your own classes and you have some error
    /// parameter passed as `Option<&mut *mut NSError>`, and you want to
    /// create and autorelease an error before returning.
    ///
    /// See [`Retained::autorelease`] and [`Retained::autorelease_mut`] for alternatives
    /// that yield safe references.
    ///
    /// This is an associated method, and must be called as
    /// `Retained::autorelease_ptr(obj)`.
    #[doc(alias = "objc_autorelease")]
    #[must_use = "if you don't intend to use the object any more, drop it as usual"]
    #[inline]
    pub fn autorelease_ptr(this: Self) -> *mut T {
        let ptr = ManuallyDrop::new(this).ptr.as_ptr();
        // SAFETY:
        // - The `ptr` is guaranteed to be valid and have at least one
        //   retain count.
        // - Because of the ManuallyDrop, we don't call the Drop
        //   implementation, so the object won't also be released there.
        let res: *mut T = unsafe { ffi::objc_autorelease(ptr.cast()) }.cast();
        debug_assert_eq!(res, ptr, "objc_autorelease did not return the same pointer");
        res
    }

    /// Autoreleases the [`Retained`], returning a reference bound to the pool.
    ///
    /// The object is not immediately released, but will be when the innermost
    /// / current autorelease pool (given as a parameter) is drained.
    ///
    /// See [`Retained::autorelease_mut`] for the mutable alternative.
    ///
    /// This is an associated method, and must be called as
    /// `Retained::autorelease(obj, pool)`.
    #[doc(alias = "objc_autorelease")]
    #[must_use = "if you don't intend to use the object any more, drop it as usual"]
    #[inline]
    #[allow(clippy::needless_lifetimes)]
    pub fn autorelease<'p>(this: Self, pool: AutoreleasePool<'p>) -> &'p T {
        let ptr = Self::autorelease_ptr(this);
        // SAFETY: The pointer is valid as a reference
        unsafe { pool.ptr_as_ref(ptr) }
    }

    /// Autoreleases the [`Retained`], returning a mutable reference bound to the
    /// pool.
    ///
    /// The object is not immediately released, but will be when the innermost
    /// / current autorelease pool (given as a parameter) is drained.
    ///
    /// See [`Retained::autorelease`] for the immutable alternative.
    ///
    /// This is an associated method, and must be called as
    /// `Retained::autorelease_mut(obj, pool)`.
    #[doc(alias = "objc_autorelease")]
    #[must_use = "if you don't intend to use the object any more, drop it as usual"]
    #[inline]
    #[allow(clippy::needless_lifetimes)]
    pub fn autorelease_mut<'p>(this: Self, pool: AutoreleasePool<'p>) -> &'p mut T
    where
        T: IsMutable,
    {
        let ptr = Self::autorelease_ptr(this);
        // SAFETY:
        // - The pointer is valid as a reference.
        // - The object is safe as mutable because of the `T: IsMutable`
        //   bound + the consumption of unique access to the `Retained`.
        unsafe { pool.ptr_as_mut(ptr) }
    }

    #[inline]
    pub(crate) fn autorelease_return_option(this: Option<Self>) -> *mut T {
        let ptr: *mut T = this
            .map(|this| ManuallyDrop::new(this).ptr.as_ptr())
            .unwrap_or_else(ptr::null_mut);

        // SAFETY: Same as `autorelease_inner`, this is just an optimization.
        let res: *mut T = unsafe { ffi::objc_autoreleaseReturnValue(ptr.cast()) }.cast();
        debug_assert_eq!(
            res, ptr,
            "objc_autoreleaseReturnValue did not return the same pointer"
        );
        res
    }

    /// Autoreleases and prepares the [`Retained`] to be returned to Objective-C.
    ///
    /// The object is not immediately released, but will be when the innermost
    /// autorelease pool is drained.
    ///
    /// This is useful when [declaring your own methods][declare] where you
    /// will often find yourself in need of returning autoreleased objects to
    /// properly follow [Cocoa's Memory Management Policy][mmRules].
    ///
    /// To that end, you could use [`Retained::autorelease`], but that would require
    /// you to have an [`AutoreleasePool`] object at hand, which you often
    /// won't have in such cases. This function doesn't require a `pool`
    /// object (but as a downside returns a pointer instead of a reference).
    ///
    /// This is also more efficient than a normal `autorelease`, since it
    /// makes a best effort attempt to hand off ownership of the retain count
    /// to a subsequent call to `objc_retainAutoreleasedReturnValue` /
    /// [`Retained::retain_autoreleased`] in the enclosing call frame.
    ///
    /// This optimization relies heavily on this function being tail called,
    /// so make sure you only call this function at the end of your method.
    ///
    /// [declare]: crate::declare
    /// [mmRules]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmRules.html
    ///
    ///
    /// # Example
    ///
    /// Returning an `Retained` from a declared method (note: the [`declare_class!`]
    /// macro supports doing this for you automatically).
    ///
    /// ```
    /// use objc2::{class, msg_send_id, sel};
    /// use objc2::declare::ClassBuilder;
    /// use objc2::rc::Retained;
    /// use objc2::runtime::{AnyClass, AnyObject, Sel};
    ///
    /// let mut builder = ClassBuilder::new("ExampleObject", class!(NSObject)).unwrap();
    ///
    /// extern "C" fn get(cls: &AnyClass, _cmd: Sel) -> *mut AnyObject {
    ///     let obj: Retained<AnyObject> = unsafe { msg_send_id![cls, new] };
    ///     Retained::autorelease_return(obj)
    /// }
    ///
    /// unsafe {
    ///     builder.add_class_method(
    ///         sel!(get),
    ///         get as extern "C" fn(_, _) -> _,
    ///     );
    /// }
    ///
    /// let cls = builder.register();
    /// ```
    ///
    /// [`declare_class!`]: crate::declare_class
    #[doc(alias = "objc_autoreleaseReturnValue")]
    #[must_use = "if you don't intend to use the object any more, drop it as usual"]
    #[inline]
    pub fn autorelease_return(this: Self) -> *mut T {
        Self::autorelease_return_option(Some(this))
    }
}

impl<T: ClassType + 'static> Retained<T>
where
    T::Super: 'static,
{
    /// Convert the object into its superclass.
    #[inline]
    pub fn into_super(this: Self) -> Retained<T::Super> {
        // SAFETY:
        // - The casted-to type is a superclass of the type.
        // - Both types are `'static`, so no lifetime information is lost
        //   (this could maybe be relaxed a bit, but let's be on the safe side
        //   for now).
        unsafe { Self::cast::<T::Super>(this) }
    }
}

// TODO: Add ?Sized bound
impl<T: Message + IsIdCloneable> Clone for Retained<T> {
    /// Makes a clone of the shared object.
    ///
    /// This increases the object's reference count.
    #[doc(alias = "objc_retain")]
    #[doc(alias = "retain")]
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY:
        // - The object is known to not be mutable due to the `IsIdCloneable`
        //   bound. Additionally, since the object is already an `Retained`, types
        //   like `NSObject` and `NSString` that have a mutable subclass is
        //   also allowed (since even if the object is originally an
        //   `Retained<NSMutableString>`, by converting it into `Retained<NSObject>` or
        //   `Retained<NSString>` that fact is wholly forgotten, and the object
        //   cannot ever be mutated again).
        // - The pointer is valid.
        let obj = unsafe { Retained::retain(self.ptr.as_ptr()) };
        // SAFETY: `objc_retain` always returns the same object pointer, and
        // the pointer is guaranteed non-null.
        unsafe { obj.unwrap_unchecked() }
    }
}

/// `#[may_dangle]` (see [this][dropck_eyepatch]) doesn't apply here since we
/// don't run `T`'s destructor (rather, we want to discourage having `T`s with
/// a destructor); and even if we did run the destructor, it would not be safe
/// to add since we cannot verify that a `dealloc` method doesn't access
/// borrowed data.
///
/// [dropck_eyepatch]: https://doc.rust-lang.org/nightly/nomicon/dropck.html#an-escape-hatch
impl<T: ?Sized> Drop for Retained<T> {
    /// Releases the retained object.
    ///
    /// The contained object's destructor (`Drop` impl, if it has one) is
    /// never run - override the `dealloc` method instead (which
    /// `declare_class!` does for you).
    #[doc(alias = "objc_release")]
    #[doc(alias = "release")]
    #[inline]
    fn drop(&mut self) {
        // We could technically run the destructor for `T` when it is mutable,
        // but that would be confusing and inconsistent since we cannot really
        // guarantee that it is run if the `Retained<T>` is passed to Objective-C.

        // SAFETY: The `ptr` is guaranteed to be valid and have at least one
        // retain count.
        unsafe { objc_release_fast(self.ptr.as_ptr().cast()) };
    }
}

impl<T: ?Sized> Deref for Retained<T> {
    type Target = T;

    /// Obtain an immutable reference to the object.
    // Box doesn't inline, but that's because it's a compiler built-in
    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: The pointer's validity is verified when the type is
        // created.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized + IsMutable> DerefMut for Retained<T> {
    /// Obtain a mutable reference to the object.
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: The pointer's validity is verified when the type is
        // created, and `Retained` is the unique owner of the object because of the
        // `IsMutable` bound, so mutability is safe.
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized> fmt::Pointer for Retained<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr.as_ptr(), f)
    }
}

#[allow(missing_debug_implementations)]
mod private {
    use crate::runtime::AnyObject;
    use crate::ClassType;
    use core::panic::{RefUnwindSafe, UnwindSafe};

    pub struct UnknownStorage<T: ?Sized>(*const T, AnyObject);

    pub struct ArcLikeStorage<T: ?Sized>(*const T);
    // SAFETY: Same as `Arc`
    unsafe impl<T: ?Sized + Sync + Send> Send for ArcLikeStorage<T> {}
    // SAFETY: Same as `Arc`
    unsafe impl<T: ?Sized + Sync + Send> Sync for ArcLikeStorage<T> {}
    impl<T: ?Sized + RefUnwindSafe> RefUnwindSafe for ArcLikeStorage<T> {}
    impl<T: ?Sized + RefUnwindSafe> UnwindSafe for ArcLikeStorage<T> {}
    impl<T: ?Sized> Unpin for ArcLikeStorage<T> {}

    pub struct BoxLikeStorage<T: ?Sized>(T);

    use crate::mutability;

    #[doc(hidden)]
    pub trait SendSyncHelper<T: ?Sized>: mutability::Mutability {
        // TODO: Move this to be a hidden type under `Mutability` once GATs
        // are in our MSRV.
        type EquivalentType: ?Sized;
    }

    impl<T: ?Sized> SendSyncHelper<T> for mutability::Root {
        // To give us freedom in the future (no `Root` types implement any
        // auto traits anyhow).
        type EquivalentType = UnknownStorage<T>;
    }

    impl<T: ?Sized> SendSyncHelper<T> for mutability::Immutable {
        type EquivalentType = ArcLikeStorage<T>;
    }

    impl<T: ?Sized> SendSyncHelper<T> for mutability::Mutable {
        type EquivalentType = BoxLikeStorage<T>;
    }

    impl<T: ?Sized, MS: ?Sized> SendSyncHelper<T> for mutability::ImmutableWithMutableSubclass<MS> {
        type EquivalentType = ArcLikeStorage<T>;
    }

    impl<T: ?Sized, IS: ?Sized> SendSyncHelper<T> for mutability::MutableWithImmutableSuperclass<IS> {
        type EquivalentType = BoxLikeStorage<T>;
    }

    impl<T: ?Sized> SendSyncHelper<T> for mutability::InteriorMutable {
        type EquivalentType = ArcLikeStorage<T>;
    }

    impl<T: ?Sized> SendSyncHelper<T> for mutability::MainThreadOnly {
        type EquivalentType = ArcLikeStorage<T>;
    }

    /// Helper struct for avoiding a gnarly ICE in `rustdoc` when generating
    /// documentation for auto traits for `Retained<T>` where `T: !ClassType`.
    ///
    /// See related issues:
    /// - <https://github.com/rust-lang/rust/issues/91380>
    /// - <https://github.com/rust-lang/rust/issues/107715>
    pub struct EquivalentType<T: ?Sized + ClassType>(
        <T::Mutability as SendSyncHelper<T>>::EquivalentType,
    )
    where
        T::Mutability: SendSyncHelper<T>;
}

// https://doc.rust-lang.org/nomicon/arc-mutex/arc-base.html#send-and-sync

/// `Retained<T>` is always `Send` if `T` is `Send + Sync`.
///
/// Additionally, for mutable types, `T` doesn't have to be `Sync` (only
/// requires `T: Send`), since it has unique access to the object.
//
// SAFETY:
// - `T: Send` is required because otherwise you could move the object to
//   another thread and let `dealloc` get called there.
// - If `T` is not mutable, `T: Sync` is required because otherwise you could
//   clone `&Retained<T>`, send it to another thread, and drop the clone last,
//   making `dealloc` get called on the other thread.
unsafe impl<T: ?Sized + ClassType + Send> Send for Retained<T>
where
    T::Mutability: private::SendSyncHelper<T>,
    private::EquivalentType<T>: Send,
{
}

/// `Retained<T>` is always `Sync` if `T` is `Send + Sync`.
///
/// Additionally, for mutable types, `T` doesn't have to be `Send` (only
/// requires `T: Sync`), since it has unique access to the object.
//
// SAFETY:
// - `T: Sync` is required because `&Retained<T>` give access to `&T`.
// - If `T` is not mutable, `T: Send` is required because otherwise you could
//   clone `&Retained<T>` from another thread, and drop the clone last, making
//   `dealloc` get called on the other thread.
unsafe impl<T: ?Sized + ClassType + Sync> Sync for Retained<T>
where
    T::Mutability: private::SendSyncHelper<T>,
    private::EquivalentType<T>: Sync,
{
}

// This is valid without `T: Unpin` because we don't implement any projection.
//
// See https://doc.rust-lang.org/1.54.0/src/alloc/boxed.rs.html#1652-1675
// and the `Arc` implementation.
impl<T: ?Sized> Unpin for Retained<T> {}

impl<T: ?Sized + RefUnwindSafe> RefUnwindSafe for Retained<T> {}

// TODO: Relax this bound
impl<T: ?Sized + RefUnwindSafe + UnwindSafe> UnwindSafe for Retained<T> {}

#[cfg(doc)]
#[allow(unused)]
struct TestDocWithNonClassType {
    id: Retained<crate::runtime::AnyObject>,
}

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use static_assertions::{assert_impl_all, assert_not_impl_any};

    use super::*;
    use crate::mutability::{Immutable, Mutable};
    use crate::rc::{autoreleasepool, RcTestObject, ThreadTestData};
    use crate::runtime::{AnyObject, NSObject, NSObjectProtocol};
    use crate::{declare_class, msg_send, DeclaredClass};

    #[test]
    fn auto_traits() {
        macro_rules! helper {
            ($name:ident, $mutability:ty) => {
                declare_class!(
                    struct $name;

                    unsafe impl ClassType for $name {
                        type Super = NSObject;
                        type Mutability = $mutability;
                        const NAME: &'static str = concat!(stringify!($name), "Test");
                    }

                    impl DeclaredClass for $name {}
                );
            };
        }

        helper!(ImmutableObject, Immutable);
        helper!(ImmutableSendObject, Immutable);
        unsafe impl Send for ImmutableSendObject {}
        helper!(ImmutableSyncObject, Immutable);
        unsafe impl Sync for ImmutableSyncObject {}
        helper!(ImmutableSendSyncObject, Immutable);
        unsafe impl Send for ImmutableSendSyncObject {}
        unsafe impl Sync for ImmutableSendSyncObject {}

        helper!(MutableObject, Mutable);
        helper!(MutableSendObject, Mutable);
        unsafe impl Send for MutableSendObject {}
        helper!(MutableSyncObject, Mutable);
        unsafe impl Sync for MutableSyncObject {}
        helper!(MutableSendSyncObject, Mutable);
        unsafe impl Send for MutableSendSyncObject {}
        unsafe impl Sync for MutableSendSyncObject {}

        assert_impl_all!(Retained<AnyObject>: Unpin);
        assert_not_impl_any!(Retained<AnyObject>: Send, Sync, UnwindSafe, RefUnwindSafe);

        assert_not_impl_any!(Retained<ImmutableObject>: Send, Sync);
        assert_not_impl_any!(Retained<ImmutableSendObject>: Send, Sync);
        assert_not_impl_any!(Retained<ImmutableSyncObject>: Send, Sync);
        assert_impl_all!(Retained<ImmutableSendSyncObject>: Send, Sync);

        assert_not_impl_any!(Retained<MutableObject>: Send, Sync);
        assert_not_impl_any!(Retained<MutableSendObject>: Sync);
        assert_impl_all!(Retained<MutableSendObject>: Send);
        assert_not_impl_any!(Retained<MutableSyncObject>: Send);
        assert_impl_all!(Retained<MutableSyncObject>: Sync);
        assert_impl_all!(Retained<MutableSendSyncObject>: Send, Sync);
    }

    #[test]
    fn test_drop() {
        let mut expected = ThreadTestData::current();

        let obj = RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();

        drop(obj);
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();
    }

    #[test]
    fn test_autorelease() {
        let obj = RcTestObject::new();
        let cloned = obj.clone();
        let mut expected = ThreadTestData::current();

        autoreleasepool(|pool| {
            let _ref = Retained::autorelease(obj, pool);
            expected.autorelease += 1;
            expected.assert_current();
            assert_eq!(cloned.retainCount(), 2);
        });
        expected.release += 1;
        expected.assert_current();
        assert_eq!(cloned.retainCount(), 1);

        autoreleasepool(|pool| {
            let _ref = Retained::autorelease(cloned, pool);
            expected.autorelease += 1;
            expected.assert_current();
        });
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();
    }

    #[test]
    fn test_clone() {
        let obj = RcTestObject::new();
        assert_eq!(obj.retainCount(), 1);
        let mut expected = ThreadTestData::current();

        expected.assert_current();
        assert_eq!(obj.retainCount(), 1);

        let cloned = obj.clone();
        expected.retain += 1;
        expected.assert_current();
        assert_eq!(cloned.retainCount(), 2);
        assert_eq!(obj.retainCount(), 2);

        let obj = Retained::into_super(Retained::into_super(obj));
        let cloned_and_type_erased = obj.clone();
        expected.retain += 1;
        expected.assert_current();
        let retain_count: usize = unsafe { msg_send![&cloned_and_type_erased, retainCount] };
        assert_eq!(retain_count, 3);
        let retain_count: usize = unsafe { msg_send![&obj, retainCount] };
        assert_eq!(retain_count, 3);

        drop(obj);
        expected.release += 1;
        expected.assert_current();
        assert_eq!(cloned.retainCount(), 2);

        drop(cloned_and_type_erased);
        expected.release += 1;
        expected.assert_current();
        assert_eq!(cloned.retainCount(), 1);

        drop(cloned);
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();
    }

    #[test]
    fn test_retain_autoreleased_works_as_retain() {
        let obj = RcTestObject::new();
        let mut expected = ThreadTestData::current();

        let ptr = Retained::as_ptr(&obj) as *mut RcTestObject;
        let _obj2 = unsafe { Retained::retain_autoreleased(ptr) }.unwrap();
        expected.retain += 1;
        expected.assert_current();
    }

    #[test]
    fn test_cast() {
        let obj: Retained<RcTestObject> = RcTestObject::new();
        let expected = ThreadTestData::current();

        // SAFETY: Any object can be cast to `AnyObject`
        let obj: Retained<AnyObject> = unsafe { Retained::cast(obj) };
        expected.assert_current();

        // SAFETY: The object was originally `__RcTestObject`
        let _obj: Retained<RcTestObject> = unsafe { Retained::cast(obj) };
        expected.assert_current();
    }

    #[repr(C)]
    struct MyObject<'a> {
        inner: NSObject,
        p: PhantomData<&'a str>,
    }

    /// Test that `Retained<T>` is covariant over `T`.
    #[allow(unused)]
    fn assert_id_variance<'b>(obj: Retained<MyObject<'static>>) -> Retained<MyObject<'b>> {
        obj
    }

    #[test]
    fn test_size_of() {
        let ptr_size = size_of::<&NSObject>();

        assert_eq!(size_of::<Retained<NSObject>>(), ptr_size);
        assert_eq!(size_of::<Option<Retained<NSObject>>>(), ptr_size);
    }
}
