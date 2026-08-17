/* ----------------------------------------------------------------------------
Copyright (c) 2019-2023, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/

/* -----------------------------------------------------------
  The following functions are to reliably find the segment or
  block that encompasses any pointer p (or NULL if it is not
  in any of our segments).
  We maintain a bitmap of all memory with 1 bit per MI_SEGMENT_SIZE (64MiB)
  set to 1 if it contains the segment meta data.
----------------------------------------------------------- */
#include "mimalloc.h"
#include "mimalloc/internal.h"
#include "mimalloc/atomic.h"

// Reduce total address space to reduce .bss  (due to the `mi_segment_map`)
#if (MI_INTPTR_SIZE > 4) && MI_TRACK_ASAN
#define MI_SEGMENT_MAP_MAX_ADDRESS    (128*1024ULL*MI_GiB)  // 128 TiB  (see issue #881)
#elif (MI_INTPTR_SIZE > 4)
#define MI_SEGMENT_MAP_MAX_ADDRESS    (48*1024ULL*MI_GiB)   // 48 TiB
#else
#define MI_SEGMENT_MAP_MAX_ADDRESS    (UINT32_MAX)
#endif

#define MI_SEGMENT_MAP_PART_SIZE      (MI_INTPTR_SIZE*MI_KiB - 128)      // 128 > sizeof(mi_memid_t) ! 
#define MI_SEGMENT_MAP_PART_BITS      (8*MI_SEGMENT_MAP_PART_SIZE)
#define MI_SEGMENT_MAP_PART_ENTRIES   (MI_SEGMENT_MAP_PART_SIZE / MI_INTPTR_SIZE)
#define MI_SEGMENT_MAP_PART_BIT_SPAN  (MI_SEGMENT_ALIGN)                 // memory area covered by 1 bit

#if (MI_SEGMENT_MAP_PART_BITS < (MI_SEGMENT_MAP_MAX_ADDRESS / MI_SEGMENT_MAP_PART_BIT_SPAN)) // prevent overflow on 32-bit (issue #1017)
#define MI_SEGMENT_MAP_PART_SPAN      (MI_SEGMENT_MAP_PART_BITS * MI_SEGMENT_MAP_PART_BIT_SPAN)
#else
#define MI_SEGMENT_MAP_PART_SPAN      MI_SEGMENT_MAP_MAX_ADDRESS
#endif

#define MI_SEGMENT_MAP_MAX_PARTS      ((MI_SEGMENT_MAP_MAX_ADDRESS / MI_SEGMENT_MAP_PART_SPAN) + 1)

// A part of the segment map.
typedef struct mi_segmap_part_s {
  mi_memid_t memid;
  _Atomic(uintptr_t) map[MI_SEGMENT_MAP_PART_ENTRIES];
} mi_segmap_part_t;

// Allocate parts on-demand to reduce .bss footprint
static _Atomic(mi_segmap_part_t*) mi_segment_map[MI_SEGMENT_MAP_MAX_PARTS]; // = { NULL, .. }

static mi_segmap_part_t* mi_segment_map_index_of(const mi_segment_t* segment, bool create_on_demand, size_t* idx, size_t* bitidx) {
  // note: segment can be invalid or NULL.
  mi_assert_internal(_mi_ptr_segment(segment + 1) == segment); // is it aligned on MI_SEGMENT_SIZE?
  *idx = 0;
  *bitidx = 0;  
  if ((uintptr_t)segment >= MI_SEGMENT_MAP_MAX_ADDRESS) return NULL;
  const uintptr_t segindex = ((uintptr_t)segment) / MI_SEGMENT_MAP_PART_SPAN;
  if (segindex >= MI_SEGMENT_MAP_MAX_PARTS) return NULL;
  mi_segmap_part_t* part = mi_atomic_load_ptr_relaxed(mi_segmap_part_t, &mi_segment_map[segindex]);

  // allocate on demand to reduce .bss footprint
  if mi_unlikely(part == NULL) {
    if (!create_on_demand) return NULL;
    mi_memid_t memid;
    part = (mi_segmap_part_t*)_mi_os_zalloc(sizeof(mi_segmap_part_t), &memid);
    if (part == NULL) return NULL;
    part->memid = memid;
    mi_segmap_part_t* expected = NULL;
    if (!mi_atomic_cas_ptr_strong_release(mi_segmap_part_t, &mi_segment_map[segindex], &expected, part)) {
      _mi_os_free(part, sizeof(mi_segmap_part_t), memid);
      part = expected;
      if (part == NULL) return NULL;
    }
  }
  mi_assert(part != NULL);
  const uintptr_t offset = ((uintptr_t)segment) % MI_SEGMENT_MAP_PART_SPAN;
  const uintptr_t bitofs = offset / MI_SEGMENT_MAP_PART_BIT_SPAN;
  *idx = bitofs / MI_INTPTR_BITS;
  *bitidx = bitofs % MI_INTPTR_BITS;
  return part;
}

void _mi_segment_map_allocated_at(const mi_segment_t* segment) {
  if (segment->memid.memkind == MI_MEM_ARENA) return; // we lookup segments first in the arena's and don't need the segment map
  size_t index;
  size_t bitidx;
  mi_segmap_part_t* part = mi_segment_map_index_of(segment, true /* alloc map if needed */, &index, &bitidx);
  if (part == NULL) return; // outside our address range..
  uintptr_t mask = mi_atomic_load_relaxed(&part->map[index]);
  uintptr_t newmask;
  do {
    newmask = (mask | ((uintptr_t)1 << bitidx));
  } while (!mi_atomic_cas_weak_release(&part->map[index], &mask, newmask));
}

void _mi_segment_map_freed_at(const mi_segment_t* segment) {
  if (segment->memid.memkind == MI_MEM_ARENA) return;
  size_t index;
  size_t bitidx;
  mi_segmap_part_t* part = mi_segment_map_index_of(segment, false /* don't alloc if not present */, &index, &bitidx);
  if (part == NULL) return; // outside our address range..
  uintptr_t mask = mi_atomic_load_relaxed(&part->map[index]);
  uintptr_t newmask;
  do {
    newmask = (mask & ~((uintptr_t)1 << bitidx));
  } while (!mi_atomic_cas_weak_release(&part->map[index], &mask, newmask));
}

// Determine the segment belonging to a pointer or NULL if it is not in a valid segment.
static mi_segment_t* _mi_segment_of(const void* p) {
  if (p == NULL) return NULL;
  mi_segment_t* segment = _mi_ptr_segment(p);  // segment can be NULL  
  size_t index;
  size_t bitidx;
  mi_segmap_part_t* part = mi_segment_map_index_of(segment, false /* dont alloc if not present */, &index, &bitidx);
  if (part == NULL) return NULL;  
  const uintptr_t mask = mi_atomic_load_relaxed(&part->map[index]);
  if mi_likely((mask & ((uintptr_t)1 << bitidx)) != 0) {
    bool cookie_ok = (_mi_ptr_cookie(segment) == segment->cookie);
    mi_assert_internal(cookie_ok); MI_UNUSED(cookie_ok);
    return segment; // yes, allocated by us
  }
  return NULL;
}

// Is this a valid pointer in our heap?
static bool mi_is_valid_pointer(const void* p) {
  // first check if it is in an arena, then check if it is OS allocated
  return (_mi_arena_contains(p) || _mi_segment_of(p) != NULL);
}

mi_decl_nodiscard mi_decl_export bool mi_is_in_heap_region(const void* p) mi_attr_noexcept {
  return mi_is_valid_pointer(p);
}

void _mi_segment_map_unsafe_destroy(void) {
  for (size_t i = 0; i < MI_SEGMENT_MAP_MAX_PARTS; i++) {
    mi_segmap_part_t* part = mi_atomic_exchange_ptr_relaxed(mi_segmap_part_t, &mi_segment_map[i], NULL);
    if (part != NULL) {
      _mi_os_free(part, sizeof(mi_segmap_part_t), part->memid);
    }
  }
}
