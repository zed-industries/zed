/* ----------------------------------------------------------------------------
Copyright (c) 2018-2024, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
#if !defined(MI_IN_ALLOC_C)
#error "this file should be included from 'alloc.c' (so aliases can work from alloc-override)"
// add includes help an IDE
#include "mimalloc.h"
#include "mimalloc/internal.h"
#include "mimalloc/prim.h"   // _mi_prim_thread_id()
#endif

// forward declarations
static void   mi_check_padding(const mi_page_t* page, const mi_block_t* block);
static bool   mi_check_is_double_free(const mi_page_t* page, const mi_block_t* block);
static size_t mi_page_usable_size_of(const mi_page_t* page, const mi_block_t* block);
static void   mi_stat_free(const mi_page_t* page, const mi_block_t* block);


// ------------------------------------------------------
// Free
// ------------------------------------------------------

// regular free of a (thread local) block pointer
// fast path written carefully to prevent spilling on the stack
static inline void mi_free_block_local(mi_page_t* page, mi_block_t* block, bool track_stats, bool check_full)
{
  // checks
  if mi_unlikely(mi_check_is_double_free(page, block)) return;
  mi_check_padding(page, block);
  if (track_stats) { mi_stat_free(page, block); }
  #if (MI_DEBUG>0) && !MI_TRACK_ENABLED  && !MI_TSAN && !MI_GUARDED
  memset(block, MI_DEBUG_FREED, mi_page_block_size(page));
  #endif
  if (track_stats) { mi_track_free_size(block, mi_page_usable_size_of(page, block)); } // faster then mi_usable_size as we already know the page and that p is unaligned

  // actual free: push on the local free list
  mi_block_set_next(page, block, page->local_free);
  page->local_free = block;
  if mi_unlikely(--page->used == 0) {
    _mi_page_retire(page);
  }
  else if mi_unlikely(check_full && mi_page_is_in_full(page)) {
    _mi_page_unfull(page);
  }
}

// Forward declaration for multi-threaded collect
static void mi_decl_noinline mi_free_try_collect_mt(mi_page_t* page, mi_block_t* mt_free) mi_attr_noexcept;

// Free a block multi-threaded
static inline void mi_free_block_mt(mi_page_t* page, mi_block_t* block) mi_attr_noexcept
{
  // adjust stats (after padding check and potentially recursive `mi_free` above)
  mi_stat_free(page, block);    // stat_free may access the padding
  mi_track_free_size(block, mi_page_usable_size_of(page, block));

  // _mi_padding_shrink(page, block, sizeof(mi_block_t));
#if (MI_DEBUG>0) && !MI_TRACK_ENABLED  && !MI_TSAN       // note: when tracking, cannot use mi_usable_size with multi-threading
  size_t dbgsize = mi_usable_size(block);
  if (dbgsize > MI_MiB) { dbgsize = MI_MiB; }
  _mi_memset_aligned(block, MI_DEBUG_FREED, dbgsize);
#endif

  // push atomically on the page thread free list
  mi_thread_free_t tf_new;
  mi_thread_free_t tf_old = mi_atomic_load_relaxed(&page->xthread_free);
  do {
    mi_block_set_next(page, block, mi_tf_block(tf_old));
    tf_new = mi_tf_create(block, true /* always use owned: try to claim it if the page is abandoned */);
  } while (!mi_atomic_cas_weak_acq_rel(&page->xthread_free, &tf_old, tf_new)); // todo: release is enough?

  // and atomically try to collect the page if it was abandoned
  const bool is_owned_now = !mi_tf_is_owned(tf_old);
  if (is_owned_now) {
    mi_assert_internal(mi_page_is_abandoned(page));
    mi_free_try_collect_mt(page,block);
  }
}


// Adjust a block that was allocated aligned, to the actual start of the block in the page.
// note: this can be called from `mi_free_generic_mt` where a non-owning thread accesses the
// `page_start` and `block_size` fields; however these are constant and the page won't be
// deallocated (as the block we are freeing keeps it alive) and thus safe to read concurrently.
mi_block_t* _mi_page_ptr_unalign(const mi_page_t* page, const void* p) {
  mi_assert_internal(page!=NULL && p!=NULL);

  size_t diff = (uint8_t*)p - mi_page_start(page);
  size_t adjust;
  if mi_likely(page->block_size_shift != 0) {
    adjust = diff & (((size_t)1 << page->block_size_shift) - 1);
  }
  else {
    adjust = diff % mi_page_block_size(page);
  }

  return (mi_block_t*)((uintptr_t)p - adjust);
}

// forward declaration for a MI_GUARDED build
#if MI_GUARDED
static void mi_block_unguard(mi_page_t* page, mi_block_t* block, void* p); // forward declaration
static inline void mi_block_check_unguard(mi_page_t* page, mi_block_t* block, void* p) {
  if (mi_block_ptr_is_guarded(block, p)) { mi_block_unguard(page, block, p); }
}
#else
static inline void mi_block_check_unguard(mi_page_t* page, mi_block_t* block, void* p) {
  MI_UNUSED(page); MI_UNUSED(block); MI_UNUSED(p);
}
#endif


// free a local pointer  (page parameter comes first for better codegen)
static void mi_decl_noinline mi_free_generic_local(mi_page_t* page, void* p) mi_attr_noexcept {
  mi_assert_internal(p!=NULL && page != NULL);
  mi_block_t* const block = (mi_page_has_aligned(page) ? _mi_page_ptr_unalign(page, p) : (mi_block_t*)p);
  mi_block_check_unguard(page, block, p);
  mi_free_block_local(page, block, true /* track stats */, true /* check for a full page */);
}

// free a pointer owned by another thread (page parameter comes first for better codegen)
static void mi_decl_noinline mi_free_generic_mt(mi_page_t* page, void* p) mi_attr_noexcept {
  mi_assert_internal(p!=NULL && page != NULL);
  mi_block_t* const block = _mi_page_ptr_unalign(page, p); // don't check `has_aligned` flag to avoid a race (issue #865)
  mi_block_check_unguard(page, block, p);
  mi_free_block_mt(page, block);
}

// generic free (for runtime integration)
void mi_decl_noinline _mi_free_generic(mi_page_t* page, bool is_local, void* p) mi_attr_noexcept {
  if (is_local) mi_free_generic_local(page,p);
           else mi_free_generic_mt(page,p);
}


// Get the page belonging to a pointer
// Does further checks in debug mode to see if this was a valid pointer.
static inline mi_page_t* mi_validate_ptr_page(const void* p, const char* msg)
{
  MI_UNUSED_RELEASE(msg);
  #if MI_DEBUG
  if mi_unlikely(((uintptr_t)p & (MI_INTPTR_SIZE - 1)) != 0 && !mi_option_is_enabled(mi_option_guarded_precise)) {
    _mi_error_message(EINVAL, "%s: invalid (unaligned) pointer: %p\n", msg, p);
    return NULL;
  }
  mi_page_t* page = _mi_safe_ptr_page(p);
  if (p != NULL && page == NULL) {
    _mi_error_message(EINVAL, "%s: invalid pointer: %p\n", msg, p);
  }
  return page;
  #else
  return _mi_ptr_page(p);
  #endif
}

// Free a block
// Fast path written carefully to prevent register spilling on the stack
void mi_free(void* p) mi_attr_noexcept
{
  mi_page_t* const page = mi_validate_ptr_page(p,"mi_free");
  if mi_unlikely(page==NULL) return;  // page will be NULL if p==NULL
  mi_assert_internal(p!=NULL && page!=NULL);

  const mi_threadid_t xtid = (_mi_prim_thread_id() ^ mi_page_xthread_id(page));
  if mi_likely(xtid == 0) {                        // `tid == mi_page_thread_id(page) && mi_page_flags(page) == 0`
    // thread-local, aligned, and not a full page
    mi_block_t* const block = (mi_block_t*)p;
    mi_free_block_local(page, block, true /* track stats */, false /* no need to check if the page is full */);
  }
  else if (xtid <= MI_PAGE_FLAG_MASK) {            // `tid == mi_page_thread_id(page) && mi_page_flags(page) != 0`
    // page is local, but is full or contains (inner) aligned blocks; use generic path
    mi_free_generic_local(page, p);
  }
  // free-ing in a page owned by a heap in another thread, or an abandoned page (not belonging to a heap)
  else if ((xtid & MI_PAGE_FLAG_MASK) == 0) {      // `tid != mi_page_thread_id(page) && mi_page_flags(page) == 0`
    // blocks are aligned (and not a full page); push on the thread_free list
    mi_block_t* const block = (mi_block_t*)p;
    mi_free_block_mt(page,block);
  }
  else {
    // page is full or contains (inner) aligned blocks; use generic multi-thread path
    mi_free_generic_mt(page, p);
  }
}


// ------------------------------------------------------
// Multi-threaded Free (`_mt`)
// ------------------------------------------------------
static bool mi_page_unown_from_free(mi_page_t* page, mi_block_t* mt_free);

static inline bool mi_page_queue_len_is_atmost( mi_heap_t* heap, size_t block_size, long atmost) {
  mi_page_queue_t* const pq = mi_page_queue(heap,block_size);
  mi_assert_internal(pq!=NULL);
  return (pq->count <= (size_t)atmost);  
}

static void mi_decl_noinline mi_free_try_collect_mt(mi_page_t* page, mi_block_t* mt_free) mi_attr_noexcept {
  mi_assert_internal(mi_page_is_owned(page));
  mi_assert_internal(mi_page_is_abandoned(page));

  // we own the page now..
  // safe to collect the thread atomic free list
  // use the `_partly` version to avoid atomic operations since we already have the `mt_free` pointing into the thread free list
  _mi_page_free_collect_partly(page, mt_free);

  #if MI_DEBUG > 1
  if (mi_page_is_singleton(page)) { mi_assert_internal(mi_page_all_free(page)); }
  #endif

  // 1. free if the page is free now  (this is updated by `_mi_page_free_collect_partly`)
  if (mi_page_all_free(page))
  {
    // first remove it from the abandoned pages in the arena (if mapped, this might wait for any readers to finish)
    _mi_arenas_page_unabandon(page);
    // we can free the page directly
    _mi_arenas_page_free(page,NULL);
    return;
  }

  // 2. we can try to reclaim the page for ourselves
  // note: reclaiming can improve benchmarks like `larson` or `rbtree-ck` a lot even in the single-threaded case,
  // since free-ing from an owned page avoids atomic operations. However, if we reclaim too eagerly in 
  // a multi-threaded scenario we may start to hold on to too much memory and reduce reuse among threads.
  // If the current heap is where the page originally came from, we reclaim much more eagerly while
  // 'cross-thread' reclaiming on free is by default off (and we only 'reclaim' these by finding the abandoned 
  // pages when we allocate a fresh page).
  if (page->block_size <= MI_SMALL_MAX_OBJ_SIZE)       // only for small sized blocks
  {
    const long reclaim_on_free = _mi_option_get_fast(mi_option_page_reclaim_on_free);
    if (reclaim_on_free >= 0) {                        // and reclaiming is allowed
      // get our heap (with the right tag)
      // note: don't use `mi_heap_get_default()` as we may just have terminated this thread and we should
      // not reinitialize the heap for this thread. (can happen due to thread-local destructors for example -- issue #944)
      mi_heap_t* heap = mi_prim_get_default_heap();
      if (heap != page->heap) {
        if (mi_heap_is_initialized(heap)) {
          heap = _mi_heap_by_tag(heap, page->heap_tag);
        }
      }
      // can we reclaim into this heap?      
      if (heap != NULL && heap->allow_page_reclaim)      
      {
        long max_reclaim = 0;
        if mi_likely(heap == page->heap) {  // did this page originate from the current heap?
          // originating heap
          max_reclaim = _mi_option_get_fast(heap->tld->is_in_threadpool ? mi_option_page_cross_thread_max_reclaim : mi_option_page_max_reclaim);          
        }
        else if (reclaim_on_free == 1 &&              // if cross-thread is allowed
                 !heap->tld->is_in_threadpool &&      // and we are not part of a threadpool
                 !mi_page_is_used_at_frac(page,8) &&  // and the page is not too full
                 _mi_arena_memid_is_suitable(page->memid, heap->exclusive_arena)) {   // and it fits our memory
          // across threads
          max_reclaim = _mi_option_get_fast(mi_option_page_cross_thread_max_reclaim);
        }
        
        if (max_reclaim < 0 || mi_page_queue_len_is_atmost(heap, page->block_size, max_reclaim)) { // are we within the reclaim limit?
          // reclaim the page into this heap
          // first remove it from the abandoned pages in the arena -- this might wait for any readers to finish
          _mi_arenas_page_unabandon(page);
          _mi_heap_page_reclaim(heap, page);
          mi_heap_stat_counter_increase(heap, pages_reclaim_on_free, 1);
          return;
        }
      }
    }
  }

  // 3. if the page is unmapped, try to reabandon so it can possibly be mapped and found for allocations
  if (!mi_page_is_used_at_frac(page, 8) &&  // only reabandon if a full page starts to have enough blocks available to prevent immediate re-abandon of a full page
      !mi_page_is_abandoned_mapped(page) && page->memid.memkind == MI_MEM_ARENA &&
      _mi_arenas_page_try_reabandon_to_mapped(page))
  {
    return;
  }


  // not reclaimed or free'd, unown again
  // _mi_page_unown(page);
  mi_page_unown_from_free(page, mt_free);
}


// release ownership of a page. This may free the page if all (other) blocks were concurrently
// freed in the meantime. Returns true if the page was freed.
// This is a specialized version of `mi_page_unown` to (try to) avoid calling `mi_page_free_collect` again.
static bool mi_page_unown_from_free(mi_page_t* page, mi_block_t* mt_free) {
  mi_assert_internal(mi_page_is_owned(page));
  mi_assert_internal(mi_page_is_abandoned(page));
  mi_assert_internal(mt_free != NULL);
  mi_assert_internal(page->used > 1);
  mi_thread_free_t tf_expect = mi_tf_create(mt_free, true);
  mi_thread_free_t tf_new    = mi_tf_create(mt_free, false);
  while mi_unlikely(!mi_atomic_cas_weak_acq_rel(&page->xthread_free, &tf_expect, tf_new)) {
    mi_assert_internal(mi_tf_is_owned(tf_expect));
    while (mi_tf_block(tf_expect) != NULL) {
      _mi_page_free_collect(page,false);  // update used
      if (mi_page_all_free(page)) {   // it may become free just before unowning it
        _mi_arenas_page_unabandon(page);
        _mi_arenas_page_free(page,NULL);
        return true;
      }
      tf_expect = mi_atomic_load_relaxed(&page->xthread_free);
    }
    mi_assert_internal(mi_tf_block(tf_expect)==NULL);
    tf_new = mi_tf_create(NULL, false);
  }
  return false;
}


// ------------------------------------------------------
// Usable size
// ------------------------------------------------------

// Bytes available in a block
static size_t mi_decl_noinline mi_page_usable_aligned_size_of(const mi_page_t* page, const void* p) mi_attr_noexcept {
  const mi_block_t* block = _mi_page_ptr_unalign(page, p);
  const size_t size = mi_page_usable_size_of(page, block);
  const ptrdiff_t adjust = (uint8_t*)p - (uint8_t*)block;
  mi_assert_internal(adjust >= 0 && (size_t)adjust <= size);
  const size_t aligned_size = (size - adjust);
  #if MI_GUARDED
  if (mi_block_ptr_is_guarded(block, p)) {
    return aligned_size - _mi_os_page_size();
  }
  #endif
  return aligned_size;
}

static inline size_t _mi_usable_size(const void* p, const char* msg) mi_attr_noexcept {
  const mi_page_t* const page = mi_validate_ptr_page(p,msg);
  if mi_unlikely(page==NULL) return 0;
  if mi_likely(!mi_page_has_aligned(page)) {
    const mi_block_t* block = (const mi_block_t*)p;
    return mi_page_usable_size_of(page, block);
  }
  else {
    // split out to separate routine for improved code generation
    return mi_page_usable_aligned_size_of(page, p);
  }
}

mi_decl_nodiscard size_t mi_usable_size(const void* p) mi_attr_noexcept {
  return _mi_usable_size(p, "mi_usable_size");
}


// ------------------------------------------------------
// Free variants
// ------------------------------------------------------

void mi_free_size(void* p, size_t size) mi_attr_noexcept {
  MI_UNUSED_RELEASE(size);
  #if MI_DEBUG
  const size_t available = _mi_usable_size(p,"mi_free_size");
  mi_assert(p == NULL || size <= available || available == 0 /* invalid pointer */ );
  #endif
  mi_free(p);
}

void mi_free_size_aligned(void* p, size_t size, size_t alignment) mi_attr_noexcept {
  MI_UNUSED_RELEASE(alignment);
  mi_assert(((uintptr_t)p % alignment) == 0);
  mi_free_size(p,size);
}

void mi_free_aligned(void* p, size_t alignment) mi_attr_noexcept {
  MI_UNUSED_RELEASE(alignment);
  mi_assert(((uintptr_t)p % alignment) == 0);
  mi_free(p);
}


// ------------------------------------------------------
// Check for double free in secure and debug mode
// This is somewhat expensive so only enabled for secure mode 4
// ------------------------------------------------------

#if (MI_ENCODE_FREELIST && (MI_SECURE>=4 || MI_DEBUG!=0))
// linear check if the free list contains a specific element
static bool mi_list_contains(const mi_page_t* page, const mi_block_t* list, const mi_block_t* elem) {
  while (list != NULL) {
    if (elem==list) return true;
    list = mi_block_next(page, list);
  }
  return false;
}

static mi_decl_noinline bool mi_check_is_double_freex(const mi_page_t* page, const mi_block_t* block) {
  // The decoded value is in the same page (or NULL).
  // Walk the free lists to verify positively if it is already freed
  if (mi_list_contains(page, page->free, block) ||
      mi_list_contains(page, page->local_free, block) ||
      mi_list_contains(page, mi_page_thread_free(page), block))
  {
    _mi_error_message(EAGAIN, "double free detected of block %p with size %zu\n", block, mi_page_block_size(page));
    return true;
  }
  return false;
}

#define mi_track_page(page,access)  { size_t psize; void* pstart = _mi_page_start(_mi_page_segment(page),page,&psize); mi_track_mem_##access( pstart, psize); }

static inline bool mi_check_is_double_free(const mi_page_t* page, const mi_block_t* block) {
  bool is_double_free = false;
  mi_block_t* n = mi_block_nextx(page, block, page->keys); // pretend it is freed, and get the decoded first field
  if (((uintptr_t)n & (MI_INTPTR_SIZE-1))==0 &&  // quick check: aligned pointer?
      (n==NULL || mi_is_in_same_page(block, n))) // quick check: in same page or NULL?
  {
    // Suspicious: decoded value a in block is in the same page (or NULL) -- maybe a double free?
    // (continue in separate function to improve code generation)
    is_double_free = mi_check_is_double_freex(page, block);
  }
  return is_double_free;
}
#else
static inline bool mi_check_is_double_free(const mi_page_t* page, const mi_block_t* block) {
  MI_UNUSED(page);
  MI_UNUSED(block);
  return false;
}
#endif


// ---------------------------------------------------------------------------
// Check for heap block overflow by setting up padding at the end of the block
// ---------------------------------------------------------------------------

#if MI_PADDING // && !MI_TRACK_ENABLED
static bool mi_page_decode_padding(const mi_page_t* page, const mi_block_t* block, size_t* delta, size_t* bsize) {
  *bsize = mi_page_usable_block_size(page);
  const mi_padding_t* const padding = (mi_padding_t*)((uint8_t*)block + *bsize);
  mi_track_mem_defined(padding,sizeof(mi_padding_t));
  *delta = padding->delta;
  uint32_t canary = padding->canary;
  uintptr_t keys[2];
  keys[0] = page->keys[0];
  keys[1] = page->keys[1];
  bool ok = (mi_ptr_encode_canary(page,block,keys) == canary && *delta <= *bsize);
  mi_track_mem_noaccess(padding,sizeof(mi_padding_t));
  return ok;
}

// Return the exact usable size of a block.
static size_t mi_page_usable_size_of(const mi_page_t* page, const mi_block_t* block) {
  size_t bsize;
  size_t delta;
  bool ok = mi_page_decode_padding(page, block, &delta, &bsize);
  mi_assert_internal(ok); mi_assert_internal(delta <= bsize);
  return (ok ? bsize - delta : 0);
}

// When a non-thread-local block is freed, it becomes part of the thread delayed free
// list that is freed later by the owning heap. If the exact usable size is too small to
// contain the pointer for the delayed list, then shrink the padding (by decreasing delta)
// so it will later not trigger an overflow error in `mi_free_block`.
void _mi_padding_shrink(const mi_page_t* page, const mi_block_t* block, const size_t min_size) {
  size_t bsize;
  size_t delta;
  bool ok = mi_page_decode_padding(page, block, &delta, &bsize);
  mi_assert_internal(ok);
  if (!ok || (bsize - delta) >= min_size) return;  // usually already enough space
  mi_assert_internal(bsize >= min_size);
  if (bsize < min_size) return;  // should never happen
  size_t new_delta = (bsize - min_size);
  mi_assert_internal(new_delta < bsize);
  mi_padding_t* padding = (mi_padding_t*)((uint8_t*)block + bsize);
  mi_track_mem_defined(padding,sizeof(mi_padding_t));
  padding->delta = (uint32_t)new_delta;
  mi_track_mem_noaccess(padding,sizeof(mi_padding_t));
}
#else
static size_t mi_page_usable_size_of(const mi_page_t* page, const mi_block_t* block) {
  MI_UNUSED(block);
  return mi_page_usable_block_size(page);
}

void _mi_padding_shrink(const mi_page_t* page, const mi_block_t* block, const size_t min_size) {
  MI_UNUSED(page);
  MI_UNUSED(block);
  MI_UNUSED(min_size);
}
#endif

#if MI_PADDING && MI_PADDING_CHECK

static bool mi_verify_padding(const mi_page_t* page, const mi_block_t* block, size_t* size, size_t* wrong) {
  size_t bsize;
  size_t delta;
  bool ok = mi_page_decode_padding(page, block, &delta, &bsize);
  *size = *wrong = bsize;
  if (!ok) return false;
  mi_assert_internal(bsize >= delta);
  *size = bsize - delta;
  if (!mi_page_is_huge(page)) {
    uint8_t* fill = (uint8_t*)block + bsize - delta;
    const size_t maxpad = (delta > MI_MAX_ALIGN_SIZE ? MI_MAX_ALIGN_SIZE : delta); // check at most the first N padding bytes
    mi_track_mem_defined(fill, maxpad);
    for (size_t i = 0; i < maxpad; i++) {
      if (fill[i] != MI_DEBUG_PADDING) {
        *wrong = bsize - delta + i;
        ok = false;
        break;
      }
    }
    mi_track_mem_noaccess(fill, maxpad);
  }
  return ok;
}

static void mi_check_padding(const mi_page_t* page, const mi_block_t* block) {
  size_t size;
  size_t wrong;
  if (!mi_verify_padding(page,block,&size,&wrong)) {
    _mi_error_message(EFAULT, "buffer overflow in heap block %p of size %zu: write after %zu bytes\n", block, size, wrong );
  }
}

#else

static void mi_check_padding(const mi_page_t* page, const mi_block_t* block) {
  MI_UNUSED(page);
  MI_UNUSED(block);
}

#endif

// only maintain stats for smaller objects if requested
#if (MI_STAT>0)
static void mi_stat_free(const mi_page_t* page, const mi_block_t* block) {
  MI_UNUSED(block);
  mi_heap_t* const heap = mi_heap_get_default();
  const size_t bsize = mi_page_usable_block_size(page);
  // #if (MI_STAT>1)
  // const size_t usize = mi_page_usable_size_of(page, block);
  // mi_heap_stat_decrease(heap, malloc_requested, usize);
  // #endif
  if (bsize <= MI_LARGE_MAX_OBJ_SIZE) {
    mi_heap_stat_decrease(heap, malloc_normal, bsize);
    #if (MI_STAT > 1)
    mi_heap_stat_decrease(heap, malloc_bins[_mi_bin(bsize)], 1);
    #endif
  }
  else {
    const size_t bpsize = mi_page_block_size(page);  // match stat in page.c:mi_huge_page_alloc
    mi_heap_stat_decrease(heap, malloc_huge, bpsize);
  }
}
#else
void mi_stat_free(const mi_page_t* page, const mi_block_t* block) {
  MI_UNUSED(page); MI_UNUSED(block);
}
#endif


// Remove guard page when building with MI_GUARDED
#if MI_GUARDED
static void mi_block_unguard(mi_page_t* page, mi_block_t* block, void* p) {
  MI_UNUSED(p);
  mi_assert_internal(mi_block_ptr_is_guarded(block, p));
  mi_assert_internal(mi_page_has_aligned(page));
  mi_assert_internal((uint8_t*)p - (uint8_t*)block >= (ptrdiff_t)sizeof(mi_block_t));
  mi_assert_internal(block->next == MI_BLOCK_TAG_GUARDED);

  const size_t bsize = mi_page_block_size(page);
  const size_t psize = _mi_os_page_size();
  mi_assert_internal(bsize > psize);
  mi_assert_internal(!page->memid.is_pinned);
  void* gpage = (uint8_t*)block + bsize - psize;
  mi_assert_internal(_mi_is_aligned(gpage, psize));
  _mi_os_unprotect(gpage, psize);
}
#endif
