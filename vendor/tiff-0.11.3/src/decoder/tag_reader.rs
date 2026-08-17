use std::io::{Read, Seek};

use crate::{
    tags::{Tag, ValueBuffer},
    Directory, TiffError, TiffFormatError, TiffResult,
};

use super::ifd::{Entry, Value};
use super::ValueReader;

pub(crate) struct TagReader<'lt, R: EntryDecoder + ?Sized> {
    pub(crate) decoder: &'lt mut R,
    pub(crate) ifd: &'lt Directory,
}

pub(crate) trait EntryDecoder {
    /// Turn an entry into a value by fetching the required bytes in the stream.
    fn entry_val(&mut self, entry: &Entry) -> TiffResult<Value>;
    /// Fill a  `ValueBuffer` by fetching the required bytes.
    fn entry_buf(&mut self, entry: &Entry, _: &mut ValueBuffer) -> TiffResult<()>;
    /// Fill the prefix of the buffer and return the actual bytes. Start interpreting the inner
    /// value at `offset`.
    fn entry_raw(&mut self, entry: &Entry, _: &mut [u8], offset: u64) -> TiffResult<usize>;
}

impl<R: Seek + Read> EntryDecoder for ValueReader<R> {
    fn entry_val(&mut self, entry: &Entry) -> TiffResult<Value> {
        entry.val(&self.limits, self.bigtiff, &mut self.reader)
    }

    fn entry_buf(&mut self, entry: &Entry, buf: &mut ValueBuffer) -> TiffResult<()> {
        entry.buffered_value(buf, &self.limits, self.bigtiff, &mut self.reader)
    }

    fn entry_raw(&mut self, entry: &Entry, buf: &mut [u8], offset: u64) -> TiffResult<usize> {
        entry.raw_value_at(buf, self.bigtiff, &mut self.reader, offset)
    }
}

impl<'a, R: EntryDecoder + ?Sized> TagReader<'a, R> {
    pub(crate) fn find_tag(&mut self, tag: Tag) -> TiffResult<Option<Value>> {
        let entry = match self.ifd.get(tag) {
            None => return Ok(None),
            Some(entry) => entry.clone(),
        };

        self.decoder.entry_val(&entry).map(Some)
    }

    /// Find a tag and fill the provided buffer with its raw data.
    ///
    /// Returns the entry of the tag if found. This signals that the buffer was overwritten.
    /// Otherwise the buffer is preserved as-is.
    pub(crate) fn find_tag_buf(
        &mut self,
        tag: Tag,
        buf: &mut ValueBuffer,
    ) -> TiffResult<Option<Entry>> {
        let entry = match self.ifd.get(tag) {
            None => return Ok(None),
            Some(entry) => entry.clone(),
        };

        self.decoder.entry_buf(&entry, buf)?;
        Ok(Some(entry))
    }

    pub(crate) fn find_tag_raw(
        &mut self,
        tag: Tag,
        buf: &mut [u8],
        at: u64,
    ) -> TiffResult<Option<usize>> {
        let entry = match self.ifd.get(tag) {
            None => return Ok(None),
            Some(entry) => entry.clone(),
        };

        let len = self.decoder.entry_raw(&entry, buf, at)?;
        Ok(Some(len))
    }

    pub(crate) fn require_tag(&mut self, tag: Tag) -> TiffResult<Value> {
        match self.find_tag(tag)? {
            Some(val) => Ok(val),
            None => Err(TiffError::FormatError(
                TiffFormatError::RequiredTagNotFound(tag),
            )),
        }
    }

    pub fn find_tag_uint_vec<T: TryFrom<u64>>(&mut self, tag: Tag) -> TiffResult<Option<Vec<T>>> {
        self.find_tag(tag)?
            .map(|v| v.into_u64_vec())
            .transpose()?
            .map(|v| {
                v.into_iter()
                    .map(|u| {
                        T::try_from(u).map_err(|_| TiffFormatError::InvalidTagValueType(tag).into())
                    })
                    .collect()
            })
            .transpose()
    }
}
