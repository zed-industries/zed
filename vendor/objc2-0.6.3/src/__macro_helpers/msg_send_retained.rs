use core::ptr;

use crate::encode::{Encode, RefEncode};
use crate::rc::Retained;
use crate::runtime::{AnyClass, MessageReceiver, Sel};
use crate::ClassType;

use super::null_error::encountered_error;
use super::{
    ConvertArguments, KindSendMessage, KindSendMessageSuper, RetainSemantics, TupleExtender,
};

//
// MsgSend
//

pub trait MsgSend<Receiver, Return> {
    #[track_caller]
    unsafe fn send_message<A: ConvertArguments>(receiver: Receiver, sel: Sel, args: A) -> Return;
}

impl<Receiver, Return, MethodFamily> MsgSend<Receiver, Return> for MethodFamily
where
    MethodFamily: RetainSemantics<Receiver, Return, KindSendMessage>,
{
    #[inline]
    unsafe fn send_message<A: ConvertArguments>(receiver: Receiver, sel: Sel, args: A) -> Return {
        let ptr = Self::prepare_message_send(receiver).__as_raw_receiver();

        // SAFETY: The writeback helper is not leaked (it is dropped at the
        // end of this scope).
        let (args, _helper) = unsafe { A::__into_arguments(args) };

        // SAFETY: Upheld by caller.
        let ret = unsafe { MessageReceiver::send_message(ptr, sel, args) };

        // SAFETY: The pointers are valid (or, in the case of the receiver
        // pointer, at least valid when the message send is not `init`).
        unsafe { Self::convert_message_return(ret, ptr, sel) }
    }
}

//
// MsgSendSuper
//

pub trait MsgSendSuper<Receiver, Return> {
    type Inner: ?Sized + RefEncode;

    #[track_caller]
    unsafe fn send_super_message<A: ConvertArguments>(
        receiver: Receiver,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> Return;

    #[inline]
    #[track_caller]
    unsafe fn send_super_message_static<A: ConvertArguments>(
        receiver: Receiver,
        sel: Sel,
        args: A,
    ) -> Return
    where
        Self::Inner: ClassType,
        <Self::Inner as ClassType>::Super: ClassType,
    {
        unsafe {
            Self::send_super_message(
                receiver,
                <Self::Inner as ClassType>::Super::class(),
                sel,
                args,
            )
        }
    }
}

impl<Receiver, Return, MethodFamily> MsgSendSuper<Receiver, Return> for MethodFamily
where
    MethodFamily: RetainSemantics<Receiver, Return, KindSendMessageSuper>,
{
    type Inner = <<MethodFamily as RetainSemantics<Receiver, Return, KindSendMessageSuper>>::ReceiverInner as MessageReceiver>::__Inner;

    #[inline]
    unsafe fn send_super_message<A: ConvertArguments>(
        receiver: Receiver,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> Return {
        let ptr = Self::prepare_message_send(receiver).__as_raw_receiver();

        // SAFETY: The writeback helper is not leaked (it is dropped at the
        // end of this scope).
        let (args, _helper) = unsafe { A::__into_arguments(args) };

        // SAFETY: Upheld by caller.
        let ret = unsafe { MessageReceiver::send_super_message(ptr, superclass, sel, args) };

        // SAFETY: The pointers are valid (or, in the case of the receiver
        // pointer, at least valid when the message send is not `init`).
        unsafe { Self::convert_message_return(ret, ptr, sel) }
    }
}

//
// MsgSendError
//

pub trait MsgSendError<Receiver, Return> {
    /// Add an extra error argument to the argument list, call `send_message`
    /// with that, and return an error if one occurred.
    #[track_caller]
    unsafe fn send_message_error<A, E>(
        receiver: Receiver,
        sel: Sel,
        args: A,
    ) -> Result<Return, Retained<E>>
    where
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: ClassType;
}

// `Option<Retained<T>>` return.
impl<Receiver, Return, MethodFamily> MsgSendError<Receiver, Retained<Return>> for MethodFamily
where
    MethodFamily: MsgSend<Receiver, Option<Retained<Return>>>,
{
    #[inline]
    unsafe fn send_message_error<A, E>(
        receiver: Receiver,
        sel: Sel,
        args: A,
    ) -> Result<Retained<Return>, Retained<E>>
    where
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: ClassType,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        let ret = unsafe { Self::send_message(receiver, sel, args) };
        // As per the Cocoa documentation:
        // > Success or failure is indicated by the return value of the
        // > method. Although Cocoa methods that indirectly return error
        // > objects in the Cocoa error domain are guaranteed to return such
        // > objects if the method indicates failure by directly returning
        // > `nil` or `NO`, you should always check that the return value is
        // > `nil` or `NO` before attempting to do anything with the `NSError`
        // > object.
        if let Some(ret) = ret {
            // In this case, the error is likely not created. If it is, it is
            // autoreleased anyhow, so it would be a waste to retain and
            // release it here.
            Ok(ret)
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

// Bool return.
impl<Receiver, MethodFamily> MsgSendError<Receiver, ()> for MethodFamily
where
    MethodFamily: MsgSend<Receiver, bool>,
{
    #[inline]
    unsafe fn send_message_error<A, E>(
        receiver: Receiver,
        sel: Sel,
        args: A,
    ) -> Result<(), Retained<E>>
    where
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: ClassType,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        let ret = unsafe { Self::send_message(receiver, sel, args) };
        if ret {
            Ok(())
        } else {
            Err(unsafe { encountered_error(err) })
        }
    }
}

//
// MsgSendSuperError
//

pub trait MsgSendSuperError<Receiver, Return> {
    type Inner: ?Sized + RefEncode;

    #[track_caller]
    unsafe fn send_super_message_error<A, E>(
        receiver: Receiver,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> Result<Return, Retained<E>>
    where
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: ClassType;

    #[track_caller]
    #[inline]
    unsafe fn send_super_message_static_error<A, E>(
        receiver: Receiver,
        sel: Sel,
        args: A,
    ) -> Result<Return, Retained<E>>
    where
        Self::Inner: ClassType,
        <Self::Inner as ClassType>::Super: ClassType,
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: ClassType,
    {
        unsafe {
            Self::send_super_message_error(
                receiver,
                <Self::Inner as ClassType>::Super::class(),
                sel,
                args,
            )
        }
    }
}

// `Option<Retained<T>>` return.
impl<Receiver, Return, MethodFamily> MsgSendSuperError<Receiver, Retained<Return>> for MethodFamily
where
    MethodFamily: MsgSendSuper<Receiver, Option<Retained<Return>>>,
{
    type Inner = <MethodFamily as MsgSendSuper<Receiver, Option<Retained<Return>>>>::Inner;

    #[inline]
    unsafe fn send_super_message_error<A, E>(
        receiver: Receiver,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> Result<Retained<Return>, Retained<E>>
    where
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: ClassType,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        // SAFETY: See `send_message_error`
        let ret = unsafe { Self::send_super_message(receiver, superclass, sel, args) };
        if let Some(ret) = ret {
            Ok(ret)
        } else {
            // SAFETY: See `send_message_error`
            Err(unsafe { encountered_error(err) })
        }
    }
}

// Bool return.
impl<Receiver, MethodFamily> MsgSendSuperError<Receiver, ()> for MethodFamily
where
    MethodFamily: MsgSendSuper<Receiver, bool>,
{
    type Inner = <MethodFamily as MsgSendSuper<Receiver, bool>>::Inner;

    #[inline]
    unsafe fn send_super_message_error<A, E>(
        receiver: Receiver,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> Result<(), Retained<E>>
    where
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: ClassType,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        // SAFETY: See `send_message_error`
        let ret = unsafe { Self::send_super_message(receiver, superclass, sel, args) };
        if ret {
            Ok(())
        } else {
            // SAFETY: See `send_message_error`
            Err(unsafe { encountered_error(err) })
        }
    }
}

#[cfg(test)]
mod tests {
    use core::mem::ManuallyDrop;

    use super::*;

    use crate::rc::{autoreleasepool, Allocated, PartialInit, RcTestObject, ThreadTestData};
    use crate::runtime::{AnyObject, NSObject, NSObjectProtocol, NSZone};
    use crate::{class, define_class, extern_methods, msg_send, test_utils, AnyThread};

    #[test]
    fn test_send_message_manuallydrop() {
        let obj = ManuallyDrop::new(test_utils::custom_object());
        unsafe {
            let _: () = msg_send![obj, release];
        };
        // `obj` is consumed, can't use here
    }

    macro_rules! test_error_bool {
        ($expected:expr, $($obj:tt)*) => {
            // Succeeds
            let res: Result<(), Retained<NSObject>> = unsafe {
                msg_send![$($obj)*, boolAndShouldError: false, error: _]
            };
            assert_eq!(res, Ok(()));
            $expected.assert_current();

            // Errors
            let res = autoreleasepool(|_pool| {
                let res: Result<(), Retained<NSObject>> = unsafe {
                    msg_send![$($obj)*, boolAndShouldError: true, error: _]
                };
                let res = res.expect_err("not err");
                $expected.alloc += 1;
                $expected.init += 1;
                $expected.autorelease += 1;
                $expected.retain += 1;
                $expected.assert_current();
                res
            });
            $expected.release += 1;
            $expected.assert_current();

            drop(res);
            $expected.release += 1;
            $expected.drop += 1;
            $expected.assert_current();
        }
    }

    define_class!(
        #[unsafe(super(RcTestObject, NSObject))]
        #[derive(Debug, PartialEq, Eq)]
        struct RcTestObjectSubclass;
    );

    #[cfg_attr(not(test), allow(unused))]
    impl RcTestObjectSubclass {
        fn new() -> Retained<Self> {
            unsafe { msg_send![Self::class(), new] }
        }
    }

    #[test]
    fn test_error_bool() {
        let mut expected = ThreadTestData::current();

        let cls = RcTestObject::class();
        test_error_bool!(expected, cls);

        let obj = RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        test_error_bool!(expected, &obj);

        let obj = RcTestObjectSubclass::new();
        expected.alloc += 1;
        expected.init += 1;
        test_error_bool!(expected, &obj);
        test_error_bool!(expected, super(&obj));
        test_error_bool!(expected, super(&obj, RcTestObjectSubclass::class()));
        test_error_bool!(expected, super(&obj, RcTestObject::class()));
    }

    mod test_trait_disambugated {
        use super::*;

        #[allow(dead_code)]
        trait Abc {
            fn send_message(&self) {}
        }

        impl<T> Abc for T {}

        #[test]
        fn test_macro_still_works() {
            let _: Retained<NSObject> = unsafe { msg_send![NSObject::class(), new] };
        }
    }

    // `new` family

    #[test]
    fn test_new() {
        let mut expected = ThreadTestData::current();
        let cls = RcTestObject::class();

        let _obj: Retained<AnyObject> = unsafe { msg_send![cls, new] };
        let _obj: Option<Retained<AnyObject>> = unsafe { msg_send![cls, new] };
        // This is just a roundabout way of calling `[__RcTestObject new]`.
        let _obj: Retained<AnyObject> = unsafe { msg_send![super(cls, cls.metaclass()), new] };
        let _obj: Option<Retained<AnyObject>> =
            unsafe { msg_send![super(cls, cls.metaclass()), new] };

        // `__RcTestObject` does not override `new`, so this just ends up
        // calling `[[__RcTestObject alloc] init]` as usual.
        let _obj: Retained<RcTestObject> =
            unsafe { msg_send![super(cls, NSObject::class().metaclass()), new] };

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

        let _obj: Retained<AnyObject> = unsafe { msg_send![&obj, newMethodOnInstance] };
        let _obj: Option<Retained<AnyObject>> = unsafe { msg_send![&obj, newMethodOnInstance] };
        let _obj: Retained<AnyObject> =
            unsafe { msg_send![super(&obj, RcTestObject::class()), newMethodOnInstance] };
        let _obj: Option<Retained<AnyObject>> =
            unsafe { msg_send![super(&obj, RcTestObject::class()), newMethodOnInstance] };
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
        let key: Retained<AnyObject> = unsafe { msg_send![class!(NSString), new] };
        let contents_value: *const AnyObject = ptr::null();
        let properties: Retained<AnyObject> = unsafe { msg_send![class!(NSDictionary), new] };

        let _obj: Option<Retained<AnyObject>> = unsafe {
            msg_send![
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
        let _val: Retained<AnyObject> = unsafe { msg_send![class!(NSValue), new] };
    }

    #[test]
    #[should_panic = "failed creating new instance using +[__RcTestObject newReturningNull]"]
    fn test_new_with_null() {
        let _obj: Retained<RcTestObject> =
            unsafe { msg_send![RcTestObject::class(), newReturningNull] };
    }

    #[test]
    #[should_panic = "failed creating new instance using +[__RcTestObject newReturningNull]"]
    fn test_super_new_with_null() {
        let _: Retained<RcTestObject> = unsafe {
            msg_send![
                super(RcTestObject::class(), RcTestObject::class().metaclass()),
                newReturningNull
            ]
        };
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject newMethodOnInstanceNull]"]
    fn test_new_any_with_null() {
        let obj = RcTestObject::new();
        let _obj: Retained<AnyObject> = unsafe { msg_send![&obj, newMethodOnInstanceNull] };
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject newMethodOnInstanceNull]"]
    fn test_super_new_any_with_null() {
        let obj = RcTestObject::new();
        let _obj: Retained<AnyObject> =
            unsafe { msg_send![super(&obj, RcTestObject::class()), newMethodOnInstanceNull] };
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
        let _obj: Retained<AnyObject> = unsafe { msg_send![obj, newMethodOnInstance] };
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
        let _obj: Retained<AnyObject> = unsafe { msg_send![super(obj), newMethodOnInstance] };
    }

    // `alloc` family

    #[test]
    fn test_alloc() {
        let mut expected = ThreadTestData::current();
        let cls = RcTestObject::class();

        let obj: Allocated<RcTestObject> = unsafe { msg_send![cls, alloc] };
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
            unsafe { msg_send![super(cls, NSObject::class().metaclass()), alloc] };
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

        let _obj: Allocated<RcTestObject> = unsafe { msg_send![cls, allocWithZone: zone] };
        expected.alloc += 1;
        expected.assert_current();

        let _obj: Allocated<RcTestObject> =
            unsafe { msg_send![super(cls, cls.metaclass()), allocWithZone: zone] };
        expected.alloc += 1;
        expected.assert_current();

        let _obj: Allocated<NSObject> =
            unsafe { msg_send![super(cls, NSObject::class().metaclass()), allocWithZone: zone] };
        expected.assert_current();
    }

    #[test]
    fn test_alloc_with_null() {
        let obj: Allocated<RcTestObject> =
            unsafe { msg_send![RcTestObject::class(), allocReturningNull] };
        assert!(Allocated::as_ptr(&obj).is_null());
    }

    // `init` family

    #[test]
    fn test_init() {
        let mut expected = ThreadTestData::current();

        let _: Retained<RcTestObject> = unsafe { msg_send![RcTestObject::alloc(), init] };
        expected.alloc += 1;
        expected.init += 1;
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();

        let obj = RcTestObject::alloc().set_ivars(());
        let _: Retained<RcTestObject> = unsafe { msg_send![super(obj), init] };
        expected.alloc += 1;
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();

        // Check allocation error before init
        let obj = RcTestObject::alloc();
        expected.alloc += 1;
        assert!(!Allocated::as_ptr(&obj).is_null());
        let _: Retained<RcTestObject> = unsafe { msg_send![obj, init] };
        expected.init += 1;
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();
    }

    #[test]
    #[should_panic = "failed initializing object with -initReturningNull"]
    fn test_init_with_null() {
        let obj: Allocated<RcTestObject> = unsafe { msg_send![RcTestObject::class(), alloc] };
        let _obj: Retained<RcTestObject> = unsafe { msg_send![obj, initReturningNull] };
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic = "messsaging init to nil")]
    #[cfg_attr(not(debug_assertions), ignore = "failed allocating object")]
    fn test_init_with_null_receiver() {
        let obj: Allocated<RcTestObject> =
            unsafe { msg_send![RcTestObject::class(), allocReturningNull] };
        let _obj: Retained<RcTestObject> = unsafe { msg_send![obj, init] };
    }

    #[test]
    #[should_panic = "tried to initialize ivars after they were already initialized"]
    #[cfg_attr(not(debug_assertions), ignore = "only checked with debug assertions")]
    fn test_super_init_not_initialized() {
        let obj = RcTestObject::alloc().set_ivars(());
        let _: Retained<RcTestObject> =
            unsafe { msg_send![super(obj, RcTestObject::class()), init] };
    }

    #[test]
    #[should_panic = "tried to finalize an already finalized object"]
    #[cfg_attr(not(debug_assertions), ignore = "only checked with debug assertions")]
    fn test_super_init_not_finalized() {
        let obj = unsafe { PartialInit::new(Allocated::into_ptr(RcTestObject::alloc())) };
        let _: Retained<RcTestObject> =
            unsafe { msg_send![super(obj, RcTestObject::class()), init] };
    }

    // `copy` family

    #[test]
    fn test_copy() {
        let obj = RcTestObject::new();
        let mut expected = ThreadTestData::current();

        let _: Retained<RcTestObject> = unsafe { msg_send![&obj, copy] };
        expected.copy += 1;
        expected.alloc += 1;
        expected.init += 1;
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();

        // `+[NSObject copy]` forwards to `copyWithZone:`, so this still
        // creates a `__RcTestObject`.
        let _: Retained<NSObject> = unsafe { msg_send![super(&obj), copy] };
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
        let _obj: Retained<RcTestObject> = unsafe { msg_send![&obj, copyReturningNull] };
    }

    #[test]
    #[should_panic = "failed copying object"]
    fn test_super_copy_with_null() {
        let obj = RcTestObject::new();
        let _obj: Retained<RcTestObject> =
            unsafe { msg_send![super(&obj, RcTestObject::class()), copyReturningNull] };
    }

    // `mutableCopy` family

    #[test]
    fn test_mutable_copy() {
        let obj = RcTestObject::new();
        let mut expected = ThreadTestData::current();

        let _: Retained<RcTestObject> = unsafe { msg_send![&obj, mutableCopy] };
        expected.mutable_copy += 1;
        expected.alloc += 1;
        expected.init += 1;
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();

        // `+[NSObject mutableCopy]` forwards to `mutableCopyWithZone:`, so
        // this still creates a `__RcTestObject`.
        let _: Retained<NSObject> = unsafe { msg_send![super(&obj), mutableCopy] };
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

        let _: Retained<RcTestObject> = unsafe { msg_send![&obj, self] };
        expected.retain += 1;
        expected.release += 1;
        expected.assert_current();

        let _: Retained<RcTestObject> = unsafe { msg_send![super(&obj), self] };
        expected.retain += 1;
        expected.release += 1;
        expected.assert_current();

        let _: Option<Retained<RcTestObject>> = unsafe { msg_send![&obj, description] };
        expected.assert_current();

        let _: Option<Retained<RcTestObject>> = unsafe { msg_send![super(&obj), description] };
        expected.assert_current();
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject methodReturningNull]"]
    fn test_normal_with_null() {
        let obj = RcTestObject::new();
        let _obj: Retained<RcTestObject> = unsafe { msg_send![&obj, methodReturningNull] };
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject aMethod:]"]
    fn test_normal_with_param_and_null() {
        let obj = RcTestObject::new();
        let _obj: Retained<RcTestObject> = unsafe { msg_send![&obj, aMethod: false] };
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic = "messsaging description to nil")]
    #[cfg_attr(
        not(debug_assertions),
        ignore = "unexpected NULL description; receiver was NULL"
    )]
    fn test_normal_with_null_receiver() {
        let obj: *const NSObject = ptr::null();
        let _obj: Retained<AnyObject> = unsafe { msg_send![obj, description] };
    }

    /// This is imperfect, but will do for now.
    const fn autorelease_skipped(self_declared: bool) -> bool {
        if cfg!(feature = "gnustep-1-7") {
            // GNUStep does the optimization a different way, so it isn't
            // optimization-dependent.
            true
        } else if cfg!(all(target_arch = "arm", panic = "unwind")) {
            // 32-bit ARM unwinding sometimes interferes with the optimization
            false
        } else if self_declared {
            // FIXME: Autorelease_return is not currently tail-called, so the
            // optimization doesn't work on define_class! functions.
            false
        } else if cfg!(feature = "catch-all") {
            // FIXME: `catch-all` is inserted before we get a chance to retain.
            false
        } else if cfg!(debug_assertions) {
            // `debug_assertions` ~proxy for if optimizations are off.
            false
        } else {
            true
        }
    }

    macro_rules! test_error_retained {
        ($expected:expr, $if_autorelease_not_skipped:expr, $sel:ident, $($obj:tt)*) => {
            // Succeeds
            let res = autoreleasepool(|_pool| {
                let res: Result<Retained<RcTestObject>, Retained<NSObject>> = unsafe {
                    msg_send![$($obj)*, $sel: false, error: _]
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
                let res: Result<Retained<RcTestObject>, Retained<NSObject>> = unsafe {
                    msg_send![$($obj)*, $sel: true, error: _]
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
    fn test_error_retained() {
        let mut expected = ThreadTestData::current();

        let cls = RcTestObject::class();
        test_error_retained!(
            expected,
            if autorelease_skipped(true) { 0 } else { 1 },
            idAndShouldError,
            cls
        );
        test_error_retained!(expected, 0, newAndShouldError, cls);

        let obj = RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        test_error_retained!(
            expected,
            if autorelease_skipped(true) { 0 } else { 1 },
            idAndShouldError,
            &obj
        );

        expected.alloc -= 1;
        expected.release -= 1;
        test_error_retained!(expected, 0, initAndShouldError, {
            expected.alloc += 1;
            expected.release += 1;
            // Drop flag ensures newly allocated objects do not drop
            // expected.drop += 1;
            RcTestObject::alloc()
        });
    }

    #[test]
    fn test_method_with_param() {
        let mut expected = ThreadTestData::current();

        let obj = RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();

        let res: Option<Retained<RcTestObject>> = unsafe { msg_send![&obj, aMethod: false] };
        assert!(res.is_none());
        expected.assert_current();

        let _res = autoreleasepool(|_pool| {
            let res: Option<Retained<RcTestObject>> = unsafe { msg_send![&obj, aMethod: true] };
            assert!(res.is_some());
            expected.alloc += 1;
            expected.init += 1;
            expected.autorelease += if autorelease_skipped(true) { 0 } else { 1 };
            expected.retain += if autorelease_skipped(true) { 0 } else { 1 };
            expected.assert_current();
            res
        });
        expected.release += if autorelease_skipped(true) { 0 } else { 1 };
        expected.assert_current();
    }

    fn create_obj() -> Retained<NSObject> {
        let obj = ManuallyDrop::new(NSObject::new());
        unsafe {
            let obj: *mut NSObject = msg_send![&*obj, autorelease];
            // All code between the `msg_send!` and the `retain_autoreleased`
            // must be able to be optimized away for this to work.
            Retained::retain_autoreleased(obj).unwrap()
        }
    }

    #[test]
    fn test_retain_autoreleased() {
        autoreleasepool(|_| {
            // Run once to allow DYLD to resolve the symbol stubs.
            // Required for making `retain_autoreleased` work on x86_64.
            let _data = create_obj();

            // When compiled in release mode / with optimizations enabled,
            // subsequent usage of `retain_autoreleased` will succeed in
            // retaining the autoreleased value!
            let expected = if autorelease_skipped(false) { 1 } else { 2 };

            let data = create_obj();
            assert_eq!(data.retainCount(), expected);

            let data = create_obj();
            assert_eq!(data.retainCount(), expected);

            // Here we manually clean up the autorelease, so it will always be 1.
            let data = autoreleasepool(|_| create_obj());
            assert_eq!(data.retainCount(), 1);
        });
    }

    #[test]
    fn msg_send_class() {
        let cls = NSObject::class();

        let retained: Retained<AnyClass> = unsafe { msg_send![cls, self] };
        assert_eq!(&*retained, cls);

        let retained: Option<Retained<AnyClass>> = unsafe { msg_send![cls, self] };
        let retained = retained.unwrap();
        assert_eq!(&*retained, cls);
    }

    impl RcTestObject {
        extern_methods!(
            #[unsafe(method(copy))]
            #[unsafe(method_family = new)]
            fn copy_new(&self) -> Retained<Self>;

            #[unsafe(method(copy))]
            #[unsafe(method_family = init)]
            fn copy_init(this: Allocated<Self>) -> Retained<Self>;

            #[unsafe(method(copy))]
            #[unsafe(method_family = copy)]
            fn copy_copy(&self) -> Retained<Self>;

            #[unsafe(method(copy))]
            #[unsafe(method_family = mutableCopy)]
            fn copy_mutable_copy(&self) -> Retained<Self>;

            #[unsafe(method(copy))]
            #[unsafe(method_family = none)]
            fn copy_none(&self) -> Retained<Self>;
        );
    }

    #[test]
    fn test_method_family() {
        // Test a few combinations of (incorrect) method families.
        let obj = RcTestObject::new();
        let mut expected = ThreadTestData::current();

        let copy = obj.copy_new();
        expected.copy += 1;
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();
        drop(copy);
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();

        let alloc = RcTestObject::alloc();
        let ptr = Allocated::as_ptr(&alloc);
        expected.alloc += 1;
        expected.assert_current();
        let copy = RcTestObject::copy_init(alloc);
        expected.copy += 1;
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();
        drop(copy);
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();
        drop(unsafe { Allocated::new(ptr.cast_mut()) });
        expected.release += 1;
        expected.assert_current();

        let copy = obj.copy_copy();
        expected.copy += 1;
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();
        drop(copy);
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();

        let copy = obj.copy_mutable_copy();
        expected.copy += 1;
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();
        drop(copy);
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();

        let copy = obj.copy_none();
        expected.copy += 1;
        expected.alloc += 1;
        expected.init += 1;
        expected.retain += 1;
        expected.assert_current();
        // SAFETY: Wrong method family specified, so we have +1 retain count
        // in excess.
        drop(unsafe { Retained::from_raw(Retained::as_ptr(&copy).cast_mut()) });
        expected.release += 1;
        expected.assert_current();
        drop(copy);
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();
    }
}
