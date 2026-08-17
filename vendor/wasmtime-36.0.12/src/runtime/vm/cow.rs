//! Copy-on-write initialization support: creation of backing images for
//! modules, and logic to support mapping these backing images into memory.

use super::sys::DecommitBehavior;
use crate::prelude::*;
use crate::runtime::vm::sys::vm::{self, MemoryImageSource};
use crate::runtime::vm::{HostAlignedByteCount, MmapOffset, MmapVec, host_page_size};
use alloc::sync::Arc;
use core::ops::Range;
use core::ptr;
use wasmtime_environ::{DefinedMemoryIndex, MemoryInitialization, Module, PrimaryMap, Tunables};

/// Backing images for memories in a module.
///
/// This is meant to be built once, when a module is first loaded/constructed,
/// and then used many times for instantiation.
pub struct ModuleMemoryImages {
    memories: PrimaryMap<DefinedMemoryIndex, Option<Arc<MemoryImage>>>,
}

impl ModuleMemoryImages {
    /// Get the MemoryImage for a given memory.
    pub fn get_memory_image(&self, defined_index: DefinedMemoryIndex) -> Option<&Arc<MemoryImage>> {
        self.memories[defined_index].as_ref()
    }
}

/// One backing image for one memory.
#[derive(Debug, PartialEq)]
pub struct MemoryImage {
    /// The platform-specific source of this image.
    ///
    /// This might be a mapped `*.cwasm` file or on Unix it could also be a
    /// `Memfd` as an anonymous file in memory on Linux. In either case this is
    /// used as the backing-source for the CoW image.
    source: MemoryImageSource,

    /// Length of image, in bytes.
    ///
    /// Note that initial memory size may be larger; leading and trailing zeroes
    /// are truncated (handled by backing fd).
    ///
    /// Must be a multiple of the system page size.
    len: HostAlignedByteCount,

    /// Image starts this many bytes into `source`.
    ///
    /// This is 0 for anonymous-backed memfd files and is the offset of the
    /// data section in a `*.cwasm` file for `*.cwasm`-backed images.
    ///
    /// Must be a multiple of the system page size.
    ///
    /// ## Notes
    ///
    /// This currently isn't a `HostAlignedByteCount` because that's a usize and
    /// this, being a file offset, is a u64.
    source_offset: u64,

    /// Image starts this many bytes into heap space.
    ///
    /// Must be a multiple of the system page size.
    linear_memory_offset: HostAlignedByteCount,
}

impl MemoryImage {
    fn new(
        page_size: u32,
        linear_memory_offset: HostAlignedByteCount,
        data: &[u8],
        mmap: Option<&MmapVec>,
    ) -> Result<Option<MemoryImage>> {
        let assert_page_aligned = |val: usize| {
            assert_eq!(val % (page_size as usize), 0);
        };
        // Sanity-check that various parameters are page-aligned.
        let len = HostAlignedByteCount::new(data.len()).expect("memory image data is page-aligned");

        // If a backing `mmap` is present then `data` should be a sub-slice of
        // the `mmap`. The sanity-checks here double-check that. Additionally
        // compilation should have ensured that the `data` section is
        // page-aligned within `mmap`, so that's also all double-checked here.
        //
        // Finally if the `mmap` itself comes from a backing file on disk, such
        // as a `*.cwasm` file, then that's a valid source of data for the
        // memory image so we simply return referencing that.
        //
        // Note that this path is platform-agnostic in the sense of all
        // platforms we support support memory mapping copy-on-write data from
        // files, but for now this is still a Linux-specific region of Wasmtime.
        // Some work will be needed to get this file compiling for macOS and
        // Windows.
        if let Some(mmap) = mmap {
            let start = mmap.as_ptr() as usize;
            let end = start + mmap.len();
            let data_start = data.as_ptr() as usize;
            let data_end = data_start + data.len();
            assert!(start <= data_start && data_end <= end);
            assert_page_aligned(start);
            assert_page_aligned(data_start);
            assert_page_aligned(data_end);

            #[cfg(feature = "std")]
            if let Some(file) = mmap.original_file() {
                if let Some(source) = MemoryImageSource::from_file(file) {
                    return Ok(Some(MemoryImage {
                        source,
                        source_offset: u64::try_from(data_start - start).unwrap(),
                        linear_memory_offset,
                        len,
                    }));
                }
            }
        }

        // If `mmap` doesn't come from a file then platform-specific mechanisms
        // may be used to place the data in a form that's amenable to an mmap.
        if let Some(source) = MemoryImageSource::from_data(data)? {
            return Ok(Some(MemoryImage {
                source,
                source_offset: 0,
                linear_memory_offset,
                len,
            }));
        }

        Ok(None)
    }

    unsafe fn map_at(&self, mmap_base: &MmapOffset) -> Result<()> {
        unsafe {
            mmap_base.map_image_at(
                &self.source,
                self.source_offset,
                self.linear_memory_offset,
                self.len,
            )
        }
    }

    unsafe fn remap_as_zeros_at(&self, base: *mut u8) -> Result<()> {
        unsafe {
            self.source.remap_as_zeros_at(
                base.add(self.linear_memory_offset.byte_count()),
                self.len.byte_count(),
            )?;
        }
        Ok(())
    }
}

impl ModuleMemoryImages {
    /// Create a new `ModuleMemoryImages` for the given module. This can be
    /// passed in as part of a `InstanceAllocationRequest` to speed up
    /// instantiation and execution by using copy-on-write-backed memories.
    pub fn new(
        module: &Module,
        wasm_data: &[u8],
        mmap: Option<&MmapVec>,
    ) -> Result<Option<ModuleMemoryImages>> {
        let map = match &module.memory_initialization {
            MemoryInitialization::Static { map } => map,
            _ => return Ok(None),
        };
        let mut memories = PrimaryMap::with_capacity(map.len());
        let page_size = crate::runtime::vm::host_page_size();
        let page_size = u32::try_from(page_size).unwrap();
        for (memory_index, init) in map {
            // mmap-based-initialization only works for defined memories with a
            // known starting point of all zeros, so bail out if the mmeory is
            // imported.
            let defined_memory = match module.defined_memory_index(memory_index) {
                Some(idx) => idx,
                None => return Ok(None),
            };

            // If there's no initialization for this memory known then we don't
            // need an image for the memory so push `None` and move on.
            let init = match init {
                Some(init) => init,
                None => {
                    memories.push(None);
                    continue;
                }
            };

            // Get the image for this wasm module  as a subslice of `wasm_data`,
            // and then use that to try to create the `MemoryImage`. If this
            // creation files then we fail creating `ModuleMemoryImages` since this
            // memory couldn't be represented.
            let data = &wasm_data[init.data.start as usize..init.data.end as usize];
            if module.memories[memory_index]
                .minimum_byte_size()
                .map_or(false, |mem_initial_len| {
                    init.offset + u64::try_from(data.len()).unwrap() > mem_initial_len
                })
            {
                // The image is rounded up to multiples of the host OS page
                // size. But if Wasm is using a custom page size, the Wasm page
                // size might be smaller than the host OS page size, and that
                // rounding might have made the image larger than the Wasm
                // memory's initial length. This is *probably* okay, since the
                // rounding would have just introduced new runs of zeroes in the
                // image, but out of an abundance of caution we don't generate
                // CoW images in this scenario.
                return Ok(None);
            }

            let offset_usize = match usize::try_from(init.offset) {
                Ok(offset) => offset,
                Err(_) => return Ok(None),
            };
            let offset = HostAlignedByteCount::new(offset_usize)
                .expect("memory init offset is a multiple of the host page size");
            let image = match MemoryImage::new(page_size, offset, data, mmap)? {
                Some(image) => image,
                None => return Ok(None),
            };

            let idx = memories.push(Some(Arc::new(image)));
            assert_eq!(idx, defined_memory);
        }

        Ok(Some(ModuleMemoryImages { memories }))
    }
}

/// Slot management of a copy-on-write image which can be reused for the pooling
/// allocator.
///
/// This data structure manages a slot of linear memory, primarily in the
/// pooling allocator, which optionally has a contiguous memory image in the
/// middle of it. Pictorially this data structure manages a virtual memory
/// region that looks like:
///
/// ```text
///   +--------------------+-------------------+--------------+--------------+
///   |   anonymous        |      optional     |   anonymous  |    PROT_NONE |
///   |     zero           |       memory      |     zero     |     memory   |
///   |    memory          |       image       |    memory    |              |
///   +--------------------+-------------------+--------------+--------------+
///   |                     <------+---------->
///   |<-----+------------>         \
///   |      \                   image.len
///   |       \
///   |  image.linear_memory_offset
///   |
///   \
///  self.base is this virtual address
///
///    <------------------+------------------------------------------------>
///                        \
///                      static_size
///
///    <------------------+---------------------------------->
///                        \
///                      accessible
/// ```
///
/// When a `MemoryImageSlot` is created it's told what the `static_size` and
/// `accessible` limits are. Initially there is assumed to be no image in linear
/// memory.
///
/// When `MemoryImageSlot::instantiate` is called then the method will perform
/// a "synchronization" to take the image from its prior state to the new state
/// for the image specified. The first instantiation for example will mmap the
/// heap image into place. Upon reuse of a slot nothing happens except possibly
/// shrinking `self.accessible`. When a new image is used then the old image is
/// mapped to anonymous zero memory and then the new image is mapped in place.
///
/// A `MemoryImageSlot` is either `dirty` or it isn't. When a `MemoryImageSlot`
/// is dirty then it is assumed that any memory beneath `self.accessible` could
/// have any value. Instantiation cannot happen into a `dirty` slot, however, so
/// the `MemoryImageSlot::clear_and_remain_ready` returns this memory back to
/// its original state to mark `dirty = false`. This is done by resetting all
/// anonymous memory back to zero and the image itself back to its initial
/// contents.
///
/// On Linux this is achieved with the `madvise(MADV_DONTNEED)` syscall. This
/// syscall will release the physical pages back to the OS but retain the
/// original mappings, effectively resetting everything back to its initial
/// state. Non-linux platforms will replace all memory below `self.accessible`
/// with a fresh zero'd mmap, meaning that reuse is effectively not supported.
#[derive(Debug)]
pub struct MemoryImageSlot {
    /// The mmap and offset within it that contains the linear memory for this
    /// slot.
    base: MmapOffset,

    /// The maximum static memory size which `self.accessible` can grow to.
    static_size: usize,

    /// An optional image that is currently being used in this linear memory.
    ///
    /// This can be `None` in which case memory is originally all zeros. When
    /// `Some` the image describes where it's located within the image.
    image: Option<Arc<MemoryImage>>,

    /// The size of the heap that is readable and writable.
    ///
    /// Note that this may extend beyond the actual linear memory heap size in
    /// the case of dynamic memories in use. Memory accesses to memory below
    /// `self.accessible` may still page fault as pages are lazily brought in
    /// but the faults will always be resolved by the kernel.
    ///
    /// Also note that this is always page-aligned.
    accessible: HostAlignedByteCount,

    /// Whether this slot may have "dirty" pages (pages written by an
    /// instantiation). Set by `instantiate()` and cleared by
    /// `clear_and_remain_ready()`, and used in assertions to ensure
    /// those methods are called properly.
    ///
    /// Invariant: if !dirty, then this memory slot contains a clean
    /// CoW mapping of `image`, if `Some(..)`, and anonymous-zero
    /// memory beyond the image up to `static_size`. The addresses
    /// from offset 0 to `self.accessible` are R+W and set to zero or the
    /// initial image content, as appropriate. Everything between
    /// `self.accessible` and `self.static_size` is inaccessible.
    dirty: bool,
}

impl MemoryImageSlot {
    /// Create a new MemoryImageSlot. Assumes that there is an anonymous
    /// mmap backing in the given range to start.
    ///
    /// The `accessible` parameter describes how much of linear memory is
    /// already mapped as R/W with all zero-bytes. The `static_size` value is
    /// the maximum size of this image which `accessible` cannot grow beyond,
    /// and all memory from `accessible` from `static_size` should be mapped as
    /// `PROT_NONE` backed by zero-bytes.
    pub(crate) fn create(
        base: MmapOffset,
        accessible: HostAlignedByteCount,
        static_size: usize,
    ) -> Self {
        MemoryImageSlot {
            base,
            static_size,
            accessible,
            image: None,
            dirty: false,
        }
    }

    pub(crate) fn set_heap_limit(&mut self, size_bytes: usize) -> Result<()> {
        let size_bytes_aligned = HostAlignedByteCount::new_rounded_up(size_bytes)?;
        assert!(size_bytes <= self.static_size);
        assert!(size_bytes_aligned.byte_count() <= self.static_size);

        // If the heap limit already addresses accessible bytes then no syscalls
        // are necessary since the data is already mapped into the process and
        // waiting to go.
        //
        // This is used for "dynamic" memories where memory is not always
        // decommitted during recycling (but it's still always reset).
        if size_bytes_aligned <= self.accessible {
            return Ok(());
        }

        // Otherwise use `mprotect` to make the new pages read/write.
        self.set_protection(self.accessible..size_bytes_aligned, true)?;
        self.accessible = size_bytes_aligned;

        Ok(())
    }

    /// Prepares this slot for the instantiation of a new instance with the
    /// provided linear memory image.
    ///
    /// The `initial_size_bytes` parameter indicates the required initial size
    /// of the heap for the instance. The `maybe_image` is an optional initial
    /// image for linear memory to contains. The `style` is the way compiled
    /// code will be accessing this memory.
    ///
    /// The purpose of this method is to take a previously pristine slot
    /// (`!self.dirty`) and transform its prior state into state necessary for
    /// the given parameters. This could include, for example:
    ///
    /// * More memory may be made read/write if `initial_size_bytes` is larger
    ///   than `self.accessible`.
    /// * For `MemoryStyle::Static` linear memory may be made `PROT_NONE` if
    ///   `self.accessible` is larger than `initial_size_bytes`.
    /// * If no image was previously in place or if the wrong image was
    ///   previously in place then `mmap` may be used to setup the initial
    ///   image.
    pub(crate) fn instantiate(
        &mut self,
        initial_size_bytes: usize,
        maybe_image: Option<&Arc<MemoryImage>>,
        ty: &wasmtime_environ::Memory,
        tunables: &Tunables,
    ) -> Result<()> {
        assert!(!self.dirty);
        assert!(
            initial_size_bytes <= self.static_size,
            "initial_size_bytes <= self.static_size failed: \
             initial_size_bytes={initial_size_bytes}, self.static_size={}",
            self.static_size
        );
        let initial_size_bytes_page_aligned =
            HostAlignedByteCount::new_rounded_up(initial_size_bytes)?;

        // First order of business is to blow away the previous linear memory
        // image if it doesn't match the image specified here. If one is
        // detected then it's reset with anonymous memory which means that all
        // of memory up to `self.accessible` will now be read/write and zero.
        //
        // Note that this intentionally a "small mmap" which only covers the
        // extent of the prior initialization image in order to preserve
        // resident memory that might come before or after the image.
        if self.image.as_ref() != maybe_image {
            self.remove_image()?;
        }

        // The next order of business is to ensure that `self.accessible` is
        // appropriate. First up is to grow the read/write portion of memory if
        // it's not large enough to accommodate `initial_size_bytes`.
        if self.accessible < initial_size_bytes_page_aligned {
            self.set_protection(self.accessible..initial_size_bytes_page_aligned, true)?;
            self.accessible = initial_size_bytes_page_aligned;
        }

        // If (1) the accessible region is not in its initial state, and (2) the
        // memory relies on virtual memory at all (i.e. has offset guard
        // pages), then we need to reset memory protections. Put another way,
        // the only time it is safe to not reset protections is when we are
        // using dynamic memory without any guard pages.
        let host_page_size_log2 = u8::try_from(host_page_size().ilog2()).unwrap();
        if initial_size_bytes_page_aligned < self.accessible
            && (tunables.memory_guard_size > 0
                || ty.can_use_virtual_memory(tunables, host_page_size_log2))
        {
            self.set_protection(initial_size_bytes_page_aligned..self.accessible, false)?;
            self.accessible = initial_size_bytes_page_aligned;
        }

        // Now that memory is sized appropriately the final operation is to
        // place the new image into linear memory. Note that this operation is
        // skipped if `self.image` matches `maybe_image`.
        assert!(initial_size_bytes <= self.accessible.byte_count());
        assert!(initial_size_bytes_page_aligned <= self.accessible);
        if self.image.as_ref() != maybe_image {
            if let Some(image) = maybe_image.as_ref() {
                assert!(
                    image
                        .linear_memory_offset
                        .checked_add(image.len)
                        .unwrap()
                        .byte_count()
                        <= initial_size_bytes
                );
                if !image.len.is_zero() {
                    unsafe {
                        image.map_at(&self.base)?;
                    }
                }
            }
            self.image = maybe_image.cloned();
        }

        // Flag ourselves as `dirty` which means that the next operation on this
        // slot is required to be `clear_and_remain_ready`.
        self.dirty = true;

        Ok(())
    }

    pub(crate) fn remove_image(&mut self) -> Result<()> {
        if let Some(image) = &self.image {
            unsafe {
                image.remap_as_zeros_at(self.base.as_mut_ptr())?;
            }
            self.image = None;
        }
        Ok(())
    }

    /// Resets this linear memory slot back to a "pristine state".
    ///
    /// This will reset the memory back to its original contents on Linux or
    /// reset the contents back to zero on other platforms. The `keep_resident`
    /// argument is the maximum amount of memory to keep resident in this
    /// process's memory on Linux. Up to that much memory will be `memset` to
    /// zero where the rest of it will be reset or released with `madvise`.
    #[allow(dead_code, reason = "only used in some cfgs")]
    pub(crate) fn clear_and_remain_ready(
        &mut self,
        keep_resident: HostAlignedByteCount,
        decommit: impl FnMut(*mut u8, usize),
    ) -> Result<()> {
        assert!(self.dirty);

        unsafe {
            self.reset_all_memory_contents(keep_resident, decommit)?;
        }

        self.dirty = false;
        Ok(())
    }

    #[allow(dead_code, reason = "only used in some cfgs")]
    unsafe fn reset_all_memory_contents(
        &mut self,
        keep_resident: HostAlignedByteCount,
        decommit: impl FnMut(*mut u8, usize),
    ) -> Result<()> {
        match vm::decommit_behavior() {
            DecommitBehavior::Zero => {
                // If we're not on Linux then there's no generic platform way to
                // reset memory back to its original state, so instead reset memory
                // back to entirely zeros with an anonymous backing.
                //
                // Additionally the previous image, if any, is dropped here
                // since it's no longer applicable to this mapping.
                self.reset_with_anon_memory()
            }
            DecommitBehavior::RestoreOriginalMapping => {
                unsafe {
                    self.reset_with_original_mapping(keep_resident, decommit);
                }
                Ok(())
            }
        }
    }

    #[allow(dead_code, reason = "only used in some cfgs")]
    unsafe fn reset_with_original_mapping(
        &mut self,
        keep_resident: HostAlignedByteCount,
        mut decommit: impl FnMut(*mut u8, usize),
    ) {
        match &self.image {
            Some(image) => {
                if image.linear_memory_offset < keep_resident {
                    // If the image starts below the `keep_resident` then
                    // memory looks something like this:
                    //
                    //               up to `keep_resident` bytes
                    //                          |
                    //          +--------------------------+  remaining_memset
                    //          |                          | /
                    //  <-------------->                <------->
                    //
                    //                              image_end
                    // 0        linear_memory_offset   |             accessible
                    // |                |              |                  |
                    // +----------------+--------------+---------+--------+
                    // |  dirty memory  |    image     |   dirty memory   |
                    // +----------------+--------------+---------+--------+
                    //
                    //  <------+-------> <-----+----->  <---+---> <--+--->
                    //         |               |            |        |
                    //         |               |            |        |
                    //   memset (1)            /            |   madvise (4)
                    //                  mmadvise (2)       /
                    //                                    /
                    //                              memset (3)
                    //
                    //
                    // In this situation there are two disjoint regions that are
                    // `memset` manually to zero. Note that `memset (3)` may be
                    // zero bytes large. Furthermore `madvise (4)` may also be
                    // zero bytes large.

                    let image_end = image
                        .linear_memory_offset
                        .checked_add(image.len)
                        .expect("image is in bounds");
                    let mem_after_image = self
                        .accessible
                        .checked_sub(image_end)
                        .expect("image_end falls before self.accessible");
                    let excess = keep_resident
                        .checked_sub(image.linear_memory_offset)
                        .expect(
                            "if statement checks that keep_resident > image.linear_memory_offset",
                        );
                    let remaining_memset = excess.min(mem_after_image);

                    // This is memset (1)
                    unsafe {
                        ptr::write_bytes(
                            self.base.as_mut_ptr(),
                            0u8,
                            image.linear_memory_offset.byte_count(),
                        );
                    }

                    // This is madvise (2)
                    unsafe {
                        self.restore_original_mapping(
                            image.linear_memory_offset,
                            image.len,
                            &mut decommit,
                        );
                    }

                    // This is memset (3)
                    unsafe {
                        ptr::write_bytes(
                            self.base.as_mut_ptr().add(image_end.byte_count()),
                            0u8,
                            remaining_memset.byte_count(),
                        );
                    }

                    // This is madvise (4)
                    unsafe {
                        self.restore_original_mapping(
                            image_end
                                .checked_add(remaining_memset)
                                .expect("image_end + remaining_memset is in bounds"),
                            mem_after_image
                                .checked_sub(remaining_memset)
                                .expect("remaining_memset defined to be <= mem_after_image"),
                            &mut decommit,
                        );
                    }
                } else {
                    // If the image starts after the `keep_resident` threshold
                    // then we memset the start of linear memory and then use
                    // madvise below for the rest of it, including the image.
                    //
                    // 0             keep_resident                   accessible
                    // |                |                                 |
                    // +----------------+---+----------+------------------+
                    // |  dirty memory      |  image   |   dirty memory   |
                    // +----------------+---+----------+------------------+
                    //
                    //  <------+-------> <-------------+----------------->
                    //         |                       |
                    //         |                       |
                    //   memset (1)                 madvise (2)
                    //
                    // Here only a single memset is necessary since the image
                    // started after the threshold which we're keeping resident.
                    // Note that the memset may be zero bytes here.

                    // This is memset (1)
                    unsafe {
                        ptr::write_bytes(self.base.as_mut_ptr(), 0u8, keep_resident.byte_count());
                    }

                    // This is madvise (2)
                    unsafe {
                        self.restore_original_mapping(
                            keep_resident,
                            self.accessible
                                .checked_sub(keep_resident)
                                .expect("keep_resident is a subset of accessible memory"),
                            decommit,
                        );
                    }
                };
            }

            // If there's no memory image for this slot then memset the first
            // bytes in the memory back to zero while using `madvise` to purge
            // the rest.
            None => {
                let size_to_memset = keep_resident.min(self.accessible);
                unsafe {
                    ptr::write_bytes(self.base.as_mut_ptr(), 0u8, size_to_memset.byte_count());
                    self.restore_original_mapping(
                        size_to_memset,
                        self.accessible
                            .checked_sub(size_to_memset)
                            .expect("size_to_memset is defined to be <= self.accessible"),
                        decommit,
                    );
                }
            }
        }
    }

    #[allow(dead_code, reason = "only used in some cfgs")]
    unsafe fn restore_original_mapping(
        &self,
        base: HostAlignedByteCount,
        len: HostAlignedByteCount,
        mut decommit: impl FnMut(*mut u8, usize),
    ) {
        assert!(base.checked_add(len).unwrap() <= self.accessible);
        if len == 0 {
            return;
        }

        assert_eq!(
            vm::decommit_behavior(),
            DecommitBehavior::RestoreOriginalMapping
        );
        unsafe {
            decommit(
                self.base.as_mut_ptr().add(base.byte_count()),
                len.byte_count(),
            );
        }
    }

    fn set_protection(&self, range: Range<HostAlignedByteCount>, readwrite: bool) -> Result<()> {
        let len = range
            .end
            .checked_sub(range.start)
            .expect("range.start <= range.end");
        assert!(range.end.byte_count() <= self.static_size);
        if len.is_zero() {
            return Ok(());
        }

        // TODO: use Mmap to change memory permissions instead of these free
        // functions.
        unsafe {
            let start = self.base.as_mut_ptr().add(range.start.byte_count());
            if readwrite {
                vm::expose_existing_mapping(start, len.byte_count())?;
            } else {
                vm::hide_existing_mapping(start, len.byte_count())?;
            }
        }

        Ok(())
    }

    pub(crate) fn has_image(&self) -> bool {
        self.image.is_some()
    }

    #[allow(dead_code, reason = "only used in some cfgs")]
    pub(crate) fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Map anonymous zeroed memory across the whole slot,
    /// inaccessible. Used both during instantiate and during drop.
    pub(crate) fn reset_with_anon_memory(&mut self) -> Result<()> {
        if self.static_size == 0 {
            assert!(self.image.is_none());
            assert_eq!(self.accessible, 0);
            return Ok(());
        }

        unsafe {
            vm::erase_existing_mapping(self.base.as_mut_ptr(), self.static_size)?;
        }

        self.image = None;
        self.accessible = HostAlignedByteCount::ZERO;

        Ok(())
    }
}

#[cfg(all(test, target_os = "linux", not(miri)))]
mod test {
    use super::*;
    use crate::runtime::vm::mmap::{AlignedLength, Mmap};
    use crate::runtime::vm::sys::vm::decommit_pages;
    use crate::runtime::vm::{HostAlignedByteCount, host_page_size};
    use std::sync::Arc;
    use wasmtime_environ::{IndexType, Limits, Memory};

    fn create_memfd_with_data(offset: usize, data: &[u8]) -> Result<MemoryImage> {
        // offset must be a multiple of the page size.
        let linear_memory_offset =
            HostAlignedByteCount::new(offset).expect("offset is page-aligned");
        // The image length is rounded up to the nearest page size
        let image_len = HostAlignedByteCount::new_rounded_up(data.len()).unwrap();

        Ok(MemoryImage {
            source: MemoryImageSource::from_data(data)?.unwrap(),
            len: image_len,
            source_offset: 0,
            linear_memory_offset,
        })
    }

    fn dummy_memory() -> Memory {
        Memory {
            idx_type: IndexType::I32,
            limits: Limits { min: 0, max: None },
            shared: false,
            page_size_log2: Memory::DEFAULT_PAGE_SIZE_LOG2,
        }
    }

    fn mmap_4mib_inaccessible() -> Arc<Mmap<AlignedLength>> {
        let four_mib = HostAlignedByteCount::new(4 << 20).expect("4 MiB is page aligned");
        Arc::new(Mmap::accessible_reserved(HostAlignedByteCount::ZERO, four_mib).unwrap())
    }

    /// Presents a part of an mmap as a mutable slice within a callback.
    ///
    /// The callback ensures that the reference no longer lives after the
    /// function is done.
    ///
    /// # Safety
    ///
    /// The caller must ensure that during this function call, the only way this
    /// region of memory is not accessed by (read from or written to) is via the
    /// reference. Making the callback `'static` goes some way towards ensuring
    /// that, but it's still possible to squirrel away a reference into global
    /// state. So don't do that.
    unsafe fn with_slice_mut(
        mmap: &Arc<Mmap<AlignedLength>>,
        range: Range<usize>,
        f: impl FnOnce(&mut [u8]) + 'static,
    ) {
        let ptr = mmap.as_ptr().cast_mut();
        let slice = unsafe {
            core::slice::from_raw_parts_mut(ptr.add(range.start), range.end - range.start)
        };
        f(slice);
    }

    #[test]
    fn instantiate_no_image() {
        let ty = dummy_memory();
        let tunables = Tunables {
            memory_reservation: 4 << 30,
            ..Tunables::default_miri()
        };
        // 4 MiB mmap'd area, not accessible
        let mmap = mmap_4mib_inaccessible();
        // Create a MemoryImageSlot on top of it
        let mut memfd =
            MemoryImageSlot::create(mmap.zero_offset(), HostAlignedByteCount::ZERO, 4 << 20);
        assert!(!memfd.is_dirty());
        // instantiate with 64 KiB initial size
        memfd.instantiate(64 << 10, None, &ty, &tunables).unwrap();
        assert!(memfd.is_dirty());

        // We should be able to access this 64 KiB (try both ends) and
        // it should consist of zeroes.
        unsafe {
            with_slice_mut(&mmap, 0..65536, |slice| {
                assert_eq!(0, slice[0]);
                assert_eq!(0, slice[65535]);
                slice[1024] = 42;
                assert_eq!(42, slice[1024]);
            });
        }

        // grow the heap
        memfd.set_heap_limit(128 << 10).unwrap();
        let slice = unsafe { mmap.slice(0..1 << 20) };
        assert_eq!(42, slice[1024]);
        assert_eq!(0, slice[131071]);
        // instantiate again; we should see zeroes, even as the
        // reuse-anon-mmap-opt kicks in
        memfd
            .clear_and_remain_ready(HostAlignedByteCount::ZERO, |ptr, len| unsafe {
                decommit_pages(ptr, len).unwrap()
            })
            .unwrap();
        assert!(!memfd.is_dirty());
        memfd.instantiate(64 << 10, None, &ty, &tunables).unwrap();
        let slice = unsafe { mmap.slice(0..65536) };
        assert_eq!(0, slice[1024]);
    }

    #[test]
    fn instantiate_image() {
        let page_size = host_page_size();
        let ty = dummy_memory();
        let tunables = Tunables {
            memory_reservation: 4 << 30,
            ..Tunables::default_miri()
        };
        // 4 MiB mmap'd area, not accessible
        let mmap = mmap_4mib_inaccessible();
        // Create a MemoryImageSlot on top of it
        let mut memfd =
            MemoryImageSlot::create(mmap.zero_offset(), HostAlignedByteCount::ZERO, 4 << 20);
        // Create an image with some data.
        let image = Arc::new(create_memfd_with_data(page_size, &[1, 2, 3, 4]).unwrap());
        // Instantiate with this image
        memfd
            .instantiate(64 << 10, Some(&image), &ty, &tunables)
            .unwrap();
        assert!(memfd.has_image());

        unsafe {
            with_slice_mut(&mmap, 0..65536, move |slice| {
                assert_eq!(&[1, 2, 3, 4], &slice[page_size..][..4]);
                slice[page_size] = 5;
            });
        }

        // Clear and re-instantiate same image
        memfd
            .clear_and_remain_ready(HostAlignedByteCount::ZERO, |ptr, len| unsafe {
                decommit_pages(ptr, len).unwrap()
            })
            .unwrap();
        memfd
            .instantiate(64 << 10, Some(&image), &ty, &tunables)
            .unwrap();
        let slice = unsafe { mmap.slice(0..65536) };
        assert_eq!(&[1, 2, 3, 4], &slice[page_size..][..4]);

        // Clear and re-instantiate no image
        memfd
            .clear_and_remain_ready(HostAlignedByteCount::ZERO, |ptr, len| unsafe {
                decommit_pages(ptr, len).unwrap()
            })
            .unwrap();
        memfd.instantiate(64 << 10, None, &ty, &tunables).unwrap();
        assert!(!memfd.has_image());
        let slice = unsafe { mmap.slice(0..65536) };
        assert_eq!(&[0, 0, 0, 0], &slice[page_size..][..4]);

        // Clear and re-instantiate image again
        memfd
            .clear_and_remain_ready(HostAlignedByteCount::ZERO, |ptr, len| unsafe {
                decommit_pages(ptr, len).unwrap()
            })
            .unwrap();
        memfd
            .instantiate(64 << 10, Some(&image), &ty, &tunables)
            .unwrap();
        let slice = unsafe { mmap.slice(0..65536) };
        assert_eq!(&[1, 2, 3, 4], &slice[page_size..][..4]);

        // Create another image with different data.
        let image2 = Arc::new(create_memfd_with_data(page_size, &[10, 11, 12, 13]).unwrap());
        memfd
            .clear_and_remain_ready(HostAlignedByteCount::ZERO, |ptr, len| unsafe {
                decommit_pages(ptr, len).unwrap()
            })
            .unwrap();
        memfd
            .instantiate(128 << 10, Some(&image2), &ty, &tunables)
            .unwrap();
        let slice = unsafe { mmap.slice(0..65536) };
        assert_eq!(&[10, 11, 12, 13], &slice[page_size..][..4]);

        // Instantiate the original image again; we should notice it's
        // a different image and not reuse the mappings.
        memfd
            .clear_and_remain_ready(HostAlignedByteCount::ZERO, |ptr, len| unsafe {
                decommit_pages(ptr, len).unwrap()
            })
            .unwrap();
        memfd
            .instantiate(64 << 10, Some(&image), &ty, &tunables)
            .unwrap();
        let slice = unsafe { mmap.slice(0..65536) };
        assert_eq!(&[1, 2, 3, 4], &slice[page_size..][..4]);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn memset_instead_of_madvise() {
        let page_size = host_page_size();
        let ty = dummy_memory();
        let tunables = Tunables {
            memory_reservation: 100 << 16,
            ..Tunables::default_miri()
        };
        let mmap = mmap_4mib_inaccessible();
        let mut memfd =
            MemoryImageSlot::create(mmap.zero_offset(), HostAlignedByteCount::ZERO, 4 << 20);

        // Test basics with the image
        for image_off in [0, page_size, page_size * 2] {
            let image = Arc::new(create_memfd_with_data(image_off, &[1, 2, 3, 4]).unwrap());
            for amt_to_memset in [0, page_size, page_size * 10, 1 << 20, 10 << 20] {
                let amt_to_memset = HostAlignedByteCount::new(amt_to_memset).unwrap();
                memfd
                    .instantiate(64 << 10, Some(&image), &ty, &tunables)
                    .unwrap();
                assert!(memfd.has_image());

                unsafe {
                    with_slice_mut(&mmap, 0..64 << 10, move |slice| {
                        if image_off > 0 {
                            assert_eq!(slice[image_off - 1], 0);
                        }
                        assert_eq!(slice[image_off + 5], 0);
                        assert_eq!(&[1, 2, 3, 4], &slice[image_off..][..4]);
                        slice[image_off] = 5;
                        assert_eq!(&[5, 2, 3, 4], &slice[image_off..][..4]);
                    })
                };

                memfd
                    .clear_and_remain_ready(amt_to_memset, |ptr, len| unsafe {
                        decommit_pages(ptr, len).unwrap()
                    })
                    .unwrap();
            }
        }

        // Test without an image
        for amt_to_memset in [0, page_size, page_size * 10, 1 << 20, 10 << 20] {
            let amt_to_memset = HostAlignedByteCount::new(amt_to_memset).unwrap();
            memfd.instantiate(64 << 10, None, &ty, &tunables).unwrap();

            unsafe {
                with_slice_mut(&mmap, 0..64 << 10, |slice| {
                    for chunk in slice.chunks_mut(1024) {
                        assert_eq!(chunk[0], 0);
                        chunk[0] = 5;
                    }
                });
            }
            memfd
                .clear_and_remain_ready(amt_to_memset, |ptr, len| unsafe {
                    decommit_pages(ptr, len).unwrap()
                })
                .unwrap();
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn dynamic() {
        let page_size = host_page_size();
        let ty = dummy_memory();
        let tunables = Tunables {
            memory_reservation: 0,
            memory_reservation_for_growth: 200,
            ..Tunables::default_miri()
        };

        let mmap = mmap_4mib_inaccessible();
        let mut memfd =
            MemoryImageSlot::create(mmap.zero_offset(), HostAlignedByteCount::ZERO, 4 << 20);
        let image = Arc::new(create_memfd_with_data(page_size, &[1, 2, 3, 4]).unwrap());
        let initial = 64 << 10;

        // Instantiate the image and test that memory remains accessible after
        // it's cleared.
        memfd
            .instantiate(initial, Some(&image), &ty, &tunables)
            .unwrap();
        assert!(memfd.has_image());

        unsafe {
            with_slice_mut(&mmap, 0..(64 << 10) + page_size, move |slice| {
                assert_eq!(&[1, 2, 3, 4], &slice[page_size..][..4]);
                slice[page_size] = 5;
                assert_eq!(&[5, 2, 3, 4], &slice[page_size..][..4]);
            });
        }

        memfd
            .clear_and_remain_ready(HostAlignedByteCount::ZERO, |ptr, len| unsafe {
                decommit_pages(ptr, len).unwrap()
            })
            .unwrap();
        let slice = unsafe { mmap.slice(0..(64 << 10) + page_size) };
        assert_eq!(&[1, 2, 3, 4], &slice[page_size..][..4]);

        // Re-instantiate make sure it preserves memory. Grow a bit and set data
        // beyond the initial size.
        memfd
            .instantiate(initial, Some(&image), &ty, &tunables)
            .unwrap();
        assert_eq!(&[1, 2, 3, 4], &slice[page_size..][..4]);

        memfd.set_heap_limit(initial * 2).unwrap();

        unsafe {
            with_slice_mut(&mmap, 0..(64 << 10) + page_size, move |slice| {
                assert_eq!(&[0, 0], &slice[initial..initial + 2]);
                slice[initial] = 100;
                assert_eq!(&[100, 0], &slice[initial..initial + 2]);
            });
        }

        memfd
            .clear_and_remain_ready(HostAlignedByteCount::ZERO, |ptr, len| unsafe {
                decommit_pages(ptr, len).unwrap()
            })
            .unwrap();

        // Test that memory is still accessible, but it's been reset
        assert_eq!(&[0, 0], &slice[initial..initial + 2]);

        // Instantiate again, and again memory beyond the initial size should
        // still be accessible. Grow into it again and make sure it works.
        memfd
            .instantiate(initial, Some(&image), &ty, &tunables)
            .unwrap();
        assert_eq!(&[0, 0], &slice[initial..initial + 2]);
        memfd.set_heap_limit(initial * 2).unwrap();

        unsafe {
            with_slice_mut(&mmap, 0..(64 << 10) + page_size, move |slice| {
                assert_eq!(&[0, 0], &slice[initial..initial + 2]);
                slice[initial] = 100;
                assert_eq!(&[100, 0], &slice[initial..initial + 2]);
            });
        }

        memfd
            .clear_and_remain_ready(HostAlignedByteCount::ZERO, |ptr, len| unsafe {
                decommit_pages(ptr, len).unwrap()
            })
            .unwrap();

        // Reset the image to none and double-check everything is back to zero
        memfd.instantiate(64 << 10, None, &ty, &tunables).unwrap();
        assert!(!memfd.has_image());
        assert_eq!(&[0, 0, 0, 0], &slice[page_size..][..4]);
        assert_eq!(&[0, 0], &slice[initial..initial + 2]);
    }
}
