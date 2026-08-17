//! # Dynamically creating classes and protocols.
use alloc::format;
use alloc::string::ToString;
use core::mem;
use core::mem::ManuallyDrop;
use core::ptr;
use core::ptr::NonNull;
use std::ffi::CString;

use crate::encode::{Encode, EncodeArguments, EncodeReturn, Encoding};
use crate::ffi;
use crate::runtime::{AnyClass, AnyObject, AnyProtocol, Bool, Imp, MethodImplementation, Sel};
use crate::sel;
use crate::Message;

fn method_type_encoding(ret: &Encoding, args: &[Encoding]) -> CString {
    // First two arguments are always self and the selector
    let mut types = format!("{ret}{}{}", <*mut AnyObject>::ENCODING, Sel::ENCODING);
    for enc in args {
        use core::fmt::Write;
        write!(&mut types, "{enc}").unwrap();
    }
    CString::new(types).unwrap()
}

trait Log2Alignment {
    const LOG2_ALIGNMENT: u8;
}

impl<T> Log2Alignment for T {
    const LOG2_ALIGNMENT: u8 = {
        let align = mem::align_of::<T>();
        assert!(
            align.count_ones() == 1,
            "alignment required to be a power of 2"
        );
        // log2 of a power of 2 is the number of trailing zeros
        align.trailing_zeros() as u8
    };
}

/// A type for declaring a new class and adding new methods and ivars to it
/// before registering it.
///
/// **Note**: You likely don't need the dynamicism that this provides!
/// Consider using the [`declare_class!`][crate::declare_class] macro instead.
///
///
/// # Example
///
/// Declare a class named `MyNumber` that has one ivar, a `u32` named `_number`
/// and a few constructor  methods and methods for interfacing with the number
/// (using interior mutability, as is common for Objective-C objects).
///
/// ```
/// use core::cell::Cell;
///
/// use objc2::declare::ClassBuilder;
/// use objc2::rc::Retained;
/// use objc2::runtime::{AnyClass, AnyObject, NSObject, Sel};
/// use objc2::{sel, msg_send, msg_send_id, ClassType};
///
/// fn register_class() -> &'static AnyClass {
///     // Inherit from NSObject
///     let mut builder = ClassBuilder::new("MyNumber", NSObject::class())
///         .expect("a class with the name MyNumber likely already exists");
///
///     // Add an instance variable of type `Cell<u32>`
///     builder.add_ivar::<Cell<u32>>("_number");
///
///     // Add an Objective-C method for initializing an instance with a number
///     //
///     // We "cheat" a bit here, and use `AnyObject` instead of `NSObject`,
///     // since only the former is allowed to be a mutable receiver (which is
///     // always safe in `init` methods, but not in others).
///     unsafe extern "C" fn init_with_number(
///         this: &mut AnyObject,
///         _cmd: Sel,
///         number: u32,
///     ) -> Option<&mut AnyObject> {
///         let this: Option<&mut AnyObject> = msg_send![super(this, NSObject::class()), init];
///         this.map(|this| {
///             let ivar = AnyClass::get("MyNumber").unwrap().instance_variable("_number").unwrap();
///             // SAFETY: The ivar is added with the same type above
///             *ivar.load_mut::<Cell<u32>>(this) = Cell::new(number);
///             this
///         })
///     }
///     unsafe {
///         builder.add_method(
///             sel!(initWithNumber:),
///             init_with_number as unsafe extern "C" fn(_, _, _) -> _,
///         );
///     }
///
///     // Add convenience method for getting a new instance with the number
///     extern "C" fn with_number(
///         cls: &AnyClass,
///         _cmd: Sel,
///         number: u32,
///     ) -> *mut NSObject {
///         let obj: Option<Retained<NSObject>> = unsafe {
///             msg_send_id![
///                 msg_send_id![cls, alloc],
///                 initWithNumber: number,
///             ]
///         };
///         obj.map(Retained::autorelease_return).unwrap_or(std::ptr::null_mut())
///     }
///     unsafe {
///         builder.add_class_method(
///             sel!(withNumber:),
///             with_number as extern "C" fn(_, _, _) -> _,
///         );
///     }
///
///     // Add an Objective-C method for setting the number
///     extern "C" fn my_number_set(this: &NSObject, _cmd: Sel, number: u32) {
///         let ivar = AnyClass::get("MyNumber").unwrap().instance_variable("_number").unwrap();
///         // SAFETY: The ivar is added with the same type above
///         unsafe { ivar.load::<Cell<u32>>(this) }.set(number);
///     }
///     unsafe {
///         builder.add_method(sel!(setNumber:), my_number_set as extern "C" fn(_, _, _));
///     }
///
///     // Add an Objective-C method for getting the number
///     extern "C" fn my_number_get(this: &NSObject, _cmd: Sel) -> u32 {
///         let ivar = AnyClass::get("MyNumber").unwrap().instance_variable("_number").unwrap();
///         // SAFETY: The ivar is added with the same type above
///         unsafe { ivar.load::<Cell<u32>>(this) }.get()
///     }
///     unsafe {
///         builder.add_method(sel!(number), my_number_get as extern "C" fn(_, _) -> _);
///     }
///
///     builder.register()
/// }
///
/// // Usage
///
/// // Note: you should only do class registration once! This can be ensured
/// // with `std::sync::Once` or the `once_cell` crate.
/// let cls = register_class();
///
/// let obj: Retained<NSObject> = unsafe {
///     msg_send_id![cls, withNumber: 42u32]
/// };
///
/// let n: u32 = unsafe { msg_send![&obj, number] };
/// assert_eq!(n, 42);
///
/// let _: () = unsafe { msg_send![&obj, setNumber: 12u32] };
/// let n: u32 = unsafe { msg_send![&obj, number] };
/// assert_eq!(n, 12);
/// ```
#[derive(Debug)]
pub struct ClassBuilder {
    // Note: Don't ever construct a &mut objc_class, since it is possible to
    // get this pointer using `AnyClass::classes`!
    cls: NonNull<ffi::objc_class>,
}

// SAFETY: The stuff that touch global state does so using locks internally.
//
// Modifying the class itself can only be done through `&mut`, so Sync is
// safe (e.g. we can't accidentally call `add_ivar` at the same time from two
// different threads).
//
// (Though actually, that would be safe since the entire runtime is locked
// when doing so...).
//
// Finally, there are no requirements that the class must be registered on the
// same thread that allocated it (so Send is safe).
unsafe impl Send for ClassBuilder {}
unsafe impl Sync for ClassBuilder {}

impl ClassBuilder {
    fn as_mut_ptr(&mut self) -> *mut ffi::objc_class {
        self.cls.as_ptr()
    }

    #[allow(unused)]
    pub(crate) fn superclass(&self) -> Option<&AnyClass> {
        // SAFETY: Though the class is not finalized, `class_getSuperclass` is
        // still safe to call.
        unsafe { AnyClass::superclass_raw(self.cls.as_ptr()) }
    }

    #[allow(unused)]
    fn name(&self) -> &str {
        // SAFETY: Same as `superclass`
        unsafe { AnyClass::name_raw(self.cls.as_ptr()) }
    }

    fn with_superclass(name: &str, superclass: Option<&AnyClass>) -> Option<Self> {
        let name = CString::new(name).unwrap();
        let super_ptr = superclass.map_or(ptr::null(), |c| c).cast();
        let cls = unsafe { ffi::objc_allocateClassPair(super_ptr, name.as_ptr(), 0) };
        NonNull::new(cls).map(|cls| Self { cls })
    }

    /// Constructs a [`ClassBuilder`] with the given name and superclass.
    ///
    /// Returns [`None`] if the class couldn't be allocated, or a class with
    /// that name already exist.
    pub fn new(name: &str, superclass: &AnyClass) -> Option<Self> {
        Self::with_superclass(name, Some(superclass))
    }

    /// Constructs a [`ClassBuilder`] declaring a new root class with the
    /// given name.
    ///
    /// Returns [`None`] if the class couldn't be allocated.
    ///
    /// An implementation for `+initialize` must also be given; the runtime
    /// calls this method for all classes, so it must be defined on root
    /// classes.
    ///
    /// Note that implementing a root class is not a simple endeavor!
    /// For example, your class probably cannot be passed to Cocoa code unless
    /// the entire `NSObject` protocol is implemented.
    /// Functionality it expects, like implementations of `-retain` and
    /// `-release` used by ARC, will not be present otherwise.
    pub fn root<F>(name: &str, intitialize_fn: F) -> Option<Self>
    where
        F: MethodImplementation<Callee = AnyClass, Arguments = (), Return = ()>,
    {
        Self::with_superclass(name, None).map(|mut this| {
            unsafe { this.add_class_method(sel!(initialize), intitialize_fn) };
            this
        })
    }

    /// Adds a method with the given name and implementation.
    ///
    ///
    /// # Panics
    ///
    /// Panics if the method wasn't sucessfully added (e.g. a method with that
    /// name already exists).
    ///
    /// May also panic if the method was detected to be invalid in some way;
    /// for example if `debug_assertions` are enabled and the method is
    /// overriding another method, we verify that their encodings are equal.
    ///
    ///
    /// # Safety
    ///
    /// The caller must ensure that the types match those that are expected
    /// when the method is invoked from Objective-C.
    pub unsafe fn add_method<T, F>(&mut self, sel: Sel, func: F)
    where
        T: Message + ?Sized,
        F: MethodImplementation<Callee = T>,
    {
        unsafe {
            self.add_method_inner(
                sel,
                F::Arguments::ENCODINGS,
                &F::Return::ENCODING_RETURN,
                func.__imp(),
            );
        }
    }

    unsafe fn add_method_inner(
        &mut self,
        sel: Sel,
        enc_args: &[Encoding],
        enc_ret: &Encoding,
        func: Imp,
    ) {
        let sel_args = sel.number_of_arguments();
        assert_eq!(
            sel_args,
            enc_args.len(),
            "selector {sel} accepts {sel_args} arguments, but function accepts {}",
            enc_args.len(),
        );

        // Verify that, if the method is present on the superclass, that the
        // encoding is correct.
        #[cfg(debug_assertions)]
        if let Some(superclass) = self.superclass() {
            if let Some(method) = superclass.instance_method(sel) {
                if let Err(err) = crate::verify::verify_method_signature(method, enc_args, enc_ret)
                {
                    panic!("declared invalid method -[{} {sel}]: {err}", self.name())
                }
            }
        }

        let types = method_type_encoding(enc_ret, enc_args);
        let success = Bool::from_raw(unsafe {
            ffi::class_addMethod(self.as_mut_ptr(), sel.as_ptr(), Some(func), types.as_ptr())
        });
        assert!(success.as_bool(), "failed to add method {sel}");
    }

    fn metaclass_mut(&mut self) -> *mut ffi::objc_class {
        unsafe { ffi::object_getClass(self.as_mut_ptr().cast()) as *mut ffi::objc_class }
    }

    /// Adds a class method with the given name and implementation.
    ///
    ///
    /// # Panics
    ///
    /// Panics in the same cases as [`add_method`][Self::add_method].
    ///
    ///
    /// # Safety
    ///
    /// The caller must ensure that the types match those that are expected
    /// when the method is invoked from Objective-C.
    pub unsafe fn add_class_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = AnyClass>,
    {
        unsafe {
            self.add_class_method_inner(
                sel,
                F::Arguments::ENCODINGS,
                &F::Return::ENCODING_RETURN,
                func.__imp(),
            );
        }
    }

    unsafe fn add_class_method_inner(
        &mut self,
        sel: Sel,
        enc_args: &[Encoding],
        enc_ret: &Encoding,
        func: Imp,
    ) {
        let sel_args = sel.number_of_arguments();
        assert_eq!(
            sel_args,
            enc_args.len(),
            "selector {sel} accepts {sel_args} arguments, but function accepts {}",
            enc_args.len(),
        );

        // Verify that, if the method is present on the superclass, that the
        // encoding is correct.
        #[cfg(debug_assertions)]
        if let Some(superclass) = self.superclass() {
            if let Some(method) = superclass.class_method(sel) {
                if let Err(err) = crate::verify::verify_method_signature(method, enc_args, enc_ret)
                {
                    panic!("declared invalid method +[{} {sel}]: {err}", self.name())
                }
            }
        }

        let types = method_type_encoding(enc_ret, enc_args);
        let success = Bool::from_raw(unsafe {
            ffi::class_addMethod(
                self.metaclass_mut(),
                sel.as_ptr(),
                Some(func),
                types.as_ptr(),
            )
        });
        assert!(success.as_bool(), "failed to add class method {sel}");
    }

    /// Adds an ivar with type `T` and the provided name.
    ///
    ///
    /// # Panics
    ///
    /// If the ivar wasn't successfully added for some reason - this usually
    /// happens if there already was an ivar with that name.
    pub fn add_ivar<T: Encode>(&mut self, name: &str) {
        // SAFETY: The encoding is correct
        unsafe { self.add_ivar_inner::<T>(name, &T::ENCODING) }
    }

    pub(crate) unsafe fn add_ivar_inner<T>(&mut self, name: &str, encoding: &Encoding) {
        unsafe { self.add_ivar_inner_mono(name, mem::size_of::<T>(), T::LOG2_ALIGNMENT, encoding) }
    }

    // Monomorphized version
    unsafe fn add_ivar_inner_mono(
        &mut self,
        name: &str,
        size: usize,
        align: u8,
        encoding: &Encoding,
    ) {
        let c_name = CString::new(name).unwrap();
        let encoding = CString::new(encoding.to_string()).unwrap();

        // Note: The Objective-C runtime contains functionality to do stuff
        // with "instance variable layouts", but we don't have to touch any of
        // that, it was only used in the garbage-collecting runtime.
        //
        // Note: On GNUStep, instance variables cannot have the same name
        // on subclasses as it has on superclasses.
        //
        // See <https://github.com/gnustep/libobjc2/issues/246>
        let success = Bool::from_raw(unsafe {
            ffi::class_addIvar(
                self.as_mut_ptr(),
                c_name.as_ptr(),
                size,
                align,
                encoding.as_ptr(),
            )
        });
        assert!(success.as_bool(), "failed to add ivar {name}");
    }

    /// Adds the given protocol to self.
    ///
    /// # Panics
    ///
    /// If the protocol wasn't successfully added.
    pub fn add_protocol(&mut self, proto: &AnyProtocol) {
        let success = unsafe { ffi::class_addProtocol(self.as_mut_ptr(), proto.as_ptr()) };
        let success = Bool::from_raw(success).as_bool();
        if cfg!(not(feature = "gnustep-1-7")) {
            assert!(success, "failed to add protocol {proto}");
        }
    }

    // fn add_property(&self, name: &str, attributes: &[ffi::objc_property_attribute_t]);

    /// Registers the [`ClassBuilder`], consuming it, and returns a reference
    /// to the newly registered [`AnyClass`].
    pub fn register(self) -> &'static AnyClass {
        // Forget self, otherwise the class will be disposed in drop
        let mut this = ManuallyDrop::new(self);
        unsafe { ffi::objc_registerClassPair(this.as_mut_ptr()) };
        unsafe { this.cls.cast::<AnyClass>().as_ref() }
    }
}

impl Drop for ClassBuilder {
    fn drop(&mut self) {
        // Disposing un-registered classes doesn't work properly on GNUStep,
        // so we register the class before disposing it.
        //
        // Doing it this way is _technically_ a race-condition, since other
        // code could read e.g. `AnyClass::classes()` and then pick the class
        // before it got disposed - but let's not worry about that for now.
        #[cfg(feature = "gnustep-1-7")]
        unsafe {
            ffi::objc_registerClassPair(self.as_mut_ptr());
        }

        unsafe { ffi::objc_disposeClassPair(self.as_mut_ptr()) }
    }
}

/// A type for declaring a new protocol and adding new methods to it
/// before registering it.
#[derive(Debug)]
pub struct ProtocolBuilder {
    proto: NonNull<AnyProtocol>,
}

// SAFETY: Similar to ClassBuilder
unsafe impl Send for ProtocolBuilder {}
unsafe impl Sync for ProtocolBuilder {}

impl ProtocolBuilder {
    fn as_mut_ptr(&mut self) -> *mut ffi::objc_protocol {
        self.proto.as_ptr().cast()
    }

    /// Constructs a [`ProtocolBuilder`] with the given name.
    ///
    /// Returns [`None`] if the protocol couldn't be allocated.
    ///
    ///
    /// # Panics
    ///
    /// Panics if the name contains an internal NULL byte.
    pub fn new(name: &str) -> Option<Self> {
        let c_name = CString::new(name).unwrap();
        let proto = unsafe { ffi::objc_allocateProtocol(c_name.as_ptr()) };
        NonNull::new(proto.cast()).map(|proto| Self { proto })
    }

    fn add_method_description_inner(
        &mut self,
        sel: Sel,
        enc_args: &[Encoding],
        enc_ret: &Encoding,
        required: bool,
        instance_method: bool,
    ) {
        let sel_args = sel.number_of_arguments();
        assert_eq!(
            sel_args,
            enc_args.len(),
            "selector {sel} accepts {sel_args} arguments, but function accepts {}",
            enc_args.len(),
        );
        let types = method_type_encoding(enc_ret, enc_args);
        unsafe {
            ffi::protocol_addMethodDescription(
                self.as_mut_ptr(),
                sel.as_ptr(),
                types.as_ptr(),
                Bool::new(required).as_raw(),
                Bool::new(instance_method).as_raw(),
            );
        }
    }

    /// Adds an instance method declaration with a given description.
    pub fn add_method_description<Args, Ret>(&mut self, sel: Sel, required: bool)
    where
        Args: EncodeArguments,
        Ret: EncodeReturn,
    {
        self.add_method_description_inner(
            sel,
            Args::ENCODINGS,
            &Ret::ENCODING_RETURN,
            required,
            true,
        );
    }

    /// Adds a class method declaration with a given description.
    pub fn add_class_method_description<Args, Ret>(&mut self, sel: Sel, required: bool)
    where
        Args: EncodeArguments,
        Ret: EncodeReturn,
    {
        self.add_method_description_inner(
            sel,
            Args::ENCODINGS,
            &Ret::ENCODING_RETURN,
            required,
            false,
        );
    }

    /// Adds a requirement on another protocol.
    pub fn add_protocol(&mut self, proto: &AnyProtocol) {
        unsafe {
            ffi::protocol_addProtocol(self.as_mut_ptr(), proto.as_ptr());
        }
    }

    /// Registers the [`ProtocolBuilder`], consuming it and returning a reference
    /// to the newly registered [`AnyProtocol`].
    pub fn register(mut self) -> &'static AnyProtocol {
        unsafe {
            ffi::objc_registerProtocol(self.as_mut_ptr());
            self.proto.as_ref()
        }
    }
}

impl Drop for ProtocolBuilder {
    fn drop(&mut self) {
        // We implement Drop to communicate to the type-system that this type
        // may drop in the future (once Apple add some way of disposing
        // protocols).
    }
}

#[cfg(test)]
mod tests {
    use core::hash::Hasher;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    use memoffset::offset_of;

    use super::*;
    use crate::encode::RefEncode;
    use crate::mutability::Immutable;
    use crate::rc::Retained;
    use crate::runtime::{NSObject, NSObjectProtocol};
    use crate::{
        declare_class, extern_methods, msg_send, msg_send_id, test_utils, ClassType, DeclaredClass,
        ProtocolType,
    };

    #[test]
    fn test_alignment() {
        assert_eq!(<()>::LOG2_ALIGNMENT, 0);

        assert_eq!(u8::LOG2_ALIGNMENT, 0);
        assert_eq!(u16::LOG2_ALIGNMENT, 1);
        assert_eq!(u32::LOG2_ALIGNMENT, 2);

        assert_eq!(
            u64::LOG2_ALIGNMENT,
            if cfg!(target_pointer_width = "64") {
                3
            } else {
                2
            }
        );

        #[repr(align(16))]
        struct Align16;
        assert_eq!(Align16::LOG2_ALIGNMENT, 4);

        #[repr(align(32))]
        struct Align32;
        assert_eq!(Align32::LOG2_ALIGNMENT, 5);

        #[repr(align(64))]
        struct Align64;
        assert_eq!(Align64::LOG2_ALIGNMENT, 6);

        #[repr(align(128))]
        struct Align128;
        assert_eq!(Align128::LOG2_ALIGNMENT, 7);

        #[repr(align(256))]
        struct Align256;
        assert_eq!(Align256::LOG2_ALIGNMENT, 8);

        #[repr(align(536870912))]
        struct Align536870912;
        assert_eq!(Align536870912::LOG2_ALIGNMENT, 29);
    }

    #[test]
    fn test_classbuilder_duplicate() {
        let cls = test_utils::custom_class();
        let builder = ClassBuilder::new("TestClassBuilderDuplicate", cls).unwrap();
        let _ = builder.register();

        assert!(ClassBuilder::new("TestClassBuilderDuplicate", cls).is_none());
    }

    #[test]
    #[should_panic = "failed to add ivar xyz"]
    fn duplicate_ivar() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new("TestClassBuilderDuplicateIvar", cls).unwrap();

        builder.add_ivar::<i32>("xyz");
        // Should panic:
        builder.add_ivar::<i32>("xyz");
    }

    #[test]
    #[should_panic = "failed to add method xyz"]
    fn duplicate_method() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new("TestClassBuilderDuplicateMethod", cls).unwrap();

        extern "C" fn xyz(_this: &NSObject, _cmd: Sel) {}

        unsafe {
            builder.add_method(sel!(xyz), xyz as extern "C" fn(_, _));
            // Should panic:
            builder.add_method(sel!(xyz), xyz as extern "C" fn(_, _));
        }
    }

    #[test]
    #[should_panic = "selector xyz: accepts 1 arguments, but function accepts 0"]
    fn wrong_arguments() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new("TestClassBuilderWrongArguments", cls).unwrap();

        extern "C" fn xyz(_this: &NSObject, _cmd: Sel) {}

        unsafe {
            // Should panic:
            builder.add_method(sel!(xyz:), xyz as extern "C" fn(_, _));
        }
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic = "declared invalid method -[TestClassBuilderInvalidMethod foo]: expected return to have type code 'I', but found 's'"
    )]
    fn invalid_method() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new("TestClassBuilderInvalidMethod", cls).unwrap();

        extern "C" fn foo(_this: &NSObject, _cmd: Sel) -> i16 {
            0
        }

        unsafe {
            builder.add_method(sel!(foo), foo as extern "C" fn(_, _) -> _);
        }
    }

    #[test]
    #[cfg_attr(
        all(debug_assertions, not(feature = "relax-sign-encoding")),
        should_panic = "declared invalid method +[TestClassBuilderInvalidClassMethod classFoo]: expected return to have type code 'I', but found 'i'"
    )]
    fn invalid_class_method() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new("TestClassBuilderInvalidClassMethod", cls).unwrap();

        extern "C" fn class_foo(_cls: &AnyClass, _cmd: Sel) -> i32 {
            0
        }

        unsafe {
            builder.add_class_method(sel!(classFoo), class_foo as extern "C" fn(_, _) -> _);
        }
    }

    #[test]
    fn inheriting_does_not_implement_protocols() {
        let builder = ClassBuilder::new(
            "TestClassBuilderInheritingDoesNotImplementProtocols",
            NSObject::class(),
        )
        .unwrap();

        let cls = builder.register();
        let conforms = cls.conforms_to(<dyn NSObjectProtocol>::protocol().unwrap());
        if cfg!(feature = "gnustep-1-7") {
            // FIXME: GNUStep works differently here!
            assert!(conforms);
        } else {
            assert!(!conforms);
        }
    }

    #[test]
    fn inherit_nsobject_add_protocol() {
        let mut builder = ClassBuilder::new(
            "TestClassBuilderInheritNSObjectAddProtocol",
            NSObject::class(),
        )
        .unwrap();

        let protocol = <dyn NSObjectProtocol>::protocol().unwrap();

        builder.add_protocol(protocol);
        let cls = builder.register();
        assert!(cls.conforms_to(protocol));
    }

    #[test]
    #[cfg_attr(
        not(feature = "gnustep-1-7"),
        should_panic = "failed to add protocol NSObject"
    )]
    fn duplicate_protocol() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new("TestClassBuilderDuplicateProtocol", cls).unwrap();

        let protocol = AnyProtocol::get("NSObject").unwrap();

        builder.add_protocol(protocol);
        // Should panic:
        builder.add_protocol(protocol);
    }

    #[test]
    fn test_classbuilder_drop() {
        let cls = test_utils::custom_class();
        let builder = ClassBuilder::new("TestClassBuilderDrop", cls).unwrap();
        drop(builder);
        // After we dropped the class, we can create a new one with the same name:
        let _builder = ClassBuilder::new("TestClassBuilderDrop", cls).unwrap();
    }

    #[test]
    fn test_custom_class() {
        // Registering the custom class is in test_utils
        let mut obj = test_utils::custom_object();
        let _: () = unsafe { msg_send![&mut obj, setFoo: 13u32] };
        let result: u32 = unsafe { msg_send![&obj, foo] };
        assert_eq!(result, 13);
    }

    #[test]
    fn test_in_all_classes() {
        fn is_present(cls: *const AnyClass) -> bool {
            // Check whether the class is present in AnyClass::classes()
            AnyClass::classes().iter().any(|item| ptr::eq(cls, *item))
        }

        let superclass = test_utils::custom_class();
        let builder = ClassBuilder::new("TestFetchWhileCreatingClass", superclass).unwrap();

        if cfg!(all(
            target_vendor = "apple",
            any(target_arch = "aarch64", target_arch = "x86_64")
        )) {
            // It is IMO a bug that it is present here!
            assert!(is_present(builder.cls.as_ptr().cast()));
        } else {
            assert!(!is_present(builder.cls.as_ptr().cast()));
        }

        let cls = builder.register();
        assert!(is_present(cls));
    }

    #[test]
    fn test_class_method() {
        let cls = test_utils::custom_class();
        let result: u32 = unsafe { msg_send![cls, classFoo] };
        assert_eq!(result, 7);
    }

    // Proof-of-concept how we could make declare_class! accept generic types.
    #[test]
    fn test_generic() {
        struct GenericDeclareClass<T>(T);

        unsafe impl<T> RefEncode for GenericDeclareClass<T> {
            const ENCODING_REF: Encoding = Encoding::Object;
        }

        unsafe impl<T> Message for GenericDeclareClass<T> {}

        unsafe impl<T> ClassType for GenericDeclareClass<T> {
            type Super = NSObject;
            type Mutability = Immutable;
            const NAME: &'static str = "GenericDeclareClass";

            #[inline]
            fn as_super(&self) -> &Self::Super {
                unimplemented!()
            }

            #[inline]
            fn as_super_mut(&mut self) -> &mut Self::Super {
                unimplemented!()
            }

            fn class() -> &'static AnyClass {
                let superclass = NSObject::class();
                let mut builder = ClassBuilder::new(Self::NAME, superclass).unwrap();

                unsafe {
                    builder.add_method(
                        sel!(generic),
                        <GenericDeclareClass<T>>::generic as unsafe extern "C" fn(_, _),
                    );
                }

                builder.register()
            }
        }

        impl<T> GenericDeclareClass<T> {
            extern "C" fn generic(&self, _cmd: Sel) {}
        }

        let _ = GenericDeclareClass::<()>::class();
    }

    #[test]
    fn test_inherited_nsobject_methods_work() {
        declare_class!(
            #[derive(Debug, PartialEq, Eq, Hash)]
            struct Custom;

            unsafe impl ClassType for Custom {
                type Super = NSObject;
                type Mutability = Immutable;
                const NAME: &'static str = "TestInheritedNSObjectMethodsWork";
            }

            impl DeclaredClass for Custom {}
        );

        extern_methods!(
            unsafe impl Custom {
                #[method_id(new)]
                fn new() -> Retained<Self>;
            }
        );

        let obj1 = Custom::new();
        let obj2 = Custom::new();

        // isEqual:
        assert_eq!(obj1, obj1);
        assert_ne!(obj1, obj2);

        // description
        let expected =
            format!("Custom {{ __superclass: ManuallyDrop {{ value: <TestInheritedNSObjectMethodsWork: {obj1:p}> }}, __ivars: PhantomData<()> }}");
        assert_eq!(format!("{obj1:?}"), expected);

        // hash
        let mut hashstate1 = DefaultHasher::new();
        let mut hashstate2 = DefaultHasher::new();

        obj1.hash(&mut hashstate1);
        obj1.hash(&mut hashstate2);

        assert_eq!(hashstate1.finish(), hashstate2.finish());

        let mut hashstate2 = DefaultHasher::new();
        obj2.hash(&mut hashstate2);
        assert_ne!(hashstate1.finish(), hashstate2.finish());

        // isKindOfClass:
        assert!(obj1.is_kind_of::<NSObject>());
        assert!(obj1.is_kind_of::<Custom>());
        assert!(obj1.is_kind_of::<Custom>());
    }

    #[test]
    #[cfg_attr(
        feature = "gnustep-1-7",
        ignore = "ivars cannot have the same name on GNUStep"
    )]
    fn test_ivar_sizing() {
        #[repr(align(16))]
        struct U128align16 {
            _inner: [u64; 2],
        }

        unsafe impl Encode for U128align16 {
            const ENCODING: Encoding = <[u64; 2]>::ENCODING;
        }

        let mut superclass =
            ClassBuilder::new("DeclareClassDuplicateIvarSuperclass", NSObject::class()).unwrap();
        superclass.add_ivar::<u8>("ivar1");
        superclass.add_ivar::<U128align16>("ivar2");
        superclass.add_ivar::<u8>("ivar3");
        superclass.add_ivar::<[u8; 0]>("ivar4");
        let superclass = superclass.register();

        let mut subclass =
            ClassBuilder::new("DeclareClassDuplicateIvarSubclass", superclass).unwrap();
        // Try to overwrite instance variables
        subclass.add_ivar::<i16>("ivar1");
        subclass.add_ivar::<usize>("ivar2");
        subclass.add_ivar::<*const AnyObject>("ivar3");
        subclass.add_ivar::<usize>("ivar4");
        let subclass = subclass.register();

        // Test that ivar layout matches that of C
        //
        // In particular, ivars are not reordered, though any extra padding on
        // superclasses are utilized on subclasses.
        #[repr(C)]
        struct NSObjectLayout {
            isa: *const AnyClass,
        }
        assert_eq!(
            NSObject::class().instance_size(),
            mem::size_of::<NSObjectLayout>(),
        );

        #[repr(C)]
        struct SuperLayout {
            isa: *const AnyClass,
            ivar1: u8,
            // Padding (7 on 64bit, 11 on 32bit)
            ivar2: U128align16,
            ivar3: u8,
            ivar4: [u8; 0],
            // Padding (15 in Rust, 7 on 64bit, 3 on 32bit)
        }
        // Class's ivar size is only rounded up to a pointer-sized boundary,
        // not all the way up to the maximum alignment.
        //
        // This is surprising, but actually fine, since Objective-C objects
        // are never packed closely like Rust structs would be in an array.
        assert_eq!(
            superclass.instance_size(),
            mem::size_of::<SuperLayout>() - 16 + mem::size_of::<*const AnyClass>(),
        );

        #[repr(C)]
        struct SubLayout {
            isa: *const AnyClass,
            ivar1: u8,
            // Padding (7 on 64bit, 11 on 32bit)
            ivar2: U128align16,
            ivar3: u8,
            ivar4: [u8; 0],
            // Padding (1)
            ivar1_b: i16,
            // Padding (4)
            ivar2_b: usize,
            ivar3_b: *const AnyObject,
            ivar4_b: usize,
        }
        assert_eq!(subclass.instance_size(), mem::size_of::<SubLayout>());

        let superclass_ivar1 = superclass.instance_variable("ivar1").unwrap();
        let superclass_ivar2 = superclass.instance_variable("ivar2").unwrap();
        let superclass_ivar3 = superclass.instance_variable("ivar3").unwrap();
        let superclass_ivar4 = superclass.instance_variable("ivar4").unwrap();
        let subclass_ivar1 = subclass.instance_variable("ivar1").unwrap();
        let subclass_ivar2 = subclass.instance_variable("ivar2").unwrap();
        let subclass_ivar3 = subclass.instance_variable("ivar3").unwrap();
        let subclass_ivar4 = subclass.instance_variable("ivar4").unwrap();

        // Ensure that duplicate names do not conflict
        assert_ne!(superclass_ivar1, subclass_ivar1);
        assert_ne!(superclass_ivar2, subclass_ivar2);
        assert_ne!(superclass_ivar3, subclass_ivar3);
        assert_ne!(superclass_ivar4, subclass_ivar4);

        // Ensure that all offsets are as expected
        assert_eq!(
            superclass_ivar1.offset(),
            offset_of!(SuperLayout, ivar1) as isize
        );
        assert_eq!(
            superclass_ivar2.offset(),
            offset_of!(SuperLayout, ivar2) as isize
        );
        assert_eq!(
            superclass_ivar3.offset(),
            offset_of!(SuperLayout, ivar3) as isize
        );
        assert_eq!(
            superclass_ivar4.offset(),
            offset_of!(SuperLayout, ivar4) as isize
        );
        assert_eq!(
            subclass_ivar1.offset(),
            offset_of!(SubLayout, ivar1_b) as isize
        );
        assert_eq!(
            subclass_ivar2.offset(),
            offset_of!(SubLayout, ivar2_b) as isize
        );
        assert_eq!(
            subclass_ivar3.offset(),
            offset_of!(SubLayout, ivar3_b) as isize
        );
        assert_eq!(
            subclass_ivar4.offset(),
            offset_of!(SubLayout, ivar4_b) as isize
        );

        // Ensure our ivar loading works correctly
        let obj: Retained<NSObject> = unsafe { msg_send_id![subclass, new] };
        let ptr = unsafe { *subclass_ivar3.load::<*const AnyObject>(&obj) };
        assert!(ptr.is_null());

        // Illustration of what goes wrong with the naive approach of loading
        // the Ivar dynamically; in short, we can't be sure of which instance
        // variable we're refering to here.
        //
        // let ivar = *obj.get_ivar::<u8>("ivar3");
    }
}
