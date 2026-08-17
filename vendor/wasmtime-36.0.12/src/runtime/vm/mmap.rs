//! Low-level abstraction for allocating and managing zero-filled pages
//! of memory.

use super::HostAlignedByteCount;
use crate::prelude::*;
use crate::runtime::vm::sys::{mmap, vm::MemoryImageSource};
use alloc::sync::Arc;
use core::ops::Range;
use core::ptr::NonNull;
#[cfg(feature = "std")]
use std::fs::File;

/// A marker type for an [`Mmap`] where both the start address and length are a
/// multiple of the host page size.
///
/// For more information, see the documentation on [`Mmap`].
#[derive(Clone, Debug)]
pub struct AlignedLength {}

/// A type of [`Mmap`] where the start address is host page-aligned, but the
/// length is possibly not a multiple of the host page size.
///
/// For more information, see the documentation on [`Mmap`].
#[derive(Clone, Debug)]
pub struct UnalignedLength {
    #[cfg(feature = "std")]
    file: Option<Arc<File>>,
}

/// A platform-independent abstraction over memory-mapped data.
///
/// The type parameter can be one of:
///
/// * [`AlignedLength`]: Both the start address and length are page-aligned
/// (i.e. a multiple of the host page size). This is always the result of an
/// mmap backed by anonymous memory.
///
/// * [`UnalignedLength`]: The start address is host page-aligned, but the
/// length is not necessarily page-aligned. This is usually backed by a file,
/// but can also be backed by anonymous memory.
///
/// ## Notes
///
/// If the length of a file is not a multiple of the host page size, [POSIX does
/// not specify any semantics][posix-mmap] for the rest of the last page. Linux
/// [does say][linux-mmap] that the rest of the page is reserved and zeroed out,
/// but for portability it's best to not assume anything about the rest of
/// memory. `UnalignedLength` achieves a type-level distinction between an mmap
/// that is backed purely by memory, and one that is possibly backed by a file.
///
/// Currently, the OS-specific `mmap` implementations in this crate do not make
/// this this distinction -- alignment is managed at this platform-independent
/// layer. It might make sense to add this distinction to the OS-specific
/// implementations in the future.
///
/// [posix-mmap]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/mmap.html
/// [linux-mmap]: https://man7.org/linux/man-pages/man2/mmap.2.html#NOTES
#[derive(Debug)]
pub struct Mmap<T> {
    sys: mmap::Mmap,
    data: T,
}

impl Mmap<AlignedLength> {
    /// Create a new `Mmap` pointing to at least `size` bytes of page-aligned
    /// accessible memory.
    pub fn with_at_least(size: usize) -> Result<Self> {
        let rounded_size = HostAlignedByteCount::new_rounded_up(size)?;
        Self::accessible_reserved(rounded_size, rounded_size)
    }

    /// Create a new `Mmap` pointing to `accessible_size` bytes of page-aligned
    /// accessible memory, within a reserved mapping of `mapping_size` bytes.
    /// `accessible_size` and `mapping_size` must be native page-size multiples.
    ///
    /// # Panics
    ///
    /// This function will panic if `accessible_size` is greater than
    /// `mapping_size`.
    pub fn accessible_reserved(
        accessible_size: HostAlignedByteCount,
        mapping_size: HostAlignedByteCount,
    ) -> Result<Self> {
        assert!(accessible_size <= mapping_size);

        if mapping_size.is_zero() {
            Ok(Mmap {
                sys: mmap::Mmap::new_empty(),
                data: AlignedLength {},
            })
        } else if accessible_size == mapping_size {
            Ok(Mmap {
                sys: mmap::Mmap::new(mapping_size)
                    .context(format!("mmap failed to allocate {mapping_size:#x} bytes"))?,
                data: AlignedLength {},
            })
        } else {
            let result = Mmap {
                sys: mmap::Mmap::reserve(mapping_size)
                    .context(format!("mmap failed to reserve {mapping_size:#x} bytes"))?,
                data: AlignedLength {},
            };
            if !accessible_size.is_zero() {
                // SAFETY: result was just created and is not in use.
                unsafe {
                    result
                        .make_accessible(HostAlignedByteCount::ZERO, accessible_size)
                        .context(format!(
                            "mmap failed to allocate {accessible_size:#x} bytes"
                        ))?;
                }
            }
            Ok(result)
        }
    }

    /// Converts this `Mmap` into a `Mmap<UnalignedLength>`.
    ///
    /// `UnalignedLength` really means "_possibly_ unaligned length", so it can
    /// be freely converted over at the cost of losing the alignment guarantee.
    pub fn into_unaligned(self) -> Mmap<UnalignedLength> {
        Mmap {
            sys: self.sys,
            data: UnalignedLength {
                #[cfg(feature = "std")]
                file: None,
            },
        }
    }

    /// Returns the length of the memory mapping as an aligned byte count.
    pub fn len_aligned(&self) -> HostAlignedByteCount {
        // SAFETY: The type parameter indicates that self.sys.len() is aligned.
        unsafe { HostAlignedByteCount::new_unchecked(self.sys.len()) }
    }

    /// Return a struct representing a page-aligned offset into the mmap.
    ///
    /// Returns an error if `offset > self.len_aligned()`.
    pub fn offset(self: &Arc<Self>, offset: HostAlignedByteCount) -> Result<MmapOffset> {
        if offset > self.len_aligned() {
            bail!(
                "offset {} is not in bounds for mmap: {}",
                offset,
                self.len_aligned()
            );
        }

        Ok(MmapOffset::new(self.clone(), offset))
    }

    /// Return an `MmapOffset` corresponding to zero bytes into the mmap.
    pub fn zero_offset(self: &Arc<Self>) -> MmapOffset {
        MmapOffset::new(self.clone(), HostAlignedByteCount::ZERO)
    }

    /// Make the memory starting at `start` and extending for `len` bytes
    /// accessible. `start` and `len` must be native page-size multiples and
    /// describe a range within `self`'s reserved memory.
    ///
    /// # Safety
    ///
    /// There must not be any other references to the region of memory being
    /// made accessible.
    ///
    /// # Panics
    ///
    /// Panics if `start + len >= self.len()`.
    pub unsafe fn make_accessible(
        &self,
        start: HostAlignedByteCount,
        len: HostAlignedByteCount,
    ) -> Result<()> {
        if len.is_zero() {
            // A zero-sized mprotect (or equivalent) is allowed on some
            // platforms but not others (notably Windows). Treat it as a no-op
            // everywhere.
            return Ok(());
        }

        let end = start
            .checked_add(len)
            .expect("start + len must not overflow");
        assert!(
            end <= self.len_aligned(),
            "start + len ({end}) must be <= mmap region {}",
            self.len_aligned()
        );

        unsafe { self.sys.make_accessible(start, len) }
    }
}

#[cfg(feature = "std")]
impl Mmap<UnalignedLength> {
    /// Creates a new `Mmap` by opening the file located at `path` and mapping
    /// it into memory.
    ///
    /// The memory is mapped in read-only mode for the entire file. If portions
    /// of the file need to be modified then the `region` crate can be use to
    /// alter permissions of each page.
    ///
    /// The memory mapping and the length of the file within the mapping are
    /// returned.
    pub fn from_file(file: Arc<File>) -> Result<Self> {
        let sys = mmap::Mmap::from_file(&file)?;
        Ok(Mmap {
            sys,
            data: UnalignedLength { file: Some(file) },
        })
    }

    /// Returns the underlying file that this mmap is mapping, if present.
    pub fn original_file(&self) -> Option<&Arc<File>> {
        self.data.file.as_ref()
    }
}

impl<T> Mmap<T> {
    /// Return the allocated memory as a slice of u8.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the range of bytes is accessible to the
    /// program and additionally has previously been initialized.
    ///
    /// # Panics
    ///
    /// Panics of the `range` provided is outside of the limits of this mmap.
    #[inline]
    pub unsafe fn slice(&self, range: Range<usize>) -> &[u8] {
        assert!(range.start <= range.end);
        assert!(range.end <= self.len());
        unsafe {
            core::slice::from_raw_parts(self.as_ptr().add(range.start), range.end - range.start)
        }
    }

    /// Return the allocated memory as a mutable slice of u8.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the range of bytes is accessible to the
    /// program and additionally has previously been initialized.
    ///
    /// # Panics
    ///
    /// Panics of the `range` provided is outside of the limits of this mmap.
    pub unsafe fn slice_mut(&mut self, range: Range<usize>) -> &mut [u8] {
        assert!(range.start <= range.end);
        assert!(range.end <= self.len());
        unsafe {
            core::slice::from_raw_parts_mut(
                self.as_mut_ptr().add(range.start),
                range.end - range.start,
            )
        }
    }

    /// Return the allocated memory as a pointer to u8.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.sys.as_send_sync_ptr().as_ptr() as *const u8
    }

    /// Return the allocated memory as a mutable pointer to u8.
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.sys.as_send_sync_ptr().as_ptr()
    }

    /// Return the allocated memory as a mutable pointer to u8.
    #[inline]
    pub fn as_non_null(&self) -> NonNull<u8> {
        self.sys.as_send_sync_ptr().as_non_null()
    }

    /// Return the length of the allocated memory.
    ///
    /// This is the byte length of this entire mapping which includes both
    /// addressable and non-addressable memory.
    ///
    /// If the length is statically known to be page-aligned via the
    /// [`AlignedLength`] type parameter, use [`Self::len_aligned`].
    #[inline]
    pub fn len(&self) -> usize {
        self.sys.len()
    }

    /// Makes the specified `range` within this `Mmap` to be read/execute.
    ///
    /// # Unsafety
    ///
    /// This method is unsafe as it's generally not valid to simply make memory
    /// executable, so it's up to the caller to ensure that everything is in
    /// order and this doesn't overlap with other memory that should only be
    /// read or only read/write.
    ///
    /// # Panics
    ///
    /// Panics of `range` is out-of-bounds or not page-aligned.
    pub unsafe fn make_executable(
        &self,
        range: Range<usize>,
        enable_branch_protection: bool,
    ) -> Result<()> {
        assert!(range.start <= self.len());
        assert!(range.end <= self.len());
        assert!(range.start <= range.end);
        assert!(
            range.start % crate::runtime::vm::host_page_size() == 0,
            "changing of protections isn't page-aligned",
        );

        if range.start == range.end {
            // A zero-sized mprotect (or equivalent) is allowed on some
            // platforms but not others (notably Windows). Treat it as a no-op
            // everywhere.
            return Ok(());
        }

        unsafe {
            self.sys
                .make_executable(range, enable_branch_protection)
                .context("failed to make memory executable")
        }
    }

    /// Makes the specified `range` within this `Mmap` to be readonly.
    pub unsafe fn make_readonly(&self, range: Range<usize>) -> Result<()> {
        assert!(range.start <= self.len());
        assert!(range.end <= self.len());
        assert!(range.start <= range.end);
        assert!(
            range.start % crate::runtime::vm::host_page_size() == 0,
            "changing of protections isn't page-aligned",
        );

        if range.start == range.end {
            // A zero-sized mprotect (or equivalent) is allowed on some
            // platforms but not others (notably Windows). Treat it as a no-op
            // everywhere.
            return Ok(());
        }

        unsafe {
            self.sys
                .make_readonly(range)
                .context("failed to make memory readonly")
        }
    }
}

fn _assert() {
    fn _assert_send_sync<T: Send + Sync>() {}
    _assert_send_sync::<Mmap<AlignedLength>>();
    _assert_send_sync::<Mmap<UnalignedLength>>();
}

impl From<Mmap<AlignedLength>> for Mmap<UnalignedLength> {
    fn from(mmap: Mmap<AlignedLength>) -> Mmap<UnalignedLength> {
        mmap.into_unaligned()
    }
}

/// A reference to an [`Mmap`], along with a host-page-aligned index within it.
///
/// The main invariant this type asserts is that the index is in bounds within
/// the `Mmap` (i.e. `self.mmap[self.offset]` is valid). In the future, this
/// type may also assert other invariants.
#[derive(Clone, Debug)]
pub struct MmapOffset {
    mmap: Arc<Mmap<AlignedLength>>,
    offset: HostAlignedByteCount,
}

impl MmapOffset {
    #[inline]
    fn new(mmap: Arc<Mmap<AlignedLength>>, offset: HostAlignedByteCount) -> Self {
        assert!(
            offset <= mmap.len_aligned(),
            "offset {} is in bounds (< {})",
            offset,
            mmap.len_aligned(),
        );
        Self { mmap, offset }
    }

    /// Returns the mmap this offset is within.
    #[inline]
    pub fn mmap(&self) -> &Arc<Mmap<AlignedLength>> {
        &self.mmap
    }

    /// Returns the host-page-aligned offset within the mmap.
    #[inline]
    pub fn offset(&self) -> HostAlignedByteCount {
        self.offset
    }

    /// Returns the raw pointer in memory represented by this offset.
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.as_non_null().as_ptr()
    }

    /// Returns the raw pointer in memory represented by this offset.
    #[inline]
    pub fn as_non_null(&self) -> NonNull<u8> {
        // SAFETY: constructor checks that offset is within this allocation.
        unsafe { self.mmap().as_non_null().byte_add(self.offset.byte_count()) }
    }

    /// Maps an image into the mmap with read/write permissions.
    ///
    /// The image is mapped at `self.mmap.as_ptr() + self.offset +
    /// memory_offset`.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that noone else has a reference to this memory.
    pub unsafe fn map_image_at(
        &self,
        image_source: &MemoryImageSource,
        source_offset: u64,
        memory_offset: HostAlignedByteCount,
        memory_len: HostAlignedByteCount,
    ) -> Result<()> {
        let total_offset = self
            .offset
            .checked_add(memory_offset)
            .expect("self.offset + memory_offset is in bounds");
        unsafe {
            self.mmap
                .sys
                .map_image_at(image_source, source_offset, total_offset, memory_len)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test zero-length calls to mprotect (or the OS equivalent).
    ///
    /// These should be treated as no-ops on all platforms. This test ensures
    /// that such calls at least don't error out.
    #[test]
    fn mprotect_zero_length() {
        let page_size = HostAlignedByteCount::host_page_size();
        let pagex2 = page_size.checked_mul(2).unwrap();
        let pagex3 = page_size.checked_mul(3).unwrap();
        let pagex4 = page_size.checked_mul(4).unwrap();

        let mem = Mmap::accessible_reserved(pagex2, pagex4).expect("allocated memory");

        unsafe {
            mem.make_accessible(pagex3, HostAlignedByteCount::ZERO)
                .expect("make_accessible succeeded");

            mem.make_executable(pagex3.byte_count()..pagex3.byte_count(), false)
                .expect("make_executable succeeded");

            mem.make_readonly(pagex3.byte_count()..pagex3.byte_count())
                .expect("make_readonly succeeded");
        };
    }
}
