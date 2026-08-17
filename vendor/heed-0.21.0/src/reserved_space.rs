use std::mem::MaybeUninit;
use std::{fmt, io};

use crate::mdb::ffi;

/// A structure that is used to improve the write speed in LMDB.
///
/// You must write the exact amount of bytes, no less, no more.
pub struct ReservedSpace<'a> {
    bytes: &'a mut [MaybeUninit<u8>],
    /// Number of bytes which have been written: all bytes in `0..written`.
    written: usize,
    /// Index of the byte which should be written next.
    ///
    /// # Safety
    ///
    /// To ensure there are no unwritten gaps in the buffer this should be kept in the range
    /// `0..=written` at all times.
    write_head: usize,
}

impl ReservedSpace<'_> {
    pub(crate) unsafe fn from_val<'a>(val: ffi::MDB_val) -> ReservedSpace<'a> {
        let len = val.mv_size;
        let ptr = val.mv_data;

        ReservedSpace {
            bytes: std::slice::from_raw_parts_mut(ptr.cast(), len),
            written: 0,
            write_head: 0,
        }
    }

    /// The total number of bytes that this memory buffer has.
    #[inline]
    pub fn size(&self) -> usize {
        self.bytes.len()
    }

    /// The remaining number of bytes that this memory buffer has.
    #[inline]
    pub fn remaining(&self) -> usize {
        self.bytes.len() - self.write_head
    }

    /// Get a slice of all the bytes that have previously been written.
    ///
    /// This can be used to write information which cannot be known until the very end of
    /// serialization. For example, this method can be used to serialize a value, then compute a
    /// checksum over the bytes, and then write that checksum to a header at the start of the
    /// reserved space.
    #[inline]
    pub fn written_mut(&mut self) -> &mut [u8] {
        let ptr = self.bytes.as_mut_ptr();
        let len = self.written;
        unsafe { std::slice::from_raw_parts_mut(ptr.cast(), len) }
    }

    /// Fills the remaining reserved space with zeroes.
    ///
    /// This can be used together with [`written_mut`](Self::written_mut) to get a mutable view of
    /// the entire reserved space.
    ///
    /// ### Note
    ///
    /// After calling this function, the entire space is considered to be filled and any
    /// further attempt to [`write`](std::io::Write::write) anything else will fail.
    #[inline]
    pub fn fill_zeroes(&mut self) {
        self.bytes[self.write_head..].fill(MaybeUninit::new(0));
        self.written = self.bytes.len();
        self.write_head = self.bytes.len();
    }

    /// Get a slice of bytes corresponding to the *entire* reserved space.
    ///
    /// It is safe to write to any byte within the slice. However, for a write past the end of the
    /// prevously written bytes to take effect, [`assume_written`](Self::assume_written) has to be
    /// called to mark those bytes as initialized.
    ///
    /// # Safety
    ///
    /// As the memory comes from within the database itself, the bytes may not yet be
    /// initialized. Thus, it is up to the caller to ensure that only initialized memory is read
    /// (ensured by the [`MaybeUninit`] API).
    #[inline]
    pub fn as_uninit_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        self.bytes
    }

    /// Marks the bytes in the range `0..len` as being initialized by advancing the internal write
    /// pointer.
    ///
    /// # Safety
    ///
    /// The caller guarantees that all bytes in the range have been initialized.
    #[inline]
    pub unsafe fn assume_written(&mut self, len: usize) {
        debug_assert!(len <= self.bytes.len());
        self.written = len;
        self.write_head = len;
    }
}

impl io::Write for ReservedSpace<'_> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write_all(buf)?;
        Ok(buf.len())
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        let remaining = unsafe { self.bytes.get_unchecked_mut(self.write_head..) };

        if buf.len() > remaining.len() {
            return Err(io::Error::from(io::ErrorKind::WriteZero));
        }

        unsafe {
            // SAFETY: we can always cast `T` -> `MaybeUninit<T>` as it's a transparent wrapper
            let buf_uninit = std::slice::from_raw_parts(buf.as_ptr().cast(), buf.len());
            remaining.as_mut_ptr().copy_from_nonoverlapping(buf_uninit.as_ptr(), buf.len());
        }

        self.write_head += buf.len();
        self.written = usize::max(self.written, self.write_head);

        Ok(())
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// ## Note
///
/// May only seek within the previously written space.
/// Attempts to do otherwise will result in an error.
impl io::Seek for ReservedSpace<'_> {
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let (base, offset) = match pos {
            io::SeekFrom::Start(start) => (start, 0),
            io::SeekFrom::End(offset) => (self.written as u64, offset),
            io::SeekFrom::Current(offset) => (self.write_head as u64, offset),
        };

        let Some(new_pos) = base.checked_add_signed(offset) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "cannot seek before start of reserved space",
            ));
        };

        if new_pos > self.written as u64 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "cannot seek past end of reserved space",
            ));
        }

        self.write_head = new_pos as usize;

        Ok(new_pos)
    }

    #[inline]
    fn rewind(&mut self) -> io::Result<()> {
        self.write_head = 0;
        Ok(())
    }

    #[inline]
    fn stream_position(&mut self) -> io::Result<u64> {
        Ok(self.write_head as u64)
    }
}

impl fmt::Debug for ReservedSpace<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ReservedSpace").finish()
    }
}
