// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Module that contains the basic infrastructure for subclassing `GObject`.

use std::{any::Any, collections::BTreeMap, marker, mem, ptr};

use super::{interface::ObjectInterface, SignalId};
use crate::{
    ffi, gobject_ffi,
    object::{IsClass, IsInterface, ObjectSubclassIs, ParentClassIs},
    prelude::*,
    translate::*,
    Closure, InterfaceInfo, Object, Type, TypeFlags, TypeInfo, Value,
};

// rustdoc-stripper-ignore-next
/// A newly registered `glib::Type` that is currently still being initialized.
///
/// This allows running additional type-setup functions.
#[derive(Debug, PartialEq, Eq)]
pub struct InitializingType<T>(pub(crate) Type, pub(crate) marker::PhantomData<*const T>);

impl<T> IntoGlib for InitializingType<T> {
    type GlibType = ffi::GType;

    #[inline]
    fn into_glib(self) -> ffi::GType {
        self.0.into_glib()
    }
}

// rustdoc-stripper-ignore-next
/// Struct used for the instance private data of the GObject.
struct PrivateStruct<T: ObjectSubclass> {
    imp: T,
    instance_data: Option<BTreeMap<Type, Box<dyn Any + Send + Sync>>>,
}

// rustdoc-stripper-ignore-next
/// Trait implemented by structs that implement a `GObject` C instance struct.
///
/// The struct must be `#[repr(C)]` and have the parent type's instance struct
/// as the first field.
///
/// See [`basic::InstanceStruct`] for a basic implementation of this that can
/// be used most of the time and should only not be used if additional fields are
/// required in the instance struct.
///
/// [`basic::InstanceStruct`]: ../basic/struct.InstanceStruct.html
pub unsafe trait InstanceStruct: Sized + 'static {
    // rustdoc-stripper-ignore-next
    /// Corresponding object subclass type for this instance struct.
    type Type: ObjectSubclass;

    // rustdoc-stripper-ignore-next
    /// Instance specific initialization.
    ///
    /// This is automatically called during instance initialization and must call `instance_init()`
    /// of the parent class.
    #[inline]
    fn instance_init(&mut self) {
        unsafe {
            let obj = from_glib_borrow::<_, Object>(self as *mut _ as *mut gobject_ffi::GObject);
            let obj = Borrowed::new(obj.into_inner().unsafe_cast());
            let mut obj = InitializingObject(obj);

            <<Self::Type as ObjectSubclass>::ParentType as IsSubclassable<Self::Type>>::instance_init(
                &mut obj,
            );
        }
    }
}

// rustdoc-stripper-ignore-next
/// Trait implemented by any type implementing `InstanceStruct` to return the implementation, private Rust struct.
pub unsafe trait InstanceStructExt: InstanceStruct {
    // rustdoc-stripper-ignore-next
    /// Returns the implementation for from this instance struct, that
    /// is the implementor of [`ObjectImpl`] or subtraits.
    ///
    /// [`ObjectImpl`]: ../object/trait.ObjectImpl.html
    #[doc(alias = "get_impl")]
    fn imp(&self) -> &Self::Type;

    // rustdoc-stripper-ignore-next
    /// Returns the class struct for this specific instance.
    #[doc(alias = "get_class")]
    fn class(&self) -> &<Self::Type as ObjectSubclass>::Class;
}

// rustdoc-stripper-ignore-next
/// Offset `ptr` by `offset` *bytes* and cast the result to `*const U`.
///
/// The result must be a correctly aligned pointer to a valid value of type `U`.
///
/// # Panics:
///
/// This function panics if debug assertions are enabled if adding `offset` causes `ptr` to
/// overflow or if the resulting pointer is not correctly aligned.
#[inline]
fn offset_ptr_by_bytes<T, U>(ptr: *const T, offset: isize) -> *const U {
    // FIXME: Use `ptr::expose_addr()` once stable
    let ptr = ptr as usize;
    let ptr = if offset < 0 {
        ptr - (-offset) as usize
    } else {
        ptr + offset as usize
    };
    debug_assert_eq!(ptr & (mem::align_of::<U>() - 1), 0);
    ptr as *const U
}

// rustdoc-stripper-ignore-next
/// Offset `ptr` by `offset` *bytes* and cast the result to `*mut U`.
///
/// The result must be a correctly aligned pointer to a valid value of type `U`.
///
/// # Panics:
///
/// This function panics if debug assertions are enabled if adding `offset` causes `ptr` to
/// overflow or if the resulting pointer is not correctly aligned.
#[inline]
fn offset_ptr_by_bytes_mut<T, U>(ptr: *mut T, offset: isize) -> *mut U {
    // FIXME: Use `ptr::expose_addr()` once stable
    let ptr = ptr as usize;
    let ptr = if offset < 0 {
        ptr - (-offset) as usize
    } else {
        ptr + offset as usize
    };
    debug_assert_eq!(ptr & (mem::align_of::<U>() - 1), 0);
    ptr as *mut U
}

unsafe impl<T: InstanceStruct> InstanceStructExt for T {
    #[inline]
    fn imp(&self) -> &Self::Type {
        unsafe {
            let data = Self::Type::type_data();
            let private_offset = data.as_ref().impl_offset();
            let imp = offset_ptr_by_bytes::<T, Self::Type>(self, private_offset);
            &*imp
        }
    }

    #[inline]
    fn class(&self) -> &<Self::Type as ObjectSubclass>::Class {
        unsafe { &**(self as *const _ as *const *const <Self::Type as ObjectSubclass>::Class) }
    }
}

// rustdoc-stripper-ignore-next
/// Trait implemented by any type implementing `ObjectSubclassIs` to return the implementation, private Rust struct.
pub trait ObjectSubclassIsExt: ObjectSubclassIs {
    // rustdoc-stripper-ignore-next
    /// Returns the implementation (the private Rust struct) of this class instance
    fn imp(&self) -> &Self::Subclass;
}

impl<T: ObjectSubclassIs<Subclass = S>, S: ObjectSubclass<Type = Self>> ObjectSubclassIsExt for T {
    #[inline]
    fn imp(&self) -> &T::Subclass {
        T::Subclass::from_obj(self)
    }
}

// rustdoc-stripper-ignore-next
/// Trait implemented by structs that implement a `GObject` C class struct.
///
/// The struct must be `#[repr(C)]` and have the parent type's class struct
/// as the first field.
///
/// See [`basic::ClassStruct`] for a basic implementation of this that can
/// be used most of the time and should only not be used if additional fields are
/// required in the class struct, e.g. for declaring new virtual methods.
///
/// [`basic::ClassStruct`]: ../basic/struct.ClassStruct.html
pub unsafe trait ClassStruct: Sized + 'static {
    // rustdoc-stripper-ignore-next
    /// Corresponding object subclass type for this class struct.
    type Type: ObjectSubclass;

    // rustdoc-stripper-ignore-next
    /// Override the vfuncs of all parent types.
    ///
    /// This is automatically called during type initialization.
    #[inline]
    fn class_init(&mut self) {
        unsafe {
            let base = &mut *(self as *mut _
                as *mut crate::Class<<Self::Type as ObjectSubclass>::ParentType>);
            <<Self::Type as ObjectSubclass>::ParentType as IsSubclassable<Self::Type>>::class_init(
                base,
            );
        }
    }
}

// rustdoc-stripper-ignore-next
/// Trait for subclassable class structs.
pub unsafe trait IsSubclassable<T: ObjectSubclass>: IsSubclassableDefault<T> {
    // rustdoc-stripper-ignore-next
    /// Override the virtual methods of this class for the given subclass and do other class
    /// initialization.
    ///
    /// This is automatically called during type initialization and must call `class_init()` of the
    /// parent class.
    #[inline]
    fn class_init(class: &mut crate::Class<Self>) {
        Self::default_class_init(class);
    }

    // rustdoc-stripper-ignore-next
    /// Instance specific initialization.
    ///
    /// This is automatically called during instance initialization and must call `instance_init()`
    /// of the parent class.
    #[inline]
    fn instance_init(instance: &mut InitializingObject<T>) {
        Self::default_instance_init(instance);
    }
}

// FIXME: It should be possible to make implemented for all instances of `IsSubclassable<T>`
// with specialization, and make it private.
#[doc(hidden)]
pub trait IsSubclassableDefault<T: ObjectSubclass>: IsClass {
    fn default_class_init(class: &mut crate::Class<Self>);
    fn default_instance_init(instance: &mut InitializingObject<T>);
}

impl<T: ObjectSubclass, U: IsSubclassable<T> + ParentClassIs> IsSubclassableDefault<T> for U
where
    U::Parent: IsSubclassable<T>,
{
    #[inline]
    fn default_class_init(class: &mut crate::Class<Self>) {
        U::Parent::class_init(class);
    }

    #[inline]
    fn default_instance_init(instance: &mut InitializingObject<T>) {
        U::Parent::instance_init(instance);
    }
}

impl<T: ObjectSubclass> IsSubclassableDefault<T> for Object {
    #[inline]
    fn default_class_init(_class: &mut crate::Class<Self>) {}

    #[inline]
    fn default_instance_init(_instance: &mut InitializingObject<T>) {}
}

pub trait IsSubclassableExt: IsClass + ParentClassIs {
    fn parent_class_init<T: ObjectSubclass>(class: &mut crate::Class<Self>)
    where
        Self::Parent: IsSubclassable<T>;
    fn parent_instance_init<T: ObjectSubclass>(instance: &mut InitializingObject<T>)
    where
        Self::Parent: IsSubclassable<T>;
}

impl<U: IsClass + ParentClassIs> IsSubclassableExt for U {
    #[inline]
    fn parent_class_init<T: ObjectSubclass>(class: &mut crate::Class<Self>)
    where
        U::Parent: IsSubclassable<T>,
    {
        Self::Parent::class_init(class);
    }

    #[inline]
    fn parent_instance_init<T: ObjectSubclass>(instance: &mut InitializingObject<T>)
    where
        U::Parent: IsSubclassable<T>,
    {
        Self::Parent::instance_init(instance);
    }
}

// rustdoc-stripper-ignore-next
/// Trait implemented by structs that implement a `GTypeInterface` C class struct.
///
/// This must only be implemented on `#[repr(C)]` structs and have an interface
/// that inherits from `gobject_ffi::GTypeInterface` as the first field.
pub unsafe trait InterfaceStruct: Sized + 'static
where
    Self: Copy,
{
    // rustdoc-stripper-ignore-next
    /// Corresponding object interface type for this class struct.
    type Type: ObjectInterface;

    // rustdoc-stripper-ignore-next
    /// Set up default implementations for interface vfuncs.
    ///
    /// This is automatically called during type initialization.
    #[inline]
    fn interface_init(&mut self) {}
}

// rustdoc-stripper-ignore-next
/// Trait for implementable interfaces.
pub unsafe trait IsImplementable<T: ObjectSubclass>: IsInterface {
    // rustdoc-stripper-ignore-next
    /// Override the virtual methods of this interface for the given subclass and do other
    /// interface initialization.
    ///
    /// This is automatically called during type initialization.
    fn interface_init(_iface: &mut crate::Interface<Self>) {}

    // rustdoc-stripper-ignore-next
    /// Instance specific initialization.
    ///
    /// This is automatically called during instance initialization.
    fn instance_init(_instance: &mut InitializingObject<T>) {}
}

unsafe extern "C" fn interface_init<T: ObjectSubclass, A: IsImplementable<T>>(
    iface: ffi::gpointer,
    _iface_data: ffi::gpointer,
) where
    <A as ObjectType>::GlibClassType: Copy,
{
    let iface = &mut *(iface as *mut crate::Interface<A>);

    let mut data = T::type_data();
    if data.as_ref().parent_ifaces.is_none() {
        data.as_mut().parent_ifaces = Some(BTreeMap::default());
    }
    {
        let copy = Box::new(*iface.as_ref());
        data.as_mut()
            .parent_ifaces
            .as_mut()
            .unwrap()
            .insert(A::static_type(), Box::into_raw(copy) as ffi::gpointer);
    }

    A::interface_init(iface);
}

// rustdoc-stripper-ignore-next
/// Trait for a type list of interfaces.
pub trait InterfaceList<T: ObjectSubclass> {
    // rustdoc-stripper-ignore-next
    /// Returns the list of types and corresponding interface infos for this list.
    fn iface_infos() -> Vec<(Type, InterfaceInfo)>;

    // rustdoc-stripper-ignore-next
    /// Runs `instance_init` on each of the `IsImplementable` items.
    fn instance_init(_instance: &mut InitializingObject<T>);
}

impl<T: ObjectSubclass> InterfaceList<T> for () {
    fn iface_infos() -> Vec<(Type, InterfaceInfo)> {
        vec![]
    }

    #[inline]
    fn instance_init(_instance: &mut InitializingObject<T>) {}
}

impl<T: ObjectSubclass, A: IsImplementable<T>> InterfaceList<T> for (A,)
where
    <A as ObjectType>::GlibClassType: Copy,
{
    fn iface_infos() -> Vec<(Type, InterfaceInfo)> {
        vec![(
            A::static_type(),
            InterfaceInfo(gobject_ffi::GInterfaceInfo {
                interface_init: Some(interface_init::<T, A>),
                ..InterfaceInfo::default().0
            }),
        )]
    }

    #[inline]
    fn instance_init(instance: &mut InitializingObject<T>) {
        A::instance_init(instance);
    }
}

// Generates all the InterfaceList impls for interface_lists of arbitrary sizes based on a list of type
// parameters like A B C. It would generate the impl then for (A, B) and (A, B, C).
macro_rules! interface_list_trait(
    ($name1:ident, $name2: ident, $($name:ident),*) => (
        interface_list_trait!(__impl $name1, $name2; $($name),*);
    );
    (__impl $($name:ident),+; $name1:ident, $($name2:ident),*) => (
        interface_list_trait_impl!($($name),+);
        interface_list_trait!(__impl $($name),+ , $name1; $($name2),*);
    );
    (__impl $($name:ident),+; $name1:ident) => (
        interface_list_trait_impl!($($name),+);
        interface_list_trait_impl!($($name),+, $name1);
    );
);

// Generates the impl block for InterfaceList on interface_lists or arbitrary sizes based on its
// arguments. Takes a list of type parameters as parameters, e.g. A B C
// and then implements the trait on (A, B, C).
macro_rules! interface_list_trait_impl(
    ($($name:ident),+) => (
        impl<T: ObjectSubclass, $($name: IsImplementable<T>),+> InterfaceList<T> for ( $($name),+ )
        where
            $(<$name as ObjectType>::GlibClassType: Copy),+
        {
            fn iface_infos() -> Vec<(Type, InterfaceInfo)> {
                vec![
                    $(
                        (
                            $name::static_type(),
                            InterfaceInfo(gobject_ffi::GInterfaceInfo {
                                interface_init: Some(interface_init::<T, $name>),
                                interface_finalize: None,
                                interface_data: ptr::null_mut(),
                            }),
                        )
                    ),+
                ]
            }

            #[inline]
            fn instance_init(instance: &mut InitializingObject<T>) {
                $(
                    $name::instance_init(instance);
                )+
            }
        }
    );
);

interface_list_trait!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);

/// Type-specific data that is filled in during type creation.
pub struct TypeData {
    type_: Type,
    parent_class: ffi::gpointer,
    parent_ifaces: Option<BTreeMap<Type, ffi::gpointer>>,
    class_data: Option<BTreeMap<Type, Box<dyn Any + Send + Sync>>>,
    private_offset: isize,
    private_imp_offset: isize,
}

unsafe impl Send for TypeData {}
unsafe impl Sync for TypeData {}

impl TypeData {
    pub const fn new() -> Self {
        Self {
            type_: Type::INVALID,
            parent_class: ::std::ptr::null_mut(),
            parent_ifaces: None,
            class_data: None,
            private_offset: 0,
            private_imp_offset: 0,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the type ID.
    #[inline]
    #[doc(alias = "get_type")]
    pub fn type_(&self) -> Type {
        self.type_
    }

    // rustdoc-stripper-ignore-next
    /// Returns a pointer to the native parent class.
    ///
    /// This is used for chaining up to the parent class' implementation
    /// of virtual methods.
    #[doc(alias = "get_parent_class")]
    #[inline]
    pub fn parent_class(&self) -> ffi::gpointer {
        debug_assert!(!self.parent_class.is_null());
        self.parent_class
    }

    // rustdoc-stripper-ignore-next
    /// Returns a pointer to the native parent interface struct for interface `type_`.
    ///
    /// This is used for chaining up to the parent interface's implementation
    /// of virtual methods.
    ///
    /// # Panics
    ///
    /// This function panics if the type to which the `TypeData` belongs does not implement the
    /// given interface or was not registered yet.
    #[doc(alias = "get_parent_interface")]
    pub fn parent_interface<I: crate::object::IsInterface>(&self) -> ffi::gpointer {
        match self.parent_ifaces {
            None => unreachable!("No parent interfaces"),
            Some(ref parent_ifaces) => *parent_ifaces
                .get(&I::static_type())
                .expect("Parent interface not found"),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns a pointer to the class implementation specific data.
    ///
    /// This is used for class implementations to store additional data.
    #[doc(alias = "get_class_data")]
    pub fn class_data<T: Any + Send + Sync + 'static>(&self, type_: Type) -> Option<&T> {
        match self.class_data {
            None => None,
            Some(ref data) => data.get(&type_).and_then(|ptr| ptr.downcast_ref()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets a mutable reference of the class implementation specific data.
    ///
    /// # Safety
    ///
    /// This can only be used while the type is being initialized.
    #[doc(alias = "get_class_data_mut")]
    pub unsafe fn class_data_mut<T: Any + Send + Sync + 'static>(
        &mut self,
        type_: Type,
    ) -> Option<&mut T> {
        match self.class_data {
            None => None,
            Some(ref mut data) => data.get_mut(&type_).and_then(|v| v.downcast_mut()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets class specific implementation data.
    ///
    /// # Safety
    ///
    /// This can only be used while the type is being initialized.
    ///
    /// # Panics
    ///
    /// If the class_data already contains a data for the specified `type_`.
    pub unsafe fn set_class_data<T: Any + Send + Sync + 'static>(&mut self, type_: Type, data: T) {
        if self.class_data.is_none() {
            self.class_data = Some(BTreeMap::default());
        }

        if let Some(ref mut class_data) = self.class_data {
            assert!(
                class_data.get(&type_).is_none(),
                "The class_data already contains a key for {type_}",
            );

            class_data.insert(type_, Box::new(data));
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the offset of the private implementation struct in bytes relative to the beginning
    /// of the instance struct.
    #[doc(alias = "get_impl_offset")]
    #[inline]
    pub fn impl_offset(&self) -> isize {
        self.private_offset + self.private_imp_offset
    }
}

impl Default for TypeData {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// Type methods required for an [`ObjectSubclass`] implementation.
///
/// This is usually generated by the [`#[object_subclass]`](crate::object_subclass) attribute macro.
pub unsafe trait ObjectSubclassType {
    // rustdoc-stripper-ignore-next
    /// Storage for the type-specific data used during registration.
    fn type_data() -> ptr::NonNull<TypeData>;

    // rustdoc-stripper-ignore-next
    /// Returns the `glib::Type` ID of the subclass.
    ///
    /// This will register the type with the type system on the first call.
    #[doc(alias = "get_type")]
    fn type_() -> Type;
}

// rustdoc-stripper-ignore-next
/// The central trait for subclassing a `GObject` type.
///
/// Links together the type name, parent type and the instance and
/// class structs for type registration and allows subclasses to
/// hook into various steps of the type registration and initialization.
///
/// See [`register_type`] for registering an implementation of this trait
/// with the type system.
///
/// [`register_type`]: fn.register_type.html
pub trait ObjectSubclass: ObjectSubclassType + Sized + 'static {
    // rustdoc-stripper-ignore-next
    /// `GObject` type name.
    ///
    /// This must be unique in the whole process.
    const NAME: &'static str;

    // rustdoc-stripper-ignore-next
    /// If this subclass is an abstract class or not.
    ///
    /// By default, all subclasses are non-abstract types but setting this to `true` will create an
    /// abstract class instead.
    ///
    /// Abstract classes can't be instantiated and require a non-abstract subclass.
    ///
    /// Optional.
    const ABSTRACT: bool = false;

    // rustdoc-stripper-ignore-next
    /// Allow name conflicts for this class.
    ///
    /// By default, trying to register a type with a name that was registered before will panic. If
    /// this is set to `true` then a new name will be selected by appending a counter.
    ///
    /// This is useful for defining new types in Rust library crates that might be linked multiple
    /// times in the same process.
    ///
    /// A consequence of setting this to `true` is that it's not guaranteed that
    /// `glib::Type::from_name(Self::NAME).unwrap() == Self::type_()`.
    ///
    /// Note that this is not allowed for dynamic types. If a dynamic type is registered and a type
    /// with that name exists already, it is assumed that they're the same.
    ///
    /// Optional.
    const ALLOW_NAME_CONFLICT: bool = false;

    // rustdoc-stripper-ignore-next
    /// Wrapper around this subclass defined with `wrapper!`
    type Type: ObjectType
        + ObjectSubclassIs<Subclass = Self>
        + FromGlibPtrFull<*mut <Self::Type as ObjectType>::GlibType>
        + FromGlibPtrBorrow<*mut <Self::Type as ObjectType>::GlibType>
        + FromGlibPtrNone<*mut <Self::Type as ObjectType>::GlibType>;

    // rustdoc-stripper-ignore-next
    /// Parent Rust type to inherit from.
    type ParentType: IsSubclassable<Self>
        + FromGlibPtrFull<*mut <Self::ParentType as ObjectType>::GlibType>
        + FromGlibPtrBorrow<*mut <Self::ParentType as ObjectType>::GlibType>
        + FromGlibPtrNone<*mut <Self::ParentType as ObjectType>::GlibType>;

    // rustdoc-stripper-ignore-next
    /// List of interfaces implemented by this type.
    type Interfaces: InterfaceList<Self>;

    // rustdoc-stripper-ignore-next
    /// The C instance struct.
    ///
    /// See [`basic::InstanceStruct`] for an basic instance struct that should be
    /// used in most cases.
    ///
    /// [`basic::InstanceStruct`]: ../basic/struct.InstanceStruct.html
    // TODO: Should default to basic::InstanceStruct<Self> once associated
    // type defaults are stabilized https://github.com/rust-lang/rust/issues/29661
    type Instance: InstanceStruct<Type = Self>;

    // rustdoc-stripper-ignore-next
    /// The C class struct.
    ///
    /// See [`basic::ClassStruct`] for an basic class struct that should be
    /// used in most cases.
    ///
    /// [`basic::ClassStruct`]: ../basic/struct.ClassStruct.html
    // TODO: Should default to basic::ClassStruct<Self> once associated
    // type defaults are stabilized https://github.com/rust-lang/rust/issues/29661
    type Class: ClassStruct<Type = Self>;

    // rustdoc-stripper-ignore-next
    /// Additional type initialization.
    ///
    /// This is called right after the type was registered and allows
    /// subclasses to do additional type-specific initialization, e.g.
    /// for implementing `GObject` interfaces.
    ///
    /// Optional
    #[inline]
    fn type_init(_type_: &mut InitializingType<Self>) {}

    /// Class initialization.
    ///
    // rustdoc-stripper-ignore-next
    /// This is called after `type_init` and before the first instance
    /// of the subclass is created. Subclasses can use this to do class-
    /// specific initialization, e.g. for registering signals on the class
    /// or calling class methods.
    ///
    /// Optional
    #[inline]
    fn class_init(_klass: &mut Self::Class) {}

    // rustdoc-stripper-ignore-next
    /// Constructor.
    ///
    /// This is called during object instantiation before further subclasses
    /// are initialized, and should return a new instance of the subclass
    /// private struct.
    ///
    /// Optional, either implement this or `with_class()`.
    fn new() -> Self {
        unimplemented!();
    }

    // rustdoc-stripper-ignore-next
    /// Constructor.
    ///
    /// This is called during object instantiation before further subclasses
    /// are initialized, and should return a new instance of the subclass
    /// private struct.
    ///
    /// Different to `new()` above it also gets the class of this type passed
    /// to itself for providing additional context.
    ///
    /// Optional, either implement this or `new()`.
    #[inline]
    fn with_class(_klass: &Self::Class) -> Self {
        Self::new()
    }

    // rustdoc-stripper-ignore-next
    /// Performs additional instance initialization.
    ///
    /// Called just after `with_class()`. At this point the initialization has not completed yet, so
    /// only a limited set of operations is safe (see `InitializingObject`).
    #[inline]
    fn instance_init(_obj: &InitializingObject<Self>) {}
}

// rustdoc-stripper-ignore-next
/// Extension methods for all `ObjectSubclass` impls.
pub trait ObjectSubclassExt: ObjectSubclass {
    // rustdoc-stripper-ignore-next
    /// Returns the corresponding object instance.
    ///
    /// Shorter alias for `instance()`.
    #[doc(alias = "get_instance")]
    fn obj(&self) -> crate::BorrowedObject<'_, Self::Type>;

    // rustdoc-stripper-ignore-next
    /// Returns the implementation from an instance.
    ///
    /// Shorter alias for `from_instance()`.
    fn from_obj(obj: &Self::Type) -> &Self;

    // rustdoc-stripper-ignore-next
    /// Returns a new reference-counted wrapper around `self`.
    fn ref_counted(&self) -> super::ObjectImplRef<Self>;

    // rustdoc-stripper-ignore-next
    /// Returns a pointer to the instance implementation specific data.
    ///
    /// This is used for the subclassing infrastructure to store additional instance data.
    #[doc(alias = "get_instance_data")]
    fn instance_data<U: Any + Send + Sync + 'static>(&self, type_: Type) -> Option<&U>;
}

impl<T: ObjectSubclass> ObjectSubclassExt for T {
    #[inline]
    fn obj(&self) -> crate::BorrowedObject<'_, Self::Type> {
        unsafe {
            let data = Self::type_data();
            let type_ = data.as_ref().type_();
            debug_assert!(type_.is_valid());

            let offset = -data.as_ref().impl_offset();
            let ptr =
                offset_ptr_by_bytes::<Self, <Self::Type as ObjectType>::GlibType>(self, offset);

            // The object might just be finalized, and in that case it's unsafe to access
            // it and use any API on it. This can only happen from inside the Drop impl
            // of Self.
            debug_assert_ne!((*(ptr as *const gobject_ffi::GObject)).ref_count, 0);

            crate::BorrowedObject::new(mut_override(ptr))
        }
    }

    #[inline]
    fn from_obj(obj: &Self::Type) -> &Self {
        unsafe {
            let ptr = obj.as_ptr() as *const Self::Instance;
            (*ptr).imp()
        }
    }

    #[inline]
    fn ref_counted(&self) -> super::ObjectImplRef<Self> {
        super::ObjectImplRef::new(self)
    }

    #[inline]
    fn instance_data<U: Any + Send + Sync + 'static>(&self, type_: Type) -> Option<&U> {
        unsafe {
            let type_data = Self::type_data();
            let self_type_ = type_data.as_ref().type_();
            debug_assert!(self_type_.is_valid());

            let offset = -type_data.as_ref().private_imp_offset;
            let ptr = offset_ptr_by_bytes::<Self, PrivateStruct<Self>>(self, offset);
            let priv_ = &*ptr;

            match priv_.instance_data {
                None => None,
                Some(ref data) => data.get(&type_).and_then(|ptr| ptr.downcast_ref()),
            }
        }
    }
}

// rustdoc-stripper-ignore-next
/// Helper trait for macros to access a subclass or its wrapper.
pub trait FromObject {
    type FromObjectType;
    fn from_object(obj: &Self::FromObjectType) -> &Self;
}

// rustdoc-stripper-ignore-next
/// An object that is currently being initialized.
///
/// Binding crates should use traits for adding methods to this struct. Only methods explicitly safe
/// to call during `instance_init()` should be added.
pub struct InitializingObject<T: ObjectSubclass>(Borrowed<T::Type>);

impl<T: ObjectSubclass> InitializingObject<T> {
    // rustdoc-stripper-ignore-next
    /// Returns a reference to the object.
    ///
    /// # Safety
    ///
    /// The returned object has not been completely initialized at this point. Use of the object
    /// should be restricted to methods that are explicitly documented to be safe to call during
    /// `instance_init()`.
    #[inline]
    pub unsafe fn as_ref(&self) -> &T::Type {
        &self.0
    }

    // rustdoc-stripper-ignore-next
    /// Returns a pointer to the object.
    ///
    /// # Safety
    ///
    /// The returned object has not been completely initialized at this point. Use of the object
    /// should be restricted to methods that are explicitly documented to be safe to call during
    /// `instance_init()`.
    #[inline]
    pub fn as_ptr(&self) -> *mut T::Type {
        self.0.as_ptr() as *const T::Type as *mut T::Type
    }

    // rustdoc-stripper-ignore-next
    /// Sets instance specific implementation data.
    ///
    /// # Panics
    ///
    /// If the instance_data already contains a data for the specified `type_`.
    pub fn set_instance_data<U: Any + Send + Sync + 'static>(&mut self, type_: Type, data: U) {
        unsafe {
            let type_data = T::type_data();
            let self_type_ = type_data.as_ref().type_();
            debug_assert!(self_type_.is_valid());

            let offset = type_data.as_ref().private_offset;

            let ptr = offset_ptr_by_bytes_mut::<
                <<T as ObjectSubclass>::Type as ObjectType>::GlibType,
                PrivateStruct<T>,
            >(self.0.as_ptr(), offset);
            let priv_ = &mut *ptr;

            if priv_.instance_data.is_none() {
                priv_.instance_data = Some(BTreeMap::default());
            }

            if let Some(ref mut instance_data) = priv_.instance_data {
                assert!(
                    instance_data.get(&type_).is_none(),
                    "The class_data already contains a key for {type_}",
                );

                instance_data.insert(type_, Box::new(data));
            }
        }
    }
}

unsafe extern "C" fn class_init<T: ObjectSubclass>(
    klass: ffi::gpointer,
    _klass_data: ffi::gpointer,
) {
    let mut data = T::type_data();

    // We have to update the private struct offset once the class is actually
    // being initialized.
    let mut private_offset = data.as_ref().private_offset as i32;
    gobject_ffi::g_type_class_adjust_private_offset(klass, &mut private_offset);
    data.as_mut().private_offset = private_offset as isize;

    // Set trampolines for the basic GObject virtual methods.
    {
        let gobject_klass = &mut *(klass as *mut gobject_ffi::GObjectClass);

        gobject_klass.finalize = Some(finalize::<T>);
    }

    // And finally peek the parent class struct (containing the parent class'
    // implementations of virtual methods for chaining up), and call the subclass'
    // class initialization function.
    {
        let klass = &mut *(klass as *mut T::Class);
        let parent_class = gobject_ffi::g_type_class_peek_parent(klass as *mut _ as ffi::gpointer)
            as *mut <T::ParentType as ObjectType>::GlibClassType;
        debug_assert!(!parent_class.is_null());

        data.as_mut().parent_class = parent_class as ffi::gpointer;

        klass.class_init();
        T::class_init(klass);
    }
}

unsafe extern "C" fn instance_init<T: ObjectSubclass>(
    obj: *mut gobject_ffi::GTypeInstance,
    klass: ffi::gpointer,
) {
    // Get offset to the storage of our private struct, create it
    // and actually store it in that place.
    let mut data = T::type_data();
    let private_offset = data.as_mut().private_offset;
    let priv_ptr = offset_ptr_by_bytes_mut::<gobject_ffi::GTypeInstance, PrivateStruct<T>>(
        obj,
        private_offset,
    );

    assert!(
        priv_ptr as usize & (mem::align_of::<PrivateStruct<T>>() - 1) == 0,
        "Private instance data has higher alignment requirements ({}) than \
         the allocation from GLib. If alignment of more than {} bytes \
         is required, store the corresponding data separately on the heap.",
        mem::align_of::<PrivateStruct<T>>(),
        2 * mem::size_of::<usize>(),
    );

    let klass = &*(klass as *const T::Class);

    let imp = T::with_class(klass);
    ptr::write(
        priv_ptr,
        PrivateStruct {
            imp,
            instance_data: None,
        },
    );

    // Any additional instance initialization.
    T::Instance::instance_init(&mut *(obj as *mut _));

    let obj = from_glib_borrow::<_, Object>(obj.cast());
    let obj = Borrowed::new(obj.into_inner().unsafe_cast());
    let mut obj = InitializingObject(obj);

    T::Interfaces::instance_init(&mut obj);
    T::instance_init(&obj);
}

unsafe extern "C" fn finalize<T: ObjectSubclass>(obj: *mut gobject_ffi::GObject) {
    // Retrieve the private struct and drop it for freeing all associated memory.
    let mut data = T::type_data();
    let private_offset = data.as_mut().private_offset;
    let priv_ptr =
        offset_ptr_by_bytes_mut::<gobject_ffi::GObject, PrivateStruct<T>>(obj, private_offset);
    ptr::drop_in_place(ptr::addr_of_mut!((*priv_ptr).imp));
    ptr::drop_in_place(ptr::addr_of_mut!((*priv_ptr).instance_data));

    // Chain up to the parent class' finalize implementation, if any.
    let parent_class = &*(data.as_ref().parent_class() as *const gobject_ffi::GObjectClass);
    if let Some(ref func) = parent_class.finalize {
        func(obj);
    }
}

// rustdoc-stripper-ignore-next
/// Register a `glib::Type` ID for `T`.
///
/// This must be called only once and will panic on a second call.
///
/// The [`object_subclass!`] macro will create a `type_()` function around this, which will
/// ensure that it's only ever called once.
///
/// [`object_subclass!`]: ../../macro.object_subclass.html
pub fn register_type<T: ObjectSubclass>() -> Type {
    // GLib aligns the type private data to two gsizes, so we can't safely store any type there that
    // requires a bigger alignment.
    assert!(
        mem::align_of::<T>() <= 2 * mem::size_of::<usize>(),
        "Alignment {} of type not supported, bigger than {}",
        mem::align_of::<T>(),
        2 * mem::size_of::<usize>(),
    );

    unsafe {
        use std::ffi::CString;

        let type_name = if T::ALLOW_NAME_CONFLICT {
            let mut i = 0;
            loop {
                let type_name = CString::new(if i == 0 {
                    T::NAME.to_string()
                } else {
                    format!("{}-{}", T::NAME, i)
                })
                .unwrap();
                if gobject_ffi::g_type_from_name(type_name.as_ptr()) == gobject_ffi::G_TYPE_INVALID
                {
                    break type_name;
                }
                i += 1;
            }
        } else {
            let type_name = CString::new(T::NAME).unwrap();
            assert_eq!(
                gobject_ffi::g_type_from_name(type_name.as_ptr()),
                gobject_ffi::G_TYPE_INVALID,
                "Type {} has already been registered",
                type_name.to_str().unwrap()
            );

            type_name
        };

        let type_ = Type::from_glib(gobject_ffi::g_type_register_static_simple(
            <T::ParentType as StaticType>::static_type().into_glib(),
            type_name.as_ptr(),
            mem::size_of::<T::Class>() as u32,
            Some(class_init::<T>),
            mem::size_of::<T::Instance>() as u32,
            Some(instance_init::<T>),
            if T::ABSTRACT {
                gobject_ffi::G_TYPE_FLAG_ABSTRACT
            } else {
                0
            },
        ));
        assert!(type_.is_valid());

        let mut data = T::type_data();
        data.as_mut().type_ = type_;

        let private_offset = gobject_ffi::g_type_add_instance_private(
            type_.into_glib(),
            mem::size_of::<PrivateStruct<T>>(),
        );
        data.as_mut().private_offset = private_offset as isize;

        // Get the offset from PrivateStruct<T> to the imp field in it. This has to go through
        // some hoops because Rust doesn't have an offsetof operator yet.
        data.as_mut().private_imp_offset = {
            // Must not be a dangling pointer so let's create some uninitialized memory
            let priv_ = mem::MaybeUninit::<PrivateStruct<T>>::uninit();
            let ptr = priv_.as_ptr();
            let imp_ptr = ptr::addr_of!((*ptr).imp);
            (imp_ptr as isize) - (ptr as isize)
        };

        let iface_types = T::Interfaces::iface_infos();
        for (iface_type, iface_info) in iface_types {
            gobject_ffi::g_type_add_interface_static(
                type_.into_glib(),
                iface_type.into_glib(),
                iface_info.as_ptr(),
            );
        }

        T::type_init(&mut InitializingType::<T>(type_, marker::PhantomData));

        type_
    }
}

// rustdoc-stripper-ignore-next
/// Registers a `glib::Type` ID for `T` as a dynamic type.
///
/// An object subclass must be explicitly registered as a dynamic type when the
/// system loads the implementation by calling [`TypePluginImpl::use_`] or more
/// specifically [`TypeModuleImpl::load`]. Therefore, unlike for object
/// subclasses registered as static types, object subclasses registered as
/// dynamic types can be registered several times.
///
/// The [`object_subclass_dynamic!`] macro helper attribute will create
/// `register_type()` and `on_implementation_load()` functions around this,
/// which will ensure that the function is called when necessary.
///
/// [`object_subclass_dynamic!`]: ../../../glib_macros/attr.object_subclass.html
/// [`TypePluginImpl::use_`]: ../type_plugin/trait.TypePluginImpl.html#method.use_
/// [`TypeModuleImpl::load`]: ../type_module/trait.TypeModuleImpl.html#method.load
pub fn register_dynamic_type<P: DynamicObjectRegisterExt, T: ObjectSubclass>(
    type_plugin: &P,
) -> Type {
    // GLib aligns the type private data to two gsizes, so we can't safely store any type there that
    // requires a bigger alignment.
    assert!(
        mem::align_of::<T>() <= 2 * mem::size_of::<usize>(),
        "Alignment {} of type not supported, bigger than {}",
        mem::align_of::<T>(),
        2 * mem::size_of::<usize>(),
    );

    unsafe {
        use std::ffi::CString;

        let type_name = CString::new(T::NAME).unwrap();

        let already_registered =
            gobject_ffi::g_type_from_name(type_name.as_ptr()) != gobject_ffi::G_TYPE_INVALID;

        let type_info = TypeInfo(gobject_ffi::GTypeInfo {
            class_size: mem::size_of::<T::Class>() as u16,
            class_init: Some(class_init::<T>),
            instance_size: mem::size_of::<T::Instance>() as u16,
            instance_init: Some(instance_init::<T>),
            ..TypeInfo::default().0
        });

        // registers the type within the `type_plugin`
        let type_ = type_plugin.register_dynamic_type(
            <T::ParentType as StaticType>::static_type(),
            type_name.to_str().unwrap(),
            &type_info,
            if T::ABSTRACT {
                TypeFlags::ABSTRACT
            } else {
                TypeFlags::NONE
            },
        );
        assert!(type_.is_valid());

        let mut data = T::type_data();
        data.as_mut().type_ = type_;

        let private_offset = mem::size_of::<PrivateStruct<T>>();
        data.as_mut().private_offset = private_offset as isize;

        // gets the offset from PrivateStruct<T> to the imp field in it. This has to go through
        // some hoops because Rust doesn't have an offsetof operator yet.
        data.as_mut().private_imp_offset = {
            // Must not be a dangling pointer so let's create some uninitialized memory
            let priv_ = mem::MaybeUninit::<PrivateStruct<T>>::uninit();
            let ptr = priv_.as_ptr();
            let imp_ptr = ptr::addr_of!((*ptr).imp);
            (imp_ptr as isize) - (ptr as isize)
        };

        let plugin_ptr = type_plugin.as_ref().to_glib_none().0;
        let iface_types = T::Interfaces::iface_infos();
        for (iface_type, iface_info) in iface_types {
            match gobject_ffi::g_type_get_plugin(iface_type.into_glib()) {
                // if interface type's plugin is null or is different to the `type_plugin`,
                // then interface can only be added as if the type was static
                iface_plugin if iface_plugin != plugin_ptr => {
                    // but adding interface to a static type can be done only once
                    if !already_registered {
                        gobject_ffi::g_type_add_interface_static(
                            type_.into_glib(),
                            iface_type.into_glib(),
                            iface_info.as_ptr(),
                        );
                    }
                }
                // else interface can be added and registered to live in the `type_plugin`
                _ => type_plugin.add_dynamic_interface(type_, iface_type, &iface_info),
            }
        }

        T::type_init(&mut InitializingType::<T>(type_, marker::PhantomData));

        type_
    }
}

pub(crate) unsafe fn signal_override_class_handler<F>(
    name: &str,
    type_: ffi::GType,
    class_handler: F,
) where
    F: Fn(&super::SignalClassHandlerToken, &[Value]) -> Option<Value> + Send + Sync + 'static,
{
    let (signal_id, _) = SignalId::parse_name(name, from_glib(type_), false)
        .unwrap_or_else(|| panic!("Signal '{name}' not found"));

    let query = signal_id.query();
    let return_type = query.return_type();

    let class_handler = Closure::new(move |values| {
        let instance = gobject_ffi::g_value_get_object(values[0].to_glib_none().0);
        let res = class_handler(
            &super::SignalClassHandlerToken(
                instance as *mut _,
                return_type.into(),
                values.as_ptr(),
            ),
            values,
        );

        if return_type == Type::UNIT {
            if let Some(ref v) = res {
                panic!(
                    "Signal has no return value but class handler returned a value of type {}",
                    v.type_()
                );
            }
        } else {
            match res {
                None => {
                    panic!("Signal has a return value but class handler returned none");
                }
                Some(ref v) => {
                    assert!(
                        v.type_().is_a(return_type.into()),
                        "Signal has a return type of {} but class handler returned {}",
                        Type::from(return_type),
                        v.type_()
                    );
                }
            }
        }

        res
    });

    gobject_ffi::g_signal_override_class_closure(
        signal_id.into_glib(),
        type_,
        class_handler.to_glib_none().0,
    );
}

pub(crate) unsafe fn signal_chain_from_overridden(
    instance: *mut gobject_ffi::GTypeInstance,
    token: &super::SignalClassHandlerToken,
    values: &[Value],
) -> Option<Value> {
    assert_eq!(instance, token.0);
    assert_eq!(
        values.as_ptr(),
        token.2,
        "Arguments must be forwarded without changes when chaining up"
    );

    let mut result = Value::from_type_unchecked(token.1);
    gobject_ffi::g_signal_chain_from_overridden(
        values.as_ptr() as *mut Value as *mut gobject_ffi::GValue,
        result.to_glib_none_mut().0,
    );
    Some(result).filter(|r| r.type_().is_valid() && r.type_() != Type::UNIT)
}
