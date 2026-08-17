/* ----------------------------------------------------------------------------
Copyright (c) 2018-2024, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
#pragma once
#ifndef MI_TYPES_H
#define MI_TYPES_H

// --------------------------------------------------------------------------
// This file contains the main type definitions for mimalloc:
// mi_heap_t      : all data for a thread-local heap, contains
//                  lists of all managed heap pages.
// mi_page_t      : a mimalloc page (usually 64KiB or 512KiB) from
//                  where objects of a single size are allocated.
//                  Note: we write "OS page" for OS memory pages while
//                  using plain "page" for mimalloc pages (`mi_page_t`).
// --------------------------------------------------------------------------


#include <mimalloc-stats.h>
#include <stddef.h>   // ptrdiff_t
#include <stdint.h>   // uintptr_t, uint16_t, etc
#include <limits.h>   // SIZE_MAX etc.
#include <errno.h>    // error codes
#include "bits.h"     // size defines (MI_INTPTR_SIZE etc), bit operations
#include "atomic.h"   // _Atomic primitives

// Minimal alignment necessary. On most platforms 16 bytes are needed
// due to SSE registers for example. This must be at least `sizeof(void*)`
#ifndef MI_MAX_ALIGN_SIZE
#define MI_MAX_ALIGN_SIZE  16   // sizeof(max_align_t)
#endif

// ------------------------------------------------------
// Variants
// ------------------------------------------------------

// Define NDEBUG in the release version to disable assertions.
// #define NDEBUG

// Define MI_TRACK_<tool> to enable tracking support
// #define MI_TRACK_VALGRIND 1
// #define MI_TRACK_ASAN     1
// #define MI_TRACK_ETW      1

// Define MI_STAT as 1 to maintain statistics; set it to 2 to have detailed statistics (but costs some performance).
// #define MI_STAT 1

// Define MI_SECURE to enable security mitigations. Level 1 has minimal performance impact,
// but protects most metadata with guard pages:
//   #define MI_SECURE 1  // guard page around metadata; check pointer validity on free
//
// Level 2 has more performance impact but protect well against various buffer overflows
// by surrounding all mimalloc pages with guard pages:
//   #define MI_SECURE 2  // guard page around each mimalloc page (can fragment VMA's with large heaps..)
//
// The next two levels can have more performance cost:
//   #define MI_SECURE 3  // randomize allocations, encode free lists (detect corrupted free list (buffer overflow), and invalid pointer free)
//   #define MI_SECURE 4  // checks for double free. (may be more expensive)

#if !defined(MI_SECURE)
#define MI_SECURE 0
#endif

// Define MI_DEBUG for debug mode
// #define MI_DEBUG 1  // basic assertion checks and statistics, check double free, corrupted free list, and invalid pointer free.
// #define MI_DEBUG 2  // + internal assertion checks
// #define MI_DEBUG 3  // + extensive internal invariant checking (cmake -DMI_DEBUG_FULL=ON)
#if !defined(MI_DEBUG)
#if defined(MI_BUILD_RELEASE) || defined(NDEBUG)
#define MI_DEBUG 0
#else
#define MI_DEBUG 2
#endif
#endif

// Statistics (0=only essential, 1=normal, 2=more fine-grained (expensive) tracking)
#ifndef MI_STAT
#if (MI_DEBUG>0)
#define MI_STAT 2
#else
#define MI_STAT 0
#endif
#endif

// Use guard pages behind objects of a certain size (set by the MIMALLOC_DEBUG_GUARDED_MIN/MAX options)
// Padding should be disabled when using guard pages
// #define MI_GUARDED 1
#if defined(MI_GUARDED)
#define MI_PADDING  0
#endif

// Reserve extra padding at the end of each block to be more resilient against heap block overflows.
// The padding can detect buffer overflow on free.
#if !defined(MI_PADDING) && (MI_SECURE>=3 || MI_DEBUG>=1 || (MI_TRACK_VALGRIND || MI_TRACK_ASAN || MI_TRACK_ETW))
#define MI_PADDING  1
#endif

// Check padding bytes; allows byte-precise buffer overflow detection
#if !defined(MI_PADDING_CHECK) && MI_PADDING && (MI_SECURE>=3 || MI_DEBUG>=1)
#define MI_PADDING_CHECK 1
#endif


// Encoded free lists allow detection of corrupted free lists
// and can detect buffer overflows, modify after free, and double `free`s.
#if (MI_SECURE>=3 || MI_DEBUG>=1)
#define MI_ENCODE_FREELIST  1
#endif

// Enable large pages for objects between 64KiB and 512KiB.
// Disabled by default as for many workloads the block sizes above 64 KiB are quite random which can lead to too many partially used large pages.
#ifndef MI_ENABLE_LARGE_PAGES
#define MI_ENABLE_LARGE_PAGES  0
#endif

// --------------------------------------------------------------
// Sizes of internal data-structures
// (comments specify sizes on 64-bit, usually 32-bit is halved)
// --------------------------------------------------------------

// Main size parameter; determines max arena sizes and max arena object sizes etc.
#ifndef MI_ARENA_SLICE_SHIFT
  #ifdef  MI_SMALL_PAGE_SHIFT   // backward compatibility
  #define MI_ARENA_SLICE_SHIFT              MI_SMALL_PAGE_SHIFT
  #else
  #define MI_ARENA_SLICE_SHIFT              (13 + MI_SIZE_SHIFT)        // 64 KiB (32 KiB on 32-bit)
  #endif
#endif
#if MI_ARENA_SLICE_SHIFT < 12
#error Arena slices should be at least 4KiB
#endif

#ifndef MI_BCHUNK_BITS_SHIFT
  #if MI_ARENA_SLICE_SHIFT <= 13    // <= 8KiB
  #define MI_BCHUNK_BITS_SHIFT              (7)   // 128 bits
  #elif MI_ARENA_SLICE_SHIFT < 16   // <= 32KiB
  #define MI_BCHUNK_BITS_SHIFT              (8)   // 256 bits
  #else 
  #define MI_BCHUNK_BITS_SHIFT              (6 + MI_SIZE_SHIFT)       // 512 bits (or 256 on 32-bit)
  #endif
#endif

#define MI_BCHUNK_BITS                    (1 << MI_BCHUNK_BITS_SHIFT)         // sub-bitmaps are "bchunks" of 512 bits
#define MI_ARENA_SLICE_SIZE               (MI_ZU(1) << MI_ARENA_SLICE_SHIFT)  // arena's allocate in slices of 64 KiB
#define MI_ARENA_SLICE_ALIGN              (MI_ARENA_SLICE_SIZE)

#define MI_ARENA_MIN_OBJ_SLICES           (1)
#define MI_ARENA_MAX_OBJ_SLICES           (MI_BCHUNK_BITS)            // 32 MiB (for now, cannot cross chunk boundaries)

#define MI_ARENA_MIN_OBJ_SIZE             (MI_ARENA_MIN_OBJ_SLICES * MI_ARENA_SLICE_SIZE)
#define MI_ARENA_MAX_OBJ_SIZE             (MI_ARENA_MAX_OBJ_SLICES * MI_ARENA_SLICE_SIZE)

#if MI_ARENA_MAX_OBJ_SIZE < MI_SIZE_SIZE*1024
#error maximum object size may be too small to hold local thread data  
#endif

#define MI_SMALL_PAGE_SIZE                MI_ARENA_MIN_OBJ_SIZE                    // 64 KiB
#define MI_MEDIUM_PAGE_SIZE               (8*MI_SMALL_PAGE_SIZE)                   // 512 KiB  (=byte in the bchunk bitmap)
#define MI_LARGE_PAGE_SIZE                (MI_SIZE_SIZE*MI_MEDIUM_PAGE_SIZE)       // 4 MiB    (=word in the bchunk bitmap)


// Maximum number of size classes. (spaced exponentially in 12.5% increments)
#if MI_BIN_HUGE != 73U
#error "mimalloc internal: expecting 73 bins"
#endif
#define MI_BIN_FULL  (MI_BIN_HUGE+1)
#define MI_BIN_COUNT (MI_BIN_FULL+1)

// We never allocate more than PTRDIFF_MAX (see also <https://sourceware.org/ml/libc-announce/2019/msg00001.html>)
#define MI_MAX_ALLOC_SIZE        PTRDIFF_MAX

// Minimal commit for a page on-demand commit (should be >= OS page size)
#define MI_PAGE_MIN_COMMIT_SIZE  MI_ARENA_SLICE_SIZE // (4*MI_KiB)


// ------------------------------------------------------
// Arena's are large reserved areas of memory allocated from
// the OS that are managed by mimalloc to efficiently
// allocate MI_ARENA_SLICE_SIZE slices of memory for the
// mimalloc pages.
// ------------------------------------------------------

// A large memory arena where pages are allocated in. 
typedef struct mi_arena_s mi_arena_t;     // defined below


// ---------------------------------------------------------------
// a memory id tracks the provenance of arena/OS allocated memory
// ---------------------------------------------------------------

// Memory can reside in arena's, direct OS allocated, meta-data pages, or statically allocated.
// The memid keeps track of this.
typedef enum mi_memkind_e {
  MI_MEM_NONE,      // not allocated
  MI_MEM_EXTERNAL,  // not owned by mimalloc but provided externally (via `mi_manage_os_memory` for example)
  MI_MEM_STATIC,    // allocated in a static area and should not be freed (the initial main heap data for example (`init.c`))
  MI_MEM_META,      // allocated with the meta data allocator (`arena-meta.c`)
  MI_MEM_OS,        // allocated from the OS
  MI_MEM_OS_HUGE,   // allocated as huge OS pages (usually 1GiB, pinned to physical memory)
  MI_MEM_OS_REMAP,  // allocated in a remapable area (i.e. using `mremap`)
  MI_MEM_ARENA      // allocated from an arena (the usual case) (`arena.c`)
} mi_memkind_t;

static inline bool mi_memkind_is_os(mi_memkind_t memkind) {
  return (memkind >= MI_MEM_OS && memkind <= MI_MEM_OS_REMAP);
}

static inline bool mi_memkind_needs_no_free(mi_memkind_t memkind) {
  return (memkind <= MI_MEM_STATIC);
}


typedef struct mi_memid_os_info {
  void*         base;               // actual base address of the block (used for offset aligned allocations)
  size_t        size;               // allocated full size
  // size_t        alignment;       // alignment at allocation
} mi_memid_os_info_t;

typedef struct mi_memid_arena_info {
  mi_arena_t*   arena;              // arena that contains this memory
  uint32_t      slice_index;        // slice index in the arena
  uint32_t      slice_count;        // allocated slices
} mi_memid_arena_info_t;

typedef struct mi_memid_meta_info {
  void*         meta_page;          // meta-page that contains the block
  uint32_t      block_index;        // block index in the meta-data page
  uint32_t      block_count;        // allocated blocks
} mi_memid_meta_info_t;

typedef struct mi_memid_s {
  union {
    mi_memid_os_info_t    os;       // only used for MI_MEM_OS
    mi_memid_arena_info_t arena;    // only used for MI_MEM_ARENA
    mi_memid_meta_info_t  meta;     // only used for MI_MEM_META
  } mem;
  mi_memkind_t  memkind;
  bool          is_pinned;          // `true` if we cannot decommit/reset/protect in this memory (e.g. when allocated using large (2Mib) or huge (1GiB) OS pages)
  bool          initially_committed;// `true` if the memory was originally allocated as committed
  bool          initially_zero;     // `true` if the memory was originally zero initialized
} mi_memid_t;


static inline bool mi_memid_is_os(mi_memid_t memid) {
  return mi_memkind_is_os(memid.memkind);
}

static inline bool mi_memid_needs_no_free(mi_memid_t memid) {
  return mi_memkind_needs_no_free(memid.memkind);
}

static inline mi_arena_t* mi_memid_arena(mi_memid_t memid) {
  return (memid.memkind == MI_MEM_ARENA ? memid.mem.arena.arena : NULL);
}


// ------------------------------------------------------
// Mimalloc pages contain allocated blocks
// ------------------------------------------------------

// The free lists use encoded next fields
// (Only actually encodes when MI_ENCODED_FREELIST is defined.)
typedef uintptr_t  mi_encoded_t;

// thread id's
typedef size_t     mi_threadid_t;

// free lists contain blocks
typedef struct mi_block_s {
  mi_encoded_t next;
} mi_block_t;


// The page flags are put in the bottom 2 bits of the thread_id (for a fast test in `mi_free`)
// `has_aligned` is true if the page has pointers at an offset in a block (so we unalign before free-ing)
// `in_full_queue` is true if the page is full and resides in the full queue (so we move it to a regular queue on free-ing)
#define MI_PAGE_IN_FULL_QUEUE         MI_ZU(0x01)
#define MI_PAGE_HAS_ALIGNED           MI_ZU(0x02)
#define MI_PAGE_FLAG_MASK             MI_ZU(0x03)
typedef size_t mi_page_flags_t;

// There are two special threadid's: 0 for abandoned threads, and 4 for abandoned & mapped threads --
// abandoned-mapped pages are abandoned but also mapped in an arena so can be quickly found for reuse.
#define MI_THREADID_ABANDONED         MI_ZU(0)
#define MI_THREADID_ABANDONED_MAPPED  (MI_PAGE_FLAG_MASK + 1)

// Thread free list.
// Points to a list of blocks that are freed by other threads.
// The least-bit is set if the page is owned by the current thread. (`mi_page_is_owned`).
// Ownership is required before we can read any non-atomic fields in the page.
// This way we can push a block on the thread free list and try to claim ownership atomically in `free.c:mi_free_block_mt`.
typedef uintptr_t mi_thread_free_t;

// A heap can serve only specific objects signified by its heap tag (e.g. various object types in CPython)
typedef uint8_t mi_heaptag_t;

// A page contains blocks of one specific size (`block_size`).
// Each page has three list of free blocks:
// `free` for blocks that can be allocated,
// `local_free` for freed blocks that are not yet available to `mi_malloc`
// `thread_free` for freed blocks by other threads
// The `local_free` and `thread_free` lists are migrated to the `free` list
// when it is exhausted. The separate `local_free` list is necessary to
// implement a monotonic heartbeat. The `thread_free` list is needed for
// avoiding atomic operations when allocating from the owning thread.
//
// `used - |thread_free|` == actual blocks that are in use (alive)
// `used - |thread_free| + |free| + |local_free| == capacity`
//
// We don't count "freed" (as |free|) but use only the `used` field to reduce
// the number of memory accesses in the `mi_page_all_free` function(s).
// Use `_mi_page_free_collect` to collect the thread_free list and update the `used` count.
//
// Notes:
// - Non-atomic fields can only be accessed if having _ownership_ (low bit of `xthread_free` is 1).
//   Combining the `thread_free` list with an ownership bit allows a concurrent `free` to atomically
//   free an object and (re)claim ownership if the page was abandoned.
// - If a page is not part of a heap it is called "abandoned"  (`heap==NULL`) -- in
//   that case the `xthreadid` is 0 or 4 (4 is for abandoned pages that
//   are in the abandoned page lists of an arena, these are called "mapped" abandoned pages).
// - page flags are in the bottom 3 bits of `xthread_id` for the fast path in `mi_free`.
// - The layout is optimized for `free.c:mi_free` and `alloc.c:mi_page_alloc`
// - Using `uint16_t` does not seem to slow things down

typedef struct mi_page_s {
  _Atomic(mi_threadid_t)    xthread_id;        // thread this page belongs to. (= `heap->thread_id (or 0 or 4 if abandoned) | page_flags`)

  mi_block_t*               free;              // list of available free blocks (`malloc` allocates from this list)
  uint16_t                  used;              // number of blocks in use (including blocks in `thread_free`)
  uint16_t                  capacity;          // number of blocks committed
  uint16_t                  reserved;          // number of blocks reserved in memory
  uint8_t                   block_size_shift;  // if not zero, then `(1 << block_size_shift) == block_size` (only used for fast path in `free.c:_mi_page_ptr_unalign`)
  uint8_t                   retire_expire;     // expiration count for retired blocks

  mi_block_t*               local_free;        // list of deferred free blocks by this thread (migrates to `free`)
  _Atomic(mi_thread_free_t) xthread_free;      // list of deferred free blocks freed by other threads (= `mi_block_t* | (1 if owned)`)

  size_t                    block_size;        // size available in each block (always `>0`)
  uint8_t*                  page_start;        // start of the blocks
  mi_heaptag_t              heap_tag;          // tag of the owning heap, used to separate heaps by object type
  bool                      free_is_zero;      // `true` if the blocks in the free list are zero initialized
                                               // padding
  #if (MI_ENCODE_FREELIST || MI_PADDING)
  uintptr_t                 keys[2];           // two random keys to encode the free lists (see `_mi_block_next`) or padding canary
  #endif

  mi_heap_t*                heap;              // the heap owning this page (or NULL for abandoned pages)
  struct mi_page_s*         next;              // next page owned by the heap with the same `block_size`
  struct mi_page_s*         prev;              // previous page owned by the heap with the same `block_size`
  size_t                    slice_committed;   // committed size relative to the first arena slice of the page data (or 0 if the page is fully committed already)
  mi_memid_t                memid;             // provenance of the page memory
} mi_page_t;


// ------------------------------------------------------
// Object sizes
// ------------------------------------------------------

#define MI_PAGE_ALIGN                     MI_ARENA_SLICE_ALIGN // pages must be aligned on this for the page map.
#define MI_PAGE_MIN_START_BLOCK_ALIGN     MI_MAX_ALIGN_SIZE    // minimal block alignment for the first block in a page (16b)
#define MI_PAGE_MAX_START_BLOCK_ALIGN2    MI_KiB               // maximal block alignment for "power of 2"-sized blocks (such that we guarantee natural alignment)
#define MI_PAGE_MAX_OVERALLOC_ALIGN       MI_ARENA_SLICE_SIZE  // (64 KiB) limit for which we overallocate in arena pages, beyond this use OS allocation

#if (MI_ENCODE_FREELIST || MI_PADDING) && MI_SIZE_SIZE == 8
#define MI_PAGE_INFO_SIZE                 ((MI_INTPTR_SHIFT+2)*32)  // 160    >= sizeof(mi_page_t)
#else
#define MI_PAGE_INFO_SIZE                 ((MI_INTPTR_SHIFT+1)*32)  // 128/96 >= sizeof(mi_page_t)
#endif

// The max object size are checked to not waste more than 12.5% internally over the page sizes.
#define MI_SMALL_MAX_OBJ_SIZE             ((MI_SMALL_PAGE_SIZE-MI_PAGE_INFO_SIZE)/8)   // < ~8 KiB
#if MI_ENABLE_LARGE_PAGES
#define MI_MEDIUM_MAX_OBJ_SIZE            ((MI_MEDIUM_PAGE_SIZE-MI_PAGE_INFO_SIZE)/8)  // < ~64 KiB
#define MI_LARGE_MAX_OBJ_SIZE             (MI_LARGE_PAGE_SIZE/8)    // <= 512KiB // note: this must be a nice power of 2 or we get rounding issues with `_mi_bin`
#else
#define MI_MEDIUM_MAX_OBJ_SIZE            (MI_MEDIUM_PAGE_SIZE/8)   // <= 64 KiB
#define MI_LARGE_MAX_OBJ_SIZE             MI_MEDIUM_MAX_OBJ_SIZE    // note: this must be a nice power of 2 or we get rounding issues with `_mi_bin`
#endif
#define MI_LARGE_MAX_OBJ_WSIZE            (MI_LARGE_MAX_OBJ_SIZE/MI_SIZE_SIZE)

#if (MI_LARGE_MAX_OBJ_WSIZE >= 655360)
#error "mimalloc internal: define more bins"
#endif


// ------------------------------------------------------
// Page kinds
// ------------------------------------------------------

typedef enum mi_page_kind_e {
  MI_PAGE_SMALL,    // small blocks go into 64KiB pages
  MI_PAGE_MEDIUM,   // medium blocks go into 512KiB pages
  MI_PAGE_LARGE,    // larger blocks go into 4MiB pages (if `MI_ENABLE_LARGE_PAGES==1`)
  MI_PAGE_SINGLETON // page containing a single block.
                    // used for blocks `> MI_LARGE_MAX_OBJ_SIZE` or an aligment `> MI_PAGE_MAX_OVERALLOC_ALIGN`.
} mi_page_kind_t;



// ------------------------------------------------------
// Heaps
//
// Provide first-class heaps to allocate from.
// A heap just owns a set of pages for allocation and
// can only be allocate/reallocate from the thread that created it.
// Freeing blocks can be done from any thread though.
//
// Per thread, there is always a default heap that is
// used for allocation; it is initialized to statically
// point to an empty heap to avoid initialization checks
// in the fast path.
// ------------------------------------------------------

// Thread local data
typedef struct mi_tld_s mi_tld_t;   // defined below

// Pages of a certain block size are held in a queue.
typedef struct mi_page_queue_s {
  mi_page_t* first;
  mi_page_t* last;
  size_t     count;
  size_t     block_size;
} mi_page_queue_t;

// Random context
typedef struct mi_random_cxt_s {
  uint32_t input[16];
  uint32_t output[16];
  int      output_available;
  bool     weak;
} mi_random_ctx_t;


// In debug mode there is a padding structure at the end of the blocks to check for buffer overflows
#if MI_PADDING
typedef struct mi_padding_s {
  uint32_t canary; // encoded block value to check validity of the padding (in case of overflow)
  uint32_t delta;  // padding bytes before the block. (mi_usable_size(p) - delta == exact allocated bytes)
} mi_padding_t;
#define MI_PADDING_SIZE   (sizeof(mi_padding_t))
#define MI_PADDING_WSIZE  ((MI_PADDING_SIZE + MI_INTPTR_SIZE - 1) / MI_INTPTR_SIZE)
#else
#define MI_PADDING_SIZE   0
#define MI_PADDING_WSIZE  0
#endif

#define MI_PAGES_DIRECT   (MI_SMALL_WSIZE_MAX + MI_PADDING_WSIZE + 1)


// A heap owns a set of pages.
struct mi_heap_s {
  mi_tld_t*             tld;                                 // thread-local data
  mi_arena_t*           exclusive_arena;                     // if the heap should only allocate from a specific arena (or NULL)
  int                   numa_node;                           // preferred numa node (or -1 for no preference)
  uintptr_t             cookie;                              // random cookie to verify pointers (see `_mi_ptr_cookie`)
  mi_random_ctx_t       random;                              // random number context used for secure allocation
  size_t                page_count;                          // total number of pages in the `pages` queues.
  size_t                page_retired_min;                    // smallest retired index (retired pages are fully free, but still in the page queues)
  size_t                page_retired_max;                    // largest retired index into the `pages` array.
  long                  generic_count;                       // how often is `_mi_malloc_generic` called?
  long                  generic_collect_count;               // how often is `_mi_malloc_generic` called without collecting?
  mi_heap_t*            next;                                // list of heaps per thread
  long                  page_full_retain;                    // how many full pages can be retained per queue (before abondoning them)
  bool                  allow_page_reclaim;                  // `true` if this heap should not reclaim abandoned pages
  bool                  allow_page_abandon;                  // `true` if this heap can abandon pages to reduce memory footprint
  uint8_t               tag;                                 // custom tag, can be used for separating heaps based on the object types
  #if MI_GUARDED
  size_t                guarded_size_min;                    // minimal size for guarded objects
  size_t                guarded_size_max;                    // maximal size for guarded objects
  size_t                guarded_sample_rate;                 // sample rate (set to 0 to disable guarded pages)
  size_t                guarded_sample_count;                // current sample count (counting down to 0)
  #endif
  mi_page_t*            pages_free_direct[MI_PAGES_DIRECT];  // optimize: array where every entry points a page with possibly free blocks in the corresponding queue for that size.
  mi_page_queue_t       pages[MI_BIN_COUNT];                 // queue of pages for each size class (or "bin")
  mi_memid_t            memid;                               // provenance of the heap struct itself (meta or os)
};


// ------------------------------------------------------
// Sub processes do not reclaim or visit pages from other sub processes. 
// These are essentially the static variables of a process, and
// usually there is only one subprocess. This can be used for example
// by CPython to have seperate interpreters within one process.
// Each thread can only belong to one subprocess.
// ------------------------------------------------------

#define MI_MAX_ARENAS   (160)   // Limited for now (and takes up .bss).. but arena's scale up exponentially (see `mi_arena_reserve`)
                                // 160 arenas is enough for ~2 TiB memory

typedef struct mi_subproc_s {
  _Atomic(size_t)       arena_count;                    // current count of arena's
  _Atomic(mi_arena_t*)  arenas[MI_MAX_ARENAS];          // arena's of this sub-process
  mi_lock_t             arena_reserve_lock;             // lock to ensure arena's get reserved one at a time
  _Atomic(int64_t)      purge_expire;                   // expiration is set if any arenas can be purged

  _Atomic(size_t)       abandoned_count[MI_BIN_COUNT];  // total count of abandoned pages for this sub-process
  mi_page_t*            os_abandoned_pages;             // list of pages that OS allocated and not in an arena (only used if `mi_option_visit_abandoned` is on)
  mi_lock_t             os_abandoned_pages_lock;        // lock for the os abandoned pages list (this lock protects list operations)

  mi_memid_t            memid;                          // provenance of this memory block (meta or OS)
  mi_stats_t            stats;                          // sub-process statistics (tld stats are merged in on thread termination)
} mi_subproc_t;



// ------------------------------------------------------
// Thread Local data
// ------------------------------------------------------

// Milliseconds as in `int64_t` to avoid overflows
typedef int64_t  mi_msecs_t;

// Thread local data
struct mi_tld_s {
  mi_threadid_t         thread_id;            // thread id of this thread
  size_t                thread_seq;           // thread sequence id (linear count of created threads)
  int                   numa_node;            // thread preferred numa node
  mi_subproc_t*         subproc;              // sub-process this thread belongs to.
  mi_heap_t*            heap_backing;         // backing heap of this thread (cannot be deleted)
  mi_heap_t*            heaps;                // list of heaps in this thread (so we can abandon all when the thread terminates)
  unsigned long long    heartbeat;            // monotonic heartbeat count
  bool                  recurse;              // true if deferred was called; used to prevent infinite recursion.
  bool                  is_in_threadpool;     // true if this thread is part of a threadpool (and can run arbitrary tasks)
  mi_stats_t            stats;                // statistics
  mi_memid_t            memid;                // provenance of the tld memory itself (meta or OS)
};


/* ----------------------------------------------------------------------------
  Arenas are fixed area's of OS memory from which we can allocate
  large blocks (>= MI_ARENA_MIN_BLOCK_SIZE).
  In contrast to the rest of mimalloc, the arenas are shared between
  threads and need to be accessed using atomic operations (using atomic `mi_bitmap_t`'s).

  Arenas are also used to for huge OS page (1GiB) reservations or for reserving
  OS memory upfront which can be improve performance or is sometimes needed
  on embedded devices. We can also employ this with WASI or `sbrk` systems
  to reserve large arenas upfront and be able to reuse the memory more effectively.
-----------------------------------------------------------------------------*/

#define MI_ARENA_BIN_COUNT      (MI_BIN_COUNT)
#define MI_ARENA_MIN_SIZE       (MI_BCHUNK_BITS * MI_ARENA_SLICE_SIZE)           // 32 MiB (or 8 MiB on 32-bit)
#define MI_ARENA_MAX_SIZE       (MI_BITMAP_MAX_BIT_COUNT * MI_ARENA_SLICE_SIZE)

typedef struct mi_bitmap_s  mi_bitmap_t;    // atomic bitmap  (defined in `src/bitmap.h`)
typedef struct mi_bbitmap_s mi_bbitmap_t;   // atomic binned bitmap (defined in `src/bitmap.h`)

// A memory arena
typedef struct mi_arena_s {
  mi_memid_t          memid;                // provenance of the memory area
  mi_subproc_t*       subproc;              // subprocess this arena belongs to (`this 'element-of' this->subproc->arenas`)

  size_t              slice_count;          // total size of the area in arena slices (of `MI_ARENA_SLICE_SIZE`)
  size_t              info_slices;          // initial slices reserved for the arena bitmaps
  int                 numa_node;            // associated NUMA node
  bool                is_exclusive;         // only allow allocations if specifically for this arena
  _Atomic(mi_msecs_t) purge_expire;         // expiration time when slices can be purged from `slices_purge`.
  mi_commit_fun_t*    commit_fun;           // custom commit/decommit memory
  void*               commit_fun_arg;       // user argument for a custom commit function

  mi_bbitmap_t*       slices_free;          // is the slice free? (a binned bitmap with size classes)
  mi_bitmap_t*        slices_committed;     // is the slice committed? (i.e. accessible)
  mi_bitmap_t*        slices_dirty;         // is the slice potentially non-zero?
  mi_bitmap_t*        slices_purge;         // slices that can be purged
  mi_bitmap_t*        pages;                // all registered pages (abandoned and owned)
  mi_bitmap_t*        pages_abandoned[MI_ARENA_BIN_COUNT];  // abandoned pages per size bin (a set bit means the start of the page)
                                            // the full queue contains abandoned full pages
  // followed by the bitmaps (whose sizes depend on the arena size)
  // note: when adding bitmaps revise `mi_arena_info_slices_needed`
} mi_arena_t;


/* -----------------------------------------------------------
  Error codes passed to `_mi_fatal_error`
  All are recoverable but EFAULT is a serious error and aborts by default in secure mode.
  For portability define undefined error codes using common Unix codes:
  <https://www-numi.fnal.gov/offline_software/srt_public_context/WebDocs/Errors/unix_system_errors.html>
----------------------------------------------------------- */

#ifndef EAGAIN         // double free
#define EAGAIN (11)
#endif
#ifndef ENOMEM         // out of memory
#define ENOMEM (12)
#endif
#ifndef EFAULT         // corrupted free-list or meta-data
#define EFAULT (14)
#endif
#ifndef EINVAL         // trying to free an invalid pointer
#define EINVAL (22)
#endif
#ifndef EOVERFLOW      // count*size overflow
#define EOVERFLOW (75)
#endif

/* -----------------------------------------------------------
  Debug constants
----------------------------------------------------------- */

#if !defined(MI_DEBUG_UNINIT)
#define MI_DEBUG_UNINIT     (0xD0)
#endif
#if !defined(MI_DEBUG_FREED)
#define MI_DEBUG_FREED      (0xDF)
#endif
#if !defined(MI_DEBUG_PADDING)
#define MI_DEBUG_PADDING    (0xDE)
#endif


#endif // MI_TYPES_H
