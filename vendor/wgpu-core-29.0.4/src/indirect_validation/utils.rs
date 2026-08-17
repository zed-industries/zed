use alloc::vec::Vec;

pub(crate) struct UniqueIndexScratch(bit_set::BitSet);

impl UniqueIndexScratch {
    pub(crate) fn new() -> Self {
        Self(bit_set::BitSet::new())
    }
}

pub(crate) struct UniqueIndex<'a, I: Iterator<Item = usize>> {
    inner: I,
    scratch: &'a mut UniqueIndexScratch,
}

impl<'a, I: Iterator<Item = usize>> UniqueIndex<'a, I> {
    fn new(inner: I, scratch: &'a mut UniqueIndexScratch) -> Self {
        scratch.0.make_empty();
        Self { inner, scratch }
    }
}

impl<'a, I: Iterator<Item = usize>> Iterator for UniqueIndex<'a, I> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find(|&i| self.scratch.0.insert(i))
    }
}

pub(crate) trait UniqueIndexExt: Iterator<Item = usize> {
    fn unique<'a>(self, scratch: &'a mut UniqueIndexScratch) -> UniqueIndex<'a, Self>
    where
        Self: Sized,
    {
        UniqueIndex::new(self, scratch)
    }
}

impl<T: Iterator<Item = usize>> UniqueIndexExt for T {}

type BufferBarrier<'b> = hal::BufferBarrier<'b, dyn hal::DynBuffer>;

pub(crate) struct BufferBarrierScratch<'b>(Vec<BufferBarrier<'b>>);

impl<'b> BufferBarrierScratch<'b> {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }
}

pub(crate) struct BufferBarriers<'a, 'b> {
    scratch: &'a mut BufferBarrierScratch<'b>,
}

impl<'a, 'b> BufferBarriers<'a, 'b> {
    pub(crate) fn new(scratch: &'a mut BufferBarrierScratch<'_>) -> Self {
        // change lifetime of buffer reference, this is safe since `scratch` is empty,
        // it was either just created or it has been cleared on `BufferBarriers::drop`
        let scratch = unsafe {
            core::mem::transmute::<&'a mut BufferBarrierScratch<'_>, &'a mut BufferBarrierScratch<'b>>(
                scratch,
            )
        };
        Self { scratch }
    }

    pub(crate) fn extend(self, iter: impl Iterator<Item = BufferBarrier<'b>>) -> Self {
        self.scratch.0.extend(iter);
        self
    }

    pub(crate) fn encode(self, encoder: &mut dyn hal::DynCommandEncoder) {
        unsafe {
            encoder.transition_buffers(&self.scratch.0);
        }
    }
}

impl<'a, 'b> Drop for BufferBarriers<'a, 'b> {
    fn drop(&mut self) {
        self.scratch.0.clear();
    }
}
