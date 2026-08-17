//! Item handling.

use crate::{MovableRef, Ref};

/// Argument to [`crate::TypedSection::offset_of`] and related section lookup APIs.
pub trait SectionItemLocation<T: ?Sized> {
    /// Address of the item's storage in the link section (not the wrapper).
    fn item_ptr(&self) -> *const T;
}

impl<T: ?Sized> SectionItemLocation<T> for &T {
    fn item_ptr(&self) -> *const T {
        *self as *const T
    }
}

impl<T> SectionItemLocation<T> for &Ref<T> {
    fn item_ptr(&self) -> *const T {
        Ref::as_ptr(self)
    }
}

impl<T> SectionItemLocation<T> for &MovableRef<T> {
    fn item_ptr(&self) -> *const T {
        MovableRef::as_ptr(self)
    }
}

/// Element type for this section handle ([`crate::TypedSection`], etc.).
pub trait SectionItemType {
    /// Item type stored or referenced in the section.
    type Item;
}

// Waiting on Rust 1.78
// #[diagnostic::on_unimplemented(message = "Incorrect section type for item")]
/// Typed section compatibility for item `T`.
pub trait SectionItemTyped<T> {
    /// Item representation for this `T`.
    type Item;
}

#[cfg(test)]
mod tests {
    use crate::item::SectionItemType;
    use core::marker::PhantomData;

    assert_type_eq!(<crate::TypedSection<u32> as SectionItemType>::Item, u32);
    assert_type_eq!(
        <crate::TypedSection<&'static u32> as SectionItemType>::Item,
        &'static u32
    );

    macro_rules! assert_type_eq {
        ($lhs:ty, $rhs:ty) => {
            const _: () = {
                struct __AssertTypeEq<T, U>(PhantomData<T>, PhantomData<U>);
                trait __AssertTypeEqT {
                    const CHECK: bool = true;
                }
                impl<T> __AssertTypeEqT for __AssertTypeEq<T, T> {}

                _ = <__AssertTypeEq<$lhs, $rhs> as __AssertTypeEqT>::CHECK;
            };
        };
    }
    pub(crate) use assert_type_eq;
}
