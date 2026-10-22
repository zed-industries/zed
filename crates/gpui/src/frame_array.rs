use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
};

#[derive(Debug)]
pub(crate) struct FrameArray<T>(Vec<T>, #[cfg(debug_assertions)] Vec<Range<usize>>);

pub(crate) trait FrameIndex {
    fn rebased(&self, old: &Self, new: &Self) -> Self;
}

impl<T> FrameIndex for FramePosition<T> {
    fn rebased(&self, old: &Self, new: &Self) -> Self {
        self.rebase(*old, *new)
    }
}

impl<T> std::ops::Index<Range<FramePosition<T>>> for FrameArray<T> {
    type Output = [T];

    fn index(&self, range: Range<FramePosition<T>>) -> &Self::Output {
        &self.0[range.start.range_to(range.end)]
    }
}

impl<T> std::ops::IndexMut<Range<FramePosition<T>>> for FrameArray<T> {
    fn index_mut(&mut self, range: Range<FramePosition<T>>) -> &mut Self::Output {
        &mut self.0[range.start.range_to(range.end)]
    }
}

impl<T> std::ops::Index<usize> for FrameArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> std::ops::IndexMut<usize> for FrameArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> std::ops::Index<Range<usize>> for FrameArray<T> {
    type Output = [T];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<T> std::ops::Index<std::ops::RangeFrom<FramePosition<T>>> for FrameArray<T> {
    type Output = [T];

    fn index(&self, range: std::ops::RangeFrom<FramePosition<T>>) -> &Self::Output {
        &self.0[range.start.offset..]
    }
}

impl<T> Default for FrameArray<T> {
    fn default() -> Self {
        Self(
            Vec::new(),
            #[cfg(debug_assertions)]
            Vec::new(),
        )
    }
}

impl<T> Deref for FrameArray<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for FrameArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T> IntoIterator for &'a FrameArray<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut FrameArray<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T> IntoIterator for FrameArray<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub(crate) struct FramePosition<T> {
    offset: usize,
    marker: PhantomData<fn() -> T>,
}

impl<T> std::fmt::Debug for FramePosition<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.offset.fmt(formatter)
    }
}

impl<T> Copy for FramePosition<T> {}

impl<T> Clone for FramePosition<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Default for FramePosition<T> {
    fn default() -> Self {
        Self {
            offset: 0,
            marker: PhantomData,
        }
    }
}

impl<T> FramePosition<T> {
    pub(crate) fn offset(self) -> usize {
        self.offset
    }

    pub(crate) fn range_to(self, end: Self) -> Range<usize> {
        self.offset..end.offset
    }

    pub(crate) fn rebase(self, old: Self, new: Self) -> Self {
        Self {
            offset: self.offset - old.offset + new.offset,
            marker: PhantomData,
        }
    }
}

impl<T> std::ops::SubAssign<usize> for FramePosition<T> {
    fn sub_assign(&mut self, count: usize) {
        self.offset -= count;
    }
}

impl<T> FrameArray<T> {
    pub(crate) fn clear(&mut self) {
        self.0.clear();
        #[cfg(debug_assertions)]
        self.1.clear();
    }

    pub(crate) fn push(&mut self, item: T) {
        self.0.push(item);
    }

    pub(crate) fn truncate(&mut self, position: FramePosition<T>) {
        self.0.truncate(position.offset());
    }

    pub(crate) fn truncate_offset(&mut self, offset: usize) {
        self.0.truncate(offset);
    }

    pub(crate) fn position(&self) -> FramePosition<T> {
        FramePosition {
            offset: self.len(),
            marker: PhantomData,
        }
    }

    pub(crate) fn carry_with(
        &mut self,
        source: &mut Self,
        range: Range<FramePosition<T>>,
        transfer: impl FnMut(&mut T) -> T,
    ) {
        #[cfg(debug_assertions)]
        {
            let range = range.start.range_to(range.end);
            if !range.is_empty() {
                debug_assert!(
                    source
                        .1
                        .iter()
                        .all(|prior| { range.end <= prior.start || prior.end <= range.start }),
                    "frame segment consumed twice"
                );
                source.1.push(range);
            }
        }
        self.extend(
            source.0[range.start.range_to(range.end)]
                .iter_mut()
                .map(transfer),
        );
    }

    pub(crate) fn carry_copy(&mut self, source: &mut Self, range: Range<FramePosition<T>>)
    where
        T: Copy,
    {
        self.carry_with(source, range, |item| *item);
    }

    pub(crate) fn carry_take(&mut self, source: &mut Self, range: Range<FramePosition<T>>)
    where
        T: Default,
    {
        self.carry_with(source, range, std::mem::take);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "frame segment consumed twice")]
    fn replay_rejects_overlapping_frame_segments() {
        let mut source = FrameArray::default();
        let start = source.position();
        source.push(1);
        source.push(2);
        let end = source.position();
        let mut destination = FrameArray::default();
        destination.carry_copy(&mut source, start..end);
        destination.carry_copy(&mut source, start..end);
    }

    #[test]
    fn carry_moves_owned_payloads_and_preserves_positions() {
        let mut source = FrameArray::default();
        source.push(Some(Box::new(1)));
        let start = source.position();
        source.push(Some(Box::new(2)));
        source.push(Some(Box::new(3)));
        let end = source.position();
        source.push(Some(Box::new(4)));

        let mut destination = FrameArray::default();
        let new_start = destination.position();
        let pointer = source[1].as_deref().expect("payload") as *const i32;
        destination.carry_take(&mut source, start..end);

        assert!(source[1].is_none());
        assert!(source[2].is_none());
        assert_eq!(source[0].as_deref(), Some(&1));
        assert_eq!(source[3].as_deref(), Some(&4));
        assert_eq!(destination[0].as_deref(), Some(&2));
        assert_eq!(destination[1].as_deref(), Some(&3));
        assert_eq!(
            destination[0].as_deref().expect("payload") as *const i32,
            pointer
        );
        assert_eq!(end.rebase(start, new_start).offset(), 2);
    }

    #[test]
    fn copy_runs_can_be_reordered_and_truncated() {
        let mut source = FrameArray::default();
        let start = source.position();
        source.push(1);
        let middle = source.position();
        source.push(2);
        source.push(3);
        let end = source.position();

        let mut destination = FrameArray::default();
        destination.carry_copy(&mut source, middle..end);
        let checkpoint = destination.position();
        destination.carry_copy(&mut source, start..middle);
        assert_eq!(destination.as_slice(), &[2, 3, 1]);
        assert_eq!(source.as_slice(), &[1, 2, 3]);
        destination.truncate(checkpoint);
        assert_eq!(destination.as_slice(), &[2, 3]);
    }
}
