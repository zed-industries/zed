use alloc::ffi::CString;
#[cfg(debug_assertions)]
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::panic::{RefUnwindSafe, UnwindSafe};
#[cfg(debug_assertions)]
use std::collections::HashSet;

use crate::encode::{Encode, Encoding};
use crate::rc::{Allocated, Retained};
use crate::runtime::{
    AnyClass, AnyObject, ClassBuilder, MessageReceiver, MethodImplementation, Sel,
};
#[cfg(debug_assertions)]
use crate::runtime::{AnyProtocol, MethodDescription};
use crate::{AnyThread, ClassType, DefinedClass, Message, ProtocolType};

use super::defined_ivars::{register_with_ivars, setup_dealloc};
use super::{CopyFamily, InitFamily, MutableCopyFamily, NewFamily, NoneFamily};

/// Helper for determining auto traits of defined classes.
///
/// This will contain either `dyn AnyThread` or `dyn MainThreadOnly`, so it
/// will have no auto traits by default.
#[derive(Debug)]
pub struct ThreadKindAutoTraits<T: ?Sized>(T);

// SAFETY: `AnyThread` does not place restrictions on thread safety.
unsafe impl Send for ThreadKindAutoTraits<dyn AnyThread> {}
// SAFETY: Same as above.
unsafe impl Sync for ThreadKindAutoTraits<dyn AnyThread> {}

// NOTE: A similar implementation for `dyn MainThreadOnly` is explicitly not
// allowed, as that would enable users to pass something that is tied to the
// main thread to other threads. Remember that we can view `MainThreadOnly`
// classes as containing a `MainThreadMarker` (which is always accessible
// using `MainThreadOnly::mtm`).
//
// impl !Send for ThreadKindAutoTraits<dyn MainThreadOnly> {}
// impl !Sync for ThreadKindAutoTraits<dyn MainThreadOnly> {}

// Thread kind does not affect pinning or unwind safety
impl<T: ?Sized> Unpin for ThreadKindAutoTraits<T> {}
impl<T: ?Sized> UnwindSafe for ThreadKindAutoTraits<T> {}
impl<T: ?Sized> RefUnwindSafe for ThreadKindAutoTraits<T> {}

// Thread kind does not affect autorelease safety.
#[cfg(feature = "unstable-autoreleasesafe")]
unsafe impl<T: ?Sized> crate::rc::AutoreleaseSafe for ThreadKindAutoTraits<T> {}

/// Helper type for implementing `MethodImplementation` with a receiver of
/// `Allocated<T>`, without exposing that implementation to users.
//
// Must be private, soundness of MethodImplementation relies on this.
#[doc(hidden)]
#[repr(transparent)]
#[derive(Debug)]
#[allow(dead_code)]
pub struct RetainedReturnValue(pub(crate) *mut AnyObject);

// SAFETY: `RetainedReturnValue` is `#[repr(transparent)]`
unsafe impl Encode for RetainedReturnValue {
    const ENCODING: Encoding = <*mut AnyObject>::ENCODING;
}

// One could imagine a different design where we had a method like
// `fn convert_receiver()`, but that won't work in `define_class!` since we
// can't actually modify the `self` argument (e.g. `let self = foo(self)` is
// not allowed).
//
// See `MsgSendRetained` and `MethodFamily` for details on the retain
// semantics we're following here.
pub trait MessageReceiveRetained<Receiver, Ret> {
    fn into_return(obj: Ret) -> RetainedReturnValue;
}

// Receiver and return type have no correlation
impl<Receiver, Ret> MessageReceiveRetained<Receiver, Ret> for NewFamily
where
    Receiver: MessageReceiver,
    Ret: MaybeOptionRetained,
{
    #[inline]
    fn into_return(obj: Ret) -> RetainedReturnValue {
        obj.consumed_return()
    }
}

// Explicitly left unimplemented for now!
// impl MessageReceiveRetained<impl MessageReceiver, Allocated<T>> for Alloc {}

// Note: `MethodImplementation` allows for `Allocated` as the receiver, so we
// restrict it here to only be when the selector is `init`.
//
// Additionally, the receiver and return type must have the same generic
// parameter `T`.
impl<Ret, T> MessageReceiveRetained<Allocated<T>, Ret> for InitFamily
where
    T: Message,
    Ret: MaybeOptionRetained<Inner = T>,
{
    #[inline]
    fn into_return(obj: Ret) -> RetainedReturnValue {
        obj.consumed_return()
    }
}

// Receiver and return type have no correlation
impl<Receiver, Ret> MessageReceiveRetained<Receiver, Ret> for CopyFamily
where
    Receiver: MessageReceiver,
    Ret: MaybeOptionRetained,
{
    #[inline]
    fn into_return(obj: Ret) -> RetainedReturnValue {
        obj.consumed_return()
    }
}

// Receiver and return type have no correlation
impl<Receiver, Ret> MessageReceiveRetained<Receiver, Ret> for MutableCopyFamily
where
    Receiver: MessageReceiver,
    Ret: MaybeOptionRetained,
{
    #[inline]
    fn into_return(obj: Ret) -> RetainedReturnValue {
        obj.consumed_return()
    }
}

// Receiver and return type have no correlation
impl<Receiver, Ret> MessageReceiveRetained<Receiver, Ret> for NoneFamily
where
    Receiver: MessageReceiver,
    Ret: MaybeOptionRetained,
{
    #[inline]
    fn into_return(obj: Ret) -> RetainedReturnValue {
        obj.autorelease_return()
    }
}

/// Helper trait for specifying an `Retained<T>` or an `Option<Retained<T>>`.
///
/// (Both of those are valid return types from define_class!
/// `#[unsafe(method_id)]`).
pub trait MaybeOptionRetained {
    type Inner;

    fn consumed_return(self) -> RetainedReturnValue;
    fn autorelease_return(self) -> RetainedReturnValue;
}

impl<T: Message> MaybeOptionRetained for Retained<T> {
    type Inner = T;

    #[inline]
    fn consumed_return(self) -> RetainedReturnValue {
        let ptr: *mut T = Retained::into_raw(self);
        RetainedReturnValue(ptr.cast())
    }

    #[inline]
    fn autorelease_return(self) -> RetainedReturnValue {
        let ptr: *mut T = Retained::autorelease_return(self);
        RetainedReturnValue(ptr.cast())
    }
}

impl<T: Message> MaybeOptionRetained for Option<Retained<T>> {
    type Inner = T;

    #[inline]
    fn consumed_return(self) -> RetainedReturnValue {
        let ptr: *mut T = Retained::consume_as_ptr_option(self);
        RetainedReturnValue(ptr.cast())
    }

    #[inline]
    fn autorelease_return(self) -> RetainedReturnValue {
        let ptr: *mut T = Retained::autorelease_return_option(self);
        RetainedReturnValue(ptr.cast())
    }
}

#[derive(Debug)]
pub struct ClassBuilderHelper<T: ?Sized> {
    builder: ClassBuilder,
    p: PhantomData<T>,
}

// Outlined for code size
#[track_caller]
fn create_builder(name: &str, superclass: &AnyClass) -> ClassBuilder {
    let c_name = CString::new(name).expect("class name must be UTF-8");
    match ClassBuilder::new(&c_name, superclass) {
        Some(builder) => builder,
        None => panic!(
            "could not create new class {name}. Perhaps a class with that name already exists?"
        ),
    }
}

impl<T: DefinedClass> ClassBuilderHelper<T> {
    #[inline]
    #[track_caller]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self
    where
        T::Super: ClassType,
    {
        let mut builder = create_builder(T::NAME, <T::Super as ClassType>::class());

        setup_dealloc::<T>(&mut builder);

        Self {
            builder,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn add_protocol_methods<P>(&mut self) -> ClassProtocolMethodsBuilder<'_, T>
    where
        P: ?Sized + ProtocolType,
    {
        let protocol = P::protocol();

        if let Some(protocol) = protocol {
            // Ignore the return value; whether the protocol is added is
            // inherently dependent on the order of the protocols.
            self.builder.add_protocol(protocol);
        }

        #[cfg(debug_assertions)]
        {
            ClassProtocolMethodsBuilder {
                builder: self,
                protocol,
                required_instance_methods: protocol
                    .map(|p| p.method_descriptions(true))
                    .unwrap_or_default(),
                optional_instance_methods: protocol
                    .map(|p| p.method_descriptions(false))
                    .unwrap_or_default(),
                registered_instance_methods: HashSet::new(),
                required_class_methods: protocol
                    .map(|p| p.class_method_descriptions(true))
                    .unwrap_or_default(),
                optional_class_methods: protocol
                    .map(|p| p.class_method_descriptions(false))
                    .unwrap_or_default(),
                registered_class_methods: HashSet::new(),
            }
        }

        #[cfg(not(debug_assertions))]
        {
            ClassProtocolMethodsBuilder { builder: self }
        }
    }

    // Addition: This restricts to callee `T`
    #[inline]
    pub unsafe fn add_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = T>,
    {
        // SAFETY: Checked by caller
        unsafe { self.builder.add_method(sel, func) }
    }

    #[inline]
    pub unsafe fn add_class_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = AnyClass>,
    {
        // SAFETY: Checked by caller
        unsafe { self.builder.add_class_method(sel, func) }
    }

    #[inline]
    pub fn register(self) -> (&'static AnyClass, isize, isize) {
        register_with_ivars::<T>(self.builder)
    }
}

/// Helper for ensuring that:
/// - Only methods on the protocol are overridden.
/// - TODO: The methods have the correct signature.
/// - All required methods are overridden.
#[derive(Debug)]
pub struct ClassProtocolMethodsBuilder<'a, T: ?Sized> {
    builder: &'a mut ClassBuilderHelper<T>,
    #[cfg(debug_assertions)]
    protocol: Option<&'static AnyProtocol>,
    #[cfg(debug_assertions)]
    required_instance_methods: Vec<MethodDescription>,
    #[cfg(debug_assertions)]
    optional_instance_methods: Vec<MethodDescription>,
    #[cfg(debug_assertions)]
    registered_instance_methods: HashSet<Sel>,
    #[cfg(debug_assertions)]
    required_class_methods: Vec<MethodDescription>,
    #[cfg(debug_assertions)]
    optional_class_methods: Vec<MethodDescription>,
    #[cfg(debug_assertions)]
    registered_class_methods: HashSet<Sel>,
}

impl<T: DefinedClass> ClassProtocolMethodsBuilder<'_, T> {
    // Addition: This restricts to callee `T`
    #[inline]
    pub unsafe fn add_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = T>,
    {
        #[cfg(debug_assertions)]
        if let Some(protocol) = self.protocol {
            let _types = self
                .required_instance_methods
                .iter()
                .chain(&self.optional_instance_methods)
                .find(|desc| desc.sel == sel)
                .map(|desc| desc.types)
                .unwrap_or_else(|| {
                    panic!(
                        "failed overriding protocol method -[{protocol} {sel}]: method not found"
                    )
                });
        }

        // SAFETY: Checked by caller
        unsafe { self.builder.add_method(sel, func) };

        #[cfg(debug_assertions)]
        if !self.registered_instance_methods.insert(sel) {
            unreachable!("already added")
        }
    }

    #[inline]
    pub unsafe fn add_class_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = AnyClass>,
    {
        #[cfg(debug_assertions)]
        if let Some(protocol) = self.protocol {
            let _types = self
                .required_class_methods
                .iter()
                .chain(&self.optional_class_methods)
                .find(|desc| desc.sel == sel)
                .map(|desc| desc.types)
                .unwrap_or_else(|| {
                    panic!(
                        "failed overriding protocol method +[{protocol} {sel}]: method not found"
                    )
                });
        }

        // SAFETY: Checked by caller
        unsafe { self.builder.add_class_method(sel, func) };

        #[cfg(debug_assertions)]
        if !self.registered_class_methods.insert(sel) {
            unreachable!("already added")
        }
    }

    #[cfg(debug_assertions)]
    pub fn finish(self) {
        let superclass = self.builder.builder.superclass();

        if let Some(protocol) = self.protocol {
            for desc in &self.required_instance_methods {
                if self.registered_instance_methods.contains(&desc.sel) {
                    continue;
                }

                // TODO: Don't do this when `NS_PROTOCOL_REQUIRES_EXPLICIT_IMPLEMENTATION`
                if superclass
                    .and_then(|superclass| superclass.instance_method(desc.sel))
                    .is_some()
                {
                    continue;
                }

                panic!(
                    "must implement required protocol method -[{protocol} {}]",
                    desc.sel
                )
            }
        }

        if let Some(protocol) = self.protocol {
            for desc in &self.required_class_methods {
                if self.registered_class_methods.contains(&desc.sel) {
                    continue;
                }

                // TODO: Don't do this when `NS_PROTOCOL_REQUIRES_EXPLICIT_IMPLEMENTATION`
                if superclass
                    .and_then(|superclass| superclass.class_method(desc.sel))
                    .is_some()
                {
                    continue;
                }

                panic!(
                    "must implement required protocol method +[{protocol} {}]",
                    desc.sel
                );
            }
        }
    }

    #[inline]
    #[cfg(not(debug_assertions))]
    pub fn finish(self) {}
}
