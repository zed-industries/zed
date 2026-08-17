//! Helper traits for Retained.

use super::Retained;
use crate::mutability::IsMutable;

/// Helper trait to implement [`Default`] on [`Retained`].
#[doc(alias = "DefaultId")]
pub trait DefaultRetained {
    /// The default [`Retained`] for a type.
    ///
    /// Soft-deprecated alias of [`DefaultRetained::default_retained`].
    fn default_id() -> Retained<Self>;

    /// The default [`Retained`] for a type.
    ///
    /// On most objects the implementation would be sending a message to the
    /// `new` selector.
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::default_id()
    }
}

impl<T: ?Sized + DefaultRetained> Default for Retained<T> {
    #[inline]
    fn default() -> Self {
        T::default_retained()
    }
}

/// Helper trait to implement [`IntoIterator`] on [`Retained`].
///
/// This should be implemented in exactly the same fashion as if you were
/// implementing `IntoIterator` for your type normally.
//
// Note that [`Box<T>` gets to cheat with regards moves][box-move], so
// `boxed.into_iter()` is possible, while `obj.into_iter()` is not possible
// without this helper trait.
//
// [box-move]: https://doc.rust-lang.org/reference/expressions.html#moved-and-copied-types
#[doc(alias = "IdIntoIterator")]
pub trait RetainedIntoIterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Which kind of iterator are we turning this into?
    type IntoIter: Iterator<Item = Self::Item>;

    /// Creates an iterator from an [`Retained`].
    ///
    /// Soft-deprecated alias of [`RetainedIntoIterator::retained_into_iter`].
    fn id_into_iter(this: Retained<Self>) -> Self::IntoIter;

    /// Creates an iterator from an [`Retained`].
    ///
    /// You would normally not call this function directly; instead, you'd
    /// call [`into_iter`](IntoIterator::into_iter) on an [`Retained`].
    #[inline]
    fn retained_into_iter(this: Retained<Self>) -> Self::IntoIter {
        Self::id_into_iter(this)
    }
}

// Note: These `IntoIterator` implementations conflict with an `Iterator`
// implementation for `Retained`.
//
// For our case however (in contrast with `Box`), that is the better tradeoff,
// which I will show with an example:
//
// ```
// let xs = Box::new(vec![]);
// for x in &xs { // Doesn't compile, `&Box` doesn't implement `IntoIterator`
//     // ...
// }
// ```
//
// Here, you're expected to write `xs.iter()` or `&**xs` instead, which is
// fairly acceptable, since usually people don't wrap things in boxes so much;
// but in Objective-C, _everything_ is wrapped in an `Retained`, and hence we should
// attempt to make that common case easier:
//
// ```
// let obj = NSArray::new(); // `Retained<NSArray<_>>`
// for item in &obj { // Should compile
//     // ...
// }
// ```
//
// The loss of the `Iterator` impl is a bit unfortunate, but not a big deal,
// since there is only one iterator in Objective-C anyhow, `NSEnumerator`, and
// for that we can make other abstractions instead.
impl<T: ?Sized + RetainedIntoIterator> IntoIterator for Retained<T> {
    type Item = <T as RetainedIntoIterator>::Item;
    type IntoIter = <T as RetainedIntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        T::retained_into_iter(self)
    }
}

impl<'a, T: ?Sized> IntoIterator for &'a Retained<T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&**self).into_iter()
    }
}

impl<'a, T: ?Sized + IsMutable> IntoIterator for &'a mut Retained<T>
where
    &'a mut T: IntoIterator,
{
    type Item = <&'a mut T as IntoIterator>::Item;
    type IntoIter = <&'a mut T as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&mut **self).into_iter()
    }
}

/// Helper trait to implement [`FromIterator`] on [`Retained`].
///
/// This should be implemented in exactly the same fashion as if you were
/// implementing `FromIterator` for your type normally.
#[doc(alias = "IdFromIterator")]
pub trait RetainedFromIterator<T>: Sized {
    /// Creates an `Retained` from an iterator.
    ///
    /// Soft-deprecated alias of [`RetainedFromIterator::retained_from_iter`].
    fn id_from_iter<I>(iter: I) -> Retained<Self>
    where
        I: IntoIterator<Item = T>;

    /// Creates an `Retained` from an iterator.
    #[inline]
    fn retained_from_iter<I>(iter: I) -> Retained<Self>
    where
        I: IntoIterator<Item = T>,
    {
        Self::id_from_iter(iter)
    }
}

impl<T, U: RetainedFromIterator<T>> FromIterator<T> for Retained<U> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        U::retained_from_iter(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutability::Mutable;
    use crate::runtime::NSObject;
    use crate::{declare_class, msg_send_id, ClassType, DeclaredClass};

    declare_class!(
        #[derive(PartialEq, Eq, Hash, Debug)]
        struct Collection;

        unsafe impl ClassType for Collection {
            type Super = NSObject;
            type Mutability = Mutable;
            const NAME: &'static str = "MyCustomCollection";
        }

        impl DeclaredClass for Collection {}
    );

    impl DefaultRetained for Collection {
        fn default_id() -> Retained<Self> {
            unsafe { msg_send_id![Collection::class(), new] }
        }
    }

    struct Iter<'a> {
        _inner: &'a Collection,
    }

    impl<'a> Iterator for Iter<'a> {
        type Item = &'a NSObject;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    impl<'a> IntoIterator for &'a Collection {
        type Item = &'a NSObject;
        type IntoIter = Iter<'a>;

        fn into_iter(self) -> Self::IntoIter {
            Iter { _inner: self }
        }
    }

    struct IterMut<'a> {
        _inner: &'a mut Collection,
    }

    impl<'a> Iterator for IterMut<'a> {
        type Item = &'a mut NSObject;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    impl<'a> IntoIterator for &'a mut Collection {
        // Usually only valid if a mutable object is stored in the collection.
        type Item = &'a mut NSObject;
        type IntoIter = IterMut<'a>;

        fn into_iter(self) -> Self::IntoIter {
            IterMut { _inner: self }
        }
    }

    struct IntoIter {
        _inner: Retained<Collection>,
    }

    impl Iterator for IntoIter {
        type Item = Retained<NSObject>;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    impl RetainedIntoIterator for Collection {
        type Item = Retained<NSObject>;
        type IntoIter = IntoIter;

        fn id_into_iter(this: Retained<Self>) -> Self::IntoIter {
            IntoIter { _inner: this }
        }
    }

    impl RetainedFromIterator<Retained<NSObject>> for Collection {
        fn id_from_iter<I: IntoIterator<Item = Retained<NSObject>>>(_iter: I) -> Retained<Self> {
            Collection::default_retained()
        }
    }

    #[test]
    fn test_default() {
        let obj1: Retained<Collection> = Default::default();
        let obj2 = Collection::default_retained();
        assert_ne!(obj1, obj2);
    }

    #[test]
    fn test_into_iter() {
        let mut obj: Retained<Collection> = Default::default();

        for _ in &*obj {}
        for _ in &obj {}

        for _ in &mut *obj {}
        for _ in &mut obj {}

        for _ in obj {}
    }

    #[test]
    fn test_from_iter() {
        let _: Retained<Collection> = [NSObject::new()].into_iter().collect();
    }
}
