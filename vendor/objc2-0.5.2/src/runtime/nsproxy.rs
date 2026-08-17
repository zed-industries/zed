use core::fmt;
use core::hash;

use crate::mutability::Root;
use crate::runtime::{AnyClass, AnyObject, NSObjectProtocol, ProtocolObject};
use crate::ClassType;

/// An abstract superclass defining an API for objects that act as
/// stand-ins for other objects or for objects that don’t exist yet.
///
/// See [Apple's documentation][apple-doc] for more information.
///
/// [apple-doc]: https://developer.apple.com/documentation/foundation/nsproxy?language=objc
#[repr(C)]
pub struct NSProxy {
    __inner: AnyObject,
}

crate::__extern_class_impl_traits! {
    unsafe impl () for NSProxy {
        INHERITS = [AnyObject];

        fn as_super(&self) {
            &self.__inner
        }

        fn as_super_mut(&mut self) {
            &mut self.__inner
        }
    }
}

unsafe impl ClassType for NSProxy {
    type Super = AnyObject;
    type Mutability = Root;
    const NAME: &'static str = "NSProxy";

    #[inline]
    fn class() -> &'static AnyClass {
        crate::__class_inner!("NSProxy", "NSProxy")
    }

    #[inline]
    fn as_super(&self) -> &Self::Super {
        &self.__inner
    }

    #[inline]
    fn as_super_mut(&mut self) -> &mut Self::Super {
        &mut self.__inner
    }
}

unsafe impl NSObjectProtocol for NSProxy {}

impl PartialEq for NSProxy {
    #[inline]
    #[doc(alias = "isEqual:")]
    fn eq(&self, other: &Self) -> bool {
        self.isEqual(other)
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
