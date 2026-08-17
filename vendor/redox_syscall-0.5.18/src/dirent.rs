use core::{
    mem::size_of,
    ops::{Deref, DerefMut},
    slice,
};

use crate::{
    error::{Error, Result, EINVAL},
    ENAMETOOLONG,
};

#[derive(Clone, Copy, Debug, Default)]
#[repr(packed)]
pub struct DirentHeader {
    pub inode: u64,
    /// A filesystem-specific opaque value used to uniquely identify directory entries. This value,
    /// in the last returned entry from a SYS_GETDENTS invocation, shall be passed to the next
    /// call.
    pub next_opaque_id: u64,
    // This struct intentionally does not include a "next" offset field, unlike Linux, to easily
    // guarantee the iterator will be reasonably deterministic, even if the scheme is adversarial.
    pub record_len: u16,
    /// A `DirentKind`.
    ///
    /// May not be directly available (Unspecified), and if so needs to be looked using fstat.
    pub kind: u8,
}

impl Deref for DirentHeader {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, size_of::<Self>()) }
    }
}

impl DerefMut for DirentHeader {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Self as *mut u8, size_of::<Self>()) }
    }
}

// Note: Must match relibc/include/bits/dirent.h
#[derive(Clone, Copy, Debug, Default)]
#[repr(u8)]
pub enum DirentKind {
    #[default]
    Unspecified = 0,

    CharDev = 2,
    Directory = 4,
    BlockDev = 6,
    Regular = 8,
    Symlink = 10,
    Socket = 12,
}

impl DirentKind {
    // TODO: derive(FromPrimitive)
    pub fn try_from_raw(raw: u8) -> Option<Self> {
        Some(match raw {
            0 => Self::Unspecified,

            2 => Self::CharDev,
            4 => Self::Directory,
            6 => Self::BlockDev,
            8 => Self::Regular,
            10 => Self::Symlink,
            12 => Self::Socket,

            _ => return None,
        })
    }
}


pub struct DirentIter<'a>(&'a [u8]);

impl<'a> DirentIter<'a> {
    pub const fn new(buffer: &'a [u8]) -> Self {
        Self(buffer)
    }
}
#[derive(Debug)]
pub struct Invalid;

impl<'a> Iterator for DirentIter<'a> {
    type Item = Result<(&'a DirentHeader, &'a [u8]), Invalid>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() < size_of::<DirentHeader>() {
            return None;
        }
        let header = unsafe { &*(self.0.as_ptr().cast::<DirentHeader>()) };
        if self.0.len() < usize::from(header.record_len) {
            return Some(Err(Invalid));
        }
        let (this, remaining) = self.0.split_at(usize::from(header.record_len));
        self.0 = remaining;

        let name_and_nul = &this[size_of::<DirentHeader>()..];
        let name = &name_and_nul[..name_and_nul.len() - 1];

        Some(Ok((header, name)))
    }
}

#[derive(Debug)]
pub struct DirentBuf<B> {
    buffer: B,

    // Exists in order to allow future extensions to the DirentHeader struct.

    // TODO: Might add an upper bound to protect against cache miss DoS. The kernel currently
    // forbids any other value than size_of::<DirentHeader>().
    header_size: u16,

    written: usize,
}
/// Abstraction between &mut [u8] and the kernel's UserSliceWo.
pub trait Buffer<'a>: Sized + 'a {
    fn empty() -> Self;
    fn length(&self) -> usize;

    /// Split all of `self` into two disjoint contiguous subbuffers of lengths `index` and `length
    /// - index` respectively.
    ///
    /// Returns None if and only if `index > length`.
    fn split_at(self, index: usize) -> Option<[Self; 2]>;

    /// Copy from `src`, lengths must match exactly.
    ///
    /// Allowed to overwrite subsequent buffer space, for performance reasons. Can be changed in
    /// the future if too restrictive.
    fn copy_from_slice_exact(self, src: &[u8]) -> Result<()>;

    /// Write zeroes to this part of the buffer.
    ///
    /// Allowed to overwrite subsequent buffer space, for performance reasons. Can be changed in
    /// the future if too restrictive.
    fn zero_out(self) -> Result<()>;
}
impl<'a> Buffer<'a> for &'a mut [u8] {
    fn empty() -> Self {
        &mut []
    }
    fn length(&self) -> usize {
        self.len()
    }

    fn split_at(self, index: usize) -> Option<[Self; 2]> {
        self.split_at_mut_checked(index).map(|(a, b)| [a, b])
    }
    fn copy_from_slice_exact(self, src: &[u8]) -> Result<()> {
        self.copy_from_slice(src);
        Ok(())
    }
    fn zero_out(self) -> Result<()> {
        self.fill(0);
        Ok(())
    }
}

pub struct DirEntry<'name> {
    pub inode: u64,
    pub next_opaque_id: u64,
    pub name: &'name str,
    pub kind: DirentKind,
}

impl<'a, B: Buffer<'a>> DirentBuf<B> {
    pub fn new(buffer: B, header_size: u16) -> Option<Self> {
        if usize::from(header_size) < size_of::<DirentHeader>() {
            return None;
        }

        Some(Self {
            buffer,
            header_size,
            written: 0,
        })
    }
    pub fn entry(&mut self, entry: DirEntry<'_>) -> Result<()> {
        let name16 = u16::try_from(entry.name.len()).map_err(|_| Error::new(EINVAL))?;
        let record_len = self
            .header_size
            .checked_add(name16)
            // XXX: NUL byte. Unfortunately this is probably the only performant way to be
            // compatible with C.
            .and_then(|l| l.checked_add(1))
            .ok_or(Error::new(ENAMETOOLONG))?;

        let [this, remaining] = core::mem::replace(&mut self.buffer, B::empty())
            .split_at(usize::from(record_len))
            .ok_or(Error::new(EINVAL))?;

        let [this_header_variable, this_name_and_nul] = this
            .split_at(usize::from(self.header_size))
            .expect("already know header_size + ... >= header_size");

        let [this_name, this_name_nul] = this_name_and_nul
            .split_at(usize::from(name16))
            .expect("already know name.len() <= name.len() + 1");

        // Every write here is currently sequential, allowing the buffer trait to do optimizations
        // where subbuffer writes are out-of-bounds (but inside the total buffer).

        let [this_header, this_header_extra] = this_header_variable
            .split_at(size_of::<DirentHeader>())
            .expect("already checked header_size <= size_of Header");

        this_header.copy_from_slice_exact(&DirentHeader {
            record_len,
            next_opaque_id: entry.next_opaque_id,
            inode: entry.inode,
            kind: entry.kind as u8,
        })?;
        this_header_extra.zero_out()?;
        this_name.copy_from_slice_exact(entry.name.as_bytes())?;
        this_name_nul.copy_from_slice_exact(&[0])?;

        self.written += usize::from(record_len);
        self.buffer = remaining;

        Ok(())
    }
    pub fn finalize(self) -> usize {
        self.written
    }
}
