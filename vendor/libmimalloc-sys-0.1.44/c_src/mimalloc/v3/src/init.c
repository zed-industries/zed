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

#define MI_MEMID_INIT(kind)   {{{NULL,0}}, kind, true /* pinned */, true /* committed */, false /* zero */ }
#define MI_MEMID_STATIC       MI_MEMID_INIT(MI_MEM_STATIC)

// Empty page used to initialize the small free pages array
const mi_page_t _mi_page_empty = {
  MI_ATOMIC_VAR_INIT(0),  // xthread_id
  NULL,                   // free
  0,                      // used
  0,                      // capacity
  0,                      // reserved capacity
  0,                      // block size shift
  0,                      // retire_expire
  NULL,                   // local_free
  MI_ATOMIC_VAR_INIT(0),  // xthread_free
  0,                      // block_size
  NULL,                   // page_start
  0,                      // heap tag
  false,                  // is_zero
  #if (MI_PADDING || MI_ENCODE_FREELIST)
  { 0, 0 },               // keys
  #endif
  NULL,                   // xheap
  NULL, NULL,             // next, prev
  MI_ARENA_SLICE_SIZE,    // page_committed
  MI_MEMID_STATIC         // memid
};

#define MI_PAGE_EMPTY() ((mi_page_t*)&_mi_page_empty)

#if (MI_PADDING>0) && (MI_INTPTR_SIZE >= 8)
#define MI_SMALL_PAGES_EMPTY  { MI_INIT128(MI_PAGE_EMPTY), MI_PAGE_EMPTY(), MI_PAGE_EMPTY() }
#elif (MI_PADDING>0)
#define MI_SMALL_PAGES_EMPTY  { MI_INIT128(MI_PAGE_EMPTY), MI_PAGE_EMPTY(), MI_PAGE_EMPTY(), MI_PAGE_EMPTY() }
#else
#define MI_SMALL_PAGES_EMPTY  { MI_INIT128(MI_PAGE_EMPTY), MI_PAGE_EMPTY() }
#endif


// Empty page queues for every bin
#define QNULL(sz)  { NULL, NULL, 0, (sz)*sizeof(uintptr_t) }
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
    QNULL(MI_LARGE_MAX_OBJ_WSIZE + 1  /* 655360, Huge queue */), \
    QNULL(MI_LARGE_MAX_OBJ_WSIZE + 2) /* Full queue */ }

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
  { MI_INIT74(MI_STAT_COUNT_NULL) }, \
  { MI_INIT5(MI_STAT_COUNT_NULL) }

// --------------------------------------------------------
// Statically allocate an empty heap as the initial
// thread local value for the default heap,
// and statically allocate the backing heap for the main
// thread so it can function without doing any allocation
// itself (as accessing a thread local for the first time
// may lead to allocation itself on some platforms)
// --------------------------------------------------------

static mi_decl_cache_align mi_subproc_t subproc_main
#if __cplusplus
= { };     // empty initializer to prevent running the constructor (with msvc)
#else
= { 0 };   // C zero initialize
#endif

static mi_decl_cache_align mi_tld_t tld_empty = {
  0,                      // thread_id
  0,                      // thread_seq
  0,                      // default numa node
  &subproc_main,          // subproc
  NULL,                   // heap_backing
  NULL,                   // heaps list
  0,                      // heartbeat
  false,                  // recurse
  false,                  // is_in_threadpool
  { MI_STAT_VERSION, MI_STATS_NULL },      // stats
  MI_MEMID_STATIC         // memid
};

mi_decl_cache_align const mi_heap_t _mi_heap_empty = {
  &tld_empty,             // tld
  NULL,                   // exclusive_arena
  0,                      // preferred numa node
  0,                      // cookie
  //{ 0, 0 },               // keys
  { {0}, {0}, 0, true },  // random
  0,                      // page count
  MI_BIN_FULL, 0,         // page retired min/max
  0, 0,                   // generic count
  NULL,                   // next
  0,                      // full page retain
  false,                  // can reclaim
  true,                   // can eager abandon
  0,                      // tag
  #if MI_GUARDED
  0, 0, 0, 1,          // count is 1 so we never write to it (see `internal.h:mi_heap_malloc_use_guarded`)
  #endif
  MI_SMALL_PAGES_EMPTY,
  MI_PAGE_QUEUES_EMPTY,
  MI_MEMID_STATIC
};

extern mi_decl_hidden mi_decl_cache_align mi_heap_t heap_main;

static mi_decl_cache_align mi_tld_t tld_main = {
  0,                      // thread_id
  0,                      // thread_seq
  0,                      // numa node
  &subproc_main,          // subproc
  &heap_main,             // heap_backing
  &heap_main,             // heaps list
  0,                      // heartbeat
  false,                  // recurse
  false,                  // is_in_threadpool
  { MI_STAT_VERSION, MI_STATS_NULL },      // stats
  MI_MEMID_STATIC         // memid
};

mi_decl_cache_align mi_heap_t heap_main = {
  &tld_main,              // thread local data
  NULL,                   // exclusive arena
  0,                      // preferred numa node
  0,                      // initial cookie
  //{ 0, 0 },               // the key of the main heap can be fixed (unlike page keys that need to be secure!)
  { {0x846ca68b}, {0}, 0, true },  // random
  0,                      // page count
  MI_BIN_FULL, 0,         // page retired min/max
  0, 0,                   // generic count
  NULL,                   // next heap
  2,                      // full page retain
  true,                   // allow page reclaim
  true,                   // allow page abandon
  0,                      // tag
  #if MI_GUARDED
  0, 0, 0, 0,
  #endif
  MI_SMALL_PAGES_EMPTY,
  MI_PAGE_QUEUES_EMPTY,
  MI_MEMID_STATIC
};


mi_threadid_t _mi_thread_id(void) mi_attr_noexcept {
  return _mi_prim_thread_id();
}

// the thread-local default heap for allocation
mi_decl_thread mi_heap_t* _mi_heap_default = (mi_heap_t*)&_mi_heap_empty;


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

// Initialize main subproc
static void mi_subproc_main_init(void) {
  if (subproc_main.memid.memkind != MI_MEM_STATIC) {
    subproc_main.memid = _mi_memid_create(MI_MEM_STATIC);
    mi_lock_init(&subproc_main.os_abandoned_pages_lock);
    mi_lock_init(&subproc_main.arena_reserve_lock);
  }
}

// Initialize main tld
static void mi_tld_main_init(void) {
  if (tld_main.thread_id == 0) {
    tld_main.thread_id = _mi_prim_thread_id();
  }
}

// Initialization of the (statically allocated) main heap, and the main tld and subproc.
static void mi_heap_main_init(void) {
  if (heap_main.cookie == 0) {
    // heap
    heap_main.cookie = 1;
    #if defined(__APPLE__) || defined(_WIN32) && !defined(MI_SHARED_LIB)
      _mi_random_init_weak(&heap_main.random);    // prevent allocation failure during bcrypt dll initialization with static linking
    #else
      _mi_random_init(&heap_main.random);
    #endif
    heap_main.cookie  = _mi_heap_random_next(&heap_main);
    //heap_main.keys[0] = _mi_heap_random_next(&heap_main);
    //heap_main.keys[1] = _mi_heap_random_next(&heap_main);
    _mi_heap_guarded_init(&heap_main);
    heap_main.allow_page_reclaim = (mi_option_get(mi_option_page_reclaim_on_free) >= 0);
    heap_main.allow_page_abandon = (mi_option_get(mi_option_page_full_retain) >= 0);
    heap_main.page_full_retain   = mi_option_get_clamp(mi_option_page_full_retain, -1, 32);

    mi_subproc_main_init();
    mi_tld_main_init();
  }
}

mi_heap_t* _mi_heap_main_get(void) {
  mi_heap_main_init();
  return &heap_main;
}


/* -----------------------------------------------------------
  Thread local data
----------------------------------------------------------- */

// Count current and total created threads
static _Atomic(size_t)  thread_count = MI_ATOMIC_VAR_INIT(1);
static _Atomic(size_t)  thread_total_count;

size_t  _mi_current_thread_count(void) {
  return mi_atomic_load_relaxed(&thread_count);
}


// The mimalloc thread local data
mi_decl_thread mi_tld_t* thread_tld = &tld_empty;

// Allocate fresh tld
static mi_tld_t* mi_tld_alloc(void) {
  mi_atomic_increment_relaxed(&thread_count);
  if (_mi_is_main_thread()) {
    return &tld_main;
  }
  else {
    // allocate tld meta-data
    // note: we need to be careful to not access the tld from `_mi_meta_zalloc`
    // (and in turn from `_mi_arena_alloc_aligned` and `_mi_os_alloc_aligned`).
    mi_memid_t memid;
    mi_tld_t* tld = (mi_tld_t*)_mi_meta_zalloc(sizeof(mi_tld_t), &memid);
    if (tld==NULL) {
      _mi_error_message(ENOMEM, "unable to allocate memory for thread local data\n");
      return NULL;
    }
    tld->memid = memid;
    tld->heap_backing = NULL;
    tld->heaps = NULL;
    tld->subproc = &subproc_main;
    tld->numa_node = _mi_os_numa_node();
    tld->thread_id = _mi_prim_thread_id();
    tld->thread_seq = mi_atomic_add_acq_rel(&thread_total_count, 1);
    tld->is_in_threadpool = _mi_prim_thread_is_in_threadpool();
    return tld;
  }
}

#define MI_TLD_INVALID  ((mi_tld_t*)1)

mi_decl_noinline static void mi_tld_free(mi_tld_t* tld) {
  if (tld != NULL && tld != MI_TLD_INVALID) {
    _mi_stats_done(&tld->stats);
    _mi_meta_free(tld, sizeof(mi_tld_t), tld->memid);
  }
  #if 0
  // do not read/write to `thread_tld` on older macOS <= 14 as that will re-initialize the thread local storage
  // (since we are calling this during pthread shutdown)
  // (and this could happen on other systems as well, so let's never do it)
  thread_tld = MI_TLD_INVALID;
  #endif
  mi_atomic_decrement_relaxed(&thread_count);
}

static mi_tld_t* mi_tld(void) {
  mi_tld_t* tld = thread_tld;
  if (tld == MI_TLD_INVALID) {
    _mi_error_message(EFAULT, "internal error: tld is accessed after the thread terminated\n");
    thread_tld = &tld_empty;
  }
  if (tld==&tld_empty) {
    thread_tld = tld = mi_tld_alloc();
  }
  return tld;
}

mi_subproc_t* _mi_subproc(void) {
  // should work without doing initialization (as it may be called from `_mi_tld -> mi_tld_alloc ... -> os_alloc -> _mi_subproc()`
  // todo: this will still fail on OS systems where the first access to a thread-local causes allocation.
  //       on such systems we can check for this with the _mi_prim_get_default_heap as those are protected (by being
  //       stored in a TLS slot for example)
  mi_heap_t* heap = mi_prim_get_default_heap();
  if (heap == NULL) {
    return _mi_subproc_main();
  }
  else {
    return heap->tld->subproc;  // avoid using thread local storage (`thread_tld`)
  }
}


mi_tld_t* _mi_thread_tld(void) mi_attr_noexcept {
  // should work without doing initialization (as it may be called from `_mi_tld -> mi_tld_alloc ... -> os_alloc -> _mi_subproc()`
  mi_heap_t* heap = mi_prim_get_default_heap();
  if (heap == NULL) {
    return &tld_empty;
  }
  else {
    return heap->tld;
  }
}

/* -----------------------------------------------------------
  Sub process
----------------------------------------------------------- */

mi_subproc_t* _mi_subproc_main(void) {
  return &subproc_main;
}

mi_subproc_id_t mi_subproc_main(void) {
  return NULL;
}

mi_subproc_id_t mi_subproc_new(void) {
  mi_memid_t memid;
  mi_subproc_t* subproc = (mi_subproc_t*)_mi_meta_zalloc(sizeof(mi_subproc_t),&memid);
  if (subproc == NULL) return NULL;
  subproc->memid = memid;
  mi_lock_init(&subproc->os_abandoned_pages_lock);
  mi_lock_init(&subproc->arena_reserve_lock);
  return subproc;
}

mi_subproc_t* _mi_subproc_from_id(mi_subproc_id_t subproc_id) {
  return (subproc_id == NULL ? &subproc_main : (mi_subproc_t*)subproc_id);
}

void mi_subproc_delete(mi_subproc_id_t subproc_id) {
  if (subproc_id == NULL) return;
  mi_subproc_t* subproc = _mi_subproc_from_id(subproc_id);
  // check if there are os pages still..
  bool safe_to_delete = false;
  mi_lock(&subproc->os_abandoned_pages_lock) {
    if (subproc->os_abandoned_pages == NULL) {
      safe_to_delete = true;
    }
  }
  if (!safe_to_delete) return;

  // merge stats back into the main subproc?
  _mi_stats_merge_from(&_mi_subproc_main()->stats, &subproc->stats);

  // safe to release
  // todo: should we refcount subprocesses?
  mi_lock_done(&subproc->os_abandoned_pages_lock);
  mi_lock_done(&subproc->arena_reserve_lock);
  _mi_meta_free(subproc, sizeof(mi_subproc_t), subproc->memid);
}

void mi_subproc_add_current_thread(mi_subproc_id_t subproc_id) {
  mi_tld_t* tld = mi_tld();
  if (tld == NULL) return;
  mi_assert(tld->subproc == &subproc_main);
  if (tld->subproc != &subproc_main) return;
  tld->subproc = _mi_subproc_from_id(subproc_id);
}


/* -----------------------------------------------------------
  Allocate heap data
----------------------------------------------------------- */

// Initialize the thread local default heap, called from `mi_thread_init`
static bool _mi_thread_heap_init(void) {
  if (mi_heap_is_initialized(mi_prim_get_default_heap())) return true;
  if (_mi_is_main_thread()) {
    // mi_assert_internal(heap_main.thread_id != 0);  // can happen on freeBSD where alloc is called before any initialization
    // the main heap is statically allocated
    mi_heap_main_init();
    _mi_heap_set_default_direct(&heap_main);
    //mi_assert_internal(_mi_heap_default->tld->heap_backing == mi_prim_get_default_heap());
  }
  else {
    // allocates tld data
    // note: we cannot access thread-locals yet as that can cause (recursive) allocation
    // (on macOS <= 14 for example where the loader allocates thread-local data on demand).
    mi_tld_t* tld = mi_tld_alloc();

    // allocate and initialize the heap
    mi_heap_t* heap = _mi_heap_create(0 /* default tag */, false /* allow destroy? */, _mi_arena_id_none(), tld);

    // associate the heap with this thread
    // (this is safe, on macOS for example, the heap is set in a dedicated TLS slot and thus does not cause recursive allocation)
    _mi_heap_set_default_direct(heap);

    // now that the heap is set for this thread, we can set the thread-local tld.
    thread_tld = tld;
  }
  return false;
}


// Free the thread local default heap (called from `mi_thread_done`)
static bool _mi_thread_heap_done(mi_heap_t* heap) {
  if (!mi_heap_is_initialized(heap)) return true;

  // reset default heap
  _mi_heap_set_default_direct(_mi_is_main_thread() ? &heap_main : (mi_heap_t*)&_mi_heap_empty);

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
  if (heap != &heap_main) {
    _mi_heap_collect_abandon(heap);
  }

  // free heap meta data
  _mi_meta_free(heap, sizeof(mi_heap_t), heap->memid);

  if (heap == &heap_main) {
    #if 0
    // never free the main thread even in debug mode; if a dll is linked statically with mimalloc,
    // there may still be delete/free calls after the mi_fls_done is called. Issue #207
    _mi_heap_destroy_pages(heap);
    mi_assert_internal(heap->tld->heap_backing == &heap_main);
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
//     use special linker section to call a destructor when the thread is done
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
  _mi_heap_set_default_direct(&heap_main);
}


bool _mi_is_main_thread(void) {
  return (tld_main.thread_id==0 || tld_main.thread_id == _mi_thread_id());
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

  mi_subproc_stat_increase(_mi_subproc_main(), threads, 1);
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
  mi_subproc_stat_decrease(_mi_subproc_main(), threads, 1);

  // check thread-id as on Windows shutdown with FLS the main (exit) thread may call this on thread-local heaps...
  if (heap->tld->thread_id != _mi_prim_thread_id()) return;

  // abandon the thread local heap
  // note: we store the tld as we should avoid reading `thread_tld` at this point (to avoid reinitializing the thread local storage)
  mi_tld_t* tld = heap->tld;
  _mi_thread_heap_done(heap);  // returns true if already ran

  // free thread local data
  mi_tld_free(tld);
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
  mi_tld_t* tld = mi_tld();
  if (tld!=NULL) {
    tld->is_in_threadpool = true;
  }
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
  _mi_random_reinit_if_weak(&heap_main.random);
}

// CPU features
mi_decl_cache_align bool _mi_cpu_has_fsrm = false;
mi_decl_cache_align bool _mi_cpu_has_erms = false;
mi_decl_cache_align bool _mi_cpu_has_popcnt = false;

#if (MI_ARCH_X64 || MI_ARCH_X86)
#if defined(__GNUC__)
#include <cpuid.h>
static bool mi_cpuid(uint32_t* regs4, uint32_t level) {
  return (__get_cpuid(level, &regs4[0], &regs4[1], &regs4[2], &regs4[3]) == 1);
}

#elif defined(_MSC_VER)
static bool mi_cpuid(uint32_t* regs4, uint32_t level) {
  __cpuid((int32_t*)regs4, (int32_t)level);
  return true;
}
#else
static bool mi_cpuid(uint32_t* regs4, uint32_t level) {
  MI_UNUSED(regs4); MI_UNUSED(level);
  return false;
}
#endif

static void mi_detect_cpu_features(void) {
  // FSRM for fast short rep movsb/stosb support (AMD Zen3+ (~2020) or Intel Ice Lake+ (~2017))
  // EMRS for fast enhanced rep movsb/stosb support
  uint32_t cpu_info[4];
  if (mi_cpuid(cpu_info, 7)) {
    _mi_cpu_has_fsrm = ((cpu_info[3] & (1 << 4)) != 0); // bit 4 of EDX : see <https://en.wikipedia.org/wiki/CPUID#EAX=7,_ECX=0:_Extended_Features>
    _mi_cpu_has_erms = ((cpu_info[1] & (1 << 9)) != 0); // bit 9 of EBX : see <https://en.wikipedia.org/wiki/CPUID#EAX=7,_ECX=0:_Extended_Features>
  }
  if (mi_cpuid(cpu_info, 1)) {
    _mi_cpu_has_popcnt = ((cpu_info[2] & (1 << 23)) != 0); // bit 23 of ECX : see <https://en.wikipedia.org/wiki/CPUID#EAX=1:_Processor_Info_and_Feature_Bits>
  }
}

#else
static void mi_detect_cpu_features(void) {
  #if MI_ARCH_ARM64
  _mi_cpu_has_popcnt = true;
  #endif
}
#endif


// Initialize the process; called by thread_init or the process loader
void mi_process_init(void) mi_attr_noexcept {
  // ensure we are called once
  static mi_atomic_once_t process_init;
	#if _MSC_VER < 1920
	mi_heap_main_init(); // vs2017 can dynamically re-initialize heap_main
	#endif
  if (!mi_atomic_once(&process_init)) return;
  _mi_process_is_initialized = true;
  _mi_verbose_message("process init: 0x%zx\n", _mi_thread_id());

  mi_detect_cpu_features();
  _mi_stats_init();
  _mi_os_init();
  _mi_page_map_init();
  mi_heap_main_init();
  mi_tld_main_init();
  // the following two can potentially allocate (on freeBSD for locks and thread keys)
  mi_subproc_main_init();
  mi_process_setup_auto_thread_done();
  mi_thread_init();

  #if defined(_WIN32) && defined(MI_WIN_USE_FLS)
  // On windows, when building as a static lib the FLS cleanup happens to early for the main thread.
  // To avoid this, set the FLS value for the main thread to NULL so the fls cleanup
  // will not call _mi_thread_done on the (still executing) main thread. See issue #508.
  _mi_prim_thread_associate_default_heap(NULL);
  #endif

  // mi_stats_reset();  // only call stat reset *after* thread init (or the heap tld == NULL)
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
      mi_reserve_os_memory((size_t)ksize*MI_KiB, true, true);
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
    _mi_arenas_unsafe_destroy_all(heap->tld);
    _mi_page_map_unsafe_destroy(_mi_subproc_main());
  }
  //_mi_page_map_unsafe_destroy(_mi_subproc_main());

  if (mi_option_is_enabled(mi_option_show_stats) || mi_option_is_enabled(mi_option_verbose)) {
    _mi_stats_print(&_mi_subproc_main()->stats, NULL, NULL);  // use always main subproc at process exit to avoid dereferencing the heap (as it may be destroyed by now)
  }
  _mi_allocator_done();
  _mi_verbose_message("process done: 0x%zx\n", tld_main.thread_id);
  os_preloading = true; // don't call the C runtime anymore
}

void mi_cdecl _mi_auto_process_done(void) mi_attr_noexcept {
  if (_mi_option_get_fast(mi_option_destroy_on_exit)>1) return;
  mi_process_done();
}
