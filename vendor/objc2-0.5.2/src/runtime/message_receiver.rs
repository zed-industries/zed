use core::ptr::NonNull;

use crate::encode::{EncodeArguments, EncodeReturn, RefEncode};
use crate::mutability::IsAllowedMutable;
use crate::runtime::{AnyClass, AnyObject, Sel};
use crate::Message;

/// Wrap the given closure in `exception::catch` if the `catch-all` feature is
/// enabled.
///
/// This is a macro to help with monomorphization when the feature is
/// disabled, as well as improving the final stack trace (`#[track_caller]`
/// doesn't really work on closures).
#[cfg(not(feature = "catch-all"))]
macro_rules! conditional_try {
    (|| $expr:expr) => {
        $expr
    };
}

#[cfg(feature = "catch-all")]
macro_rules! conditional_try {
    (|| $expr:expr) => {{
        let f = core::panic::AssertUnwindSafe(|| $expr);
        match crate::exception::catch(f) {
            Ok(r) => r,
            Err(exception) => {
                if let Some(exception) = exception {
                    panic!("uncaught {exception:?}")
                } else {
                    panic!("uncaught exception nil")
                }
            }
        }
    }};
}

// More information on how objc_msgSend works:
// <https://web.archive.org/web/20200118080513/http://www.friday.com/bbum/2009/12/18/objc_msgsend-part-1-the-road-map/>
// <https://www.mikeash.com/pyblog/objc_msgsends-new-prototype.html>
// <https://www.mikeash.com/pyblog/friday-qa-2012-11-16-lets-build-objc_msgsend.html>
#[cfg(all(target_vendor = "apple", not(feature = "gnustep-1-7")))]
mod msg_send_primitive {
    #[allow(unused_imports)]
    use core::mem;

    #[allow(unused_imports)]
    use crate::encode::Encoding;
    use crate::encode::{EncodeArguments, EncodeReturn};
    use crate::ffi;
    use crate::runtime::{AnyClass, AnyObject, Imp, Sel};

    /// On the below architectures we can statically find the correct method to
    /// call from the return type, by looking at its `EncodeReturn` impl.
    #[allow(clippy::missing_safety_doc)]
    unsafe trait MsgSendFn: EncodeReturn {
        const MSG_SEND: Imp;
        const MSG_SEND_SUPER: Imp;
    }

    #[cfg(target_arch = "aarch64")]
    /// `objc_msgSend_stret` is not even available in arm64.
    ///
    /// <https://twitter.com/gparker/status/378079715824660480>
    unsafe impl<T: EncodeReturn> MsgSendFn for T {
        const MSG_SEND: Imp = ffi::objc_msgSend;
        const MSG_SEND_SUPER: Imp = ffi::objc_msgSendSuper;
    }

    #[cfg(target_arch = "arm")]
    /// Double-word sized fundamental data types don't use stret, but any
    /// composite type larger than 4 bytes does.
    ///
    /// <https://web.archive.org/web/20191016000656/http://infocenter.arm.com/help/topic/com.arm.doc.ihi0042f/IHI0042F_aapcs.pdf>
    /// <https://developer.arm.com/documentation/ihi0042/latest>
    /// <https://github.com/llvm/llvm-project/blob/llvmorg-17.0.6/clang/lib/CodeGen/Targets/ARM.cpp#L531>
    unsafe impl<T: EncodeReturn> MsgSendFn for T {
        const MSG_SEND: Imp = {
            if let Encoding::LongLong | Encoding::ULongLong | Encoding::Double = T::ENCODING_RETURN
            {
                ffi::objc_msgSend
            } else if mem::size_of::<T>() <= 4 {
                ffi::objc_msgSend
            } else {
                ffi::objc_msgSend_stret
            }
        };
        const MSG_SEND_SUPER: Imp = {
            if let Encoding::LongLong | Encoding::ULongLong | Encoding::Double = T::ENCODING_RETURN
            {
                ffi::objc_msgSendSuper
            } else if mem::size_of::<T>() <= 4 {
                ffi::objc_msgSendSuper
            } else {
                ffi::objc_msgSendSuper_stret
            }
        };
    }

    #[cfg(target_arch = "x86")]
    /// Structures 1 or 2 bytes in size are placed in EAX.
    /// Structures 4 or 8 bytes in size are placed in: EAX and EDX.
    /// Structures of other sizes are placed at the address supplied by the caller.
    ///
    /// <https://developer.apple.com/library/mac/documentation/DeveloperTools/Conceptual/LowLevelABI/130-IA-32_Function_Calling_Conventions/IA32.html>
    /// <https://github.com/llvm/llvm-project/blob/llvmorg-17.0.6/clang/lib/CodeGen/Targets/X86.cpp#L472>
    unsafe impl<T: EncodeReturn> MsgSendFn for T {
        const MSG_SEND: Imp = {
            // See https://github.com/apple-oss-distributions/objc4/blob/objc4-818.2/runtime/message.h#L156-L172
            if let Encoding::Float | Encoding::Double | Encoding::LongDouble = T::ENCODING_RETURN {
                ffi::objc_msgSend_fpret
            } else if let 0 | 1 | 2 | 4 | 8 = mem::size_of::<T>() {
                ffi::objc_msgSend
            } else {
                ffi::objc_msgSend_stret
            }
        };
        const MSG_SEND_SUPER: Imp = {
            if let 0 | 1 | 2 | 4 | 8 = mem::size_of::<T>() {
                ffi::objc_msgSendSuper
            } else {
                ffi::objc_msgSendSuper_stret
            }
        };
    }

    #[cfg(target_arch = "x86_64")]
    /// If the size of an object is larger than two eightbytes, it has class
    /// MEMORY. If the type has class MEMORY, then the caller provides space for
    /// the return value and passes the address of this storage.
    ///
    /// <https://www.uclibc.org/docs/psABI-x86_64.pdf>
    /// <https://github.com/llvm/llvm-project/blob/llvmorg-17.0.6/clang/lib/CodeGen/Targets/X86.cpp#L2532>
    unsafe impl<T: EncodeReturn> MsgSendFn for T {
        const MSG_SEND: Imp = {
            // See https://github.com/apple-oss-distributions/objc4/blob/objc4-818.2/runtime/message.h#L156-L172
            if let Encoding::LongDouble = T::ENCODING_RETURN {
                ffi::objc_msgSend_fpret
            } else if let Encoding::LongDoubleComplex = T::ENCODING_RETURN {
                ffi::objc_msgSend_fp2ret
            } else if mem::size_of::<T>() <= 16 {
                ffi::objc_msgSend
            } else {
                ffi::objc_msgSend_stret
            }
        };
        const MSG_SEND_SUPER: Imp = {
            if mem::size_of::<T>() <= 16 {
                ffi::objc_msgSendSuper
            } else {
                ffi::objc_msgSendSuper_stret
            }
        };
    }

    #[inline]
    #[track_caller]
    pub(crate) unsafe fn send<A: EncodeArguments, R: EncodeReturn>(
        receiver: *mut AnyObject,
        sel: Sel,
        args: A,
    ) -> R {
        let msg_send_fn = R::MSG_SEND;
        // Note: Modern Objective-C compilers have a workaround to ensure that
        // messages to `nil` with a struct return produces `mem::zeroed()`,
        // see:
        // <https://www.sealiesoftware.com/blog/archive/2012/2/29/objc_explain_return_value_of_message_to_nil.html>
        //
        // We _could_ technically do something similar, but since we're
        // disallowing messages to `nil` with `debug_assertions` enabled
        // anyhow, and since Rust has a much stronger type-system that
        // disallows NULL/nil in most cases, we won't bother supporting it.
        unsafe { A::__invoke(msg_send_fn, receiver, sel, args) }
    }

    #[inline]
    #[track_caller]
    pub(crate) unsafe fn send_super<A: EncodeArguments, R: EncodeReturn>(
        receiver: *mut AnyObject,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R {
        let superclass: *const AnyClass = superclass;
        let mut sup = ffi::objc_super {
            receiver: receiver.cast(),
            super_class: superclass.cast(),
        };
        let receiver: *mut ffi::objc_super = &mut sup;
        let receiver = receiver.cast();

        let msg_send_fn = R::MSG_SEND_SUPER;
        unsafe { A::__invoke(msg_send_fn, receiver, sel, args) }
    }
}

#[cfg(feature = "gnustep-1-7")]
mod msg_send_primitive {
    use core::mem;

    use crate::encode::{EncodeArguments, EncodeReturn};
    use crate::ffi;
    use crate::runtime::{AnyClass, AnyObject, Imp, Sel};

    #[inline]
    fn unwrap_msg_send_fn(msg_send_fn: Option<Imp>) -> Imp {
        match msg_send_fn {
            Some(msg_send_fn) => msg_send_fn,
            None => {
                // SAFETY: This will never be NULL, even if the selector is not
                // found a callable function pointer will still be returned!
                //
                // `clang` doesn't insert a NULL check here either.
                unsafe { core::hint::unreachable_unchecked() }
            }
        }
    }

    #[track_caller]
    pub(crate) unsafe fn send<A: EncodeArguments, R: EncodeReturn>(
        receiver: *mut AnyObject,
        sel: Sel,
        args: A,
    ) -> R {
        // If `receiver` is NULL, objc_msg_lookup will return a standard
        // C-method taking two arguments, the receiver and the selector.
        //
        // Transmuting and calling such a function with multiple parameters is
        // safe as long as the return value is a primitive (and e.g. not a big
        // struct or array).
        //
        // However, when the return value is a floating point value, the float
        // will end up as some undefined value, usually NaN, which is
        // incompatible with Apple's platforms. As such, we insert this extra
        // NULL check here.
        if receiver.is_null() {
            // SAFETY: Caller guarantees that messages to NULL-receivers only
            // return pointers or primitive values, and a mem::zeroed pointer
            // / primitive is just a NULL-pointer or a zeroed primitive.
            return unsafe { mem::zeroed() };
        }

        let msg_send_fn = unsafe { ffi::objc_msg_lookup(receiver.cast(), sel.as_ptr()) };
        let msg_send_fn = unwrap_msg_send_fn(msg_send_fn);
        unsafe { A::__invoke(msg_send_fn, receiver, sel, args) }
    }

    #[track_caller]
    pub(crate) unsafe fn send_super<A: EncodeArguments, R: EncodeReturn>(
        receiver: *mut AnyObject,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R {
        if receiver.is_null() {
            // SAFETY: Same as in `send`.
            return unsafe { mem::zeroed() };
        }

        let superclass: *const AnyClass = superclass;
        let sup = ffi::objc_super {
            receiver: receiver.cast(),
            super_class: superclass.cast(),
        };
        let msg_send_fn = unsafe { ffi::objc_msg_lookup_super(&sup, sel.as_ptr()) };
        let msg_send_fn = unwrap_msg_send_fn(msg_send_fn);
        unsafe { A::__invoke(msg_send_fn, receiver, sel, args) }
    }
}

#[cfg(all(not(target_vendor = "apple"), not(feature = "gnustep-1-7")))]
mod msg_send_primitive {
    use crate::encode::{EncodeArguments, EncodeReturn};
    use crate::runtime::{AnyClass, AnyObject, Sel};

    #[track_caller]
    pub(crate) unsafe fn send<A: EncodeArguments, R: EncodeReturn>(
        _receiver: *mut AnyObject,
        _sel: Sel,
        _args: A,
    ) -> R {
        unimplemented!("no runtime chosen")
    }

    #[track_caller]
    pub(crate) unsafe fn send_super<A: EncodeArguments, R: EncodeReturn>(
        _receiver: *mut AnyObject,
        _superclass: &AnyClass,
        _sel: Sel,
        _args: A,
    ) -> R {
        unimplemented!("no runtime chosen")
    }
}

/// Help with monomorphizing in framework crates
#[cfg(debug_assertions)]
#[track_caller]
fn msg_send_check(
    obj: Option<&AnyObject>,
    sel: Sel,
    args: &[crate::encode::Encoding],
    ret: &crate::encode::Encoding,
) {
    let cls = if let Some(obj) = obj {
        obj.class()
    } else {
        panic_null(sel)
    };

    msg_send_check_class(cls, sel, args, ret);
}

#[cfg(debug_assertions)]
#[track_caller]
fn msg_send_check_class(
    cls: &AnyClass,
    sel: Sel,
    args: &[crate::encode::Encoding],
    ret: &crate::encode::Encoding,
) {
    use crate::verify::{verify_method_signature, Inner, VerificationError};

    let err = if let Some(method) = cls.instance_method(sel) {
        if let Err(err) = verify_method_signature(method, args, ret) {
            err
        } else {
            return;
        }
    } else {
        VerificationError::from(Inner::MethodNotFound)
    };

    panic_verify(cls, sel, &err);
}

#[cfg(debug_assertions)]
#[track_caller]
fn panic_null(sel: Sel) -> ! {
    panic!("messsaging {sel} to nil")
}

#[cfg(debug_assertions)]
#[track_caller]
fn panic_verify(cls: &AnyClass, sel: Sel, err: &crate::runtime::VerificationError) -> ! {
    panic!(
        "invalid message send to {}[{cls} {sel}]: {err}",
        if cls.is_metaclass() { "+" } else { "-" },
    )
}

mod private {
    pub trait Sealed {}
}

/// Types that can directly be used as the receiver of Objective-C messages.
///
/// Examples include objects pointers, class pointers, and block pointers.
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait MessageReceiver: private::Sealed + Sized {
    #[doc(hidden)]
    type __Inner: ?Sized + RefEncode;

    #[doc(hidden)]
    fn __as_raw_receiver(self) -> *mut AnyObject;

    /// Sends a message to the receiver with the given selector and arguments.
    ///
    /// This should be used instead of the [`performSelector:`] family of
    /// methods, as this is both more performant and flexible than that.
    ///
    /// The correct version of `objc_msgSend` will be chosen based on the
    /// return type. For more information, see [the Messaging section in
    /// Apple's Objective-C Runtime Programming Guide][guide-messaging].
    ///
    /// If the selector is known at compile-time, it is recommended to use the
    /// [`msg_send!`] macro rather than this method.
    ///
    /// [`performSelector:`]: https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418867-performselector?language=objc
    /// [guide-messaging]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtHowMessagingWorks.html
    ///
    ///
    /// # Safety
    ///
    /// This shares the same safety requirements as [`msg_send!`].
    ///
    /// The added invariant is that the selector must take the same number of
    /// arguments as is given.
    ///
    /// [`msg_send!`]: crate::msg_send
    ///
    ///
    /// # Example
    ///
    /// Call the `copy` method, but using a dynamic selector instead.
    ///
    /// ```no_run
    /// use objc2::rc::Retained;
    /// use objc2::runtime::MessageReceiver;
    /// use objc2::sel;
    /// # use objc2::runtime::NSObject as MyObject;
    ///
    /// let obj = MyObject::new();
    /// // SAFETY: The `copy` method takes no arguments, and returns an object
    /// let copy: *mut MyObject = unsafe { obj.send_message(sel!(copy), ()) };
    /// // SAFETY: The `copy` method returns an object with +1 retain count
    /// let copy = unsafe { Retained::from_raw(copy) }.unwrap();
    /// ```
    #[inline]
    #[track_caller]
    #[doc(alias = "performSelector")]
    #[doc(alias = "performSelector:")]
    #[doc(alias = "performSelector:withObject:")]
    #[doc(alias = "performSelector:withObject:withObject:")]
    unsafe fn send_message<A: EncodeArguments, R: EncodeReturn>(self, sel: Sel, args: A) -> R {
        let receiver = self.__as_raw_receiver();
        #[cfg(debug_assertions)]
        {
            // SAFETY: Caller ensures only valid or NULL pointers.
            let obj = unsafe { receiver.as_ref() };
            msg_send_check(obj, sel, A::ENCODINGS, &R::ENCODING_RETURN);
        }

        // SAFETY: Upheld by caller
        //
        // The @catch is safe since message sending primitives are guaranteed
        // to do Objective-C compatible unwinding.
        unsafe { conditional_try!(|| msg_send_primitive::send(receiver, sel, args)) }
    }

    /// Sends a message to a specific superclass with the given selector and
    /// arguments.
    ///
    /// The correct version of `objc_msgSend_super` will be chosen based on the
    /// return type. For more information, see the section on "Sending
    /// Messages" in Apple's [documentation][runtime].
    ///
    /// If the selector is known at compile-time, it is recommended to use the
    /// [`msg_send!(super(...), ...)`] macro rather than this method.
    ///
    /// [runtime]: https://developer.apple.com/documentation/objectivec/objective-c_runtime?language=objc
    ///
    ///
    /// # Safety
    ///
    /// This shares the same safety requirements as
    /// [`msg_send!(super(...), ...)`].
    ///
    /// The added invariant is that the selector must take the same number of
    /// arguments as is given.
    ///
    /// [`msg_send!(super(...), ...)`]: crate::msg_send
    #[inline]
    #[track_caller]
    unsafe fn send_super_message<A: EncodeArguments, R: EncodeReturn>(
        self,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R {
        let receiver = self.__as_raw_receiver();
        #[cfg(debug_assertions)]
        {
            if receiver.is_null() {
                panic_null(sel);
            }
            msg_send_check_class(superclass, sel, A::ENCODINGS, &R::ENCODING_RETURN);
        }

        // SAFETY: Same as in `send_message`
        unsafe {
            conditional_try!(|| msg_send_primitive::send_super(receiver, superclass, sel, args))
        }
    }
}

// Note that we implement MessageReceiver for unsized types as well, this is
// to support `extern type`s in the future, not because we want to allow DSTs.

impl<T: ?Sized + Message> private::Sealed for *const T {}
unsafe impl<T: ?Sized + Message> MessageReceiver for *const T {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        (self as *mut T).cast()
    }
}

impl<T: ?Sized + Message> private::Sealed for *mut T {}
unsafe impl<T: ?Sized + Message> MessageReceiver for *mut T {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        self.cast()
    }
}

impl<T: ?Sized + Message> private::Sealed for NonNull<T> {}
unsafe impl<T: ?Sized + Message> MessageReceiver for NonNull<T> {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        self.as_ptr().cast()
    }
}

impl<'a, T: ?Sized + Message> private::Sealed for &'a T {}
unsafe impl<'a, T: ?Sized + Message> MessageReceiver for &'a T {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        let ptr: *const T = self;
        (ptr as *mut T).cast()
    }
}

impl<'a, T: ?Sized + Message + IsAllowedMutable> private::Sealed for &'a mut T {}
unsafe impl<'a, T: ?Sized + Message + IsAllowedMutable> MessageReceiver for &'a mut T {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        let ptr: *mut T = self;
        ptr.cast()
    }
}

impl private::Sealed for *const AnyClass {}
unsafe impl MessageReceiver for *const AnyClass {
    type __Inner = AnyClass;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        (self as *mut AnyClass).cast()
    }
}

impl<'a> private::Sealed for &'a AnyClass {}
unsafe impl<'a> MessageReceiver for &'a AnyClass {
    type __Inner = AnyClass;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        let ptr: *const AnyClass = self;
        (ptr as *mut AnyClass).cast()
    }
}

#[cfg(test)]
mod tests {
    use core::ptr;

    use super::*;
    use crate::mutability;
    use crate::rc::{Allocated, Retained};
    use crate::runtime::NSObject;
    use crate::test_utils;
    use crate::{declare_class, msg_send, msg_send_id, ClassType, DeclaredClass};

    declare_class!(
        struct MutableObject;

        unsafe impl ClassType for MutableObject {
            type Super = NSObject;
            type Mutability = mutability::Mutable;
            const NAME: &'static str = "TestMutableObject";
        }

        impl DeclaredClass for MutableObject {}
    );

    #[allow(unused)]
    fn test_different_receivers(mut obj: Retained<MutableObject>) {
        unsafe {
            let x = &mut obj;
            let _: () = msg_send![x, mutable1];
            // let _: () = msg_send![x, mutable2];
            let _: () = msg_send![&mut *obj, mutable1];
            let _: () = msg_send![&mut *obj, mutable2];
            #[allow(clippy::needless_borrow)]
            let obj: NonNull<MutableObject> = (&mut *obj).into();
            let _: () = msg_send![obj, mutable1];
            let _: () = msg_send![obj, mutable2];
            let obj: *mut MutableObject = obj.as_ptr();
            let _: () = msg_send![obj, mutable1];
            let _: () = msg_send![obj, mutable2];
        }
    }

    #[test]
    fn test_send_message() {
        let mut obj = test_utils::custom_object();
        let result: u32 = unsafe {
            let _: () = msg_send![&mut obj, setFoo: 4u32];
            msg_send![&obj, foo]
        };
        assert_eq!(result, 4);
    }

    #[test]
    fn test_send_message_stret() {
        let obj = test_utils::custom_object();
        let result: test_utils::CustomStruct = unsafe { msg_send![&obj, customStruct] };
        let expected = test_utils::CustomStruct {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
        };
        assert_eq!(result, expected);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic = "messsaging description to nil")]
    fn test_send_message_nil() {
        let nil: *mut NSObject = ::core::ptr::null_mut();

        // This result should not be relied on
        let result: Option<Retained<NSObject>> = unsafe { msg_send_id![nil, description] };
        assert!(result.is_none());

        // This result should not be relied on
        let result: usize = unsafe { msg_send![nil, hash] };
        assert_eq!(result, 0);

        // This result should not be relied on
        #[cfg(target_pointer_width = "16")]
        let result: f32 = 0.0;
        #[cfg(target_pointer_width = "32")]
        let result: f32 = unsafe { msg_send![nil, floatValue] };
        #[cfg(target_pointer_width = "64")]
        let result: f64 = unsafe { msg_send![nil, doubleValue] };
        assert_eq!(result, 0.0);

        // This result should not be relied on
        let result: Option<Retained<NSObject>> =
            unsafe { msg_send_id![nil, multiple: 1u32, arguments: 2i8] };
        assert!(result.is_none());

        // This result should not be relied on
        let obj = unsafe { Allocated::new(ptr::null_mut()) };
        let result: Option<Retained<NSObject>> = unsafe { msg_send_id![obj, init] };
        assert!(result.is_none());
    }

    #[test]
    fn test_send_message_super() {
        let mut obj = test_utils::custom_subclass_object();
        let superclass = test_utils::custom_class();
        unsafe {
            let _: () = msg_send![&mut obj, setFoo: 4u32];
            let foo: u32 = msg_send![super(&obj, superclass), foo];
            assert_eq!(foo, 4);

            // The subclass is overriden to return foo + 2
            let foo: u32 = msg_send![&obj, foo];
            assert_eq!(foo, 6);
        }
    }

    #[test]
    #[cfg_attr(
        feature = "gnustep-1-7",
        ignore = "GNUStep deadlocks here for some reason"
    )]
    fn test_send_message_class_super() {
        let cls = test_utils::custom_subclass();
        let superclass = test_utils::custom_class();
        unsafe {
            let foo: u32 = msg_send![super(cls, superclass.metaclass()), classFoo];
            assert_eq!(foo, 7);

            // The subclass is overriden to return + 2
            let foo: u32 = msg_send![cls, classFoo];
            assert_eq!(foo, 9);
        }
    }
}
