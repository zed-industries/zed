use super::{expect, from_str, ProcResult};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io};

fn convert_to_kibibytes(num: u64, unit: &str) -> ProcResult<u64> {
    match unit {
        "B" => Ok(num),
        "KiB" | "kiB" | "kB" | "KB" => Ok(num * 1024),
        "MiB" | "miB" | "MB" | "mB" => Ok(num * 1024 * 1024),
        "GiB" | "giB" | "GB" | "gB" => Ok(num * 1024 * 1024 * 1024),
        unknown => Err(build_internal_error!(format!("Unknown unit type {}", unknown))),
    }
}

/// This  struct  reports  statistics about memory usage on the system, based on
/// the `/proc/meminfo` file.
///
/// It is used by `free(1)` to report the amount of free and used memory (both
/// physical  and  swap)  on  the  system  as well as the shared memory and
/// buffers used by the kernel.  Each struct member is generally reported in
/// bytes, but a few are unitless values.
///
/// Except as noted below, all of the fields have been present since at least
/// Linux 2.6.0.  Some fields are optional and are present only if the kernel
/// was configured with various options; those dependencies are noted in the list.
///
/// **Notes**
///
/// While the file shows kilobytes (kB; 1 kB equals 1000 B),
/// it is actually kibibytes (KiB; 1 KiB equals 1024 B).
///
/// All sizes are converted to bytes. Unitless values, like `hugepages_total` are not affected.
///
/// This imprecision in /proc/meminfo is known,
/// but is not corrected due to legacy concerns -
/// programs rely on /proc/meminfo to specify size with the "kB" string.
///
/// New fields to this struct may be added at any time (even without a major or minor semver bump).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[allow(non_snake_case)]
#[non_exhaustive]
pub struct Meminfo {
    /// Total usable RAM (i.e., physical RAM minus a few reserved bits and the kernel binary code).
    pub mem_total: u64,
    /// The sum of [LowFree](#structfield.low_free) + [HighFree](#structfield.high_free).
    pub mem_free: u64,
    /// An estimate of how much memory is available for starting new applications, without swapping.
    ///
    /// (since Linux 3.14)
    pub mem_available: Option<u64>,
    /// Relatively temporary storage for raw disk blocks that shouldn't get tremendously large (20MB or so).
    pub buffers: u64,
    /// In-memory cache for files read from the disk (the page cache).  Doesn't include SwapCached.
    pub cached: u64,
    /// Memory  that  once  was  swapped  out, is swapped back in but still also is in the swap
    /// file.
    ///
    /// (If memory pressure is high, these pages don't need to be swapped out again
    /// because they are already in the swap file.  This saves I/O.)
    pub swap_cached: u64,
    /// Memory that has been used more recently and usually not reclaimed unless absolutely
    /// necessary.
    pub active: u64,
    /// Memory which has been less recently used.  It is more eligible to be reclaimed for other
    /// purposes.
    pub inactive: u64,
    /// [To be documented.]
    ///
    /// (since Linux 2.6.28)
    pub active_anon: Option<u64>,
    /// [To be documented.]
    ///
    /// (since Linux 2.6.28)
    pub inactive_anon: Option<u64>,
    /// [To be documented.]
    ///
    /// (since Linux 2.6.28)
    pub active_file: Option<u64>,
    /// [To be documented.]
    ///
    /// (since Linux 2.6.28)
    pub inactive_file: Option<u64>,
    /// [To be documented.]
    ///
    /// (From Linux 2.6.28 to 2.6.30, CONFIG_UNEVICTABLE_LRU was required.)
    pub unevictable: Option<u64>,
    /// [To be documented.]
    ///
    /// (From Linux 2.6.28 to 2.6.30, CONFIG_UNEVICTABLE_LRU was required.)
    pub mlocked: Option<u64>,
    /// Total amount of highmem.
    ///
    /// Highmem is all memory above ~860MB of physical  memory.  Highmem areas are for use by
    /// user-space programs, or for the page cache.  The kernel must use tricks to access this
    /// memory, making it slower to access than lowmem.
    ///
    /// (Starting with Linux 2.6.19, CONFIG_HIGHMEM is required.)
    pub high_total: Option<u64>,
    /// Amount of free highmem.
    ///
    /// (Starting with Linux 2.6.19, CONFIG_HIGHMEM is required.)
    pub high_free: Option<u64>,
    /// Total amount of lowmem.
    ///
    /// Lowmem is memory which can be used for every thing  that highmem can be used for,
    /// but it is also available for the kernel's use for its own data structures.
    /// Among many other things, it is where everything from Slab is allocated.
    /// Bad things happen when you're out of lowmem.
    ///
    /// (Starting with Linux 2.6.19, CONFIG_HIGHMEM is required.)
    pub low_total: Option<u64>,
    /// Amount of free lowmem.
    ///
    /// (Starting with Linux 2.6.19, CONFIG_HIGHMEM is required.)
    pub low_free: Option<u64>,
    /// [To be documented.]
    ///
    /// (since Linux 2.6.29.  CONFIG_MMU is required.)
    pub mmap_copy: Option<u64>,
    /// Total amount of swap space available.
    pub swap_total: u64,
    /// Amount of swap space that is currently unused.
    pub swap_free: u64,
    /// Memory which is waiting to get written back to the disk.
    pub dirty: u64,
    /// Memory which is actively being written back to the disk.
    pub writeback: u64,
    /// Non-file backed pages mapped into user-space page tables.
    ///
    /// (since Linux 2.6.18)
    pub anon_pages: Option<u64>,
    /// Files which have been mapped into memory (with mmap(2)), such as libraries.
    pub mapped: u64,
    /// Amount of memory consumed in tmpfs(5) filesystems.
    ///
    /// (since Linux 2.6.32)
    pub shmem: Option<u64>,
    /// In-kernel data structures cache.
    pub slab: u64,
    /// Part of Slab, that can be reclaimed on memory pressure.
    ///
    /// (since Linux 2.6.19)
    pub s_reclaimable: Option<u64>,
    /// Part of Slab, that cannot be reclaimed on memory pressure.
    ///
    /// (since Linux 2.6.19)
    pub s_unreclaim: Option<u64>,
    /// Amount of memory allocated to kernel stacks.
    ///
    /// (since Linux 2.6.32)
    pub kernel_stack: Option<u64>,
    /// Amount of memory dedicated to the lowest level of page tables.
    ///
    /// (since Linux 2.6.18)
    pub page_tables: Option<u64>,
    /// Amount of memory allocated for seconary page tables. This currently includes KVM mmu
    /// allocations on x86 and arm64.
    ///
    /// (since Linux 6.1)
    pub secondary_page_tables: Option<u64>,
    /// [To be documented.]
    ///
    /// (CONFIG_QUICKLIST is required.  Since Linux 2.6.27)
    pub quicklists: Option<u64>,
    /// NFS pages sent to the server, but not yet committed to stable storage.
    ///
    /// (since Linux 2.6.18)
    pub nfs_unstable: Option<u64>,
    /// Memory used for block device "bounce buffers".
    ///
    /// (since Linux 2.6.18)
    pub bounce: Option<u64>,
    /// Memory used by FUSE for temporary writeback buffers.
    ///
    /// (since Linux 2.6.26)
    pub writeback_tmp: Option<u64>,
    /// This is the total amount of memory currently available to be allocated on the system,
    /// expressed  in bytes.
    ///
    /// This  limit  is adhered  to  only if strict overcommit
    /// accounting is enabled (mode 2 in /proc/sys/vm/overcommit_memory).  The limit is calculated
    /// according to the formula described under /proc/sys/vm/overcommit_memory.  For further
    /// details, see the kernel source  file
    /// [Documentation/vm/overcommit-accounting](https://www.kernel.org/doc/Documentation/vm/overcommit-accounting).
    ///
    /// (since Linux 2.6.10)
    pub commit_limit: Option<u64>,
    /// The  amount of memory presently allocated on the system.
    ///
    /// The committed memory is a sum of all of the memory which has been allocated
    /// by processes, even if it has not been "used" by them as of yet.  A process which allocates 1GB of memory  (using  malloc(3)
    /// or  similar),  but  touches only 300MB of that memory will show up as using only 300MB of memory even if it has the address space
    /// allocated for the entire 1GB.
    ///
    /// This 1GB is memory which has been "committed" to by the VM and can be used at any  time  by  the  allocating  application.   With
    /// strict  overcommit  enabled  on  the  system  (mode 2 in /proc/sys/vm/overcommit_memory), allocations which would exceed the Committed_AS
    /// mitLimit will not be permitted.  This is useful if one needs to guarantee that processes will not fail due to lack of memory once
    /// that memory has been successfully allocated.
    pub committed_as: u64,
    /// Total size of vmalloc memory area.
    pub vmalloc_total: u64,
    /// Amount of vmalloc area which is used.
    pub vmalloc_used: u64,
    /// Largest contiguous block of vmalloc area which is free.
    pub vmalloc_chunk: u64,
    /// [To be documented.]
    ///
    /// (CONFIG_MEMORY_FAILURE is required.  Since Linux 2.6.32)
    pub hardware_corrupted: Option<u64>,
    /// Non-file backed huge pages mapped into user-space page tables.
    ///
    /// (CONFIG_TRANSPARENT_HUGEPAGE is required.  Since Linux 2.6.38)
    pub anon_hugepages: Option<u64>,
    /// Memory used by shared memory (shmem) and tmpfs(5) allocated with huge pages
    ///
    /// (CONFIG_TRANSPARENT_HUGEPAGE is required.  Since Linux 4.8)
    pub shmem_hugepages: Option<u64>,
    /// Shared memory mapped into user space with huge pages.
    ///
    /// (CONFIG_TRANSPARENT_HUGEPAGE is required.  Since Linux 4.8)
    pub shmem_pmd_mapped: Option<u64>,
    /// Total CMA (Contiguous Memory Allocator) pages.
    ///
    /// (CONFIG_CMA is required.  Since Linux 3.1)
    pub cma_total: Option<u64>,
    /// Free CMA (Contiguous Memory Allocator) pages.
    ///
    /// (CONFIG_CMA is required.  Since Linux 3.1)
    pub cma_free: Option<u64>,
    /// The size of the pool of huge pages.
    ///
    /// CONFIG_HUGETLB_PAGE is required.)
    pub hugepages_total: Option<u64>,
    /// The number of huge pages in the pool that are not yet allocated.
    ///
    /// (CONFIG_HUGETLB_PAGE is required.)
    pub hugepages_free: Option<u64>,
    /// This is the number of huge pages for which a commitment to allocate from the pool has been
    /// made, but no allocation has yet been made.
    ///
    /// These reserved huge pages guarantee that an application will be able  to  allocate  a
    /// huge page from the pool of huge pages at fault time.
    ///
    /// (CONFIG_HUGETLB_PAGE  is  required.  Since Linux 2.6.17)
    pub hugepages_rsvd: Option<u64>,
    /// This is the number of huge pages in the pool above the value in /proc/sys/vm/nr_hugepages.
    ///
    /// The maximum number of surplus huge pages is controlled by /proc/sys/vm/nr_overcommit_hugepages.
    ///
    /// (CONFIG_HUGETLB_PAGE  is  required.  Since Linux 2.6.24)
    pub hugepages_surp: Option<u64>,
    /// The size of huge pages.
    ///
    /// (CONFIG_HUGETLB_PAGE is required.)
    pub hugepagesize: Option<u64>,
    /// Number of bytes of RAM linearly mapped by kernel in 4kB pages.  (x86.)
    ///
    /// (since Linux 2.6.27)
    pub direct_map_4k: Option<u64>,
    /// Number of bytes of RAM linearly mapped by kernel in 4MB pages.
    ///
    /// (x86 with CONFIG_X86_64 or CONFIG_X86_PAE enabled.  Since Linux 2.6.27)
    pub direct_map_4M: Option<u64>,
    /// Number of bytes of RAM linearly mapped by kernel in 2MB pages.
    ///
    /// (x86 with neither CONFIG_X86_64 nor CONFIG_X86_PAE enabled.  Since Linux 2.6.27)
    pub direct_map_2M: Option<u64>,
    /// (x86 with CONFIG_X86_64 and CONFIG_X86_DIRECT_GBPAGES enabled.  Since Linux 2.6.27)
    pub direct_map_1G: Option<u64>,

    /// needs documentation
    pub hugetlb: Option<u64>,

    /// Memory allocated to the per-cpu alloctor used to back per-cpu allocations.
    ///
    /// This stat excludes the cost of metadata.
    pub per_cpu: Option<u64>,

    /// Kernel allocations that the kernel will attempt to reclaim under memory pressure.
    ///
    /// Includes s_reclaimable, and other direct allocations with a shrinker.
    pub k_reclaimable: Option<u64>,

    /// Undocumented field
    ///
    /// (CONFIG_TRANSPARENT_HUGEPAGE is requried.  Since Linux 5.4)
    pub file_pmd_mapped: Option<u64>,

    /// Undocumented field
    ///
    /// (CONFIG_TRANSPARENT_HUGEPAGE is required.  Since Linux 5.4)
    pub file_huge_pages: Option<u64>,

    /// Memory consumed by the zswap backend (compressed size).
    ///
    /// (CONFIG_ZSWAP is required.  Since Linux 5.19)
    pub z_swap: Option<u64>,

    /// Amount of anonymous memory stored in zswap (original size).
    ///
    /// (CONFIG_ZSWAP is required.  Since Linux 5.19)
    pub z_swapped: Option<u64>,
}

impl super::FromBufRead for Meminfo {
    fn from_buf_read<R: io::BufRead>(r: R) -> ProcResult<Self> {
        let mut map = HashMap::new();

        for line in r.lines() {
            let line = expect!(line);
            if line.is_empty() {
                continue;
            }
            let mut s = line.split_whitespace();
            let field = expect!(s.next(), "no field");
            let value = expect!(s.next(), "no value");
            let unit = s.next(); // optional

            let value = from_str!(u64, value);

            let value = if let Some(unit) = unit {
                convert_to_kibibytes(value, unit)?
            } else {
                value
            };

            map.insert(field[..field.len() - 1].to_string(), value);
        }

        // use 'remove' to move the value out of the hashmap
        // if there's anything still left in the map at the end, that
        // means we probably have a bug/typo, or are out-of-date
        let meminfo = Meminfo {
            mem_total: expect!(map.remove("MemTotal")),
            mem_free: expect!(map.remove("MemFree")),
            mem_available: map.remove("MemAvailable"),
            buffers: expect!(map.remove("Buffers")),
            cached: expect!(map.remove("Cached")),
            swap_cached: expect!(map.remove("SwapCached")),
            active: expect!(map.remove("Active")),
            inactive: expect!(map.remove("Inactive")),
            active_anon: map.remove("Active(anon)"),
            inactive_anon: map.remove("Inactive(anon)"),
            active_file: map.remove("Active(file)"),
            inactive_file: map.remove("Inactive(file)"),
            unevictable: map.remove("Unevictable"),
            mlocked: map.remove("Mlocked"),
            high_total: map.remove("HighTotal"),
            high_free: map.remove("HighFree"),
            low_total: map.remove("LowTotal"),
            low_free: map.remove("LowFree"),
            mmap_copy: map.remove("MmapCopy"),
            swap_total: expect!(map.remove("SwapTotal")),
            swap_free: expect!(map.remove("SwapFree")),
            dirty: expect!(map.remove("Dirty")),
            writeback: expect!(map.remove("Writeback")),
            anon_pages: map.remove("AnonPages"),
            mapped: expect!(map.remove("Mapped")),
            shmem: map.remove("Shmem"),
            slab: expect!(map.remove("Slab")),
            s_reclaimable: map.remove("SReclaimable"),
            s_unreclaim: map.remove("SUnreclaim"),
            kernel_stack: map.remove("KernelStack"),
            page_tables: map.remove("PageTables"),
            secondary_page_tables: map.remove("SecPageTables"),
            quicklists: map.remove("Quicklists"),
            nfs_unstable: map.remove("NFS_Unstable"),
            bounce: map.remove("Bounce"),
            writeback_tmp: map.remove("WritebackTmp"),
            commit_limit: map.remove("CommitLimit"),
            committed_as: expect!(map.remove("Committed_AS")),
            vmalloc_total: expect!(map.remove("VmallocTotal")),
            vmalloc_used: expect!(map.remove("VmallocUsed")),
            vmalloc_chunk: expect!(map.remove("VmallocChunk")),
            hardware_corrupted: map.remove("HardwareCorrupted"),
            anon_hugepages: map.remove("AnonHugePages"),
            shmem_hugepages: map.remove("ShmemHugePages"),
            shmem_pmd_mapped: map.remove("ShmemPmdMapped"),
            cma_total: map.remove("CmaTotal"),
            cma_free: map.remove("CmaFree"),
            hugepages_total: map.remove("HugePages_Total"),
            hugepages_free: map.remove("HugePages_Free"),
            hugepages_rsvd: map.remove("HugePages_Rsvd"),
            hugepages_surp: map.remove("HugePages_Surp"),
            hugepagesize: map.remove("Hugepagesize"),
            direct_map_4k: map.remove("DirectMap4k"),
            direct_map_4M: map.remove("DirectMap4M"),
            direct_map_2M: map.remove("DirectMap2M"),
            direct_map_1G: map.remove("DirectMap1G"),
            k_reclaimable: map.remove("KReclaimable"),
            per_cpu: map.remove("Percpu"),
            hugetlb: map.remove("Hugetlb"),
            file_pmd_mapped: map.remove("FilePmdMapped"),
            file_huge_pages: map.remove("FileHugePages"),
            z_swap: map.remove("Zswap"),
            z_swapped: map.remove("Zswapped"),
        };

        if cfg!(test) {
            assert!(map.is_empty(), "meminfo map is not empty: {:#?}", map);
        }

        Ok(meminfo)
    }
}
