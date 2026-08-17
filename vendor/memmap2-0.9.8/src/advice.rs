/// Values supported by [`Mmap::advise`][crate::Mmap::advise] and [`MmapMut::advise`][crate::MmapMut::advise] functions.
///
/// See [madvise()](https://man7.org/linux/man-pages/man2/madvise.2.html) map page.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Advice {
    /// **MADV_NORMAL**
    ///
    /// No special treatment.  This is the default.
    Normal = libc::MADV_NORMAL,

    /// **MADV_RANDOM**
    ///
    /// Expect page references in random order.  (Hence, read
    /// ahead may be less useful than normally.)
    Random = libc::MADV_RANDOM,

    /// **MADV_SEQUENTIAL**
    ///
    /// Expect page references in sequential order.  (Hence, pages
    /// in the given range can be aggressively read ahead, and may
    /// be freed soon after they are accessed.)
    Sequential = libc::MADV_SEQUENTIAL,

    /// **MADV_WILLNEED**
    ///
    /// Expect access in the near future.  (Hence, it might be a
    /// good idea to read some pages ahead.)
    WillNeed = libc::MADV_WILLNEED,

    /// **MADV_DONTFORK** - Linux only (since Linux 2.6.16)
    ///
    /// Do not make the pages in this range available to the child
    /// after a fork(2).  This is useful to prevent copy-on-write
    /// semantics from changing the physical location of a page if
    /// the parent writes to it after a fork(2).  (Such page
    /// relocations cause problems for hardware that DMAs into the
    /// page.)
    #[cfg(target_os = "linux")]
    DontFork = libc::MADV_DONTFORK,

    /// **MADV_DOFORK** - Linux only (since Linux 2.6.16)
    ///
    /// Undo the effect of MADV_DONTFORK, restoring the default
    /// behavior, whereby a mapping is inherited across fork(2).
    #[cfg(target_os = "linux")]
    DoFork = libc::MADV_DOFORK,

    /// **MADV_MERGEABLE** - Linux only (since Linux 2.6.32)
    ///
    /// Enable Kernel Samepage Merging (KSM) for the pages in the
    /// range specified by addr and length.  The kernel regularly
    /// scans those areas of user memory that have been marked as
    /// mergeable, looking for pages with identical content.
    /// These are replaced by a single write-protected page (which
    /// is automatically copied if a process later wants to update
    /// the content of the page).  KSM merges only private
    /// anonymous pages (see mmap(2)).
    ///
    /// The KSM feature is intended for applications that generate
    /// many instances of the same data (e.g., virtualization
    /// systems such as KVM).  It can consume a lot of processing
    /// power; use with care.  See the Linux kernel source file
    /// Documentation/admin-guide/mm/ksm.rst for more details.
    ///
    /// The MADV_MERGEABLE and MADV_UNMERGEABLE operations are
    /// available only if the kernel was configured with
    /// CONFIG_KSM.
    #[cfg(target_os = "linux")]
    Mergeable = libc::MADV_MERGEABLE,

    /// **MADV_UNMERGEABLE** - Linux only (since Linux 2.6.32)
    ///
    /// Undo the effect of an earlier MADV_MERGEABLE operation on
    /// the specified address range; KSM unmerges whatever pages
    /// it had merged in the address range specified by addr and
    /// length.
    #[cfg(target_os = "linux")]
    Unmergeable = libc::MADV_UNMERGEABLE,

    /// **MADV_HUGEPAGE** - Linux only (since Linux 2.6.38)
    ///
    /// Enable Transparent Huge Pages (THP) for pages in the range
    /// specified by addr and length.  Currently, Transparent Huge
    /// Pages work only with private anonymous pages (see
    /// mmap(2)).  The kernel will regularly scan the areas marked
    /// as huge page candidates to replace them with huge pages.
    /// The kernel will also allocate huge pages directly when the
    /// region is naturally aligned to the huge page size (see
    /// posix_memalign(2)).
    ///
    /// This feature is primarily aimed at applications that use
    /// large mappings of data and access large regions of that
    /// memory at a time (e.g., virtualization systems such as
    /// QEMU).  It can very easily waste memory (e.g., a 2 MB
    /// mapping that only ever accesses 1 byte will result in 2 MB
    /// of wired memory instead of one 4 KB page).  See the Linux
    /// kernel source file
    /// Documentation/admin-guide/mm/transhuge.rst for more
    /// details.
    ///
    /// Most common kernels configurations provide MADV_HUGEPAGE-
    /// style behavior by default, and thus MADV_HUGEPAGE is
    /// normally not necessary.  It is mostly intended for
    /// embedded systems, where MADV_HUGEPAGE-style behavior may
    /// not be enabled by default in the kernel.  On such systems,
    /// this flag can be used in order to selectively enable THP.
    /// Whenever MADV_HUGEPAGE is used, it should always be in
    /// regions of memory with an access pattern that the
    /// developer knows in advance won't risk to increase the
    /// memory footprint of the application when transparent
    /// hugepages are enabled.
    ///
    /// The MADV_HUGEPAGE and MADV_NOHUGEPAGE operations are
    /// available only if the kernel was configured with
    /// CONFIG_TRANSPARENT_HUGEPAGE.
    #[cfg(target_os = "linux")]
    HugePage = libc::MADV_HUGEPAGE,

    /// **MADV_NOHUGEPAGE** - Linux only (since Linux 2.6.38)
    ///
    /// Ensures that memory in the address range specified by addr
    /// and length will not be backed by transparent hugepages.
    #[cfg(target_os = "linux")]
    NoHugePage = libc::MADV_NOHUGEPAGE,

    /// **MADV_DONTDUMP** - Linux only (since Linux 3.4)
    ///
    /// Exclude from a core dump those pages in the range
    /// specified by addr and length.  This is useful in
    /// applications that have large areas of memory that are
    /// known not to be useful in a core dump.  The effect of
    /// **MADV_DONTDUMP** takes precedence over the bit mask that is
    /// set via the `/proc/[pid]/coredump_filter` file (see
    /// core(5)).
    #[cfg(target_os = "linux")]
    DontDump = libc::MADV_DONTDUMP,

    /// **MADV_DODUMP** - Linux only (since Linux 3.4)
    ///
    /// Undo the effect of an earlier MADV_DONTDUMP.
    #[cfg(target_os = "linux")]
    DoDump = libc::MADV_DODUMP,

    /// **MADV_HWPOISON** - Linux only (since Linux 2.6.32)
    ///
    /// Poison the pages in the range specified by addr and length
    /// and handle subsequent references to those pages like a
    /// hardware memory corruption.  This operation is available
    /// only for privileged (CAP_SYS_ADMIN) processes.  This
    /// operation may result in the calling process receiving a
    /// SIGBUS and the page being unmapped.
    ///
    /// This feature is intended for testing of memory error-
    /// handling code; it is available only if the kernel was
    /// configured with CONFIG_MEMORY_FAILURE.
    #[cfg(target_os = "linux")]
    HwPoison = libc::MADV_HWPOISON,

    /// **MADV_POPULATE_READ** - Linux only (since Linux 5.14)
    ///
    /// Populate  (prefault)  page  tables readable, faulting in all
    /// pages in the range just as  if  manually  reading  from  each
    /// page; however, avoid the actual memory access that would have
    /// been performed after handling the fault.
    ///
    /// In contrast to MAP_POPULATE, MADV_POPULATE_READ does not hide
    /// errors,  can  be  applied to (parts of) existing mappings and
    /// will always populate (prefault) page  tables  readable.   One
    /// example  use  case is prefaulting a file mapping, reading all
    /// file content from disk; however, pages won't be  dirtied  and
    /// consequently  won't  have  to  be  written  back to disk when
    /// evicting the pages from memory.
    ///
    /// Depending on the underlying mapping, map the shared zeropage,
    /// preallocate  memory  or  read the underlying file; files with
    /// holes might or might not preallocate blocks.   If  populating
    /// fails, a SIGBUS signal is not generated; instead, an error is
    /// returned.
    ///
    /// If MADV_POPULATE_READ succeeds, all  page  tables  have  been
    /// populated  (prefaulted) readable once.  If MADV_POPULATE_READ
    /// fails, some page tables might have been populated.
    ///
    /// MADV_POPULATE_READ cannot be applied to mappings without read
    /// permissions  and  special  mappings,  for  example,  mappings
    /// marked with kernel-internal flags such as VM_PFNMAP or VM_IO,
    /// or secret memory regions created using memfd_secret(2).
    ///
    /// Note  that with MADV_POPULATE_READ, the process can be killed
    /// at any moment when the system runs out of memory.
    #[cfg(target_os = "linux")]
    PopulateRead = libc::MADV_POPULATE_READ,

    /// **MADV_POPULATE_WRITE** - Linux only (since Linux 5.14)
    ///
    /// Populate (prefault) page tables  writable,  faulting  in  all
    /// pages  in  the range just as if manually writing to each each
    /// page; however, avoid the actual memory access that would have
    /// been performed after handling the fault.
    ///
    /// In  contrast  to  MAP_POPULATE,  MADV_POPULATE_WRITE does not
    /// hide errors, can be applied to (parts of)  existing  mappings
    /// and  will  always  populate  (prefault) page tables writable.
    /// One example use case is preallocating  memory,  breaking  any
    /// CoW (Copy on Write).
    ///
    /// Depending  on  the  underlying mapping, preallocate memory or
    /// read the underlying file; files with holes  will  preallocate
    /// blocks.   If  populating fails, a SIGBUS signal is not gener‐
    /// ated; instead, an error is returned.
    ///
    /// If MADV_POPULATE_WRITE succeeds, all page  tables  have  been
    /// populated (prefaulted) writable once.  If MADV_POPULATE_WRITE
    /// fails, some page tables might have been populated.
    ///
    /// MADV_POPULATE_WRITE cannot be  applied  to  mappings  without
    /// write permissions and special mappings, for example, mappings
    /// marked with kernel-internal flags such as VM_PFNMAP or VM_IO,
    /// or secret memory regions created using memfd_secret(2).
    ///
    /// Note that with MADV_POPULATE_WRITE, the process can be killed
    /// at any moment when the system runs out of memory.
    #[cfg(target_os = "linux")]
    PopulateWrite = libc::MADV_POPULATE_WRITE,

    /// **MADV_ZERO_WIRED_PAGES** - Darwin only
    ///
    /// Indicates that the application would like the wired pages in this address range to be
    /// zeroed out if the address range is deallocated without first unwiring the pages (i.e.
    /// a munmap(2) without a preceding munlock(2) or the application quits).  This is used
    /// with `madvise()` system call.
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    ZeroWiredPages = libc::MADV_ZERO_WIRED_PAGES,
}

/// Values supported by [`Mmap::unchecked_advise`][crate::Mmap::unchecked_advise] and [`MmapMut::unchecked_advise`][crate::MmapMut::unchecked_advise] functions.
///
/// These flags can be passed to the [madvise (2)][man_page] system call
/// and effects on the mapped pages which are conceptually writes,
/// i.e. the change the observable contents of these pages which
/// implies undefined behaviour if the mapping is still borrowed.
///
/// Hence, these potentially unsafe flags must be used with the unsafe
/// methods and the programmer has to justify that the code
/// does not keep any borrows of the mapping active while the mapped pages
/// are updated by the kernel's memory management subsystem.
///
/// [man_page]: https://man7.org/linux/man-pages/man2/madvise.2.html
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum UncheckedAdvice {
    /// **MADV_DONTNEED**
    ///
    /// Do not expect access in the near future.  (For the time
    /// being, the application is finished with the given range,
    /// so the kernel can free resources associated with it.)
    ///
    /// After a successful MADV_DONTNEED operation, the semantics
    /// of memory access in the specified region are changed:
    /// subsequent accesses of pages in the range will succeed,
    /// but will result in either repopulating the memory contents
    /// from the up-to-date contents of the underlying mapped file
    /// (for shared file mappings, shared anonymous mappings, and
    /// shmem-based techniques such as System V shared memory
    /// segments) or zero-fill-on-demand pages for anonymous
    /// private mappings.
    ///
    /// Note that, when applied to shared mappings, MADV_DONTNEED
    /// might not lead to immediate freeing of the pages in the
    /// range.  The kernel is free to delay freeing the pages
    /// until an appropriate moment.  The resident set size (RSS)
    /// of the calling process will be immediately reduced
    /// however.
    ///
    /// **MADV_DONTNEED** cannot be applied to locked pages, Huge TLB
    /// pages, or VM_PFNMAP pages.  (Pages marked with the kernel-
    /// internal VM_PFNMAP flag are special memory areas that are
    /// not managed by the virtual memory subsystem.  Such pages
    /// are typically created by device drivers that map the pages
    /// into user space.)
    ///
    /// # Safety
    ///
    /// Using the returned value with conceptually write to the
    /// mapped pages, i.e. borrowing the mapping when the pages
    /// are freed results in undefined behaviour.
    DontNeed = libc::MADV_DONTNEED,

    //
    // The rest are Linux-specific
    //
    /// **MADV_FREE** - Linux (since Linux 4.5) and Darwin
    ///
    /// The application no longer requires the pages in the range
    /// specified by addr and len.  The kernel can thus free these
    /// pages, but the freeing could be delayed until memory
    /// pressure occurs.  For each of the pages that has been
    /// marked to be freed but has not yet been freed, the free
    /// operation will be canceled if the caller writes into the
    /// page.  After a successful MADV_FREE operation, any stale
    /// data (i.e., dirty, unwritten pages) will be lost when the
    /// kernel frees the pages.  However, subsequent writes to
    /// pages in the range will succeed and then kernel cannot
    /// free those dirtied pages, so that the caller can always
    /// see just written data.  If there is no subsequent write,
    /// the kernel can free the pages at any time.  Once pages in
    /// the range have been freed, the caller will see zero-fill-
    /// on-demand pages upon subsequent page references.
    ///
    /// The MADV_FREE operation can be applied only to private
    /// anonymous pages (see mmap(2)).  In Linux before version
    /// 4.12, when freeing pages on a swapless system, the pages
    /// in the given range are freed instantly, regardless of
    /// memory pressure.
    ///
    /// # Safety
    ///
    /// Using the returned value with conceptually write to the
    /// mapped pages, i.e. borrowing the mapping while the pages
    /// are still being freed results in undefined behaviour.
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "ios"))]
    Free = libc::MADV_FREE,

    /// **MADV_REMOVE** - Linux only (since Linux 2.6.16)
    ///
    /// Free up a given range of pages and its associated backing
    /// store.  This is equivalent to punching a hole in the
    /// corresponding byte range of the backing store (see
    /// fallocate(2)).  Subsequent accesses in the specified
    /// address range will see bytes containing zero.
    ///
    /// The specified address range must be mapped shared and
    /// writable.  This flag cannot be applied to locked pages,
    /// Huge TLB pages, or VM_PFNMAP pages.
    ///
    /// In the initial implementation, only tmpfs(5) was supported
    /// **MADV_REMOVE**; but since Linux 3.5, any filesystem which
    /// supports the fallocate(2) FALLOC_FL_PUNCH_HOLE mode also
    /// supports MADV_REMOVE.  Hugetlbfs fails with the error
    /// EINVAL and other filesystems fail with the error
    /// EOPNOTSUPP.
    ///
    /// # Safety
    ///
    /// Using the returned value with conceptually write to the
    /// mapped pages, i.e. borrowing the mapping when the pages
    /// are freed results in undefined behaviour.
    #[cfg(target_os = "linux")]
    Remove = libc::MADV_REMOVE,

    /// **MADV_FREE_REUSABLE** - Darwin only
    ///
    /// Behaves like **MADV_FREE**, but the freed pages are accounted for in the RSS of the process.
    ///
    /// # Safety
    ///
    /// Using the returned value with conceptually write to the
    /// mapped pages, i.e. borrowing the mapping while the pages
    /// are still being freed results in undefined behaviour.
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    FreeReusable = libc::MADV_FREE_REUSABLE,

    /// **MADV_FREE_REUSE** - Darwin only
    ///
    /// Marks a memory region previously freed by **MADV_FREE_REUSABLE** as non-reusable, accounts
    /// for the pages in the RSS of the process. Pages that have been freed will be replaced by
    /// zero-filled pages on demand, other pages will be left as is.
    ///
    /// # Safety
    ///
    /// Using the returned value with conceptually write to the
    /// mapped pages, i.e. borrowing the mapping while the pages
    /// are still being freed results in undefined behaviour.
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    FreeReuse = libc::MADV_FREE_REUSE,
}

// Future expansion:
// MADV_SOFT_OFFLINE  (since Linux 2.6.33)
// MADV_WIPEONFORK  (since Linux 4.14)
// MADV_KEEPONFORK  (since Linux 4.14)
// MADV_COLD  (since Linux 5.4)
// MADV_PAGEOUT  (since Linux 5.4)

#[cfg(target_os = "linux")]
impl Advice {
    /// Performs a runtime check if this advice is supported by the kernel.
    /// Only supported on Linux. See the [`madvise(2)`] man page.
    ///
    /// [`madvise(2)`]: https://man7.org/linux/man-pages/man2/madvise.2.html#VERSIONS
    pub fn is_supported(self) -> bool {
        (unsafe { libc::madvise(std::ptr::null_mut(), 0, self as libc::c_int) }) == 0
    }
}

#[cfg(target_os = "linux")]
impl UncheckedAdvice {
    /// Performs a runtime check if this advice is supported by the kernel.
    /// Only supported on Linux. See the [`madvise(2)`] man page.
    ///
    /// [`madvise(2)`]: https://man7.org/linux/man-pages/man2/madvise.2.html#VERSIONS
    pub fn is_supported(self) -> bool {
        (unsafe { libc::madvise(std::ptr::null_mut(), 0, self as libc::c_int) }) == 0
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "linux")]
    #[test]
    fn test_is_supported() {
        use super::*;

        assert!(Advice::Normal.is_supported());
        assert!(Advice::Random.is_supported());
        assert!(Advice::Sequential.is_supported());
        assert!(Advice::WillNeed.is_supported());

        assert!(UncheckedAdvice::DontNeed.is_supported());
    }
}
