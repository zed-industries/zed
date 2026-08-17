use std::mem::MaybeUninit;
use std::ops::{IndexMut, RangeBounds};

pub(crate) enum Buffer<'a> {
    Uninit(&'a mut [MaybeUninit<u8>]),
    Init(&'a mut [u8]),
}

impl<'b> Buffer<'b> {
    #[inline(always)]
    pub(crate) fn reborrow<'m>(&'m mut self) -> Buffer<'m>
    where
        'b: 'm,
    {
        match &mut *self {
            Self::Uninit(uninit) => Buffer::Uninit(&mut uninit[..]),
            Self::Init(init) => Buffer::Init(&mut init[..]),
        }
    }

    #[inline(always)]
    pub(crate) fn index_mut(self, range: impl RangeBounds<usize>) -> Self {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());

        match self {
            Buffer::Uninit(uninit) => Buffer::Uninit(uninit.index_mut(range)),
            Buffer::Init(init) => Buffer::Init(init.index_mut(range)),
        }
    }

    pub(crate) fn copy_from_slice(&mut self, input: &[u8]) {
        match self {
            Buffer::Uninit(uninit) => {
                // TODO: replace with write_copy_of_slice when rust-lang/rust#79995 get stabilized.
                debug_assert_eq!(uninit.len(), input.len());
                uninit
                    .iter_mut()
                    .zip(input)
                    .for_each(|(maybe_uninit, byte)| {
                        maybe_uninit.write(*byte);
                    });
            }
            Buffer::Init(init) => init.copy_from_slice(input),
        };
    }

    #[inline(always)]
    pub(crate) fn len(&self) -> usize {
        match self {
            Buffer::Uninit(uninit) => uninit.len(),
            Buffer::Init(init) => init.len(),
        }
    }

    #[inline(always)]
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
