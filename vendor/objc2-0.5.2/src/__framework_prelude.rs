//! Helper import prelude for framework crates.

// Note: While this is not public, it is still a breaking change to remove
// entries in here, since framework crates rely on it.

pub use core::ffi::c_void;
pub use core::marker::PhantomData;
pub use core::ptr::NonNull;
pub use std::os::raw::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulong, c_ulonglong, c_ushort,
};

pub use crate::encode::{Encode, Encoding, RefEncode};
pub use crate::ffi::{NSInteger, NSIntegerMax, NSUInteger, NSUIntegerMax, IMP};
pub use crate::mutability::{
    Immutable, ImmutableWithMutableSubclass, InteriorMutable, IsIdCloneable, IsMainThreadOnly,
    IsRetainable, MainThreadOnly, Mutable, MutableWithImmutableSuperclass,
};
pub use crate::rc::{Allocated, DefaultId, DefaultRetained, Id, Retained};
pub use crate::runtime::{
    AnyClass, AnyObject, Bool, NSObject, NSObjectProtocol, ProtocolObject, Sel,
};
pub use crate::{
    __inner_extern_class, extern_category, extern_class, extern_methods, extern_protocol,
    ClassType, Message, ProtocolType,
};

// TODO
pub type AnyProtocol = AnyObject;
pub type TodoFunction = *const c_void;
pub type TodoClass = AnyObject;
pub type TodoProtocols = AnyObject;
