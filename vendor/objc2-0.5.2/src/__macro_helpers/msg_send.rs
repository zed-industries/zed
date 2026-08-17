use core::mem::ManuallyDrop;
use core::ptr;

use crate::encode::RefEncode;
use crate::mutability::IsMutable;
use crate::rc::Retained;
use crate::runtime::{AnyClass, AnyObject, MessageReceiver, Sel};
use crate::{ClassType, Encode, Message};

use super::{ConvertArguments, ConvertReturn, TupleExtender};

pub trait MsgSend: Sized {
    type Inner: ?Sized + RefEncode;

    fn into_raw_receiver(self) -> *mut AnyObject;

    #[inline]
    #[track_caller]
    unsafe fn send_message<A, R>(self, sel: Sel, args: A) -> R
    where
        A: ConvertArguments,
        R: ConvertReturn,
    {
        let (args, stored) = A::__into_arguments(args);

        // SAFETY: Upheld by caller
        let result = unsafe { MessageReceiver::send_message(self.into_raw_receiver(), sel, args) };

        // TODO: If we want `objc_retainAutoreleasedReturnValue` to
        // work, we must not do any work before it has been run; so
        // somehow, we should do that _before_ this call!
        //
        // SAFETY: The argument was passed to the message sending
        // function, and the stored values are only processed this
        // once. See `src/__macro_helpers/writeback.rs` for
        // details.
        unsafe { A::__process_after_message_send(stored) };

        R::__from_return(result)
    }

    #[inline]
    #[track_caller]
    unsafe fn send_super_message<A, R>(self, superclass: &AnyClass, sel: Sel, args: A) -> R
    where
        A: ConvertArguments,
        R: ConvertReturn,
    {
        let (args, stored) = A::__into_arguments(args);

        // SAFETY: Upheld by caller
        let result = unsafe {
            MessageReceiver::send_super_message(self.into_raw_receiver(), superclass, sel, args)
        };

        // SAFETY: Same as in send_message above.
        unsafe { A::__process_after_message_send(stored) };

        R::__from_return(result)
    }

    #[inline]
    #[track_caller]
    unsafe fn send_super_message_static<A, R>(self, sel: Sel, args: A) -> R
    where
        Self::Inner: ClassType,
        <Self::Inner as ClassType>::Super: ClassType,
        A: ConvertArguments,
        R: ConvertReturn,
    {
        unsafe { self.send_super_message(<Self::Inner as ClassType>::Super::class(), sel, args) }
    }

    // Error functions below. See MsgSendId::send_message_id_error for further
    // details.
    //
    // Some of this could be abstracted away using closures, but that would
    // interfere with `#[track_caller]`, so we avoid doing that.

    #[inline]
    #[track_caller]
    unsafe fn send_message_error<A, E>(self, sel: Sel, args: A) -> Result<(), Retained<E>>
    where
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: Message,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        let res: bool = unsafe { self.send_message(sel, args) };
        if res {
            Ok(())
        } else {
            Err(unsafe { encountered_error(err) })
        }
    }

    #[inline]
    #[track_caller]
    unsafe fn send_super_message_error<A, E>(
        self,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> Result<(), Retained<E>>
    where
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: Message,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        let res: bool = unsafe { self.send_super_message(superclass, sel, args) };
        if res {
            Ok(())
        } else {
            Err(unsafe { encountered_error(err) })
        }
    }

    #[inline]
    #[track_caller]
    unsafe fn send_super_message_static_error<A, E>(
        self,
        sel: Sel,
        args: A,
    ) -> Result<(), Retained<E>>
    where
        Self::Inner: ClassType,
        <Self::Inner as ClassType>::Super: ClassType,
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: Message,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        let res: bool = unsafe { self.send_super_message_static(sel, args) };
        if res {
            Ok(())
        } else {
            Err(unsafe { encountered_error(err) })
        }
    }
}

#[cold]
#[track_caller]
unsafe fn encountered_error<E: Message>(err: *mut E) -> Retained<E> {
    // SAFETY: Ensured by caller
    unsafe { Retained::retain(err) }
        .expect("error parameter should be set if the method returns NO")
}

impl<T: ?Sized + MessageReceiver> MsgSend for T {
    type Inner = T::__Inner;

    #[inline]
    fn into_raw_receiver(self) -> *mut AnyObject {
        MessageReceiver::__as_raw_receiver(self)
    }
}

impl<'a, T: ?Sized + Message> MsgSend for &'a Retained<T> {
    type Inner = T;

    #[inline]
    fn into_raw_receiver(self) -> *mut AnyObject {
        (Retained::as_ptr(self) as *mut T).cast()
    }
}

impl<'a, T: ?Sized + Message + IsMutable> MsgSend for &'a mut Retained<T> {
    type Inner = T;

    #[inline]
    fn into_raw_receiver(self) -> *mut AnyObject {
        Retained::as_mut_ptr(self).cast()
    }
}

impl<T: ?Sized + Message> MsgSend for ManuallyDrop<Retained<T>> {
    type Inner = T;

    #[inline]
    fn into_raw_receiver(self) -> *mut AnyObject {
        Retained::into_raw(ManuallyDrop::into_inner(self)).cast()
    }
}

#[cfg(test)]
mod tests {
    use crate::rc::{autoreleasepool, RcTestObject, ThreadTestData};
    use crate::runtime::NSObject;
    use crate::{
        declare_class, msg_send, msg_send_id, mutability, test_utils, ClassType, DeclaredClass,
    };

    use super::*;

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
            let res: Result<(), Retained<RcTestObject>> = unsafe {
                msg_send![$($obj)*, boolAndShouldError: false, error: _]
            };
            assert_eq!(res, Ok(()));
            $expected.assert_current();

            // Errors
            let res = autoreleasepool(|_pool| {
                // `Ok` type is inferred to be `()`
                let res: Retained<RcTestObject> = unsafe {
                    msg_send![$($obj)*, boolAndShouldError: true, error: _]
                }.expect_err("not err");
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

    declare_class!(
        #[derive(Debug, PartialEq, Eq)]
        struct RcTestObjectSubclass;

        unsafe impl ClassType for RcTestObjectSubclass {
            #[inherits(NSObject)]
            type Super = RcTestObject;
            type Mutability = mutability::Immutable;
            const NAME: &'static str = "RcTestObjectSubclass";
        }

        impl DeclaredClass for RcTestObjectSubclass {}
    );

    #[cfg_attr(not(test), allow(unused))]
    impl RcTestObjectSubclass {
        fn new() -> Retained<Self> {
            unsafe { msg_send_id![Self::class(), new] }
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
}
