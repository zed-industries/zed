/* ----------------------------------------------------------------------------
Copyright (c) 2018-2025, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
#pragma once
#ifndef MIMALLOC_STATS_H
#define MIMALLOC_STATS_H

#include <mimalloc.h>
#include <stdint.h>

#define MI_STAT_VERSION   1   // increased on every backward incompatible change

// count allocation over time
typedef struct mi_stat_count_s {
  int64_t total;                              // total allocated
  int64_t peak;                               // peak allocation
  int64_t current;                            // current allocation
} mi_stat_count_t;

// counters only increase
typedef struct mi_stat_counter_s {
  int64_t total;                              // total count
} mi_stat_counter_t;

#define MI_STAT_FIELDS() \
  MI_STAT_COUNT(pages)                      /* count of mimalloc pages */ \
  MI_STAT_COUNT(reserved)                   /* reserved memory bytes */ \
  MI_STAT_COUNT(committed)                  /* committed bytes */ \
  MI_STAT_COUNT(reset)                      /* reset bytes */ \
  MI_STAT_COUNT(purged)                     /* purged bytes */ \
  MI_STAT_COUNT(page_committed)             /* committed memory inside pages */ \
  MI_STAT_COUNT(pages_abandoned)            /* abandonded pages count */ \
  MI_STAT_COUNT(threads)                    /* number of threads */ \
  MI_STAT_COUNT(malloc_normal)              /* allocated bytes <= MI_LARGE_OBJ_SIZE_MAX */ \
  MI_STAT_COUNT(malloc_huge)                /* allocated bytes in huge pages */ \
  MI_STAT_COUNT(malloc_requested)           /* malloc requested bytes */ \
  \
  MI_STAT_COUNTER(mmap_calls) \
  MI_STAT_COUNTER(commit_calls) \
  MI_STAT_COUNTER(reset_calls) \
  MI_STAT_COUNTER(purge_calls) \
  MI_STAT_COUNTER(arena_count)              /* number of memory arena's */ \
  MI_STAT_COUNTER(malloc_normal_count)      /* number of blocks <= MI_LARGE_OBJ_SIZE_MAX */ \
  MI_STAT_COUNTER(malloc_huge_count)        /* number of huge bloks */ \
  MI_STAT_COUNTER(malloc_guarded_count)     /* number of allocations with guard pages */ \
  \
  /* internal statistics */ \
  MI_STAT_COUNTER(arena_rollback_count) \
  MI_STAT_COUNTER(arena_purges) \
  MI_STAT_COUNTER(pages_extended)           /* number of page extensions */ \
  MI_STAT_COUNTER(pages_retire)             /* number of pages that are retired */ \
  MI_STAT_COUNTER(page_searches)            /* searches for a fresh page */ \
  /* only on v1 and v2 */ \
  MI_STAT_COUNT(segments) \
  MI_STAT_COUNT(segments_abandoned) \
  MI_STAT_COUNT(segments_cache) \
  MI_STAT_COUNT(_segments_reserved) \
  /* only on v3 */ \
  MI_STAT_COUNTER(pages_reclaim_on_alloc) \
  MI_STAT_COUNTER(pages_reclaim_on_free) \
  MI_STAT_COUNTER(pages_reabandon_full) \
  MI_STAT_COUNTER(pages_unabandon_busy_wait) \


// Define the statistics structure
#define MI_BIN_HUGE             (73U)   // see types.h
#define MI_STAT_COUNT(stat)     mi_stat_count_t stat;
#define MI_STAT_COUNTER(stat)   mi_stat_counter_t stat;

typedef struct mi_stats_s
{
  int version;

  MI_STAT_FIELDS()

  // future extension
  mi_stat_count_t   _stat_reserved[4];
  mi_stat_counter_t _stat_counter_reserved[4];

  // size segregated statistics
  mi_stat_count_t   malloc_bins[MI_BIN_HUGE+1];   // allocation per size bin
  mi_stat_count_t   page_bins[MI_BIN_HUGE+1];     // pages allocated per size bin
} mi_stats_t;

#undef MI_STAT_COUNT
#undef MI_STAT_COUNTER

// Exported definitions
#ifdef __cplusplus
extern "C" {
#endif

mi_decl_export void  mi_stats_get( size_t stats_size, mi_stats_t* stats ) mi_attr_noexcept;
mi_decl_export char* mi_stats_get_json( size_t buf_size, char* buf ) mi_attr_noexcept;    // use mi_free to free the result if the input buf == NULL

#ifdef __cplusplus
}
#endif

#endif // MIMALLOC_STATS_H
