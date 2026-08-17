//! Helper import prelude for framework crates.

// Note: While this is not public, it is still a breaking change to remove
// entries in here, since framework crates rely on it.

pub use crate::encode::{Encode, Encoding, RefEncode};
pub use crate::ffi::{NSInteger, NSIntegerMax, NSUInteger, NSUIntegerMax};
pub use crate::rc::{Allocated, DefaultRetained, Retained};
pub use crate::runtime::{
    AnyClass, AnyObject, AnyProtocol, Bool, Imp, NSObject, NSObjectProtocol, ProtocolObject, Sel,
};
pub use crate::{
    cf_objc2_type, extern_class, extern_conformance, extern_methods, extern_protocol, ClassType,
    MainThreadMarker, MainThreadOnly, Message, ProtocolType,
};
