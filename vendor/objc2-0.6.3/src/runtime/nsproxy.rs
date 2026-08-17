//! Defined here instead of in `objc2-foundation` since it's a root object,
//! and the `extern_class!` macro doesn't support those (yet).
use core::fmt;
use core::hash;

use crate::runtime::{AnyClass, AnyObject, NSObjectProtocol, ProtocolObject};
use crate::{extern_conformance, AnyThread, ClassType, DowncastTarget};

/// An abstract superclass defining an API for objects that act as
/// stand-ins for other objects or for objects that don’t exist yet.
///
/// See [Apple's documentation][apple-doc] for more information.
///
/// [apple-doc]: https://developer.apple.com/documentation/foundation/nsproxy?language=objc
#[repr(C)]
pub struct NSProxy {
    __superclass: AnyObject,
}

crate::__extern_class_impl_traits! {
    ()
    (unsafe impl)
    (NSProxy)
    (AnyObject)
}

unsafe impl ClassType for NSProxy {
    type Super = AnyObject;
    type ThreadKind = dyn AnyThread;
    const NAME: &'static str = "NSProxy";

    #[inline]
    fn class() -> &'static AnyClass {
        crate::__class_inner!("NSProxy", "NSProxy")
    }

    #[inline]
    fn as_super(&self) -> &Self::Super {
        &self.__superclass
    }

    const __INNER: () = ();

    // We don't know anything about NSProxy's thread safety, so we don't do
    // the same workaround for that as we do for NSObject.
    type __SubclassingType = Self;
}

unsafe impl DowncastTarget for NSProxy {}

extern_conformance!(
    unsafe impl NSObjectProtocol for NSProxy {}
);

impl PartialEq for NSProxy {
    #[inline]
    #[doc(alias = "isEqual:")]
    fn eq(&self, other: &Self) -> bool {
        self.isEqual(Some(other))
    }
}

impl Eq for NSProxy {}

impl hash::Hash for NSProxy {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        <NSProxy as NSObjectProtocol>::hash(self).hash(state);
    }
}

impl fmt::Debug for NSProxy {
    #[inline]
    #[doc(alias = "description")]
    #[doc(alias = "debugDescription")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let obj: &ProtocolObject<dyn NSObjectProtocol> = ProtocolObject::from_ref(self);
        obj.fmt(f)
    }
}
