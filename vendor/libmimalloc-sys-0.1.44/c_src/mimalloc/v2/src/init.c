/* ----------------------------------------------------------------------------
Copyright (c) 2018-2022, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
#include "mimalloc.h"
#include "mimalloc/internal.h"
#include "mimalloc/prim.h"

#include <string.h>  // memcpy, memset
#include <stdlib.h>  // atexit


// Empty page used to initialize the small free pages array
const mi_page_t _mi_page_empty = {
  0,
  false, false, false, false,
  0,       // capacity
  0,       // reserved capacity
  { 0 },   // flags
  false,   // is_zero
  0,       // retire_expire
  NULL,    // free
  NULL,    // local_free
  0,       // used
  0,       // block size shift
  0,       // heap tag
  0,       // block_size
  NULL,    // page_start
  #if (MI_PADDING || MI_ENCODE_FREELIST)
  { 0, 0 },
  #endif
  MI_ATOMIC_VAR_INIT(0), // xthread_free
  MI_ATOMIC_VAR_INIT(0), // xheap
  NULL, NULL
  , { 0 }  // padding
};

#define MI_PAGE_EMPTY() ((mi_page_t*)&_mi_page_empty)

#if (MI_SMALL_WSIZE_MAX==128)
#if (MI_PADDING>0) && (MI_INTPTR_SIZE >= 8)
#define MI_SMALL_PAGES_EMPTY  { MI_INIT128(MI_PAGE_EMPTY), MI_PAGE_EMPTY(), MI_PAGE_EMPTY() }
#elif (MI_PADDING>0)
#define MI_SMALL_PAGES_EMPTY  { MI_INIT128(MI_PAGE_EMPTY), MI_PAGE_EMPTY(), MI_PAGE_EMPTY(), MI_PAGE_EMPTY() }
#else
#define MI_SMALL_PAGES_EMPTY  { MI_INIT128(MI_PAGE_EMPTY), MI_PAGE_EMPTY() }
#endif
#else
#error "define right initialization sizes corresponding to MI_SMALL_WSIZE_MAX"
#endif

// Empty page queues for every bin
#define QNULL(sz)  { NULL, NULL, (sz)*sizeof(uintptr_t) }
#define MI_PAGE_QUEUES_EMPTY \
  { QNULL(1), \
    QNULL(     1), QNULL(     2), QNULL(     3), QNULL(     4), QNULL(     5), QNULL(     6), QNULL(     7), QNULL(     8), /* 8 */ \
    QNULL(    10), QNULL(    12), QNULL(    14), QNULL(    16), QNULL(    20), QNULL(    24), QNULL(    28), QNULL(    32), /* 16 */ \
    QNULL(    40), QNULL(    48), QNULL(    56), QNULL(    64), QNULL(    80), QNULL(    96), QNULL(   112), QNULL(   128), /* 24 */ \
    QNULL(   160), QNULL(   192), QNULL(   224), QNULL(   256), QNULL(   320), QNULL(   384), QNULL(   448), QNULL(   512), /* 32 */ \
    QNULL(   640), QNULL(   768), QNULL(   896), QNULL(  1024), QNULL(  1280), QNULL(  1536), QNULL(  1792), QNULL(  2048), /* 40 */ \
    QNULL(  2560), QNULL(  3072), QNULL(  3584), QNULL(  4096), QNULL(  5120), QNULL(  6144), QNULL(  7168), QNULL(  8192), /* 48 */ \
    QNULL( 10240), QNULL( 12288), QNULL( 14336), QNULL( 16384), QNULL( 20480), QNULL( 24576), QNULL( 28672), QNULL( 32768), /* 56 */ \
    QNULL( 40960), QNULL( 49152), QNULL( 57344), QNULL( 65536), QNULL( 81920), QNULL( 98304), QNULL(114688), QNULL(131072), /* 64 */ \
    QNULL(163840), QNULL(196608), QNULL(229376), QNULL(262144), QNULL(327680), QNULL(393216), QNULL(458752), QNULL(524288), /* 72 */ \
    QNULL(MI_MEDIUM_OBJ_WSIZE_MAX + 1  /* 655360, Huge queue */), \
    QNULL(MI_MEDIUM_OBJ_WSIZE_MAX + 2) /* Full queue */ }

#define MI_STAT_COUNT_NULL()  {0,0,0}

// Empty statistics
#define MI_STATS_NULL  \
  MI_STAT_COUNT_NULL(), MI_STAT_COUNT_NULL(), MI_STAT_COUNT_NULL(), MI_STAT_COUNT_NULL(), \
  MI_STAT_COUNT_NULL(), MI_STAT_COUNT_NULL(), MI_STAT_COUNT_NULL(), MI_STAT_COUNT_NULL(), \
  MI_STAT_COUNT_NULL(), MI_STAT_COUNT_NULL(), MI_STAT_COUNT_NULL(), \
  { 0 }, { 0 }, { 0 }, { 0 }, \
  { 0 }, { 0 }, { 0 }, { 0 }, \
  \
  { 0 }, { 0 }, { 0 }, { 0 }, { 0 }, \
  MI_INIT4(MI_STAT_COUNT_NULL), \
  { 0 }, { 0 }, { 0 }, { 0 },  \
  \
  { MI_INIT4(MI_STAT_COUNT_NULL) }, \
  { { 0 }, { 0 }, { 0 }, { 0 } }, \
  \
  { MI_INIT74(MI_STAT_COUNT_NULL) }, \
  { MI_INIT74(MI_STAT_COUNT_NULL) }


// Empty slice span queues for every bin
#define SQNULL(sz)  { NULL, NULL, sz }
#define MI_SEGMENT_SPAN_QUEUES_EMPTY \
  { SQNULL(1), \
    SQNULL(     1), SQNULL(     2), SQNULL(     3), SQNULL(     4), SQNULL(     5), SQNULL(     6), SQNULL(     7), SQNULL(    10), /*  8 */ \
    SQNULL(    12), SQNULL(    14), SQNULL(    16), SQNULL(    20), SQNULL(    24), SQNULL(    28), SQNULL(    32), SQNULL(    40), /* 16 */ \
    SQNULL(    48), SQNULL(    56), SQNULL(    64), SQNULL(    80), SQNULL(    96), SQNULL(   112), SQNULL(   128), SQNULL(   160), /* 24 */ \
    SQNULL(   192), SQNULL(   224), SQNULL(   256), SQNULL(   320), SQNULL(   384), SQNULL(   448), SQNULL(   512), SQNULL(   640), /* 32 */ \
    SQNULL(   768), SQNULL(   896), SQNULL(  1024) /* 35 */ }


// --------------------------------------------------------
// Statically allocate an empty heap as the initial
// thread local value for the default heap,
// and statically allocate the backing heap for the main
// thread so it can function without doing any allocation
// itself (as accessing a thread local for the first time
// may lead to allocation itself on some platforms)
// --------------------------------------------------------

mi_decl_cache_align const mi_heap_t _mi_heap_empty = {
  NULL,
  MI_ATOMIC_VAR_INIT(NULL),
  0,                // tid
  0,                // cookie
  0,                // arena id
  { 0, 0 },         // keys
  { {0}, {0}, 0, true }, // random
  0,                // page count
  MI_BIN_FULL, 0,   // page retired min/max
  0, 0,             // generic count
  NULL,             // next
  false,            // can reclaim
  0,                // tag
  #if MI_GUARDED
  0, 0, 0, 1,       // count is 1 so we never write to it (see `internal.h:mi_heap_malloc_use_guarded`)
  #endif
  MI_SMALL_PAGES_EMPTY,
  MI_PAGE_QUEUES_EMPTY
};

static mi_decl_cache_align mi_subproc_t mi_subproc_default;

#define tld_empty_stats  ((mi_stats_t*)((uint8_t*)&tld_empty + offsetof(mi_tld_t,stats)))

mi_decl_cache_align static const mi_tld_t tld_empty = {
  0,
  false,
  NULL, NULL,
  { MI_SEGMENT_SPAN_QUEUES_EMPTY, 0, 0, 0, 0, 0, &mi_subproc_default, tld_empty_stats }, // segments
  { MI_STAT_VERSION, MI_STATS_NULL }       // stats
};

mi_threadid_t _mi_thread_id(void) mi_attr_noexcept {
  return _mi_prim_thread_id();
}

// the thread-local default heap for allocation
mi_decl_thread mi_heap_t* _mi_heap_default = (mi_heap_t*)&_mi_heap_empty;

extern mi_decl_hidden mi_heap_t _mi_heap_main;

static mi_decl_cache_align mi_tld_t tld_main = {
  0, false,
  &_mi_heap_main, & _mi_heap_main,
  { MI_SEGMENT_SPAN_QUEUES_EMPTY, 0, 0, 0, 0, 0, &mi_subproc_default, &tld_main.stats }, // segments
  { MI_STAT_VERSION, MI_STATS_NULL }       // stats
};

mi_decl_cache_align mi_heap_t _mi_heap_main = {
  &tld_main,
  MI_ATOMIC_VAR_INIT(NULL),
  0,                // thread id
  0,                // initial cookie
  0,                // arena id
  { 0, 0 },         // the key of the main heap can be fixed (unlike page keys that need to be secure!)
  { {0x846ca68b}, {0}, 0, true },  // random
  0,                // page count
  MI_BIN_FULL, 0,   // page retired min/max
  0, 0,             // generic count
  NULL,             // next heap
  false,            // can reclaim
  0,                // tag
  #if MI_GUARDED
  0, 0, 0, 0,
  #endif
  MI_SMALL_PAGES_EMPTY,
  MI_PAGE_QUEUES_EMPTY
};

bool _mi_process_is_initialized = false;  // set to `true` in `mi_process_init`.

mi_stats_t _mi_stats_main = { MI_STAT_VERSION, MI_STATS_NULL };

#if MI_GUARDED
mi_decl_export void mi_heap_guarded_set_sample_rate(mi_heap_t* heap, size_t sample_rate, size_t seed) {
  heap->guarded_sample_rate  = sample_rate;
  heap->guarded_sample_count = sample_rate;  // count down samples
  if (heap->guarded_sample_rate > 1) {
    if (seed == 0) {
      seed = _mi_heap_random_next(heap);
    }
    heap->guarded_sample_count = (seed % heap->guarded_sample_rate) + 1;  // start at random count between 1 and `sample_rate`
  }
}

mi_decl_export void mi_heap_guarded_set_size_bound(mi_heap_t* heap, size_t min, size_t max) {
  heap->guarded_size_min = min;
  heap->guarded_size_max = (min > max ? min : max);
}

void _mi_heap_guarded_init(mi_heap_t* heap) {
  mi_heap_guarded_set_sample_rate(heap,
    (size_t)mi_option_get_clamp(mi_option_guarded_sample_rate, 0, LONG_MAX),
    (size_t)mi_option_get(mi_option_guarded_sample_seed));
  mi_heap_guarded_set_size_bound(heap,
    (size_t)mi_option_get_clamp(mi_option_guarded_min, 0, LONG_MAX),
    (size_t)mi_option_get_clamp(mi_option_guarded_max, 0, LONG_MAX) );
}
#else
mi_decl_export void mi_heap_guarded_set_sample_rate(mi_heap_t* heap, size_t sample_rate, size_t seed) {
  MI_UNUSED(heap); MI_UNUSED(sample_rate); MI_UNUSED(seed);
}

mi_decl_export void mi_heap_guarded_set_size_bound(mi_heap_t* heap, size_t min, size_t max) {
  MI_UNUSED(heap); MI_UNUSED(min); MI_UNUSED(max);
}
void _mi_heap_guarded_init(mi_heap_t* heap) {
  MI_UNUSED(heap);
}
#endif


static void mi_heap_main_init(void) {
  if (_mi_heap_main.cookie == 0) {
    _mi_heap_main.thread_id = _mi_thread_id();
    _mi_heap_main.cookie = 1;
    #if defined(_WIN32) && !defined(MI_SHARED_LIB)
      _mi_random_init_weak(&_mi_heap_main.random);    // prevent allocation failure during bcrypt dll initialization with static linking
    #else
      _mi_random_init(&_mi_heap_main.random);
    #endif
    _mi_heap_main.cookie  = _mi_heap_random_next(&_mi_heap_main);
    _mi_heap_main.keys[0] = _mi_heap_random_next(&_mi_heap_main);
    _mi_heap_main.keys[1] = _mi_heap_random_next(&_mi_heap_main);
    mi_lock_init(&mi_subproc_default.abandoned_os_lock);
    mi_lock_init(&mi_subproc_default.abandoned_os_visit_lock);
    _mi_heap_guarded_init(&_mi_heap_main);
  }
}

mi_heap_t* _mi_heap_main_get(void) {
  mi_heap_main_init();
  return &_mi_heap_main;
}

/* -----------------------------------------------------------
  Sub process
----------------------------------------------------------- */

mi_subproc_id_t mi_subproc_main(void) {
  return NULL;
}

mi_subproc_id_t mi_subproc_new(void) {
  mi_memid_t memid = _mi_memid_none();
  mi_subproc_t* subproc = (mi_subproc_t*)_mi_arena_meta_zalloc(sizeof(mi_subproc_t), &memid);
  if (subproc == NULL) return NULL;
  subproc->memid = memid;
  subproc->abandoned_os_list = NULL;
  mi_lock_init(&subproc->abandoned_os_lock);
  mi_lock_init(&subproc->abandoned_os_visit_lock);
  return subproc;
}

mi_subproc_t* _mi_subproc_from_id(mi_subproc_id_t subproc_id) {
  return (subproc_id == NULL ? &mi_subproc_default : (mi_subproc_t*)subproc_id);
}

void mi_subproc_delete(mi_subproc_id_t subproc_id) {
  if (subproc_id == NULL) return;
  mi_subproc_t* subproc = _mi_subproc_from_id(subproc_id);
  // check if there are no abandoned segments still..
  bool safe_to_delete = false;
  mi_lock(&subproc->abandoned_os_lock) {
    if (subproc->abandoned_os_list == NULL) {
      safe_to_delete = true;
    }
  }
  if (!safe_to_delete) return;
  // safe to release
  // todo: should we refcount subprocesses?
  mi_lock_done(&subproc->abandoned_os_lock);
  mi_lock_done(&subproc->abandoned_os_visit_lock);
  _mi_arena_meta_free(subproc, subproc->memid, sizeof(mi_subproc_t));
}

void mi_subproc_add_current_thread(mi_subproc_id_t subproc_id) {
  mi_heap_t* heap = mi_heap_get_default();
  if (heap == NULL) return;
  mi_assert(heap->tld->segments.subproc == &mi_subproc_default);
  if (heap->tld->segments.subproc != &mi_subproc_default) return;
  heap->tld->segments.subproc = _mi_subproc_from_id(subproc_id);
}



/* -----------------------------------------------------------
  Initialization and freeing of the thread local heaps
----------------------------------------------------------- */

// note: in x64 in release build `sizeof(mi_thread_data_t)` is under 4KiB (= OS page size).
typedef struct mi_thread_data_s {
  mi_heap_t  heap;   // must come first due to cast in `_mi_heap_done`
  mi_tld_t   tld;
  mi_memid_t memid;  // must come last due to zero'ing
} mi_thread_data_t;


// Thread meta-data is allocated directly from the OS. For
// some programs that do not use thread pools and allocate and
// destroy many OS threads, this may causes too much overhead
// per thread so we maintain a small cache of recently freed metadata.

#define TD_CACHE_SIZE (32)
static _Atomic(mi_thread_data_t*) td_cache[TD_CACHE_SIZE];

static mi_thread_data_t* mi_thread_data_zalloc(void) {
  // try to find thread metadata in the cache
  mi_thread_data_t* td = NULL;
  for (int i = 0; i < TD_CACHE_SIZE; i++) {
    td = mi_atomic_load_ptr_relaxed(mi_thread_data_t, &td_cache[i]);
    if (td != NULL) {
      // found cached allocation, try use it
      td = mi_atomic_exchange_ptr_acq_rel(mi_thread_data_t, &td_cache[i], NULL);
      if (td != NULL) {
        _mi_memzero(td, offsetof(mi_thread_data_t,memid));
        return td;
      }
    }
  }

  // if that fails, allocate as meta data
  mi_memid_t memid;
  td = (mi_thread_data_t*)_mi_os_zalloc(sizeof(mi_thread_data_t), &memid);
  if (td == NULL) {
    // if this fails, try once more. (issue #257)
    td = (mi_thread_data_t*)_mi_os_zalloc(sizeof(mi_thread_data_t), &memid);
    if (td == NULL) {
      // really out of memory
      _mi_error_message(ENOMEM, "unable to allocate thread local heap metadata (%zu bytes)\n", sizeof(mi_thread_data_t));
      return NULL;
    }
  }
  td->memid = memid;
  return td;
}

static void mi_thread_data_free( mi_thread_data_t* tdfree ) {
  // try to add the thread metadata to the cache
  for (int i = 0; i < TD_CACHE_SIZE; i++) {
    mi_thread_data_t* td = mi_atomic_load_ptr_relaxed(mi_thread_data_t, &td_cache[i]);
    if (td == NULL) {
      mi_thread_data_t* expected = NULL;
      if (mi_atomic_cas_ptr_weak_acq_rel(mi_thread_data_t, &td_cache[i], &expected, tdfree)) {
        return;
      }
    }
  }
  // if that fails, just free it directly
  _mi_os_free(tdfree, sizeof(mi_thread_data_t), tdfree->memid);
}

void _mi_thread_data_collect(void) {
  // free all thread metadata from the cache
  for (int i = 0; i < TD_CACHE_SIZE; i++) {
    mi_thread_data_t* td = mi_atomic_load_ptr_relaxed(mi_thread_data_t, &td_cache[i]);
    if (td != NULL) {
      td = mi_atomic_exchange_ptr_acq_rel(mi_thread_data_t, &td_cache[i], NULL);
      if (td != NULL) {
        _mi_os_free(td, sizeof(mi_thread_data_t), td->memid);
      }
    }
  }
}

// Initialize the thread local default heap, called from `mi_thread_init`
static bool _mi_thread_heap_init(void) {
  if (mi_heap_is_initialized(mi_prim_get_default_heap())) return true;
  if (_mi_is_main_thread()) {
    // mi_assert_internal(_mi_heap_main.thread_id != 0);  // can happen on freeBSD where alloc is called before any initialization
    // the main heap is statically allocated
    mi_heap_main_init();
    _mi_heap_set_default_direct(&_mi_heap_main);
    //mi_assert_internal(_mi_heap_default->tld->heap_backing == mi_prim_get_default_heap());
  }
  else {
    // use `_mi_os_alloc` to allocate directly from the OS
    mi_thread_data_t* td = mi_thread_data_zalloc();
    if (td == NULL) return false;

    mi_tld_t*  tld = &td->tld;
    mi_heap_t* heap = &td->heap;
    _mi_tld_init(tld, heap);  // must be before `_mi_heap_init`
    _mi_heap_init(heap, tld, _mi_arena_id_none(), false /* can reclaim */, 0 /* default tag */);
    _mi_heap_set_default_direct(heap);
  }
  return false;
}

// initialize thread local data
void _mi_tld_init(mi_tld_t* tld, mi_heap_t* bheap) {
  _mi_memcpy_aligned(tld, &tld_empty, sizeof(mi_tld_t));
  tld->heap_backing = bheap;
  tld->heaps = NULL;
  tld->segments.subproc = &mi_subproc_default;
  tld->segments.stats = &tld->stats;
}

// Free the thread local default heap (called from `mi_thread_done`)
static bool _mi_thread_heap_done(mi_heap_t* heap) {
  if (!mi_heap_is_initialized(heap)) return true;

  // reset default heap
  _mi_heap_set_default_direct(_mi_is_main_thread() ? &_mi_heap_main : (mi_heap_t*)&_mi_heap_empty);

  // switch to backing heap
  heap = heap->tld->heap_backing;
  if (!mi_heap_is_initialized(heap)) return false;

  // delete all non-backing heaps in this thread
  mi_heap_t* curr = heap->tld->heaps;
  while (curr != NULL) {
    mi_heap_t* next = curr->next; // save `next` as `curr` will be freed
    if (curr != heap) {
      mi_assert_internal(!mi_heap_is_backing(curr));
      mi_heap_delete(curr);
    }
    curr = next;
  }
  mi_assert_internal(heap->tld->heaps == heap && heap->next == NULL);
  mi_assert_internal(mi_heap_is_backing(heap));

  // collect if not the main thread
  if (heap != &_mi_heap_main) {
    _mi_heap_collect_abandon(heap);
  }

  // merge stats
  _mi_stats_done(&heap->tld->stats);

  // free if not the main thread
  if (heap != &_mi_heap_main) {
    // the following assertion does not always hold for huge segments as those are always treated
    // as abondened: one may allocate it in one thread, but deallocate in another in which case
    // the count can be too large or negative. todo: perhaps not count huge segments? see issue #363
    // mi_assert_internal(heap->tld->segments.count == 0 || heap->thread_id != _mi_thread_id());
    mi_thread_data_free((mi_thread_data_t*)heap);
  }
  else {
    #if 0
    // never free the main thread even in debug mode; if a dll is linked statically with mimalloc,
    // there may still be delete/free calls after the mi_fls_done is called. Issue #207
    _mi_heap_destroy_pages(heap);
    mi_assert_internal(heap->tld->heap_backing == &_mi_heap_main);
    #endif
  }
  return false;
}



// --------------------------------------------------------
// Try to run `mi_thread_done()` automatically so any memory
// owned by the thread but not yet released can be abandoned
// and re-owned by another thread.
//
// 1. windows dynamic library:
//     call from DllMain on DLL_THREAD_DETACH
// 2. windows static library:
//     use `FlsAlloc` to call a destructor when the thread is done
// 3. unix, pthreads:
//     use a pthread key to call a destructor when a pthread is done
//
// In the last two cases we also need to call `mi_process_init`
// to set up the thread local keys.
// --------------------------------------------------------

// Set up handlers so `mi_thread_done` is called automatically
static void mi_process_setup_auto_thread_done(void) {
  static bool tls_initialized = false; // fine if it races
  if (tls_initialized) return;
  tls_initialized = true;
  _mi_prim_thread_init_auto_done();
  _mi_heap_set_default_direct(&_mi_heap_main);
}


bool _mi_is_main_thread(void) {
  return (_mi_heap_main.thread_id==0 || _mi_heap_main.thread_id == _mi_thread_id());
}

static _Atomic(size_t) thread_count = MI_ATOMIC_VAR_INIT(1);

size_t  _mi_current_thread_count(void) {
  return mi_atomic_load_relaxed(&thread_count);
}

// This is called from the `mi_malloc_generic`
void mi_thread_init(void) mi_attr_noexcept
{
  // ensure our process has started already
  mi_process_init();

  // initialize the thread local default heap
  // (this will call `_mi_heap_set_default_direct` and thus set the
  //  fiber/pthread key to a non-zero value, ensuring `_mi_thread_done` is called)
  if (_mi_thread_heap_init()) return;  // returns true if already initialized

  _mi_stat_increase(&_mi_stats_main.threads, 1);
  mi_atomic_increment_relaxed(&thread_count);
  //_mi_verbose_message("thread init: 0x%zx\n", _mi_thread_id());
}

void mi_thread_done(void) mi_attr_noexcept {
  _mi_thread_done(NULL);
}

void _mi_thread_done(mi_heap_t* heap)
{
  // calling with NULL implies using the default heap
  if (heap == NULL) {
    heap = mi_prim_get_default_heap();
    if (heap == NULL) return;
  }

  // prevent re-entrancy through heap_done/heap_set_default_direct (issue #699)
  if (!mi_heap_is_initialized(heap)) {
    return;
  }

  // adjust stats
  mi_atomic_decrement_relaxed(&thread_count);
  _mi_stat_decrease(&_mi_stats_main.threads, 1);

  // check thread-id as on Windows shutdown with FLS the main (exit) thread may call this on thread-local heaps...
  if (heap->thread_id != _mi_thread_id()) return;

  // abandon the thread local heap
  if (_mi_thread_heap_done(heap)) return;  // returns true if already ran
}

void _mi_heap_set_default_direct(mi_heap_t* heap)  {
  mi_assert_internal(heap != NULL);
  #if defined(MI_TLS_SLOT)
  mi_prim_tls_slot_set(MI_TLS_SLOT,heap);
  #elif defined(MI_TLS_PTHREAD_SLOT_OFS)
  *mi_prim_tls_pthread_heap_slot() = heap;
  #elif defined(MI_TLS_PTHREAD)
  // we use _mi_heap_default_key
  #else
  _mi_heap_default = heap;
  #endif

  // ensure the default heap is passed to `_mi_thread_done`
  // setting to a non-NULL value also ensures `mi_thread_done` is called.
  _mi_prim_thread_associate_default_heap(heap);
}

void mi_thread_set_in_threadpool(void) mi_attr_noexcept {
  // nothing
}

// --------------------------------------------------------
// Run functions on process init/done, and thread init/done
// --------------------------------------------------------
static bool os_preloading = true;    // true until this module is initialized

// Returns true if this module has not been initialized; Don't use C runtime routines until it returns false.
bool mi_decl_noinline _mi_preloading(void) {
  return os_preloading;
}

// Returns true if mimalloc was redirected
mi_decl_nodiscard bool mi_is_redirected(void) mi_attr_noexcept {
  return _mi_is_redirected();
}

// Called once by the process loader from `src/prim/prim.c`
void _mi_auto_process_init(void) {
  mi_heap_main_init();
  #if defined(__APPLE__) || defined(MI_TLS_RECURSE_GUARD)
  volatile mi_heap_t* dummy = _mi_heap_default; // access TLS to allocate it before setting tls_initialized to true;
  if (dummy == NULL) return;                    // use dummy or otherwise the access may get optimized away (issue #697)
  #endif
  os_preloading = false;
  mi_assert_internal(_mi_is_main_thread());
  _mi_options_init();
  mi_process_setup_auto_thread_done();
  mi_process_init();
  if (_mi_is_redirected()) _mi_verbose_message("malloc is redirected.\n");

  // show message from the redirector (if present)
  const char* msg = NULL;
  _mi_allocator_init(&msg);
  if (msg != NULL && (mi_option_is_enabled(mi_option_verbose) || mi_option_is_enabled(mi_option_show_errors))) {
    _mi_fputs(NULL,NULL,NULL,msg);
  }

  // reseed random
  _mi_random_reinit_if_weak(&_mi_heap_main.random);
}

#if defined(_WIN32) && (defined(_M_IX86) || defined(_M_X64))
#include <intrin.h>
mi_decl_cache_align bool _mi_cpu_has_fsrm = false;
mi_decl_cache_align bool _mi_cpu_has_erms = false;

static void mi_detect_cpu_features(void) {
  // FSRM for fast short rep movsb/stosb support (AMD Zen3+ (~2020) or Intel Ice Lake+ (~2017))
  // EMRS for fast enhanced rep movsb/stosb support
  int32_t cpu_info[4];
  __cpuid(cpu_info, 7);
  _mi_cpu_has_fsrm = ((cpu_info[3] & (1 << 4)) != 0); // bit 4 of EDX : see <https://en.wikipedia.org/wiki/CPUID#EAX=7,_ECX=0:_Extended_Features>
  _mi_cpu_has_erms = ((cpu_info[1] & (1 << 9)) != 0); // bit 9 of EBX : see <https://en.wikipedia.org/wiki/CPUID#EAX=7,_ECX=0:_Extended_Features>
}
#else
static void mi_detect_cpu_features(void) {
  // nothing
}
#endif

// Initialize the process; called by thread_init or the process loader
void mi_process_init(void) mi_attr_noexcept {
  // ensure we are called once
  static mi_atomic_once_t process_init;
	#if _MSC_VER < 1920
	mi_heap_main_init(); // vs2017 can dynamically re-initialize _mi_heap_main
	#endif
  if (!mi_atomic_once(&process_init)) return;
  _mi_process_is_initialized = true;
  _mi_verbose_message("process init: 0x%zx\n", _mi_thread_id());
  mi_process_setup_auto_thread_done();

  mi_detect_cpu_features();
  _mi_os_init();
  mi_heap_main_init();
  mi_thread_init();

  #if defined(_WIN32)
  // On windows, when building as a static lib the FLS cleanup happens to early for the main thread.
  // To avoid this, set the FLS value for the main thread to NULL so the fls cleanup
  // will not call _mi_thread_done on the (still executing) main thread. See issue #508.
  _mi_prim_thread_associate_default_heap(NULL);
  #endif

  mi_stats_reset();  // only call stat reset *after* thread init (or the heap tld == NULL)
  mi_track_init();

  if (mi_option_is_enabled(mi_option_reserve_huge_os_pages)) {
    size_t pages = mi_option_get_clamp(mi_option_reserve_huge_os_pages, 0, 128*1024);
    long reserve_at = mi_option_get(mi_option_reserve_huge_os_pages_at);
    if (reserve_at != -1) {
      mi_reserve_huge_os_pages_at(pages, reserve_at, pages*500);
    } else {
      mi_reserve_huge_os_pages_interleave(pages, 0, pages*500);
    }
  }
  if (mi_option_is_enabled(mi_option_reserve_os_memory)) {
    long ksize = mi_option_get(mi_option_reserve_os_memory);
    if (ksize > 0) {
      mi_reserve_os_memory((size_t)ksize*MI_KiB, true /* commit? */, true /* allow large pages? */);
    }
  }
}

// Called when the process is done (cdecl as it is used with `at_exit` on some platforms)
void mi_cdecl mi_process_done(void) mi_attr_noexcept {
  // only shutdown if we were initialized
  if (!_mi_process_is_initialized) return;
  // ensure we are called once
  static bool process_done = false;
  if (process_done) return;
  process_done = true;

  // get the default heap so we don't need to acces thread locals anymore
  mi_heap_t* heap = mi_prim_get_default_heap();  // use prim to not initialize any heap
  mi_assert_internal(heap != NULL);

  // release any thread specific resources and ensure _mi_thread_done is called on all but the main thread
  _mi_prim_thread_done_auto_done();


  #ifndef MI_SKIP_COLLECT_ON_EXIT
    #if (MI_DEBUG || !defined(MI_SHARED_LIB))
    // free all memory if possible on process exit. This is not needed for a stand-alone process
    // but should be done if mimalloc is statically linked into another shared library which
    // is repeatedly loaded/unloaded, see issue #281.
    mi_heap_collect(heap, true /* force */ );
    #endif
  #endif

  // Forcefully release all retained memory; this can be dangerous in general if overriding regular malloc/free
  // since after process_done there might still be other code running that calls `free` (like at_exit routines,
  // or C-runtime termination code.
  if (mi_option_is_enabled(mi_option_destroy_on_exit)) {
    mi_heap_collect(heap, true /* force */);
    _mi_heap_unsafe_destroy_all(heap);     // forcefully release all memory held by all heaps (of this thread only!)
    _mi_arena_unsafe_destroy_all();
    _mi_segment_map_unsafe_destroy();
  }

  if (mi_option_is_enabled(mi_option_show_stats) || mi_option_is_enabled(mi_option_verbose)) {
    mi_stats_print(NULL);
  }
  _mi_allocator_done();
  _mi_verbose_message("process done: 0x%zx\n", _mi_heap_main.thread_id);
  os_preloading = true; // don't call the C runtime anymore
}

void mi_cdecl _mi_auto_process_done(void) mi_attr_noexcept {
  if (_mi_option_get_fast(mi_option_destroy_on_exit)>1) return;
  mi_process_done();
}
