use core::ptr::{self, NonNull};

use crate::encode::{Encode, RefEncode};
use crate::rc::{Allocated, PartialInit, Retained};
use crate::runtime::{AnyClass, AnyObject, Sel};
use crate::{sel, ClassType, DeclaredClass, Message};

use super::declared_ivars::set_finalized;
use super::{Alloc, ConvertArguments, CopyOrMutCopy, Init, MsgSend, New, Other, TupleExtender};

pub trait MsgSendId<T, U> {
    #[track_caller]
    unsafe fn send_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = U>>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> R;

    /// Add an extra error argument to the argument list, call
    /// `send_message_id` with that, and return an error if one occurred.
    #[inline]
    #[track_caller]
    unsafe fn send_message_id_error<A, E, R>(obj: T, sel: Sel, args: A) -> Result<R, Retained<E>>
    where
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: Message,
        Option<R>: MaybeUnwrap<Input = U>,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        let res: Option<R> = unsafe { Self::send_message_id(obj, sel, args) };
        // As per the Cocoa documentation:
        // > Success or failure is indicated by the return value of the
        // > method. Although Cocoa methods that indirectly return error
        // > objects in the Cocoa error domain are guaranteed to return such
        // > objects if the method indicates failure by directly returning
        // > `nil` or `NO`, you should always check that the return value is
        // > `nil` or `NO` before attempting to do anything with the `NSError`
        // > object.
        if let Some(res) = res {
            // In this case, the error is likely not created. If it is, it is
            // autoreleased anyhow, so it would be a waste to retain and
            // release it here.
            Ok(res)
        } else {
            // In this case, the error has very likely been created, but has
            // been autoreleased (as is common for "out parameters", see
            // `src/__macro_helpers/writeback.rs`). Hence we need to retain it
            // if we want it to live across autorelease pools.
            //
            // SAFETY: The message send is guaranteed to populate the error
            // object, or leave it as NULL. The error is shared, and all
            // holders of the error know this, so is safe to retain.
            Err(unsafe { encountered_error(err) })
        }
    }
}

/// new: T -> Option<Retained<U>>
/// alloc: &AnyClass -> Allocated<T>
/// init: PartialInit<T> -> Option<Retained<T>> // Changed
/// copy/mutableCopy: T -> Option<Retained<U>>
/// others: T -> Option<Retained<U>>
#[doc(hidden)]
pub trait MsgSendSuperId<T, U> {
    type Inner: ?Sized + RefEncode;

    unsafe fn send_super_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = U>>(
        obj: T,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R;

    #[inline]
    #[track_caller]
    unsafe fn send_super_message_id_static<A: ConvertArguments, R: MaybeUnwrap<Input = U>>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> R
    where
        Self::Inner: ClassType,
        <Self::Inner as ClassType>::Super: ClassType,
    {
        unsafe {
            Self::send_super_message_id(obj, <Self::Inner as ClassType>::Super::class(), sel, args)
        }
    }

    #[inline]
    #[track_caller]
    unsafe fn send_super_message_id_error<A, E, R>(
        obj: T,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> Result<R, Retained<E>>
    where
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: Message,
        Option<R>: MaybeUnwrap<Input = U>,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        // SAFETY: See `send_message_id_error`
        let res: Option<R> = unsafe { Self::send_super_message_id(obj, superclass, sel, args) };
        if let Some(res) = res {
            Ok(res)
        } else {
            // SAFETY: See `send_message_id_error`
            Err(unsafe { encountered_error(err) })
        }
    }

    #[inline]
    #[track_caller]
    unsafe fn send_super_message_id_static_error<A, E, R>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> Result<R, Retained<E>>
    where
        Self::Inner: ClassType,
        <Self::Inner as ClassType>::Super: ClassType,
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: Message,
        Option<R>: MaybeUnwrap<Input = U>,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        // SAFETY: See `send_message_id_error`
        let res: Option<R> = unsafe { Self::send_super_message_id_static(obj, sel, args) };
        if let Some(res) = res {
            Ok(res)
        } else {
            // SAFETY: See `send_message_id_error`
            Err(unsafe { encountered_error(err) })
        }
    }
}

// Marked `cold` to tell the optimizer that errors are comparatively rare.
// And intentionally not inlined, for much the same reason.
#[cold]
#[track_caller]
unsafe fn encountered_error<E: Message>(err: *mut E) -> Retained<E> {
    // SAFETY: Ensured by caller
    unsafe { Retained::retain(err) }
        .expect("error parameter should be set if the method returns NULL")
}

impl<T: MsgSend, U: ?Sized + Message> MsgSendId<T, Option<Retained<U>>> for New {
    #[inline]
    unsafe fn send_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = Option<Retained<U>>>>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> R {
        let ptr = obj.into_raw_receiver();
        // SAFETY: Checked by caller
        let obj = unsafe { MsgSend::send_message(ptr, sel, args) };
        // SAFETY: The selector is `new`, so this has +1 retain count
        let obj = unsafe { Retained::from_raw(obj) };

        // SAFETY: The object is still valid after a message send to a `new`
        // method - it would not be if the method was `init`.
        R::maybe_unwrap::<Self>(obj, (unsafe { ptr.as_ref() }, sel))
    }
}

impl<T: MsgSend, U: ?Sized + Message> MsgSendSuperId<T, Option<Retained<U>>> for New {
    type Inner = T::Inner;

    #[inline]
    unsafe fn send_super_message_id<
        A: ConvertArguments,
        R: MaybeUnwrap<Input = Option<Retained<U>>>,
    >(
        obj: T,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R {
        let ptr = obj.into_raw_receiver();
        // SAFETY: Same as in `send_message_id`
        let obj = unsafe { MsgSend::send_super_message(ptr, superclass, sel, args) };
        // SAFETY: Same as in `send_message_id`
        let obj = unsafe { Retained::from_raw(obj) };
        // SAFETY: Same as in `send_message_id`
        R::maybe_unwrap::<Self>(obj, (unsafe { ptr.as_ref() }, sel))
    }
}

impl<T: Message> MsgSendId<&'_ AnyClass, Allocated<T>> for Alloc {
    #[inline]
    unsafe fn send_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = Allocated<T>>>(
        cls: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R {
        // SAFETY: Checked by caller
        let obj = unsafe { MsgSend::send_message(cls, sel, args) };
        // SAFETY: The selector is `alloc`, so this has +1 retain count
        let obj = unsafe { Allocated::new(obj) };
        R::maybe_unwrap::<Self>(obj, ())
    }
}

impl<T: ?Sized + Message> MsgSendSuperId<&'_ AnyClass, Allocated<T>> for Alloc {
    type Inner = AnyClass;

    #[inline]
    unsafe fn send_super_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = Allocated<T>>>(
        cls: &AnyClass,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R {
        // SAFETY: Same as in `send_message_id`
        let obj = unsafe { MsgSend::send_super_message(cls, superclass, sel, args) };
        // SAFETY: Same as in `send_message_id`
        let obj = unsafe { Allocated::new(obj) };
        R::maybe_unwrap::<Self>(obj, ())
    }
}

impl Alloc {
    /// Fast path optimization for `msg_send_id![cls, alloc]`.
    #[inline]
    pub unsafe fn send_message_id_alloc<T: Message, R: MaybeUnwrap<Input = Allocated<T>>>(
        cls: &AnyClass,
    ) -> R {
        // Available on non-fragile Apple runtimes.
        #[cfg(all(
            target_vendor = "apple",
            not(all(target_os = "macos", target_arch = "x86"))
        ))]
        {
            // SAFETY: Checked by caller
            let obj: *mut T = unsafe { crate::ffi::objc_alloc(cls.as_ptr()).cast() };
            // SAFETY: The object is newly allocated, so this has +1 retain count
            let obj = unsafe { Allocated::new(obj) };
            R::maybe_unwrap::<Alloc>(obj, ())
        }
        #[cfg(not(all(
            target_vendor = "apple",
            not(all(target_os = "macos", target_arch = "x86"))
        )))]
        {
            // SAFETY: Checked by caller
            unsafe { Alloc::send_message_id(cls, sel!(alloc), ()) }
        }
    }
}

impl<T: ?Sized + Message> MsgSendId<Allocated<T>, Option<Retained<T>>> for Init {
    #[inline]
    unsafe fn send_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = Option<Retained<T>>>>(
        obj: Allocated<T>,
        sel: Sel,
        args: A,
    ) -> R {
        let ptr = Allocated::into_ptr(obj);
        // SAFETY: `ptr` may be null here, but that's fine since the return
        // is `*mut T`, which is one of the few types where messages to nil is
        // allowed.
        //
        // We do this for efficiency, to avoid having a branch that the user
        // did not intend after every `alloc`.
        let obj = unsafe { MsgSend::send_message(ptr, sel, args) };
        // SAFETY: The selector is `init`, so this has +1 retain count
        let obj = unsafe { Retained::from_raw(obj) };
        R::maybe_unwrap::<Self>(obj, (ptr.cast(), sel))
    }
}

impl<T: DeclaredClass> MsgSendSuperId<PartialInit<T>, Option<Retained<T>>> for Init {
    type Inner = T;

    #[inline]
    unsafe fn send_super_message_id<
        A: ConvertArguments,
        R: MaybeUnwrap<Input = Option<Retained<T>>>,
    >(
        obj: PartialInit<T>,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R {
        let ptr = PartialInit::into_ptr(obj);
        // SAFETY: Same as `send_message_id`.
        let ptr = unsafe { MsgSend::send_super_message(ptr, superclass, sel, args) };
        // SAFETY: The returned pointer is the same as the one we passed in.
        //
        // TODO: If this is not the case, a lot will have gone wrong anyhow,
        // so unsure if we can do anything better than just ignore the issue?
        if let Some(ptr) = NonNull::new(ptr) {
            unsafe { set_finalized(ptr) };
        }
        // SAFETY: Same as `send_message_id`
        let obj = unsafe { Retained::from_raw(ptr) };
        R::maybe_unwrap::<Self>(obj, (ptr.cast(), sel))
    }
}

impl<T: MsgSend, U: ?Sized + Message> MsgSendId<T, Option<Retained<U>>> for CopyOrMutCopy {
    #[inline]
    unsafe fn send_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = Option<Retained<U>>>>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> R {
        // SAFETY: Checked by caller
        let obj = unsafe { MsgSend::send_message(obj, sel, args) };
        // SAFETY: The selector is `copy` or `mutableCopy`, so this has +1
        // retain count
        let obj = unsafe { Retained::from_raw(obj) };
        R::maybe_unwrap::<Self>(obj, ())
    }
}

impl<T: MsgSend, U: ?Sized + Message> MsgSendSuperId<T, Option<Retained<U>>> for CopyOrMutCopy {
    type Inner = T::Inner;

    #[inline]
    unsafe fn send_super_message_id<
        A: ConvertArguments,
        R: MaybeUnwrap<Input = Option<Retained<U>>>,
    >(
        obj: T,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R {
        // SAFETY: Same as in `send_message_id`
        let obj = unsafe { MsgSend::send_super_message(obj, superclass, sel, args) };
        // SAFETY: Same as in `send_message_id`
        let obj = unsafe { Retained::from_raw(obj) };
        R::maybe_unwrap::<Self>(obj, ())
    }
}

impl<T: MsgSend, U: Message> MsgSendId<T, Option<Retained<U>>> for Other {
    #[inline]
    unsafe fn send_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = Option<Retained<U>>>>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> R {
        let ptr = obj.into_raw_receiver();
        // SAFETY: Checked by caller
        let obj = unsafe { MsgSend::send_message(ptr, sel, args) };
        // All code between the message send and the `retain_autoreleased`
        // must be able to be optimized away for this to work.

        // SAFETY: The selector is not `new`, `alloc`, `init`, `copy` nor
        // `mutableCopy`, so the object must be manually retained.
        let obj = unsafe { Retained::retain_autoreleased(obj) };

        // SAFETY: The object is still valid after a message send to a
        // normal method - it would not be if the method was `init`.
        R::maybe_unwrap::<Self>(obj, (unsafe { ptr.as_ref() }, sel))
    }
}

impl<T: MsgSend, U: Message> MsgSendSuperId<T, Option<Retained<U>>> for Other {
    type Inner = T::Inner;

    #[inline]
    unsafe fn send_super_message_id<
        A: ConvertArguments,
        R: MaybeUnwrap<Input = Option<Retained<U>>>,
    >(
        obj: T,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R {
        let ptr = obj.into_raw_receiver();
        // SAFETY: Same as `send_message_id`
        let obj = unsafe { MsgSend::send_super_message(ptr, superclass, sel, args) };
        // SAFETY: Same as `send_message_id`
        let obj = unsafe { Retained::retain_autoreleased(obj) };
        // SAFETY: Same as `send_message_id`
        R::maybe_unwrap::<Self>(obj, (unsafe { ptr.as_ref() }, sel))
    }
}

pub trait MaybeUnwrap {
    type Input;
    #[track_caller]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Self::Input, args: F::Args) -> Self;
}

impl<T: ?Sized> MaybeUnwrap for Option<Retained<T>> {
    type Input = Option<Retained<T>>;

    #[inline]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Option<Retained<T>>, _args: F::Args) -> Self {
        obj
    }
}

impl<T: ?Sized> MaybeUnwrap for Retained<T> {
    type Input = Option<Retained<T>>;

    #[inline]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Option<Retained<T>>, args: F::Args) -> Self {
        match obj {
            Some(obj) => obj,
            None => F::failed(args),
        }
    }
}

impl<T: ?Sized> MaybeUnwrap for Allocated<T> {
    type Input = Allocated<T>;

    #[inline]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Allocated<T>, _args: F::Args) -> Self {
        obj
    }
}

// Note: It would have been much easier to do this kind of thing using
// closures, but then `track_caller` doesn't work properly!
//
// Also note: This behavior (that #[method_id(...)] always unwraps instead of
// using `unwrap_unchecked`) is relied upon by `header-translator` for
// soundness, see e.g. `parse_property_return`.
pub trait MsgSendIdFailed<'a> {
    type Args;

    #[track_caller]
    fn failed(args: Self::Args) -> !;
}

impl<'a> MsgSendIdFailed<'a> for New {
    type Args = (Option<&'a AnyObject>, Sel);

    #[cold]
    fn failed((obj, sel): Self::Args) -> ! {
        if let Some(obj) = obj {
            let cls = obj.class();
            if cls.is_metaclass() {
                if sel == sel!(new) {
                    panic!("failed creating new instance of {cls}")
                } else {
                    panic!("failed creating new instance using +[{cls} {sel}]")
                }
            } else {
                panic!("unexpected NULL returned from -[{cls} {sel}]")
            }
        } else {
            panic!("unexpected NULL {sel}; receiver was NULL");
        }
    }
}

impl<'a> MsgSendIdFailed<'a> for Alloc {
    type Args = ();

    #[cold]
    fn failed(_: Self::Args) -> ! {
        unreachable!()
    }
}

impl MsgSendIdFailed<'_> for Init {
    type Args = (*mut AnyObject, Sel);

    #[cold]
    fn failed((ptr, sel): Self::Args) -> ! {
        if ptr.is_null() {
            panic!("failed allocating object")
        } else {
            // We can't really display a more descriptive message here since the
            // object is consumed by `init` and may not be valid any more.
            if sel == sel!(init) {
                panic!("failed initializing object")
            } else {
                panic!("failed initializing object with -{sel}")
            }
        }
    }
}

impl MsgSendIdFailed<'_> for CopyOrMutCopy {
    type Args = ();

    #[cold]
    fn failed(_: Self::Args) -> ! {
        panic!("failed copying object")
    }
}

impl<'a> MsgSendIdFailed<'a> for Other {
    type Args = (Option<&'a AnyObject>, Sel);

    #[cold]
    fn failed((obj, sel): Self::Args) -> ! {
        if let Some(obj) = obj {
            let cls = obj.class();
            panic!(
                "unexpected NULL returned from {}[{cls} {sel}]",
                if cls.is_metaclass() { "+" } else { "-" },
            )
        } else {
            panic!("unexpected NULL {sel}; receiver was NULL");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::rc::{autoreleasepool, RcTestObject, ThreadTestData};
    use crate::runtime::{NSObject, NSZone};
    use crate::{class, msg_send_id};

    mod test_trait_disambugated {
        use super::*;

        #[allow(dead_code)]
        trait Abc {
            fn send_message_id(&self) {}
        }

        impl<T> Abc for T {}

        #[test]
        fn test_macro_still_works() {
            let _: Retained<NSObject> = unsafe { msg_send_id![NSObject::class(), new] };
        }
    }

    // `new` family

    #[test]
    fn test_new() {
        let mut expected = ThreadTestData::current();
        let cls = RcTestObject::class();

        let _obj: Retained<AnyObject> = unsafe { msg_send_id![cls, new] };
        let _obj: Option<Retained<AnyObject>> = unsafe { msg_send_id![cls, new] };
        // This is just a roundabout way of calling `[__RcTestObject new]`.
        let _obj: Retained<AnyObject> = unsafe { msg_send_id![super(cls, cls.metaclass()), new] };
        let _obj: Option<Retained<AnyObject>> =
            unsafe { msg_send_id![super(cls, cls.metaclass()), new] };

        // `__RcTestObject` does not override `new`, so this just ends up
        // calling `[[__RcTestObject alloc] init]` as usual.
        let _obj: Retained<RcTestObject> =
            unsafe { msg_send_id![super(cls, NSObject::class().metaclass()), new] };

        expected.alloc += 5;
        expected.init += 5;
        expected.assert_current();
    }

    #[test]
    fn test_new_not_on_class() {
        let mut expected = ThreadTestData::current();
        let obj = RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();

        let _obj: Retained<AnyObject> = unsafe { msg_send_id![&obj, newMethodOnInstance] };
        let _obj: Option<Retained<AnyObject>> = unsafe { msg_send_id![&obj, newMethodOnInstance] };
        let _obj: Retained<AnyObject> =
            unsafe { msg_send_id![super(&obj, RcTestObject::class()), newMethodOnInstance] };
        let _obj: Option<Retained<AnyObject>> =
            unsafe { msg_send_id![super(&obj, RcTestObject::class()), newMethodOnInstance] };
        expected.alloc += 4;
        expected.init += 4;
        expected.assert_current();
    }

    #[test]
    // newScriptingObjectOfClass only available on macOS
    #[cfg_attr(not(all(target_vendor = "apple", target_os = "macos")), ignore)]
    fn test_new_with_args() {
        let mut expected = ThreadTestData::current();

        let object_class = RcTestObject::class();
        let key: Retained<AnyObject> = unsafe { msg_send_id![class!(NSString), new] };
        let contents_value: *const AnyObject = ptr::null();
        let properties: Retained<AnyObject> = unsafe { msg_send_id![class!(NSDictionary), new] };

        let _obj: Option<Retained<AnyObject>> = unsafe {
            msg_send_id![
                NSObject::class(),
                newScriptingObjectOfClass: object_class,
                forValueForKey: &*key,
                withContentsValue: contents_value,
                properties: &*properties,
            ]
        };
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();
    }

    #[test]
    #[should_panic = "failed creating new instance of NSValue"]
    // GNUStep instead returns an invalid instance that panics on accesses
    #[cfg_attr(feature = "gnustep-1-7", ignore)]
    fn new_nsvalue_fails() {
        let _val: Retained<AnyObject> = unsafe { msg_send_id![class!(NSValue), new] };
    }

    #[test]
    #[should_panic = "failed creating new instance using +[__RcTestObject newReturningNull]"]
    fn test_new_with_null() {
        let _obj: Retained<RcTestObject> =
            unsafe { msg_send_id![RcTestObject::class(), newReturningNull] };
    }

    #[test]
    #[should_panic = "failed creating new instance using +[__RcTestObject newReturningNull]"]
    fn test_super_new_with_null() {
        let _: Retained<RcTestObject> = unsafe {
            msg_send_id![
                super(RcTestObject::class(), RcTestObject::class().metaclass()),
                newReturningNull
            ]
        };
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject newMethodOnInstanceNull]"]
    fn test_new_any_with_null() {
        let obj = RcTestObject::new();
        let _obj: Retained<AnyObject> = unsafe { msg_send_id![&obj, newMethodOnInstanceNull] };
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject newMethodOnInstanceNull]"]
    fn test_super_new_any_with_null() {
        let obj = RcTestObject::new();
        let _obj: Retained<AnyObject> =
            unsafe { msg_send_id![super(&obj, RcTestObject::class()), newMethodOnInstanceNull] };
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic = "messsaging newMethodOnInstance to nil"
    )]
    #[cfg_attr(
        not(debug_assertions),
        ignore = "unexpected NULL newMethodOnInstance; receiver was NULL"
    )]
    fn test_new_any_with_null_receiver() {
        let obj: *const NSObject = ptr::null();
        let _obj: Retained<AnyObject> = unsafe { msg_send_id![obj, newMethodOnInstance] };
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic = "messsaging newMethodOnInstance to nil"
    )]
    #[cfg_attr(
        not(debug_assertions),
        ignore = "unexpected NULL newMethodOnInstance; receiver was NULL"
    )]
    fn test_super_new_any_with_null_receiver() {
        let obj: *const RcTestObject = ptr::null();
        let _obj: Retained<AnyObject> = unsafe { msg_send_id![super(obj), newMethodOnInstance] };
    }

    // `alloc` family

    #[test]
    fn test_alloc() {
        let mut expected = ThreadTestData::current();
        let cls = RcTestObject::class();

        let obj: Allocated<RcTestObject> = unsafe { msg_send_id![cls, alloc] };
        expected.alloc += 1;
        expected.assert_current();

        drop(obj);
        expected.release += 1;
        // Drop flag ensures uninitialized do not Drop
        // expected.drop += 1;
        expected.assert_current();

        // `+[NSObject alloc]` forwards to `allocWithZone:`, so this still
        // allocates a `__RcTestObject`.
        let _: Allocated<NSObject> =
            unsafe { msg_send_id![super(cls, NSObject::class().metaclass()), alloc] };
        expected.alloc += 1;
        expected.release += 1;
        // Drop flag ensures uninitialized do not Drop
        // expected.drop += 1;
        expected.assert_current();
    }

    #[test]
    fn test_alloc_with_zone() {
        let mut expected = ThreadTestData::current();
        let cls = RcTestObject::class();
        let zone: *const NSZone = ptr::null();

        let _obj: Allocated<RcTestObject> = unsafe { msg_send_id![cls, allocWithZone: zone] };
        expected.alloc += 1;
        expected.assert_current();

        let _obj: Allocated<RcTestObject> =
            unsafe { msg_send_id![super(cls, cls.metaclass()), allocWithZone: zone] };
        expected.alloc += 1;
        expected.assert_current();

        let _obj: Allocated<NSObject> =
            unsafe { msg_send_id![super(cls, NSObject::class().metaclass()), allocWithZone: zone] };
        expected.assert_current();
    }

    #[test]
    fn test_alloc_with_null() {
        let obj: Allocated<RcTestObject> =
            unsafe { msg_send_id![RcTestObject::class(), allocReturningNull] };
        assert!(Allocated::as_ptr(&obj).is_null());
    }

    // `init` family

    #[test]
    fn test_init() {
        let mut expected = ThreadTestData::current();

        let _: Retained<RcTestObject> = unsafe { msg_send_id![RcTestObject::alloc(), init] };
        expected.alloc += 1;
        expected.init += 1;
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();

        let obj = RcTestObject::alloc().set_ivars(());
        let _: Retained<RcTestObject> = unsafe { msg_send_id![super(obj), init] };
        expected.alloc += 1;
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();

        // Check allocation error before init
        let obj = RcTestObject::alloc();
        expected.alloc += 1;
        assert!(!Allocated::as_ptr(&obj).is_null());
        let _: Retained<RcTestObject> = unsafe { msg_send_id![obj, init] };
        expected.init += 1;
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();
    }

    #[test]
    #[should_panic = "failed initializing object with -initReturningNull"]
    fn test_init_with_null() {
        let obj: Allocated<RcTestObject> = unsafe { msg_send_id![RcTestObject::class(), alloc] };
        let _obj: Retained<RcTestObject> = unsafe { msg_send_id![obj, initReturningNull] };
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic = "messsaging init to nil")]
    #[cfg_attr(not(debug_assertions), ignore = "failed allocating object")]
    fn test_init_with_null_receiver() {
        let obj: Allocated<RcTestObject> =
            unsafe { msg_send_id![RcTestObject::class(), allocReturningNull] };
        let _obj: Retained<RcTestObject> = unsafe { msg_send_id![obj, init] };
    }

    #[test]
    #[should_panic = "tried to initialize ivars after they were already initialized"]
    #[cfg_attr(not(debug_assertions), ignore = "only checked with debug assertions")]
    #[cfg_attr(
        all(
            debug_assertions,
            any(feature = "unstable-c-unwind", target_arch = "x86")
        ),
        ignore = "panicking in `init` requires that we emit the function as `C-unwind`"
    )]
    fn test_super_init_not_initialized() {
        let obj = RcTestObject::alloc().set_ivars(());
        let _: Retained<RcTestObject> =
            unsafe { msg_send_id![super(obj, RcTestObject::class()), init] };
    }

    #[test]
    #[should_panic = "tried to finalize an already finalized object"]
    #[cfg_attr(not(debug_assertions), ignore = "only checked with debug assertions")]
    fn test_super_init_not_finalized() {
        let obj = unsafe { PartialInit::new(Allocated::into_ptr(RcTestObject::alloc())) };
        let _: Retained<RcTestObject> =
            unsafe { msg_send_id![super(obj, RcTestObject::class()), init] };
    }

    // `copy` family

    #[test]
    fn test_copy() {
        let obj = RcTestObject::new();
        let mut expected = ThreadTestData::current();

        let _: Retained<RcTestObject> = unsafe { msg_send_id![&obj, copy] };
        expected.copy += 1;
        expected.alloc += 1;
        expected.init += 1;
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();

        // `+[NSObject copy]` forwards to `copyWithZone:`, so this still
        // creates a `__RcTestObject`.
        let _: Retained<NSObject> = unsafe { msg_send_id![super(&obj), copy] };
        expected.copy += 1;
        expected.alloc += 1;
        expected.init += 1;
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();
    }

    #[test]
    #[should_panic = "failed copying object"]
    fn test_copy_with_null() {
        let obj = RcTestObject::new();
        let _obj: Retained<RcTestObject> = unsafe { msg_send_id![&obj, copyReturningNull] };
    }

    #[test]
    #[should_panic = "failed copying object"]
    fn test_super_copy_with_null() {
        let obj = RcTestObject::new();
        let _obj: Retained<RcTestObject> =
            unsafe { msg_send_id![super(&obj, RcTestObject::class()), copyReturningNull] };
    }

    // `mutableCopy` family

    #[test]
    fn test_mutable_copy() {
        let obj = RcTestObject::new();
        let mut expected = ThreadTestData::current();

        let _: Retained<RcTestObject> = unsafe { msg_send_id![&obj, mutableCopy] };
        expected.mutable_copy += 1;
        expected.alloc += 1;
        expected.init += 1;
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();

        // `+[NSObject mutableCopy]` forwards to `mutableCopyWithZone:`, so
        // this still creates a `__RcTestObject`.
        let _: Retained<NSObject> = unsafe { msg_send_id![super(&obj), mutableCopy] };
        expected.mutable_copy += 1;
        expected.alloc += 1;
        expected.init += 1;
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();
    }

    // No method family

    #[test]
    fn test_normal() {
        let obj = RcTestObject::new();
        let mut expected = ThreadTestData::current();

        let _: Retained<RcTestObject> = unsafe { msg_send_id![&obj, self] };
        expected.retain += 1;
        expected.release += 1;
        expected.assert_current();

        let _: Retained<RcTestObject> = unsafe { msg_send_id![super(&obj), self] };
        expected.retain += 1;
        expected.release += 1;
        expected.assert_current();

        let _: Option<Retained<RcTestObject>> = unsafe { msg_send_id![&obj, description] };
        expected.assert_current();

        let _: Option<Retained<RcTestObject>> = unsafe { msg_send_id![super(&obj), description] };
        expected.assert_current();
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject methodReturningNull]"]
    fn test_normal_with_null() {
        let obj = RcTestObject::new();
        let _obj: Retained<RcTestObject> = unsafe { msg_send_id![&obj, methodReturningNull] };
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject aMethod:]"]
    fn test_normal_with_param_and_null() {
        let obj = RcTestObject::new();
        let _obj: Retained<RcTestObject> = unsafe { msg_send_id![&obj, aMethod: false] };
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic = "messsaging description to nil")]
    #[cfg_attr(
        not(debug_assertions),
        ignore = "unexpected NULL description; receiver was NULL"
    )]
    fn test_normal_with_null_receiver() {
        let obj: *const NSObject = ptr::null();
        let _obj: Retained<AnyObject> = unsafe { msg_send_id![obj, description] };
    }

    // This is imperfect, but will do for now.
    // See also `tests/id_retain_autoreleased.rs`.
    //
    // Work around https://github.com/rust-lang/rust-clippy/issues/9737:
    #[allow(clippy::if_same_then_else)]
    const IF_AUTORELEASE_NOT_SKIPPED: usize = if cfg!(feature = "gnustep-1-7") {
        1
    } else if cfg!(target_arch = "x86") {
        // x86 autorelease_return is not currently tail-called, so the
        // optimization doesn't work on declare_class! functions.
        2
    } else if cfg!(target_arch = "aarch64") {
        // Currently doesn't work
        2
    } else if cfg!(any(debug_assertions, feature = "catch-all")) {
        2
    } else {
        1
    } - 1;

    // 32-bit ARM unwinding sometimes interferes with the optimization
    const IF_AUTORELEASE_NOT_SKIPPED_ARM_HACK: usize = {
        if cfg!(all(target_arch = "arm", panic = "unwind")) {
            1
        } else {
            IF_AUTORELEASE_NOT_SKIPPED
        }
    };

    macro_rules! test_error_id {
        ($expected:expr, $if_autorelease_not_skipped:expr, $sel:ident, $($obj:tt)*) => {
            // Succeeds
            let res = autoreleasepool(|_pool| {
                let res: Result<Retained<RcTestObject>, Retained<RcTestObject>> = unsafe {
                    msg_send_id![$($obj)*, $sel: false, error: _]
                };
                let res = res.expect("not ok");
                $expected.alloc += 1;
                $expected.init += 1;
                $expected.autorelease += $if_autorelease_not_skipped;
                $expected.retain += $if_autorelease_not_skipped;
                $expected.assert_current();
                res
            });
            $expected.release += $if_autorelease_not_skipped;
            $expected.assert_current();

            drop(res);
            $expected.release += 1;
            $expected.drop += 1;
            $expected.assert_current();

            // Errors
            let res = autoreleasepool(|_pool| {
                let res: Result<Retained<RcTestObject>, Retained<RcTestObject>> = unsafe {
                    msg_send_id![$($obj)*, $sel: true, error: _]
                };
                $expected.alloc += 1;
                $expected.init += 1;
                $expected.autorelease += 1;
                $expected.retain += 1;
                $expected.assert_current();
                res.expect_err("not err")
            });
            $expected.release += 1;
            $expected.assert_current();

            drop(res);
            $expected.release += 1;
            $expected.drop += 1;
            $expected.assert_current();
        }
    }

    #[test]
    fn test_error_id() {
        let mut expected = ThreadTestData::current();

        let cls = RcTestObject::class();
        test_error_id!(
            expected,
            IF_AUTORELEASE_NOT_SKIPPED_ARM_HACK,
            idAndShouldError,
            cls
        );
        test_error_id!(expected, 0, newAndShouldError, cls);

        let obj = RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        test_error_id!(
            expected,
            IF_AUTORELEASE_NOT_SKIPPED_ARM_HACK,
            idAndShouldError,
            &obj
        );

        expected.alloc -= 1;
        expected.release -= 1;
        test_error_id!(expected, 0, initAndShouldError, {
            expected.alloc += 1;
            expected.release += 1;
            // Drop flag ensures newly allocated objects do not drop
            // expected.drop += 1;
            RcTestObject::alloc()
        });
    }

    #[test]
    fn test_method_id_with_param() {
        let mut expected = ThreadTestData::current();

        let obj = RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();

        let res: Option<Retained<RcTestObject>> = unsafe { msg_send_id![&obj, aMethod: false] };
        assert!(res.is_none());
        expected.assert_current();

        let _res = autoreleasepool(|_pool| {
            let res: Option<Retained<RcTestObject>> = unsafe { msg_send_id![&obj, aMethod: true] };
            assert!(res.is_some());
            expected.alloc += 1;
            expected.init += 1;
            expected.autorelease += IF_AUTORELEASE_NOT_SKIPPED_ARM_HACK;
            expected.retain += IF_AUTORELEASE_NOT_SKIPPED_ARM_HACK;
            expected.assert_current();
            res
        });
        expected.release += IF_AUTORELEASE_NOT_SKIPPED_ARM_HACK;
        expected.assert_current();
    }
}
