// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use std::{
    marker::PhantomData,
    rc::{self, Rc},
    sync::{self, Arc},
};

// rustdoc-stripper-ignore-next
/// Trait for generalizing downgrading a strong reference to a weak reference.
pub trait Downgrade
where
    Self: Sized,
{
    // rustdoc-stripper-ignore-next
    /// Weak reference type.
    type Weak: Upgrade;

    // rustdoc-stripper-ignore-next
    /// Downgrade to a weak reference.
    fn downgrade(&self) -> Self::Weak;
}

// rustdoc-stripper-ignore-next
/// Trait for generalizing upgrading a weak reference to a strong reference.
pub trait Upgrade
where
    Self: Sized,
{
    // rustdoc-stripper-ignore-next
    /// Strong reference type.
    type Strong;

    // rustdoc-stripper-ignore-next
    /// Try upgrading a weak reference to a strong reference.
    fn upgrade(&self) -> Option<Self::Strong>;
}

impl<T: Downgrade + ObjectType> Upgrade for crate::WeakRef<T> {
    type Strong = T;

    fn upgrade(&self) -> Option<Self::Strong> {
        self.upgrade()
    }
}

impl<T> Downgrade for PhantomData<T> {
    type Weak = PhantomData<T>;

    fn downgrade(&self) -> Self::Weak {
        PhantomData
    }
}

impl<T: Downgrade> Downgrade for &T {
    type Weak = T::Weak;

    fn downgrade(&self) -> Self::Weak {
        T::downgrade(*self)
    }
}

impl<T> Downgrade for Arc<T> {
    type Weak = sync::Weak<T>;

    fn downgrade(&self) -> Self::Weak {
        Arc::downgrade(self)
    }
}

impl<T> Upgrade for PhantomData<T> {
    type Strong = PhantomData<T>;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(PhantomData)
    }
}

impl<T> Upgrade for sync::Weak<T> {
    type Strong = Arc<T>;

    fn upgrade(&self) -> Option<Self::Strong> {
        self.upgrade()
    }
}

impl<T> Downgrade for Rc<T> {
    type Weak = rc::Weak<T>;

    fn downgrade(&self) -> Self::Weak {
        Rc::downgrade(self)
    }
}

impl<T> Upgrade for rc::Weak<T> {
    type Strong = Rc<T>;

    fn upgrade(&self) -> Option<Self::Strong> {
        self.upgrade()
    }
}
