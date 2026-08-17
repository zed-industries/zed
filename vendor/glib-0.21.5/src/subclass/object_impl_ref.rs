// Take a look at the license at the top of the repository in the LICENSE file.

use std::{cmp, fmt, hash};

use super::prelude::*;
use crate::{
    clone::{Downgrade, Upgrade},
    prelude::*,
    WeakRef,
};

// rustdoc-stripper-ignore-next
/// Reference-counted wrapper around an [`ObjectSubclass`] reference.
///
/// This can be used for passing into closures as strong or weak reference without manually going
/// from the implementation type to the instance type and back.
pub struct ObjectImplRef<T: ObjectSubclass>(T::Type);

unsafe impl<T: ObjectSubclass + Send + Sync> Send for ObjectImplRef<T> {}
unsafe impl<T: ObjectSubclass + Send + Sync> Sync for ObjectImplRef<T> {}

impl<T: ObjectSubclass> ObjectImplRef<T> {
    // rustdoc-stripper-ignore-next
    /// Create a new reference-counting wrapper around `imp`.
    #[inline]
    pub fn new(imp: &T) -> Self {
        Self(imp.obj().clone())
    }

    // rustdoc-stripper-ignore-next
    /// Downgrade to a weak reference.
    ///
    /// This can be upgraded to a strong reference again via [`ObjectImplWeakRef::upgrade`].
    #[inline]
    pub fn downgrade(&self) -> ObjectImplWeakRef<T> {
        ObjectImplWeakRef(self.0.downgrade())
    }
}

impl<T: ObjectSubclass> Clone for ObjectImplRef<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ObjectSubclass> fmt::Debug for ObjectImplRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T::Type as fmt::Debug>::fmt(&self.0, f)
    }
}

impl<T: ObjectSubclass> std::ops::Deref for ObjectImplRef<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        T::from_obj(&self.0)
    }
}

impl<T: ObjectSubclass> Downgrade for ObjectImplRef<T> {
    type Weak = ObjectImplWeakRef<T>;

    #[inline]
    fn downgrade(&self) -> Self::Weak {
        self.downgrade()
    }
}

impl<T: ObjectSubclass> PartialOrd for ObjectImplRef<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: ObjectSubclass, OT: crate::object::ObjectType> PartialOrd<OT> for ObjectImplRef<T>
where
    T::Type: PartialOrd<OT>,
{
    #[inline]
    fn partial_cmp(&self, other: &OT) -> Option<cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T: ObjectSubclass> Ord for ObjectImplRef<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: ObjectSubclass> PartialEq for ObjectImplRef<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: ObjectSubclass, OT: crate::object::ObjectType> PartialEq<OT> for ObjectImplRef<T>
where
    T::Type: PartialEq<OT>,
{
    #[inline]
    fn eq(&self, other: &OT) -> bool {
        self.0 == *other
    }
}

impl<T: ObjectSubclass> Eq for ObjectImplRef<T> {}

impl<T: ObjectSubclass> hash::Hash for ObjectImplRef<T> {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.0.hash(state)
    }
}

// rustdoc-stripper-ignore-next
/// Weak reference to an [`ObjectSubclass`] reference.
pub struct ObjectImplWeakRef<T: ObjectSubclass>(WeakRef<T::Type>);

unsafe impl<T: ObjectSubclass + Send + Sync> Send for ObjectImplWeakRef<T> {}
unsafe impl<T: ObjectSubclass + Send + Sync> Sync for ObjectImplWeakRef<T> {}

impl<T: ObjectSubclass> ObjectImplWeakRef<T> {
    // rustdoc-stripper-ignore-next
    /// Upgrade to a strong reference, if possible.
    ///
    /// This will return `None` if the underlying object was freed in the meantime.
    #[inline]
    pub fn upgrade(&self) -> Option<ObjectImplRef<T>> {
        let obj = self.0.upgrade()?;
        Some(ObjectImplRef(obj))
    }
}

impl<T: ObjectSubclass> Clone for ObjectImplWeakRef<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ObjectSubclass> fmt::Debug for ObjectImplWeakRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <crate::WeakRef<T::Type> as fmt::Debug>::fmt(&self.0, f)
    }
}

impl<T: ObjectSubclass> Upgrade for ObjectImplWeakRef<T> {
    type Strong = ObjectImplRef<T>;

    #[inline]
    fn upgrade(&self) -> Option<Self::Strong> {
        self.upgrade()
    }
}
