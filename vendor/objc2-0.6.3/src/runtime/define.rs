//! # Dynamically creating classes and protocols.
use alloc::ffi::CString;
use alloc::format;
use alloc::string::ToString;
use core::ffi::CStr;
use core::mem;
use core::mem::ManuallyDrop;
use core::ptr;
use core::ptr::NonNull;

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

/// A type for creating a new class and adding new methods and ivars to it
/// before registering it.
///
/// **Note**: You likely don't need the dynamicism that this provides!
/// Consider using the [`define_class!`][crate::define_class] macro instead.
///
///
/// # Example
///
/// Create a class named `MyNumber` that has one ivar, a `u32` named `_number`
/// and a few constructor  methods and methods for interfacing with the number
/// (using interior mutability, as is common for Objective-C objects).
///
/// ```
/// use core::cell::Cell;
///
/// use objc2::rc::Retained;
/// use objc2::runtime::{AnyClass, AnyObject, ClassBuilder, NSObject, Sel};
/// use objc2::{sel, msg_send, ClassType};
///
/// fn register_class() -> &'static AnyClass {
///     // Inherit from NSObject
///     let mut builder = ClassBuilder::new(c"MyNumber", NSObject::class())
///         .expect("a class with the name MyNumber likely already exists");
///
///     // Add an instance variable of type `Cell<u32>`
///     builder.add_ivar::<Cell<u32>>(c"_number");
///
///     // Add an Objective-C method for initializing an instance with a number
///     //
///     // We "cheat" a bit here, and use `AnyObject` instead of `NSObject`,
///     // since only the former is allowed to be a mutable receiver (which is
///     // always safe in `init` methods, but not in others).
///     unsafe extern "C-unwind" fn init_with_number(
///         this: &mut AnyObject,
///         _cmd: Sel,
///         number: u32,
///     ) -> Option<&mut AnyObject> {
///         let this: Option<&mut AnyObject> = msg_send![super(this, NSObject::class()), init];
///         this.map(|this| {
///             let ivar = AnyClass::get(c"MyNumber").unwrap().instance_variable(c"_number").unwrap();
///             // SAFETY: The ivar is added with the same type above
///             *ivar.load_mut::<Cell<u32>>(this) = Cell::new(number);
///             this
///         })
///     }
///     unsafe {
///         builder.add_method(
///             sel!(initWithNumber:),
///             init_with_number as unsafe extern "C-unwind" fn(_, _, _) -> _,
///         );
///     }
///
///     // Add convenience method for getting a new instance with the number
///     extern "C-unwind" fn with_number(
///         cls: &AnyClass,
///         _cmd: Sel,
///         number: u32,
///     ) -> *mut NSObject {
///         let obj: Option<Retained<NSObject>> = unsafe {
///             msg_send![
///                 msg_send![cls, alloc],
///                 initWithNumber: number,
///             ]
///         };
///         obj.map(Retained::autorelease_return).unwrap_or(std::ptr::null_mut())
///     }
///     unsafe {
///         builder.add_class_method(
///             sel!(withNumber:),
///             with_number as extern "C-unwind" fn(_, _, _) -> _,
///         );
///     }
///
///     // Add an Objective-C method for setting the number
///     extern "C-unwind" fn my_number_set(this: &NSObject, _cmd: Sel, number: u32) {
///         let ivar = AnyClass::get(c"MyNumber").unwrap().instance_variable(c"_number").unwrap();
///         // SAFETY: The ivar is added with the same type above
///         unsafe { ivar.load::<Cell<u32>>(this) }.set(number);
///     }
///     unsafe {
///         builder.add_method(sel!(setNumber:), my_number_set as extern "C-unwind" fn(_, _, _));
///     }
///
///     // Add an Objective-C method for getting the number
///     extern "C-unwind" fn my_number_get(this: &NSObject, _cmd: Sel) -> u32 {
///         let ivar = AnyClass::get(c"MyNumber").unwrap().instance_variable(c"_number").unwrap();
///         // SAFETY: The ivar is added with the same type above
///         unsafe { ivar.load::<Cell<u32>>(this) }.get()
///     }
///     unsafe {
///         builder.add_method(sel!(number), my_number_get as extern "C-unwind" fn(_, _) -> _);
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
///     msg_send![cls, withNumber: 42u32]
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
    // Note: Don't ever construct a &mut AnyClass, since it is possible to
    // get this pointer using `AnyClass::classes`!
    cls: NonNull<AnyClass>,
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
    fn as_mut_ptr(&mut self) -> *mut AnyClass {
        self.cls.as_ptr()
    }

    #[allow(unused)]
    pub(crate) fn superclass(&self) -> Option<&AnyClass> {
        // SAFETY: Though the class is not finalized, `class_getSuperclass` is
        // still safe to call.
        unsafe { AnyClass::superclass_raw(self.cls.as_ptr()) }
    }

    #[allow(unused)]
    fn name(&self) -> &CStr {
        // SAFETY: Same as `superclass`
        unsafe { AnyClass::name_raw(self.cls.as_ptr()) }
    }

    #[inline]
    fn with_superclass(name: &CStr, superclass: Option<&AnyClass>) -> Option<Self> {
        let super_ptr = superclass.map_or(ptr::null(), |c| c).cast();
        let cls = unsafe { ffi::objc_allocateClassPair(super_ptr, name.as_ptr(), 0) };
        NonNull::new(cls).map(|cls| Self { cls })
    }

    /// Constructs a [`ClassBuilder`] with the given name and superclass.
    ///
    /// Returns [`None`] if the class couldn't be allocated, or a class with
    /// that name already exist.
    #[inline]
    pub fn new(name: &CStr, superclass: &AnyClass) -> Option<Self> {
        Self::with_superclass(name, Some(superclass))
    }

    /// Constructs a [`ClassBuilder`] that will construct a new root class
    /// with the given name.
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
    pub fn root<F>(name: &CStr, initialize_fn: F) -> Option<Self>
    where
        F: MethodImplementation<Callee = AnyClass, Arguments = (), Return = ()>,
    {
        Self::with_superclass(name, None).map(|mut this| {
            unsafe { this.add_class_method(sel!(initialize), initialize_fn) };
            this
        })
    }

    /// Adds a method with the given name and implementation.
    ///
    ///
    /// # Panics
    ///
    /// Panics if the method wasn't successfully added (e.g. a method with that
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
        #[cfg(all(debug_assertions, not(feature = "disable-encoding-assertions")))]
        if let Some(superclass) = self.superclass() {
            if let Some(method) = superclass.instance_method(sel) {
                if let Err(err) = crate::verify::verify_method_signature(method, enc_args, enc_ret)
                {
                    panic!(
                        "defined invalid method -[{} {sel}]: {err}",
                        self.name().to_string_lossy()
                    )
                }
            }
        }

        let types = method_type_encoding(enc_ret, enc_args);
        let success = unsafe { ffi::class_addMethod(self.as_mut_ptr(), sel, func, types.as_ptr()) };
        assert!(success.as_bool(), "failed to add method {sel}");
    }

    fn metaclass_mut(&mut self) -> *mut AnyClass {
        unsafe { ffi::object_getClass(self.as_mut_ptr().cast()) as *mut AnyClass }
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
        #[cfg(all(debug_assertions, not(feature = "disable-encoding-assertions")))]
        if let Some(superclass) = self.superclass() {
            if let Some(method) = superclass.class_method(sel) {
                if let Err(err) = crate::verify::verify_method_signature(method, enc_args, enc_ret)
                {
                    panic!(
                        "defined invalid method +[{} {sel}]: {err}",
                        self.name().to_string_lossy()
                    )
                }
            }
        }

        let types = method_type_encoding(enc_ret, enc_args);
        let success =
            unsafe { ffi::class_addMethod(self.metaclass_mut(), sel, func, types.as_ptr()) };
        assert!(success.as_bool(), "failed to add class method {sel}");
    }

    /// Adds an ivar with type `T` and the provided name.
    ///
    ///
    /// # Panics
    ///
    /// If the ivar wasn't successfully added for some reason - this usually
    /// happens if there already was an ivar with that name.
    pub fn add_ivar<T: Encode>(&mut self, name: &CStr) {
        // SAFETY: The encoding is correct
        unsafe { self.add_ivar_inner::<T>(name, &T::ENCODING) }
    }

    pub(crate) unsafe fn add_ivar_inner<T>(&mut self, name: &CStr, encoding: &Encoding) {
        unsafe { self.add_ivar_inner_mono(name, mem::size_of::<T>(), T::LOG2_ALIGNMENT, encoding) }
    }

    // Monomorphized version
    unsafe fn add_ivar_inner_mono(
        &mut self,
        name: &CStr,
        size: usize,
        align: u8,
        encoding: &Encoding,
    ) {
        let encoding = CString::new(encoding.to_string()).unwrap();

        // Note: The Objective-C runtime contains functionality to do stuff
        // with "instance variable layouts", but we don't have to touch any of
        // that, it was only used in the garbage-collecting runtime.
        //
        // Note: On GNUStep, instance variables cannot have the same name
        // on subclasses as it has on superclasses.
        //
        // See <https://github.com/gnustep/libobjc2/issues/246>
        let success = unsafe {
            ffi::class_addIvar(
                self.as_mut_ptr(),
                name.as_ptr(),
                size,
                align,
                encoding.as_ptr(),
            )
        };
        assert!(success.as_bool(), "failed to add ivar {name:?}");
    }

    /// Makes the class conform to the given protocol.
    ///
    /// This will also make the class conform to any super-protocols that the
    /// given protocol may have.
    ///
    /// Returns whether the class did not already conform to the protocol.
    /// This may commonly return false if you first add e.g.
    /// `NSProgressReporting`, and then later try to add `NSObjectProtocol`,
    /// which is a super-protocol thereof.
    #[inline]
    pub fn add_protocol(&mut self, proto: &AnyProtocol) -> bool {
        let success = unsafe { ffi::class_addProtocol(self.as_mut_ptr(), proto) };
        success.as_bool()
    }

    // fn add_property(&self, name: &CStr, attributes: &[ffi::objc_property_attribute_t]);

    /// Registers the [`ClassBuilder`], consuming it, and returns a reference
    /// to the newly registered [`AnyClass`].
    #[inline]
    pub fn register(self) -> &'static AnyClass {
        // Forget self, otherwise the class will be disposed in drop
        let mut this = ManuallyDrop::new(self);
        unsafe { ffi::objc_registerClassPair(this.as_mut_ptr()) };
        unsafe { this.cls.as_ref() }
    }
}

impl Drop for ClassBuilder {
    #[inline]
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

/// A type for creating a new protocol and adding new methods to it
/// before registering it.
#[derive(Debug)]
pub struct ProtocolBuilder {
    proto: NonNull<AnyProtocol>,
}

// SAFETY: Similar to ClassBuilder
unsafe impl Send for ProtocolBuilder {}
unsafe impl Sync for ProtocolBuilder {}

impl ProtocolBuilder {
    fn as_mut_ptr(&mut self) -> *mut AnyProtocol {
        self.proto.as_ptr()
    }

    /// Constructs a [`ProtocolBuilder`] with the given name.
    ///
    /// Returns [`None`] if the protocol couldn't be allocated.
    ///
    ///
    /// # Panics
    ///
    /// Panics if the name contains an internal NULL byte.
    #[inline]
    pub fn new(name: &CStr) -> Option<Self> {
        let proto = unsafe { ffi::objc_allocateProtocol(name.as_ptr()) };
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
                sel,
                types.as_ptr(),
                Bool::new(required),
                Bool::new(instance_method),
            );
        }
    }

    /// Add an instance method with a given description.
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

    /// Add a class method with a given description.
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
        unsafe { ffi::protocol_addProtocol(self.as_mut_ptr(), proto) };
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
    #[inline]
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
    use crate::rc::Retained;
    use crate::runtime::{NSObject, NSObjectProtocol};
    use crate::{define_class, extern_methods, msg_send, test_utils, ClassType, ProtocolType};

    // TODO: Remove once c"" strings are in MSRV
    fn c(s: &str) -> CString {
        CString::new(s).unwrap()
    }

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
        let builder = ClassBuilder::new(&c("TestClassBuilderDuplicate"), cls).unwrap();
        let _ = builder.register();

        assert!(ClassBuilder::new(&c("TestClassBuilderDuplicate"), cls).is_none());
    }

    #[test]
    #[should_panic = "failed to add ivar \"xyz\""]
    fn duplicate_ivar() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new(&c("TestClassBuilderDuplicateIvar"), cls).unwrap();

        builder.add_ivar::<i32>(&c("xyz"));
        // Should panic:
        builder.add_ivar::<i32>(&c("xyz"));
    }

    #[test]
    #[should_panic = "failed to add method xyz"]
    fn duplicate_method() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new(&c("TestClassBuilderDuplicateMethod"), cls).unwrap();

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
        let mut builder = ClassBuilder::new(&c("TestClassBuilderWrongArguments"), cls).unwrap();

        extern "C" fn xyz(_this: &NSObject, _cmd: Sel) {}

        unsafe {
            // Should panic:
            builder.add_method(sel!(xyz:), xyz as extern "C" fn(_, _));
        }
    }

    #[test]
    #[cfg_attr(
        all(debug_assertions, not(feature = "disable-encoding-assertions")),
        should_panic = "defined invalid method -[TestClassBuilderInvalidMethod foo]: expected return to have type code 'I', but found 's'"
    )]
    fn invalid_method() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new(&c("TestClassBuilderInvalidMethod"), cls).unwrap();

        extern "C" fn foo(_this: &NSObject, _cmd: Sel) -> i16 {
            0
        }

        unsafe {
            builder.add_method(sel!(foo), foo as extern "C" fn(_, _) -> _);
        }
    }

    #[test]
    #[cfg_attr(
        all(
            debug_assertions,
            not(feature = "disable-encoding-assertions"),
            not(feature = "relax-sign-encoding")
        ),
        should_panic = "defined invalid method +[TestClassBuilderInvalidClassMethod classFoo]: expected return to have type code 'I', but found 'i'"
    )]
    fn invalid_class_method() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new(&c("TestClassBuilderInvalidClassMethod"), cls).unwrap();

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
            &c("TestClassBuilderInheritingDoesNotImplementProtocols"),
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
            &c("TestClassBuilderInheritNSObjectAddProtocol"),
            NSObject::class(),
        )
        .unwrap();

        let protocol = <dyn NSObjectProtocol>::protocol().unwrap();

        // GNUStep is more eagerly returning false in the case where we
        // inherit something that implements the protocol.
        if cfg!(feature = "gnustep-1-7") {
            assert!(!builder.add_protocol(protocol));
        } else {
            assert!(builder.add_protocol(protocol));
        }

        let cls = builder.register();
        assert!(cls.conforms_to(protocol));
    }

    #[test]
    fn duplicate_protocol() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new(&c("TestClassBuilderDuplicateProtocol"), cls).unwrap();

        let protocol = ProtocolBuilder::new(&c("TestClassBuilderDuplicateProtocol"))
            .unwrap()
            .register();

        assert!(builder.add_protocol(protocol));
        assert!(!builder.add_protocol(protocol));
    }

    #[test]
    fn add_protocol_subprotocol_ordering() {
        // The value returned by `class_addProtocol` is inherently dependent
        // on the order in which you add the super- and subprotocols.
        let builder = ProtocolBuilder::new(&c("Superprotocol")).unwrap();
        let superprotocol = builder.register();

        let mut builder = ProtocolBuilder::new(&c("Subprotocol")).unwrap();
        builder.add_protocol(superprotocol);
        let subprotocol = builder.register();

        let mut builder =
            ClassBuilder::new(&c("AddProtocolSuperThenSub"), NSObject::class()).unwrap();
        assert!(builder.add_protocol(superprotocol));
        assert!(builder.add_protocol(subprotocol));
        let _cls = builder.register();

        let mut builder =
            ClassBuilder::new(&c("AddProtocolSubThenSuper"), NSObject::class()).unwrap();
        assert!(builder.add_protocol(subprotocol));
        assert!(!builder.add_protocol(superprotocol));
        let _cls = builder.register();
    }

    #[test]
    fn test_classbuilder_drop() {
        let cls = test_utils::custom_class();
        let builder = ClassBuilder::new(&c("TestClassBuilderDrop"), cls).unwrap();
        drop(builder);
        // After we dropped the class, we can create a new one with the same name:
        let _builder = ClassBuilder::new(&c("TestClassBuilderDrop"), cls).unwrap();
    }

    #[test]
    fn test_custom_class() {
        // Registering the custom class is in test_utils
        let obj = test_utils::custom_object();
        let _: () = unsafe { msg_send![&obj, setFoo: 13u32] };
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
        let builder = ClassBuilder::new(&c("TestFetchWhileCreatingClass"), superclass).unwrap();

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

    // Proof-of-concept how we could make define_class! accept generic types.
    #[test]
    fn test_generic() {
        struct GenericDefineClass<T>(T);

        unsafe impl<T> RefEncode for GenericDefineClass<T> {
            const ENCODING_REF: Encoding = Encoding::Object;
        }

        unsafe impl<T> Message for GenericDefineClass<T> {}

        unsafe impl<T> ClassType for GenericDefineClass<T> {
            type Super = NSObject;
            type ThreadKind = <Self::Super as ClassType>::ThreadKind;
            const NAME: &'static str = "GenericDefineClass";

            #[inline]
            fn as_super(&self) -> &Self::Super {
                unimplemented!()
            }

            fn class() -> &'static AnyClass {
                let superclass = NSObject::class();
                let mut builder = ClassBuilder::new(&c(Self::NAME), superclass).unwrap();

                unsafe {
                    builder.add_method(
                        sel!(generic),
                        <GenericDefineClass<T>>::generic as unsafe extern "C" fn(_, _),
                    );
                }

                builder.register()
            }

            const __INNER: () = ();

            type __SubclassingType = Self;
        }

        impl<T> GenericDefineClass<T> {
            extern "C" fn generic(&self, _cmd: Sel) {}
        }

        let _ = GenericDefineClass::<()>::class();
    }

    #[test]
    fn test_inherited_nsobject_methods_work() {
        define_class!(
            #[unsafe(super(NSObject))]
            #[name = "TestInheritedNSObjectMethodsWork"]
            #[derive(Debug, PartialEq, Eq, Hash)]
            struct Custom;
        );

        impl Custom {
            extern_methods!(
                #[unsafe(method(new))]
                fn new() -> Retained<Self>;
            );
        }

        let obj1 = Custom::new();
        let obj2 = Custom::new();

        // isEqual:
        assert_eq!(obj1, obj1);
        assert_ne!(obj1, obj2);

        // description
        let expected =
            format!("Custom {{ super: <TestInheritedNSObjectMethodsWork: {obj1:p}>, ivars: () }}");
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
        assert!(obj1.isKindOfClass(NSObject::class()));
        assert!(obj1.isKindOfClass(Custom::class()));
        assert!((**obj1).isKindOfClass(Custom::class()));
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
            ClassBuilder::new(&c("DefineClassDuplicateIvarSuperclass"), NSObject::class()).unwrap();
        superclass.add_ivar::<u8>(&c("ivar1"));
        superclass.add_ivar::<U128align16>(&c("ivar2"));
        superclass.add_ivar::<u8>(&c("ivar3"));
        superclass.add_ivar::<[u8; 0]>(&c("ivar4"));
        let superclass = superclass.register();

        let mut subclass =
            ClassBuilder::new(&c("DefineClassDuplicateIvarSubclass"), superclass).unwrap();
        // Try to overwrite instance variables
        subclass.add_ivar::<i16>(&c("ivar1"));
        subclass.add_ivar::<usize>(&c("ivar2"));
        subclass.add_ivar::<*const AnyObject>(&c("ivar3"));
        subclass.add_ivar::<usize>(&c("ivar4"));
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

        let superclass_ivar1 = superclass.instance_variable(&c("ivar1")).unwrap();
        let superclass_ivar2 = superclass.instance_variable(&c("ivar2")).unwrap();
        let superclass_ivar3 = superclass.instance_variable(&c("ivar3")).unwrap();
        let superclass_ivar4 = superclass.instance_variable(&c("ivar4")).unwrap();
        let subclass_ivar1 = subclass.instance_variable(&c("ivar1")).unwrap();
        let subclass_ivar2 = subclass.instance_variable(&c("ivar2")).unwrap();
        let subclass_ivar3 = subclass.instance_variable(&c("ivar3")).unwrap();
        let subclass_ivar4 = subclass.instance_variable(&c("ivar4")).unwrap();

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
        let obj: Retained<NSObject> = unsafe { msg_send![subclass, new] };
        let ptr = unsafe { *subclass_ivar3.load::<*const AnyObject>(&obj) };
        assert!(ptr.is_null());

        // Illustration of what goes wrong with the naive approach of loading
        // the Ivar dynamically; in short, we can't be sure of which instance
        // variable we're referring to here.
        //
        // let ivar = *obj.get_ivar::<u8>("ivar3");
    }

    #[test]
    fn auto_name() {
        define_class!(
            #[unsafe(super(NSObject))]
            #[ivars = ()]
            struct AutoName;
        );

        let expected = format!(
            "objc2::runtime::define::tests::AutoName{}",
            env!("CARGO_PKG_VERSION")
        );

        let cls = AutoName::class();
        assert_eq!(cls.name().to_str().unwrap(), expected);
        assert_eq!(AutoName::NAME, expected);
    }
}
