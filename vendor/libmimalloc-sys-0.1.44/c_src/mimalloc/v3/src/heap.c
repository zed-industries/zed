/*----------------------------------------------------------------------------
Copyright (c) 2018-2021, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/

#include "mimalloc.h"
#include "mimalloc/internal.h"
#include "mimalloc/prim.h"  // mi_prim_get_default_heap

#if defined(_MSC_VER) && (_MSC_VER < 1920)
#pragma warning(disable:4204)  // non-constant aggregate initializer
#endif

/* -----------------------------------------------------------
  Helpers
----------------------------------------------------------- */

// return `true` if ok, `false` to break
typedef bool (heap_page_visitor_fun)(mi_heap_t* heap, mi_page_queue_t* pq, mi_page_t* page, void* arg1, void* arg2);

// Visit all pages in a heap; returns `false` if break was called.
static bool mi_heap_visit_pages(mi_heap_t* heap, heap_page_visitor_fun* fn, void* arg1, void* arg2)
{
  if (heap==NULL || heap->page_count==0) return 0;

  // visit all pages
  #if MI_DEBUG>1
  size_t total = heap->page_count;
  size_t count = 0;
  #endif

  for (size_t i = 0; i <= MI_BIN_FULL; i++) {
    mi_page_queue_t* pq = &heap->pages[i];
    mi_page_t* page = pq->first;
    while(page != NULL) {
      mi_page_t* next = page->next; // save next in case the page gets removed from the queue
      mi_assert_internal(mi_page_heap(page) == heap);
      #if MI_DEBUG>1
      count++;
      #endif
      if (!fn(heap, pq, page, arg1, arg2)) return false;
      page = next; // and continue
    }
  }
  mi_assert_internal(count == total);
  return true;
}


#if MI_DEBUG>=2
static bool mi_heap_page_is_valid(mi_heap_t* heap, mi_page_queue_t* pq, mi_page_t* page, void* arg1, void* arg2) {
  MI_UNUSED(arg1);
  MI_UNUSED(arg2);
  MI_UNUSED(pq);
  mi_assert_internal(mi_page_heap(page) == heap);
  mi_assert_expensive(_mi_page_is_valid(page));
  return true;
}
#endif
#if MI_DEBUG>=3
static bool mi_heap_is_valid(mi_heap_t* heap) {
  mi_assert_internal(heap!=NULL);
  mi_heap_visit_pages(heap, &mi_heap_page_is_valid, NULL, NULL);
  for (size_t bin = 0; bin < MI_BIN_COUNT; bin++) {
    mi_assert_internal(_mi_page_queue_is_valid(heap, &heap->pages[bin]));
  }
  return true;
}
#endif




/* -----------------------------------------------------------
  "Collect" pages by migrating `local_free` and `thread_free`
  lists and freeing empty pages. This is done when a thread
  stops (and in that case abandons pages if there are still
  blocks alive)
----------------------------------------------------------- */

typedef enum mi_collect_e {
  MI_NORMAL,
  MI_FORCE,
  MI_ABANDON
} mi_collect_t;


static bool mi_heap_page_collect(mi_heap_t* heap, mi_page_queue_t* pq, mi_page_t* page, void* arg_collect, void* arg2 ) {
  MI_UNUSED(arg2);
  MI_UNUSED(heap);
  mi_assert_internal(mi_heap_page_is_valid(heap, pq, page, NULL, NULL));
  mi_collect_t collect = *((mi_collect_t*)arg_collect);
  _mi_page_free_collect(page, collect >= MI_FORCE);
  if (mi_page_all_free(page)) {
    // no more used blocks, free the page.
    // note: this will free retired pages as well.
    _mi_page_free(page, pq);
  }
  else if (collect == MI_ABANDON) {
    // still used blocks but the thread is done; abandon the page
    _mi_page_abandon(page, pq);
  }
  return true; // don't break
}


static void mi_heap_collect_ex(mi_heap_t* heap, mi_collect_t collect)
{
  if (heap==NULL || !mi_heap_is_initialized(heap)) return;
  mi_assert_expensive(mi_heap_is_valid(heap));

  const bool force = (collect >= MI_FORCE);
  _mi_deferred_free(heap, force);

  // python/cpython#112532: we may be called from a thread that is not the owner of the heap
  // const bool is_main_thread = (_mi_is_main_thread() && heap->thread_id == _mi_thread_id());

  // if (_mi_is_main_thread()) { mi_debug_show_arenas(true, false, false); }

  // collect retired pages
  _mi_heap_collect_retired(heap, force);

  // collect all pages owned by this thread
  mi_heap_visit_pages(heap, &mi_heap_page_collect, &collect, NULL);

  // collect arenas (this is program wide so don't force purges on abandonment of threads)
  //mi_atomic_storei64_release(&heap->tld->subproc->purge_expire, 1);
  _mi_arenas_collect(collect == MI_FORCE /* force purge? */, collect >= MI_FORCE /* visit all? */, heap->tld);

  // merge statistics
  if (collect <= MI_FORCE) { _mi_stats_merge_thread(heap->tld); }
}

void _mi_heap_collect_abandon(mi_heap_t* heap) {
  mi_heap_collect_ex(heap, MI_ABANDON);
}

void mi_heap_collect(mi_heap_t* heap, bool force) mi_attr_noexcept {
  mi_heap_collect_ex(heap, (force ? MI_FORCE : MI_NORMAL));
}

void mi_collect(bool force) mi_attr_noexcept {
  mi_heap_collect(mi_prim_get_default_heap(), force);
}


/* -----------------------------------------------------------
  Heap new
----------------------------------------------------------- */

mi_heap_t* mi_heap_get_default(void) {
  mi_heap_t* heap = mi_prim_get_default_heap();
  if mi_unlikely(!mi_heap_is_initialized(heap)) {
    mi_thread_init();
    heap = mi_prim_get_default_heap();
  }
  return heap;
}

static bool mi_heap_is_default(const mi_heap_t* heap) {
  return (heap == mi_prim_get_default_heap());
}


mi_heap_t* mi_heap_get_backing(void) {
  mi_heap_t* heap = mi_heap_get_default();
  mi_assert_internal(heap!=NULL);
  mi_heap_t* bheap = heap->tld->heap_backing;
  mi_assert_internal(bheap!=NULL);
  mi_assert_internal(bheap->tld->thread_id == _mi_thread_id());
  return bheap;
}

// todo: make order of parameters consistent (but would that break compat with CPython?)
void _mi_heap_init(mi_heap_t* heap, mi_arena_id_t arena_id, bool allow_destroy, uint8_t heap_tag, mi_tld_t* tld)
{
  mi_assert_internal(heap!=NULL);
  mi_memid_t memid = heap->memid;
  _mi_memcpy_aligned(heap, &_mi_heap_empty, sizeof(mi_heap_t));
  heap->memid = memid;
  heap->tld   = tld;  // avoid reading the thread-local tld during initialization
  heap->tag   = heap_tag;
  heap->numa_node = tld->numa_node;
  heap->exclusive_arena    = _mi_arena_from_id(arena_id);
  heap->allow_page_reclaim = (!allow_destroy && mi_option_get(mi_option_page_reclaim_on_free) >= 0);
  heap->allow_page_abandon = (!allow_destroy && mi_option_get(mi_option_page_full_retain) >= 0);
  heap->page_full_retain = mi_option_get_clamp(mi_option_page_full_retain, -1, 32);
  if (heap->tld->is_in_threadpool) {
    // if we run as part of a thread pool it is better to not arbitrarily reclaim abandoned pages into our heap.
    // this is checked in `free.c:mi_free_try_collect_mt`
    // .. but abandoning is good in this case: halve the full page retain (possibly to 0)
    // (so blocked threads do not hold on to too much memory)
    if (heap->page_full_retain > 0) {
      heap->page_full_retain = heap->page_full_retain / 4;
    }
  }

  if (heap->tld->heap_backing == NULL) {
    heap->tld->heap_backing = heap;  // first heap becomes the backing heap
    _mi_random_init(&heap->random);
  }
  else {
    _mi_random_split(&heap->tld->heap_backing->random, &heap->random);
  }
  heap->cookie  = _mi_heap_random_next(heap) | 1;
  //heap->keys[0] = _mi_heap_random_next(heap);
  //heap->keys[1] = _mi_heap_random_next(heap);*/
  _mi_heap_guarded_init(heap);

  // push on the thread local heaps list
  heap->next = heap->tld->heaps;
  heap->tld->heaps = heap;
}

mi_heap_t* _mi_heap_create(int heap_tag, bool allow_destroy, mi_arena_id_t arena_id, mi_tld_t* tld) {
  mi_assert_internal(tld!=NULL);
  mi_assert(heap_tag >= 0 && heap_tag < 256);
  // allocate and initialize a heap
  mi_memid_t memid;
  mi_heap_t* heap;
  if (arena_id == _mi_arena_id_none()) {
    heap = (mi_heap_t*)_mi_meta_zalloc(sizeof(mi_heap_t), &memid);
  }
  else {
    // heaps associated wita a specific arena are allocated in that arena
    // note: takes up at least one slice which is quite wasteful...
    heap = (mi_heap_t*)_mi_arenas_alloc(_mi_subproc(), _mi_align_up(sizeof(mi_heap_t),MI_ARENA_MIN_OBJ_SIZE), true, true, _mi_arena_from_id(arena_id), tld->thread_seq, tld->numa_node, &memid);
  }
  if (heap==NULL) {
    _mi_error_message(ENOMEM, "unable to allocate heap meta-data\n");
    return NULL;
  }
  heap->memid = memid;
  _mi_heap_init(heap, arena_id, allow_destroy, (uint8_t)heap_tag, tld);
  return heap;
}

mi_decl_nodiscard mi_heap_t* mi_heap_new_ex(int heap_tag, bool allow_destroy, mi_arena_id_t arena_id) {
  mi_heap_t* bheap = mi_heap_get_backing();
  mi_assert_internal(bheap != NULL);
  return _mi_heap_create(heap_tag, allow_destroy, arena_id, bheap->tld);
}

mi_decl_nodiscard mi_heap_t* mi_heap_new_in_arena(mi_arena_id_t arena_id) {
  return mi_heap_new_ex(0 /* default heap tag */, false /* allow destroy? */, arena_id);
}

mi_decl_nodiscard mi_heap_t* mi_heap_new(void) {
  // don't reclaim abandoned memory or otherwise destroy is unsafe
  return mi_heap_new_ex(0 /* default heap tag */, true /* allow destroy? */, _mi_arena_id_none());
}

bool _mi_heap_memid_is_suitable(mi_heap_t* heap, mi_memid_t memid) {
  return _mi_arena_memid_is_suitable(memid, heap->exclusive_arena);
}

uintptr_t _mi_heap_random_next(mi_heap_t* heap) {
  return _mi_random_next(&heap->random);
}

void mi_heap_set_numa_affinity(mi_heap_t* heap, int numa_node) {
  if (heap == NULL) return;
  heap->numa_node = (numa_node < 0 ? -1 : numa_node % _mi_os_numa_node_count());
}

// zero out the page queues
static void mi_heap_reset_pages(mi_heap_t* heap) {
  mi_assert_internal(heap != NULL);
  mi_assert_internal(mi_heap_is_initialized(heap));
  // TODO: copy full empty heap instead?
  _mi_memset(&heap->pages_free_direct, 0, sizeof(heap->pages_free_direct));
  _mi_memcpy_aligned(&heap->pages, &_mi_heap_empty.pages, sizeof(heap->pages));
  // heap->thread_delayed_free = NULL;
  heap->page_count = 0;
}

// called from `mi_heap_destroy` and `mi_heap_delete` to free the internal heap resources.
static void mi_heap_free(mi_heap_t* heap, bool do_free_mem) {
  mi_assert(heap != NULL);
  mi_assert_internal(mi_heap_is_initialized(heap));
  if (heap==NULL || !mi_heap_is_initialized(heap)) return;
  if (mi_heap_is_backing(heap)) return; // dont free the backing heap

  // reset default
  if (mi_heap_is_default(heap)) {
    _mi_heap_set_default_direct(heap->tld->heap_backing);
  }

  // remove ourselves from the thread local heaps list
  // linear search but we expect the number of heaps to be relatively small
  mi_heap_t* prev = NULL;
  mi_heap_t* curr = heap->tld->heaps;
  while (curr != heap && curr != NULL) {
    prev = curr;
    curr = curr->next;
  }
  mi_assert_internal(curr == heap);
  if (curr == heap) {
    if (prev != NULL) { prev->next = heap->next; }
                 else { heap->tld->heaps = heap->next; }
  }
  mi_assert_internal(heap->tld->heaps != NULL);

  // and free the used memory
  if (do_free_mem) {
    _mi_meta_free(heap, sizeof(*heap), heap->memid);
  }
}

// return a heap on the same thread as `heap` specialized for the specified tag (if it exists)
mi_heap_t* _mi_heap_by_tag(mi_heap_t* heap, uint8_t tag) {
  if (heap->tag == tag) {
    return heap;
  }
  for (mi_heap_t *curr = heap->tld->heaps; curr != NULL; curr = curr->next) {
    if (curr->tag == tag) {
      return curr;
    }
  }
  return NULL;
}

/* -----------------------------------------------------------
  Heap destroy
----------------------------------------------------------- */

static bool _mi_heap_page_destroy(mi_heap_t* heap, mi_page_queue_t* pq, mi_page_t* page, void* arg1, void* arg2) {
  MI_UNUSED(arg1);
  MI_UNUSED(arg2);
  MI_UNUSED(pq);

  // ensure no more thread_delayed_free will be added
  //_mi_page_use_delayed_free(page, MI_NEVER_DELAYED_FREE, false);

  // stats
  const size_t bsize = mi_page_block_size(page);
  if (bsize > MI_LARGE_MAX_OBJ_SIZE) {
    mi_heap_stat_decrease(heap, malloc_huge, bsize);
  }
  #if (MI_STAT>0)
  _mi_page_free_collect(page, false);  // update used count
  const size_t inuse = page->used;
  if (bsize <= MI_LARGE_MAX_OBJ_SIZE) {
    mi_heap_stat_decrease(heap, malloc_normal, bsize * inuse);
    #if (MI_STAT>1)
    mi_heap_stat_decrease(heap, malloc_bins[_mi_bin(bsize)], inuse);
    #endif
  }
  // mi_heap_stat_decrease(heap, malloc_requested, bsize * inuse);  // todo: off for aligned blocks...
  #endif

  /// pretend it is all free now
  mi_assert_internal(mi_page_thread_free(page) == NULL);
  page->used = 0;

  // and free the page
  // mi_page_free(page,false);
  page->next = NULL;
  page->prev = NULL;
  mi_page_set_heap(page, NULL);
  _mi_arenas_page_free(page, heap->tld);

  return true; // keep going
}

void _mi_heap_destroy_pages(mi_heap_t* heap) {
  mi_heap_visit_pages(heap, &_mi_heap_page_destroy, NULL, NULL);
  mi_heap_reset_pages(heap);
}

#if MI_TRACK_HEAP_DESTROY
static bool mi_cdecl mi_heap_track_block_free(const mi_heap_t* heap, const mi_heap_area_t* area, void* block, size_t block_size, void* arg) {
  MI_UNUSED(heap); MI_UNUSED(area);  MI_UNUSED(arg); MI_UNUSED(block_size);
  mi_track_free_size(block,mi_usable_size(block));
  return true;
}
#endif

void mi_heap_destroy(mi_heap_t* heap) {
  mi_assert(heap != NULL);
  mi_assert(mi_heap_is_initialized(heap));
  mi_assert(!heap->allow_page_reclaim);
  mi_assert(!heap->allow_page_abandon);
  mi_assert_expensive(mi_heap_is_valid(heap));
  if (heap==NULL || !mi_heap_is_initialized(heap)) return;
  #if MI_GUARDED
  // _mi_warning_message("'mi_heap_destroy' called but MI_GUARDED is enabled -- using `mi_heap_delete` instead (heap at %p)\n", heap);
  mi_heap_delete(heap);
  return;
  #else
  if (heap->allow_page_reclaim) {
    _mi_warning_message("'mi_heap_destroy' called but ignored as the heap was not created with 'allow_destroy' (heap at %p)\n", heap);
    // don't free in case it may contain reclaimed pages,
    mi_heap_delete(heap);
  }
  else {
    // track all blocks as freed
    #if MI_TRACK_HEAP_DESTROY
    mi_heap_visit_blocks(heap, true, mi_heap_track_block_free, NULL);
    #endif
    // free all pages
    _mi_heap_destroy_pages(heap);
    mi_heap_free(heap,true);
  }
  #endif
}

// forcefully destroy all heaps in the current thread
void _mi_heap_unsafe_destroy_all(mi_heap_t* heap) {
  mi_assert_internal(heap != NULL);
  if (heap == NULL) return;
  mi_heap_t* curr = heap->tld->heaps;
  while (curr != NULL) {
    mi_heap_t* next = curr->next;
    if (!curr->allow_page_reclaim) {
      mi_heap_destroy(curr);
    }
    else {
      _mi_heap_destroy_pages(curr);
    }
    curr = next;
  }
}

/* -----------------------------------------------------------
  Safe Heap delete
----------------------------------------------------------- */

// Transfer the pages from one heap to the other
//static void mi_heap_absorb(mi_heap_t* heap, mi_heap_t* from) {
//  mi_assert_internal(heap!=NULL);
//  if (from==NULL || from->page_count == 0) return;
//
//  // transfer all pages by appending the queues; this will set a new heap field
//  for (size_t i = 0; i <= MI_BIN_FULL; i++) {
//    mi_page_queue_t* pq = &heap->pages[i];
//    mi_page_queue_t* append = &from->pages[i];
//    size_t pcount = _mi_page_queue_append(heap, pq, append);
//    heap->page_count += pcount;
//    from->page_count -= pcount;
//  }
//  mi_assert_internal(from->page_count == 0);
//
//  // and reset the `from` heap
//  mi_heap_reset_pages(from);
//}

//// are two heaps compatible with respect to heap-tag, exclusive arena etc.
//static bool mi_heaps_are_compatible(mi_heap_t* heap1, mi_heap_t* heap2) {
//  return (heap1->tag == heap2->tag &&                   // store same kind of objects
//          heap1->tld->subproc == heap2->tld->subproc && // same sub-process
//          heap1->arena_id == heap2->arena_id);          // same arena preference
//}

// Safe delete a heap without freeing any still allocated blocks in that heap.
void mi_heap_delete(mi_heap_t* heap)
{
  mi_assert(heap != NULL);
  mi_assert(mi_heap_is_initialized(heap));
  mi_assert_expensive(mi_heap_is_valid(heap));
  if (heap==NULL || !mi_heap_is_initialized(heap)) return;

  // abandon all pages
  _mi_heap_collect_abandon(heap);

  mi_assert_internal(heap->page_count==0);
  mi_heap_free(heap,true);
}

mi_heap_t* mi_heap_set_default(mi_heap_t* heap) {
  mi_assert(heap != NULL);
  mi_assert(mi_heap_is_initialized(heap));
  if (heap==NULL || !mi_heap_is_initialized(heap)) return NULL;
  mi_assert_expensive(mi_heap_is_valid(heap));
  mi_heap_t* old = mi_prim_get_default_heap();
  _mi_heap_set_default_direct(heap);
  return old;
}


/* -----------------------------------------------------------
  Load/unload heaps
----------------------------------------------------------- */
void mi_heap_unload(mi_heap_t* heap) {
  mi_assert(mi_heap_is_initialized(heap));
  mi_assert_expensive(mi_heap_is_valid(heap));
  if (heap==NULL || !mi_heap_is_initialized(heap)) return;
  if (heap->exclusive_arena == NULL) {
    _mi_warning_message("cannot unload heaps that are not associated with an exclusive arena\n");
    return;
  }

  // abandon all pages so all thread'id in the pages are cleared
  _mi_heap_collect_abandon(heap);
  mi_assert_internal(heap->page_count==0);

  // remove from heap list
  mi_heap_free(heap, false /* but don't actually free the memory */);

  // disassociate from the current thread-local and static state
  heap->tld = NULL;
  return;
}

bool mi_heap_reload(mi_heap_t* heap, mi_arena_id_t arena_id) {
  mi_assert(mi_heap_is_initialized(heap));
  if (heap==NULL || !mi_heap_is_initialized(heap)) return false;
  if (heap->exclusive_arena == NULL) {
    _mi_warning_message("cannot reload heaps that were not associated with an exclusive arena\n");
    return false;
  }
  if (heap->tld != NULL) {
    _mi_warning_message("cannot reload heaps that were not unloaded first\n");
    return false;
  }
  mi_arena_t* arena = _mi_arena_from_id(arena_id);
  if (heap->exclusive_arena != arena) {
    _mi_warning_message("trying to reload a heap at a different arena address: %p vs %p\n", heap->exclusive_arena, arena);
    return false;
  }

  mi_assert_internal(heap->page_count==0);

  // re-associate with the current thread-local and static state
  heap->tld = mi_heap_get_default()->tld;

  // reinit direct pages (as we may be in a different process)
  mi_assert_internal(heap->page_count == 0);
  for (size_t i = 0; i < MI_PAGES_DIRECT; i++) {
    heap->pages_free_direct[i] = (mi_page_t*)&_mi_page_empty;
  }

  // push on the thread local heaps list
  heap->next = heap->tld->heaps;
  heap->tld->heaps = heap;
  return true;
}

/* -----------------------------------------------------------
  Analysis
----------------------------------------------------------- */

// static since it is not thread safe to access heaps from other threads.
static mi_heap_t* mi_heap_of_block(const void* p) {
  if (p == NULL) return NULL;
  mi_page_t* page = _mi_ptr_page(p); // TODO: check pointer validity?
  return mi_page_heap(page);
}

bool mi_heap_contains_block(mi_heap_t* heap, const void* p) {
  mi_assert(heap != NULL);
  if (heap==NULL || !mi_heap_is_initialized(heap)) return false;
  return (heap == mi_heap_of_block(p));
}


static bool mi_heap_page_check_owned(mi_heap_t* heap, mi_page_queue_t* pq, mi_page_t* page, void* p, void* vfound) {
  MI_UNUSED(heap);
  MI_UNUSED(pq);
  bool* found = (bool*)vfound;
  void* start = mi_page_start(page);
  void* end   = (uint8_t*)start + (page->capacity * mi_page_block_size(page));
  *found = (p >= start && p < end);
  return (!*found); // continue if not found
}

bool mi_heap_check_owned(mi_heap_t* heap, const void* p) {
  mi_assert(heap != NULL);
  if (heap==NULL || !mi_heap_is_initialized(heap)) return false;
  if (((uintptr_t)p & (MI_INTPTR_SIZE - 1)) != 0) return false;  // only aligned pointers
  bool found = false;
  mi_heap_visit_pages(heap, &mi_heap_page_check_owned, (void*)p, &found);
  return found;
}

bool mi_check_owned(const void* p) {
  return mi_heap_check_owned(mi_prim_get_default_heap(), p);
}

/* -----------------------------------------------------------
  Visit all heap blocks and areas
  Todo: enable visiting abandoned pages, and
        enable visiting all blocks of all heaps across threads
----------------------------------------------------------- */

void _mi_heap_area_init(mi_heap_area_t* area, mi_page_t* page) {
  const size_t bsize = mi_page_block_size(page);
  const size_t ubsize = mi_page_usable_block_size(page);
  area->reserved = page->reserved * bsize;
  area->committed = page->capacity * bsize;
  area->blocks = mi_page_start(page);
  area->used = page->used;   // number of blocks in use (#553)
  area->block_size = ubsize;
  area->full_block_size = bsize;
  area->heap_tag = page->heap_tag;
}


static void mi_get_fast_divisor(size_t divisor, uint64_t* magic, size_t* shift) {
  mi_assert_internal(divisor > 0 && divisor <= UINT32_MAX);
  *shift = MI_SIZE_BITS - mi_clz(divisor - 1);
  *magic = ((((uint64_t)1 << 32) * (((uint64_t)1 << *shift) - divisor)) / divisor + 1);
}

static size_t mi_fast_divide(size_t n, uint64_t magic, size_t shift) {
  mi_assert_internal(n <= UINT32_MAX);
  const uint64_t hi = ((uint64_t)n * magic) >> 32;
  return (size_t)((hi + n) >> shift);
}

bool _mi_heap_area_visit_blocks(const mi_heap_area_t* area, mi_page_t* page, mi_block_visit_fun* visitor, void* arg) {
  mi_assert(area != NULL);
  if (area==NULL) return true;
  mi_assert(page != NULL);
  if (page == NULL) return true;

  _mi_page_free_collect(page,true);              // collect both thread_delayed and local_free
  mi_assert_internal(page->local_free == NULL);
  if (page->used == 0) return true;

  size_t psize;
  uint8_t* const pstart = mi_page_area(page, &psize);
  mi_heap_t* const heap = mi_page_heap(page);
  const size_t bsize    = mi_page_block_size(page);
  const size_t ubsize   = mi_page_usable_block_size(page); // without padding

  // optimize page with one block
  if (page->capacity == 1) {
    mi_assert_internal(page->used == 1 && page->free == NULL);
    return visitor(mi_page_heap(page), area, pstart, ubsize, arg);
  }
  mi_assert(bsize <= UINT32_MAX);

  // optimize full pages
  if (page->used == page->capacity) {
    uint8_t* block = pstart;
    for (size_t i = 0; i < page->capacity; i++) {
      if (!visitor(heap, area, block, ubsize, arg)) return false;
      block += bsize;
    }
    return true;
  }

  // create a bitmap of free blocks.
  #define MI_MAX_BLOCKS   (MI_SMALL_PAGE_SIZE / sizeof(void*))
  uintptr_t free_map[MI_MAX_BLOCKS / MI_INTPTR_BITS];
  const uintptr_t bmapsize = _mi_divide_up(page->capacity, MI_INTPTR_BITS);
  memset(free_map, 0, bmapsize * sizeof(intptr_t));
  if (page->capacity % MI_INTPTR_BITS != 0) {
    // mark left-over bits at the end as free
    size_t shift   = (page->capacity % MI_INTPTR_BITS);
    uintptr_t mask = (UINTPTR_MAX << shift);
    free_map[bmapsize - 1] = mask;
  }

  // fast repeated division by the block size
  uint64_t magic;
  size_t   shift;
  mi_get_fast_divisor(bsize, &magic, &shift);

  #if MI_DEBUG>1
  size_t free_count = 0;
  #endif
  for (mi_block_t* block = page->free; block != NULL; block = mi_block_next(page, block)) {
    #if MI_DEBUG>1
    free_count++;
    #endif
    mi_assert_internal((uint8_t*)block >= pstart && (uint8_t*)block < (pstart + psize));
    size_t offset = (uint8_t*)block - pstart;
    mi_assert_internal(offset % bsize == 0);
    mi_assert_internal(offset <= UINT32_MAX);
    size_t blockidx = mi_fast_divide(offset, magic, shift);
    mi_assert_internal(blockidx == offset / bsize);
    mi_assert_internal(blockidx < MI_MAX_BLOCKS);
    size_t bitidx = (blockidx / MI_INTPTR_BITS);
    size_t bit = blockidx - (bitidx * MI_INTPTR_BITS);
    free_map[bitidx] |= ((uintptr_t)1 << bit);
  }
  mi_assert_internal(page->capacity == (free_count + page->used));

  // walk through all blocks skipping the free ones
  #if MI_DEBUG>1
  size_t used_count = 0;
  #endif
  uint8_t* block = pstart;
  for (size_t i = 0; i < bmapsize; i++) {
    if (free_map[i] == 0) {
      // every block is in use
      for (size_t j = 0; j < MI_INTPTR_BITS; j++) {
        #if MI_DEBUG>1
        used_count++;
        #endif
        if (!visitor(heap, area, block, ubsize, arg)) return false;
        block += bsize;
      }
    }
    else {
      // visit the used blocks in the mask
      uintptr_t m = ~free_map[i];
      while (m != 0) {
        #if MI_DEBUG>1
        used_count++;
        #endif
        size_t bitidx = mi_ctz(m);
        if (!visitor(heap, area, block + (bitidx * bsize), ubsize, arg)) return false;
        m &= m - 1;  // clear least significant bit
      }
      block += bsize * MI_INTPTR_BITS;
    }
  }
  mi_assert_internal(page->used == used_count);
  return true;
}



// Separate struct to keep `mi_page_t` out of the public interface
typedef struct mi_heap_area_ex_s {
  mi_heap_area_t area;
  mi_page_t* page;
} mi_heap_area_ex_t;

typedef bool (mi_heap_area_visit_fun)(const mi_heap_t* heap, const mi_heap_area_ex_t* area, void* arg);

static bool mi_heap_visit_areas_page(mi_heap_t* heap, mi_page_queue_t* pq, mi_page_t* page, void* vfun, void* arg) {
  MI_UNUSED(heap);
  MI_UNUSED(pq);
  mi_heap_area_visit_fun* fun = (mi_heap_area_visit_fun*)vfun;
  mi_heap_area_ex_t xarea;
  xarea.page = page;
  _mi_heap_area_init(&xarea.area, page);
  return fun(heap, &xarea, arg);
}

// Visit all heap pages as areas
static bool mi_heap_visit_areas(const mi_heap_t* heap, mi_heap_area_visit_fun* visitor, void* arg) {
  if (visitor == NULL) return false;
  return mi_heap_visit_pages((mi_heap_t*)heap, &mi_heap_visit_areas_page, (void*)(visitor), arg); // note: function pointer to void* :-{
}

// Just to pass arguments
typedef struct mi_visit_blocks_args_s {
  bool  visit_blocks;
  mi_block_visit_fun* visitor;
  void* arg;
} mi_visit_blocks_args_t;

static bool mi_heap_area_visitor(const mi_heap_t* heap, const mi_heap_area_ex_t* xarea, void* arg) {
  mi_visit_blocks_args_t* args = (mi_visit_blocks_args_t*)arg;
  if (!args->visitor(heap, &xarea->area, NULL, xarea->area.block_size, args->arg)) return false;
  if (args->visit_blocks) {
    return _mi_heap_area_visit_blocks(&xarea->area, xarea->page, args->visitor, args->arg);
  }
  else {
    return true;
  }
}

// Visit all blocks in a heap
bool mi_heap_visit_blocks(const mi_heap_t* heap, bool visit_blocks, mi_block_visit_fun* visitor, void* arg) {
  mi_visit_blocks_args_t args = { visit_blocks, visitor, arg };
  return mi_heap_visit_areas(heap, &mi_heap_area_visitor, &args);
}
