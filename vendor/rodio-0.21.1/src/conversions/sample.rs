use dasp_sample::{FromSample, ToSample};
use std::marker::PhantomData;

/// Converts the samples data type to `O`.
#[derive(Clone, Debug)]
pub struct SampleTypeConverter<I, O> {
    input: I,
    marker: PhantomData<O>,
}

impl<I, O> SampleTypeConverter<I, O> {
    /// Builds a new converter.
    #[inline]
    pub fn new(input: I) -> SampleTypeConverter<I, O> {
        SampleTypeConverter {
            input,
            marker: PhantomData,
        }
    }

    /// Destroys this iterator and returns the underlying iterator.
    #[inline]
    pub fn into_inner(self) -> I {
        self.input
    }

    /// get mutable access to the iterator
    #[inline]
    pub fn inner_mut(&mut self) -> &mut I {
        &mut self.input
    }
}

impl<I, O> Iterator for SampleTypeConverter<I, O>
where
    I: Iterator,
    I::Item: ToSample<O>,
{
    type Item = O;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.input.next().map(|s| s.to_sample_())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I, O> ExactSizeIterator for SampleTypeConverter<I, O>
where
    I: ExactSizeIterator,
    O: FromSample<I::Item>,
{
}
