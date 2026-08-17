use core::mem::ManuallyDrop;

use crate::encode::EncodeReturn;
use crate::rc::{Allocated, PartialInit, Retained};
use crate::runtime::{AnyClass, AnyObject, MessageReceiver, Sel};
use crate::{sel, DefinedClass, Message};

use super::defined_ivars::set_finalized;
use super::{
    AllocFamily, ConvertReturn, CopyFamily, InitFamily, MutableCopyFamily, NewFamily, NoneFamily,
};

// TODO: Differentiate between instance and class methods?

#[derive(Debug, Copy, Clone)]
pub struct KindSendMessageSuper;

#[derive(Debug, Copy, Clone)]
pub struct KindSendMessage;

#[derive(Debug, Copy, Clone)]
pub struct KindDefined;

trait SendMessage {}

impl SendMessage for KindSendMessage {}
impl SendMessage for KindSendMessageSuper {}

trait NotSuper {}

impl NotSuper for KindSendMessage {}
impl NotSuper for KindDefined {}

trait IsSuper {}

impl IsSuper for KindSendMessageSuper {}

/// Trait for restricting the possible Receiver/Return type combinations.
///
/// This is used to convert receiver and return types in `extern_methods!`
/// and `msg_send!`.
///
/// Having both the receiver, return type and whether the message send is a
/// super-message send is especially important for `init` methods, which must
/// return the same type as they were called with, and must restrict the
/// receiver to either `Allocated` or `PartialInit` depending on the super-ness.
///
/// # Summary
///
/// ```text
/// new: Receiver -> Option<Retained<Return>>
/// alloc: &AnyClass -> Allocated<Return>
/// init: Allocated<T> -> Option<Retained<T>>
/// init super: PartialInit<T> -> Option<Retained<T>>
/// copy: Receiver -> Option<Retained<Return>>
/// mutableCopy: Receiver -> Option<Retained<Return>>
/// others: Receiver -> Option<Retained<Return>>
/// ```
pub trait RetainSemantics<Receiver, Return, Kind>: Sized {
    /// The inner receiver type that is used directly at the ABI boundary of
    /// the message send.
    type ReceiverInner: MessageReceiver;

    /// Prepare the receiver for sending a message.
    fn prepare_message_send(receiver: Receiver) -> Self::ReceiverInner;

    /// Prepare the receiver for receiving a message.
    ///
    ///
    /// # Safety
    ///
    /// The receiver must be valid.
    unsafe fn prepare_defined_method(receiver: Self::ReceiverInner) -> Receiver;

    /// The inner return type that is used directly at the ABI boundary of the
    /// message send.
    type ReturnInner: EncodeReturn;

    /// Convert a received return value according to the specified retain
    /// semantics.
    ///
    ///
    /// # Panics
    ///
    /// If conversion fails (such as when converting to `Retained<T>`), this
    /// uses the receiver and selector to panic with a nice error message.
    ///
    /// NOTE: The behavior that returning `Retained<T>` always unwraps
    /// instead of using `unwrap_unchecked` is relied upon by
    /// `header-translator` for soundness, see e.g. `parse_property_return`.
    ///
    ///
    /// # Safety
    ///
    /// The return value must be valid, and the receiver pointer must have
    /// come from `prepare_message_send`.
    #[track_caller]
    unsafe fn convert_message_return(
        ret: Self::ReturnInner,
        receiver_ptr: *mut AnyObject,
        sel: Sel,
    ) -> Return;

    // TODO: Use this in `define_class!`.
    fn convert_defined_return(ret: Return) -> Self::ReturnInner;
}

//
// Implementations for all method families.
//

/// Generic implementation allowing message sending to normal encode return
/// types.
impl<Receiver, Return, Kind, MethodFamily> RetainSemantics<Receiver, Return, Kind> for MethodFamily
where
    Receiver: MessageReceiver,
    Return: ConvertReturn<MethodFamily>,
{
    type ReceiverInner = Receiver;

    #[inline]
    fn prepare_message_send(receiver: Receiver) -> Receiver {
        receiver
    }

    #[inline]
    unsafe fn prepare_defined_method(receiver: Receiver) -> Receiver {
        receiver
    }

    type ReturnInner = Return::Inner;

    #[inline]
    unsafe fn convert_message_return(
        ret: Return::Inner,
        receiver_ptr: *mut AnyObject,
        sel: Sel,
    ) -> Return {
        // SAFETY: Upheld by caller.
        unsafe { Return::convert_message_return(ret, receiver_ptr, sel) }
    }

    #[inline]
    fn convert_defined_return(ret: Return) -> Return::Inner {
        Return::convert_defined_return(ret)
    }
}

/// Convenience implementation for sending messages to `&Retained<T>`.
impl<'a, T, Return, Kind, MethodFamily> RetainSemantics<&'a Retained<T>, Return, Kind>
    for MethodFamily
where
    T: Message,
    // Only allow on message sends, this won't work in `define_class!`.
    Kind: SendMessage,
    MethodFamily: RetainSemantics<*const T, Return, Kind>,
{
    type ReceiverInner = *const T;

    #[inline]
    fn prepare_message_send(receiver: &'a Retained<T>) -> *const T {
        &**receiver
    }

    #[inline]
    unsafe fn prepare_defined_method(_receiver: *const T) -> &'a Retained<T> {
        // Should be prevented statically by `Kind: SendMessage`.
        unreachable!()
    }

    type ReturnInner = <Self as RetainSemantics<*const T, Return, Kind>>::ReturnInner;

    #[inline]
    unsafe fn convert_message_return(
        ret: Self::ReturnInner,
        receiver_ptr: *mut AnyObject,
        sel: Sel,
    ) -> Return {
        unsafe {
            <Self as RetainSemantics<*const T, Return, Kind>>::convert_message_return(
                ret,
                receiver_ptr,
                sel,
            )
        }
    }

    #[inline]
    fn convert_defined_return(ret: Return) -> Self::ReturnInner {
        <Self as RetainSemantics<*const T, Return, Kind>>::convert_defined_return(ret)
    }
}

/// Convenience implementation for sending messages to `ManuallyDrop<Retained<T>>`.
impl<T, Return, Kind, MethodFamily> RetainSemantics<ManuallyDrop<Retained<T>>, Return, Kind>
    for MethodFamily
where
    T: Message,
    // Only allow on message sends, this doesn't make sense to do in
    // `define_class!`.
    Kind: SendMessage,
    MethodFamily: RetainSemantics<*mut T, Return, Kind>,
{
    type ReceiverInner = *mut T;

    #[inline]
    fn prepare_message_send(receiver: ManuallyDrop<Retained<T>>) -> *mut T {
        Retained::into_raw(ManuallyDrop::into_inner(receiver))
    }

    #[inline]
    unsafe fn prepare_defined_method(_receiver: *mut T) -> ManuallyDrop<Retained<T>> {
        // Should be prevented statically by `Kind: SendMessage`.
        unreachable!()
    }

    type ReturnInner = <Self as RetainSemantics<*mut T, Return, Kind>>::ReturnInner;

    #[inline]
    unsafe fn convert_message_return(
        ret: Self::ReturnInner,
        receiver_ptr: *mut AnyObject,
        sel: Sel,
    ) -> Return {
        unsafe {
            <Self as RetainSemantics<*mut T, Return, Kind>>::convert_message_return(
                ret,
                receiver_ptr,
                sel,
            )
        }
    }

    #[inline]
    fn convert_defined_return(ret: Return) -> Self::ReturnInner {
        <Self as RetainSemantics<*mut T, Return, Kind>>::convert_defined_return(ret)
    }
}

//
// NewFamily
//
// Receiver -> Option<Retained<Return>>
// Receiver -> Retained<Return>
//

impl<T: Message> ConvertReturn<NewFamily> for Option<Retained<T>> {
    type Inner = *mut T;

    #[inline]
    unsafe fn convert_message_return(
        inner: Self::Inner,
        _receiver_ptr: *mut AnyObject,
        _sel: Sel,
    ) -> Self {
        // SAFETY: The selector is `new`, so this has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        unsafe { Retained::from_raw(inner) }
    }

    #[inline]
    fn convert_defined_return(self) -> Self::Inner {
        // Return with +1 retain count.
        Retained::consume_as_ptr_option(self)
    }
}

impl<T: Message> ConvertReturn<NewFamily> for Retained<T> {
    type Inner = *mut T;

    #[inline]
    unsafe fn convert_message_return(
        inner: Self::Inner,
        receiver_ptr: *mut AnyObject,
        sel: Sel,
    ) -> Self {
        // SAFETY: The selector is `new`, so this has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        let ret = unsafe { Retained::from_raw(inner) };
        if let Some(ret) = ret {
            ret
        } else {
            // SAFETY: The receiver is still valid after a message send to
            // a `new` method - it would not be if the method was `init`.
            let receiver = unsafe { receiver_ptr.as_ref() };
            new_fail(receiver, sel)
        }
    }

    #[inline]
    fn convert_defined_return(self) -> Self::Inner {
        // Return with +1 retain count.
        Retained::into_raw(self)
    }
}

#[cold]
#[track_caller]
fn new_fail(receiver: Option<&AnyObject>, sel: Sel) -> ! {
    if let Some(receiver) = receiver {
        let cls = receiver.class();
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

//
// AllocFamily
//
// &AnyClass -> Allocated<Return>
//

// Restrict `alloc` methods to the simplest case of a class reference. This
// could possibly be relaxed in the future, but there's likely no need for
// that.
impl<'a, Return, Kind> RetainSemantics<&'a AnyClass, Allocated<Return>, Kind> for AllocFamily
where
    Return: Message,
{
    type ReceiverInner = &'a AnyClass;

    #[inline]
    fn prepare_message_send(receiver: &'a AnyClass) -> &'a AnyClass {
        receiver
    }

    #[inline]
    unsafe fn prepare_defined_method(receiver: &'a AnyClass) -> &'a AnyClass {
        receiver
    }

    type ReturnInner = *mut Return;

    #[inline]
    unsafe fn convert_message_return(
        ret: *mut Return,
        _receiver_ptr: *mut AnyObject,
        _sel: Sel,
    ) -> Allocated<Return> {
        // SAFETY: The selector is `alloc`, so this has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        unsafe { Allocated::new(ret) }
    }

    #[inline]
    fn convert_defined_return(ret: Allocated<Return>) -> *mut Return {
        // Return with +1 retain count.
        Allocated::into_ptr(ret)
    }
}

//
// InitFamily
//
// We restrict `init` methods such that, if they return `Retained<T>`, the
// receiver must be `Allocated<T>` with the same generic parameter.
//
// Simple return types have no restriction.
//
// See <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#method-families>
//
// normal:
// Allocated<T> -> Option<Retained<T>>
// Allocated<T> -> Retained<T>
//
// super:
// PartialInit<T> -> Option<Retained<T>>
// PartialInit<T> -> Retained<T>
//

impl<T, Kind> RetainSemantics<Allocated<T>, Option<Retained<T>>, Kind> for InitFamily
where
    T: Message,
    Kind: NotSuper,
{
    type ReceiverInner = *mut T;

    #[inline]
    fn prepare_message_send(receiver: Allocated<T>) -> *mut T {
        // Pass the +1 retain count to the callee.
        Allocated::into_ptr(receiver)
    }

    #[inline]
    unsafe fn prepare_defined_method(receiver: *mut T) -> Allocated<T> {
        // SAFETY: The receiver of an `init` method is `Allocated` and
        // consumed, and thus has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        unsafe { Allocated::new(receiver) }
    }

    type ReturnInner = *mut T;

    #[inline]
    unsafe fn convert_message_return(
        ret: *mut T,
        _receiver_ptr: *mut AnyObject,
        _sel: Sel,
    ) -> Option<Retained<T>> {
        // SAFETY: The selector is `init`, so this has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        unsafe { Retained::from_raw(ret) }
    }

    #[inline]
    fn convert_defined_return(ret: Option<Retained<T>>) -> *mut T {
        // Return with +1 retain count.
        Retained::consume_as_ptr_option(ret)
    }
}

impl<T, Kind> RetainSemantics<Allocated<T>, Retained<T>, Kind> for InitFamily
where
    T: Message,
    Kind: NotSuper,
{
    type ReceiverInner = *mut T;

    #[inline]
    fn prepare_message_send(receiver: Allocated<T>) -> *mut T {
        // Pass the +1 retain count to the callee.
        Allocated::into_ptr(receiver)
    }

    #[inline]
    unsafe fn prepare_defined_method(receiver: *mut T) -> Allocated<T> {
        // SAFETY: The receiver of an `init` method is `Allocated` and
        // consumed, and thus has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        unsafe { Allocated::new(receiver) }
    }

    type ReturnInner = *mut T;

    #[inline]
    unsafe fn convert_message_return(
        ret: *mut T,
        receiver_ptr: *mut AnyObject,
        sel: Sel,
    ) -> Retained<T> {
        // SAFETY: The selector is `init`, so this has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        let ret = unsafe { Retained::from_raw(ret) };
        if let Some(ret) = ret {
            ret
        } else {
            init_fail(receiver_ptr, sel)
        }
    }

    #[inline]
    fn convert_defined_return(ret: Retained<T>) -> *mut T {
        // Return with +1 retain count.
        Retained::into_raw(ret)
    }
}

impl<T, Kind> RetainSemantics<PartialInit<T>, Option<Retained<T>>, Kind> for InitFamily
where
    T: DefinedClass,
    Kind: IsSuper,
{
    type ReceiverInner = *mut T;

    #[inline]
    fn prepare_message_send(receiver: PartialInit<T>) -> *mut T {
        // Pass the +1 retain count to the callee.
        PartialInit::into_ptr(receiver)
    }

    #[inline]
    unsafe fn prepare_defined_method(receiver: *mut T) -> PartialInit<T> {
        // SAFETY: The receiver of a super `init` method is `PartialInit` and
        // consumed, and thus has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        unsafe { PartialInit::new(receiver) }
    }

    type ReturnInner = *mut T;

    #[inline]
    unsafe fn convert_message_return(
        ret: *mut T,
        _receiver_ptr: *mut AnyObject,
        _sel: Sel,
    ) -> Option<Retained<T>> {
        // SAFETY: The selector is `init`, so this has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        let ret = unsafe { Retained::from_raw(ret) };
        if let Some(ret) = ret {
            // SAFETY: We just got back from `init`, so the super initializer
            // will have run.
            //
            // Caller ensures that the pointer is valid.
            unsafe { set_finalized(ret.as_nonnull_ptr()) };

            Some(ret)
        } else {
            None
        }
    }

    #[inline]
    fn convert_defined_return(ret: Option<Retained<T>>) -> *mut T {
        // Return with +1 retain count.
        Retained::consume_as_ptr_option(ret)
    }
}

impl<T, Kind> RetainSemantics<PartialInit<T>, Retained<T>, Kind> for InitFamily
where
    T: DefinedClass,
    Kind: IsSuper,
{
    type ReceiverInner = *mut T;

    #[inline]
    fn prepare_message_send(receiver: PartialInit<T>) -> *mut T {
        // Pass the +1 retain count to the callee.
        PartialInit::into_ptr(receiver)
    }

    #[inline]
    unsafe fn prepare_defined_method(receiver: *mut T) -> PartialInit<T> {
        // SAFETY: The receiver of a super `init` method is `PartialInit` and
        // consumed, and thus has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        unsafe { PartialInit::new(receiver) }
    }

    type ReturnInner = *mut T;

    #[inline]
    unsafe fn convert_message_return(
        ret: *mut T,
        receiver_ptr: *mut AnyObject,
        sel: Sel,
    ) -> Retained<T> {
        // SAFETY: The selector is `init`, so this has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        let ret = unsafe { Retained::from_raw(ret) };
        if let Some(ret) = ret {
            // SAFETY: We just got back from `init`, so the super initializer
            // will have run.
            //
            // Caller ensures that the pointer is valid.
            unsafe { set_finalized(ret.as_nonnull_ptr()) };

            ret
        } else {
            init_fail(receiver_ptr, sel)
        }
    }

    #[inline]
    fn convert_defined_return(ret: Retained<T>) -> *mut T {
        // Return with +1 retain count.
        Retained::into_raw(ret)
    }
}

#[cold]
#[track_caller]
fn init_fail(receiver: *mut AnyObject, sel: Sel) -> ! {
    if receiver.is_null() {
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

//
// CopyFamily
//
// Receiver -> Option<Retained<T>>
// Receiver -> Retained<T>
//

impl<T: Message> ConvertReturn<CopyFamily> for Option<Retained<T>> {
    type Inner = *mut T;

    #[inline]
    unsafe fn convert_message_return(
        inner: Self::Inner,
        _receiver_ptr: *mut AnyObject,
        _sel: Sel,
    ) -> Self {
        // SAFETY: The selector is `copy`, so this has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        unsafe { Retained::from_raw(inner) }
    }

    #[inline]
    fn convert_defined_return(self) -> Self::Inner {
        // Return with +1 retain count.
        Retained::consume_as_ptr_option(self)
    }
}

impl<T: Message> ConvertReturn<CopyFamily> for Retained<T> {
    type Inner = *mut T;

    #[inline]
    unsafe fn convert_message_return(
        inner: Self::Inner,
        _receiver_ptr: *mut AnyObject,
        _sel: Sel,
    ) -> Self {
        // SAFETY: The selector is `copy`, so this has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        let ret = unsafe { Retained::from_raw(inner) };
        if let Some(ret) = ret {
            ret
        } else {
            copy_fail()
        }
    }

    #[inline]
    fn convert_defined_return(self) -> Self::Inner {
        // Return with +1 retain count.
        Retained::into_raw(self)
    }
}

#[cold]
#[track_caller]
fn copy_fail() -> ! {
    panic!("failed copying object")
}

//
// MutableCopyFamily
//
// Receiver -> Option<Retained<T>>
// Receiver -> Retained<T>
//

impl<T: Message> ConvertReturn<MutableCopyFamily> for Option<Retained<T>> {
    type Inner = *mut T;

    #[inline]
    unsafe fn convert_message_return(
        inner: Self::Inner,
        _receiver_ptr: *mut AnyObject,
        _sel: Sel,
    ) -> Self {
        // SAFETY: The selector is `mutableCopy`, so this has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        unsafe { Retained::from_raw(inner) }
    }

    #[inline]
    fn convert_defined_return(self) -> Self::Inner {
        // Return with +1 retain count.
        Retained::consume_as_ptr_option(self)
    }
}

impl<T: Message> ConvertReturn<MutableCopyFamily> for Retained<T> {
    type Inner = *mut T;

    #[inline]
    unsafe fn convert_message_return(
        inner: Self::Inner,
        _receiver_ptr: *mut AnyObject,
        _sel: Sel,
    ) -> Self {
        // SAFETY: The selector is `mutableCopy`, so this has +1 retain count.
        //
        // Validity of the pointer is upheld by the caller.
        let ret = unsafe { Retained::from_raw(inner) };
        if let Some(ret) = ret {
            ret
        } else {
            mutable_copy_fail()
        }
    }

    #[inline]
    fn convert_defined_return(self) -> Self::Inner {
        // Return with +1 retain count.
        Retained::into_raw(self)
    }
}

#[cold]
#[track_caller]
fn mutable_copy_fail() -> ! {
    panic!("failed copying object")
}

//
// NoneFamily
//
// Receiver -> Option<Retained<T>>
// Receiver -> Retained<T>
//

impl<T: Message> ConvertReturn<NoneFamily> for Option<Retained<T>> {
    type Inner = *mut T;

    #[inline]
    unsafe fn convert_message_return(
        inner: Self::Inner,
        _receiver_ptr: *mut AnyObject,
        _sel: Sel,
    ) -> Self {
        // NOTE: All code between the message send and `retain_autoreleased`
        // must be able to be optimized away for it to work optimally.

        // SAFETY: The selector is not `new`, `alloc`, `init`, `copy` nor
        // `mutableCopy`, so the object must be manually retained.
        //
        // Validity of the pointer is upheld by the caller.
        unsafe { Retained::retain_autoreleased(inner) }
    }

    #[inline]
    fn convert_defined_return(self) -> Self::Inner {
        Retained::autorelease_return_option(self)
    }
}

impl<T: Message> ConvertReturn<NoneFamily> for Retained<T> {
    type Inner = *mut T;

    #[inline]
    unsafe fn convert_message_return(
        inner: Self::Inner,
        receiver_ptr: *mut AnyObject,
        sel: Sel,
    ) -> Self {
        // SAFETY: The selector is not `new`, `alloc`, `init`, `copy` nor
        // `mutableCopy`, so the object must be manually retained.
        //
        // Validity of the pointer is upheld by the caller.
        let ret = unsafe { Retained::retain_autoreleased(inner) };
        if let Some(ret) = ret {
            ret
        } else {
            // SAFETY: The receiver is still valid after a message send to
            // a `none` method - it would not be if the method was `init`.
            let receiver = unsafe { receiver_ptr.as_ref() };
            none_fail(receiver, sel)
        }
    }

    #[inline]
    fn convert_defined_return(self) -> Self::Inner {
        Retained::autorelease_return(self)
    }
}

#[cold]
#[track_caller]
fn none_fail(receiver: Option<&AnyObject>, sel: Sel) -> ! {
    if let Some(receiver) = receiver {
        let cls = receiver.class();
        panic!(
            "unexpected NULL returned from {}[{cls} {sel}]",
            if cls.is_metaclass() { "+" } else { "-" },
        )
    } else {
        panic!("unexpected NULL {sel}; receiver was NULL");
    }
}
