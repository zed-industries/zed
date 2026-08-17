use std::ops::Range;

use bytes::Bytes;

#[derive(Debug)]
pub(crate) struct Row {
    pub(crate) storage: Bytes,
    pub(crate) values: Vec<Option<Range<usize>>>,
}

impl Row {
    pub(crate) fn get(&self, index: usize) -> Option<&[u8]> {
        self.values[index].clone().map(|col| &self.storage[col])
    }
}
