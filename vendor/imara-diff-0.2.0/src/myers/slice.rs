use std::mem::take;
use std::ops::RangeBounds;

use crate::intern::Token;
use crate::myers::preprocess::PreprocessedFile;
use crate::util::common_edges;

#[derive(Default)]
pub struct FileSlice<'a> {
    pub tokens: &'a [Token],
    indices: &'a [u32],
    changed: &'a mut [bool],
}

impl<'a> FileSlice<'a> {
    pub fn new(file: &'a PreprocessedFile, changed: &'a mut [bool]) -> Self {
        Self {
            tokens: &file.tokens,
            indices: &file.indices,
            changed,
        }
    }

    pub fn mark_changed(&mut self) {
        for &i in self.indices {
            self.changed[i as usize] = true;
        }
    }

    pub fn borrow(&mut self) -> FileSlice {
        FileSlice {
            tokens: self.tokens,
            changed: self.changed,
            indices: self.indices,
        }
    }

    pub fn slice<R: RangeBounds<u32>>(self, range: R) -> Self {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&start) => start,
            std::ops::Bound::Excluded(&start) => start + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(&end) => end + 1,
            std::ops::Bound::Excluded(&end) => end,
            std::ops::Bound::Unbounded => self.len(),
        };

        Self {
            tokens: &self.tokens[start as usize..end as usize],
            changed: self.changed,
            indices: &self.indices[start as usize..end as usize],
        }
    }

    pub fn strip_common(&mut self, other: &mut Self) {
        let (start, common_postfix) = common_edges(self.tokens, other.tokens);
        let end = self.len() - common_postfix;
        *self = take(self).slice(start..end);
        let end = other.len() - common_postfix;
        *other = take(other).slice(start..end)
    }

    pub fn len(&self) -> u32 {
        self.tokens.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}
