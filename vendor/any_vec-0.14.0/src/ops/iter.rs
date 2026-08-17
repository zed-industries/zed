use core::iter::FusedIterator;

pub trait Iterable {
    type Iter: Iterator;
    fn iter(&self) -> &Self::Iter;
    fn iter_mut(&mut self) -> &mut Self::Iter;
}

/// Iterator over [`AnyVec`] slice. Will do some action on destruction.
///
/// [`AnyVec`]: crate::AnyVec
pub struct Iter<I: Iterable>(pub(crate) I);

impl<I: Iterable> Iterator
    for Iter<I>
{
    type Item = <I::Iter as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.iter().size_hint()
    }
}

impl<I: Iterable> DoubleEndedIterator
    for Iter<I>
where
    I::Iter: DoubleEndedIterator
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().next_back()
    }
}

impl<I: Iterable> ExactSizeIterator
for
    Iter<I>
where
    I::Iter: ExactSizeIterator
{
    #[inline]
    fn len(&self) -> usize {
        self.0.iter().len()
    }
}

impl<I: Iterable> FusedIterator
for
    Iter<I>
where
    I::Iter: FusedIterator
{}