//! Batched field inversion APIs, using [Montgomery's trick].
//!
//! [Montgomery's trick]: https://zcash.github.io/halo2/background/fields.html#montgomerys-trick

use subtle::ConstantTimeEq;

use crate::Field;

/// Extension trait for iterators over mutable field elements which allows those field
/// elements to be inverted in a batch.
///
/// `I: IntoIterator<Item = &'a mut F: Field + ConstantTimeEq>` implements this trait when
/// the `alloc` feature flag is enabled.
///
/// For non-allocating contexts, see the [`BatchInverter`] struct.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub trait BatchInvert<F: Field> {
    /// Consumes this iterator and inverts each field element (when nonzero). Zero-valued
    /// elements are left as zero.
    ///
    /// Returns the inverse of the product of all nonzero field elements.
    fn batch_invert(self) -> F;
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, F, I> BatchInvert<F> for I
where
    F: Field + ConstantTimeEq,
    I: IntoIterator<Item = &'a mut F>,
{
    fn batch_invert(self) -> F {
        let mut acc = F::one();
        let iter = self.into_iter();
        let mut tmp = alloc::vec::Vec::with_capacity(iter.size_hint().0);
        for p in iter {
            let q = *p;
            tmp.push((acc, p));
            acc = F::conditional_select(&(acc * q), &acc, q.ct_eq(&F::zero()));
        }
        acc = acc.invert().unwrap();
        let allinv = acc;

        for (tmp, p) in tmp.into_iter().rev() {
            let skip = p.ct_eq(&F::zero());

            let tmp = tmp * acc;
            acc = F::conditional_select(&(acc * *p), &acc, skip);
            *p = F::conditional_select(&tmp, p, skip);
        }

        allinv
    }
}

/// A non-allocating batch inverter.
pub struct BatchInverter {}

impl BatchInverter {
    /// Inverts each field element in `elements` (when nonzero). Zero-valued elements are
    /// left as zero.
    ///
    /// - `scratch_space` is a slice of field elements that can be freely overwritten.
    ///
    /// Returns the inverse of the product of all nonzero field elements.
    ///
    /// # Panics
    ///
    /// This function will panic if `elements.len() != scratch_space.len()`.
    pub fn invert_with_external_scratch<F>(elements: &mut [F], scratch_space: &mut [F]) -> F
    where
        F: Field + ConstantTimeEq,
    {
        assert_eq!(elements.len(), scratch_space.len());

        let mut acc = F::one();
        for (p, scratch) in elements.iter().zip(scratch_space.iter_mut()) {
            *scratch = acc;
            acc = F::conditional_select(&(acc * *p), &acc, p.ct_eq(&F::zero()));
        }
        acc = acc.invert().unwrap();
        let allinv = acc;

        for (p, scratch) in elements.iter_mut().zip(scratch_space.iter()).rev() {
            let tmp = *scratch * acc;
            let skip = p.ct_eq(&F::zero());
            acc = F::conditional_select(&(acc * *p), &acc, skip);
            *p = F::conditional_select(&tmp, &p, skip);
        }

        allinv
    }

    /// Inverts each field element in `items` (when nonzero). Zero-valued elements are
    /// left as zero.
    ///
    /// - `element` is a function that extracts the element to be inverted from `items`.
    /// - `scratch_space` is a function that extracts the scratch space from `items`.
    ///
    /// Returns the inverse of the product of all nonzero field elements.
    pub fn invert_with_internal_scratch<F, T, TE, TS>(
        items: &mut [T],
        element: TE,
        scratch_space: TS,
    ) -> F
    where
        F: Field + ConstantTimeEq,
        TE: Fn(&mut T) -> &mut F,
        TS: Fn(&mut T) -> &mut F,
    {
        let mut acc = F::one();
        for item in items.iter_mut() {
            *(scratch_space)(item) = acc;
            let p = (element)(item);
            acc = F::conditional_select(&(acc * *p), &acc, p.ct_eq(&F::zero()));
        }
        acc = acc.invert().unwrap();
        let allinv = acc;

        for item in items.iter_mut().rev() {
            let tmp = *(scratch_space)(item) * acc;
            let p = (element)(item);
            let skip = p.ct_eq(&F::zero());
            acc = F::conditional_select(&(acc * *p), &acc, skip);
            *p = F::conditional_select(&tmp, &p, skip);
        }

        allinv
    }
}
