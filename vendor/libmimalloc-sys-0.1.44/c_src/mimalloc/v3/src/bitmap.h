/* ----------------------------------------------------------------------------
Copyright (c) 2019-2024 Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/

/* ----------------------------------------------------------------------------
Concurrent bitmap that can set/reset sequences of bits atomically
---------------------------------------------------------------------------- */
#pragma once
#ifndef MI_BITMAP_H
#define MI_BITMAP_H

/* --------------------------------------------------------------------------------
  Atomic bitmaps with release/acquire guarantees:

  `mi_bfield_t`: is a single machine word that can efficiently be bit counted (usually `size_t`)
      each bit usually represents a single MI_ARENA_SLICE_SIZE in an arena (64 KiB).
      We need 16K bits to represent a 1GiB arena.

  `mi_bchunk_t`: a chunk of bfield's of a total of MI_BCHUNK_BITS (= 512 on 64-bit, 256 on 32-bit)
      allocations never span across chunks -- so MI_ARENA_MAX_OBJ_SIZE is the number
      of bits in a chunk times the MI_ARENA_SLICE_SIZE (512 * 64KiB = 32 MiB).
      These chunks are cache-aligned and we can use AVX2/AVX512/NEON/SVE/SVE2/etc. instructions
      to scan for bits (perhaps) more efficiently.

      We allocate byte-sized ranges aligned to bytes in the bfield, and bfield-sized
      ranges aligned to a bfield.

    Searching linearly through the chunks would be too slow (16K bits per GiB).
    Instead we add a "chunkmap" to do a two-level search (more or less a btree of depth 2).

   `mi_bchunkmap_t` (== `mi_bchunk_t`): for each chunk we track if it has (potentially) any bit set.
      The chunkmap has 1 bit per chunk that is set if the chunk potentially has a bit set.
      This is used to avoid scanning every chunk. (and thus strictly an optimization)
      It is conservative: it is fine to set a bit in the chunk map even if the chunk turns out
      to have no bits set. It is also allowed to briefly have a clear bit even if the
      chunk has bits set -- as long as we guarantee that the bit will be set later on;
      (this allows us to set the chunkmap bit right after we set a bit in the corresponding chunk).

      However, when we clear a bit in a chunk, and the chunk is indeed all clear, we
      cannot safely clear the bit corresponding to the chunk in the chunkmap since it
      may race with another thread setting a bit in the same chunk. Therefore, when
      clearing, we first test if a chunk is clear, then clear the chunkmap bit, and
      then test again to catch any set bits that we may have missed.

      Since the chunkmap may thus be briefly out-of-sync, this means that we may sometimes
      not find a free page even though it's there (but we accept this as we avoid taking
      full locks). (Another way to do this is to use an epoch but we like to avoid that complexity
      for now).

   `mi_bitmap_t`: a bitmap with N chunks. A bitmap has a chunkmap of MI_BCHUNK_BITS (512)
      and thus has at most 512 chunks (=2^18 bits x 64 KiB slices = 16 GiB max arena size).
      The minimum is 1 chunk which is a 32 MiB arena.

   For now, the implementation assumes MI_HAS_FAST_BITSCAN and uses trailing-zero-count
   and pop-count (but we think it can be adapted work reasonably well on older hardware too)
--------------------------------------------------------------------------------------------- */

// A word-size bit field.
typedef size_t mi_bfield_t;

#define MI_BFIELD_BITS_SHIFT         (MI_SIZE_SHIFT+3)
#define MI_BFIELD_BITS               (1 << MI_BFIELD_BITS_SHIFT)
#define MI_BFIELD_SIZE               (MI_BFIELD_BITS/8)
#define MI_BFIELD_LO_BIT8            (((~(mi_bfield_t)0))/0xFF)         // 0x01010101 ..
#define MI_BFIELD_HI_BIT8            (MI_BFIELD_LO_BIT8 << 7)           // 0x80808080 ..

#define MI_BCHUNK_SIZE               (MI_BCHUNK_BITS / 8)
#define MI_BCHUNK_FIELDS             (MI_BCHUNK_BITS / MI_BFIELD_BITS)  // 8 on both 64- and 32-bit


// some compiler (msvc in C mode) cannot have expressions in the alignment attribute
#if MI_BCHUNK_SIZE==64
#define mi_decl_bchunk_align  mi_decl_align(64)
#elif MI_BCHUNK_SIZE==32
#define mi_decl_bchunk_align  mi_decl_align(32)
#else
#define mi_decl_bchunk_align  mi_decl_align(MI_BCHUNK_SIZE)
#endif
 

// A bitmap chunk contains 512 bits on 64-bit  (256 on 32-bit)
typedef mi_decl_bchunk_align struct mi_bchunk_s {
  _Atomic(mi_bfield_t) bfields[MI_BCHUNK_FIELDS];
} mi_bchunk_t;


// The chunkmap has one bit per corresponding chunk that is set if the chunk potentially has bits set.
// The chunkmap is itself a chunk.
typedef mi_bchunk_t mi_bchunkmap_t;

#define MI_BCHUNKMAP_BITS             MI_BCHUNK_BITS

#define MI_BITMAP_MAX_CHUNK_COUNT     (MI_BCHUNKMAP_BITS)
#define MI_BITMAP_MIN_CHUNK_COUNT     (1)
#if MI_SIZE_BITS > 32
#define MI_BITMAP_DEFAULT_CHUNK_COUNT     (64)  // 2 GiB on 64-bit -- this is for the page map
#else
#define MI_BITMAP_DEFAULT_CHUNK_COUNT      (1)
#endif
#define MI_BITMAP_MAX_BIT_COUNT       (MI_BITMAP_MAX_CHUNK_COUNT * MI_BCHUNK_BITS)  // 16 GiB arena
#define MI_BITMAP_MIN_BIT_COUNT       (MI_BITMAP_MIN_CHUNK_COUNT * MI_BCHUNK_BITS)  // 32 MiB arena
#define MI_BITMAP_DEFAULT_BIT_COUNT   (MI_BITMAP_DEFAULT_CHUNK_COUNT * MI_BCHUNK_BITS)  // 2 GiB arena


// An atomic bitmap
typedef mi_decl_bchunk_align struct mi_bitmap_s {
  _Atomic(size_t)  chunk_count;         // total count of chunks (0 < N <= MI_BCHUNKMAP_BITS)
  size_t           _padding[MI_BCHUNK_SIZE/MI_SIZE_SIZE - 1];    // suppress warning on msvc
  mi_bchunkmap_t   chunkmap;
  mi_bchunk_t      chunks[MI_BITMAP_DEFAULT_CHUNK_COUNT];        // usually dynamic MI_BITMAP_MAX_CHUNK_COUNT
} mi_bitmap_t;


static inline size_t mi_bitmap_chunk_count(const mi_bitmap_t* bitmap) {
  return mi_atomic_load_relaxed(&((mi_bitmap_t*)bitmap)->chunk_count);
}

static inline size_t mi_bitmap_max_bits(const mi_bitmap_t* bitmap) {
  return (mi_bitmap_chunk_count(bitmap) * MI_BCHUNK_BITS);
}



/* --------------------------------------------------------------------------------
  Atomic bitmap operations
-------------------------------------------------------------------------------- */

// Many operations are generic over setting or clearing the bit sequence: we use `mi_xset_t` for this (true if setting, false if clearing)
typedef bool  mi_xset_t;
#define MI_BIT_SET    (true)
#define MI_BIT_CLEAR  (false)


// Required size of a bitmap to represent `bit_count` bits.
size_t mi_bitmap_size(size_t bit_count, size_t* chunk_count);

// Initialize a bitmap to all clear; avoid a mem_zero if `already_zero` is true
// returns the size of the bitmap.
size_t mi_bitmap_init(mi_bitmap_t* bitmap, size_t bit_count, bool already_zero);

// Set/clear a sequence of `n` bits in the bitmap (and can cross chunks).
// Not atomic so only use if still local to a thread.
void mi_bitmap_unsafe_setN(mi_bitmap_t* bitmap, size_t idx, size_t n);


// Set a bit in the bitmap; returns `true` if it atomically transitioned from 0 to 1
bool mi_bitmap_set(mi_bitmap_t* bitmap, size_t idx);

// Clear a bit in the bitmap; returns `true` if it atomically transitioned from 1 to 0
bool mi_bitmap_clear(mi_bitmap_t* bitmap, size_t idx);

// Set a sequence of `n` bits in the bitmap; returns `true` if atomically transitioned from all 0's to 1's
// `n` cannot cross chunk boundaries (and `n <= MI_BCHUNK_BITS`)!
// If `already_set` is not NULL, it is set to count of bits were already all set.
// (this is used for correct statistics if commiting over a partially committed area)
bool mi_bitmap_setN(mi_bitmap_t* bitmap, size_t idx, size_t n, size_t* already_set);

// Clear a sequence of `n` bits in the bitmap; returns `true` if atomically transitioned from all 1's to 0's
// `n` cannot cross chunk boundaries (and `n <= MI_BCHUNK_BITS`)!
bool mi_bitmap_clearN(mi_bitmap_t* bitmap, size_t idx, size_t n);


// Is a sequence of n bits already all set/cleared?
bool mi_bitmap_is_xsetN(mi_xset_t set, mi_bitmap_t* bitmap, size_t idx, size_t n);

// Is a sequence of n bits already set?
// (Used to check if a memory range is already committed)
static inline bool mi_bitmap_is_setN(mi_bitmap_t* bitmap, size_t idx, size_t n) {
  return mi_bitmap_is_xsetN(MI_BIT_SET, bitmap, idx, n);
}

// Is a sequence of n bits already clear?
static inline bool mi_bitmap_is_clearN(mi_bitmap_t* bitmap, size_t idx, size_t n) {
  return mi_bitmap_is_xsetN(MI_BIT_CLEAR, bitmap, idx, n);
}

static inline bool mi_bitmap_is_set(mi_bitmap_t* bitmap, size_t idx) {
  return mi_bitmap_is_setN(bitmap, idx, 1);
}

static inline bool mi_bitmap_is_clear(mi_bitmap_t* bitmap, size_t idx) {
  return mi_bitmap_is_clearN(bitmap, idx, 1);
}

// Called once a bit is cleared to see if the memory slice can be claimed.
typedef bool (mi_claim_fun_t)(size_t slice_index, mi_arena_t* arena, mi_heaptag_t heap_tag, bool* keep_set);

// Find a set bits in the bitmap, atomically clear it, and check if `claim` returns true.
// If not claimed, continue on (potentially setting the bit again depending on `keep_set`).
// Returns true on success, and in that case sets the index: `0 <= *pidx <= MI_BITMAP_MAX_BITS-n`.
mi_decl_nodiscard bool mi_bitmap_try_find_and_claim(mi_bitmap_t* bitmap, size_t tseq, size_t* pidx,
                                                    mi_claim_fun_t* claim, mi_arena_t* arena, mi_heaptag_t heap_tag );


// Atomically clear a bit but only if it is set. Will block otherwise until the bit is set.
// This is used to delay free-ing a page that it at the same time being considered to be
// allocated from `mi_arena_try_abandoned` (and is in the `claim` function of `mi_bitmap_try_find_and_claim`).
void mi_bitmap_clear_once_set(mi_bitmap_t* bitmap, size_t idx);


// If a bit is set in the bitmap, return `true` and set `idx` to the index of the highest bit.
// Otherwise return `false` (and `*idx` is undefined).
// Used for unloading arena's
bool mi_bitmap_bsr(mi_bitmap_t* bitmap, size_t* idx);

// Return count of all set bits in a bitmap.
size_t mi_bitmap_popcount(mi_bitmap_t* bitmap);


typedef bool (mi_forall_set_fun_t)(size_t slice_index, size_t slice_count, mi_arena_t* arena, void* arg2);

// Visit all set bits in a bitmap (`slice_count == 1`)
bool _mi_bitmap_forall_set(mi_bitmap_t* bitmap, mi_forall_set_fun_t* visit, mi_arena_t* arena, void* arg);

// Visit all set bits in a bitmap with larger ranges if possible (`slice_count >= 1`)
bool _mi_bitmap_forall_setc_ranges(mi_bitmap_t* bitmap, mi_forall_set_fun_t* visit, mi_arena_t* arena, void* arg);


// Count all set bits in given range in the bitmap. (cannot cross chunks)
size_t mi_bitmap_popcountN( mi_bitmap_t* bitmap, size_t idx, size_t n);

/* ----------------------------------------------------------------------------
  Binned concurrent bitmap
  Assigns a size class to each chunk such that small blocks don't cause too
  much fragmentation since we keep chunks for larger blocks separate.
---------------------------------------------------------------------------- */

// mi_chunkbin_t is defined in mimalloc-stats.h

static inline mi_chunkbin_t mi_chunkbin_inc(mi_chunkbin_t bbin) {
  mi_assert_internal(bbin < MI_CBIN_COUNT);
  return (mi_chunkbin_t)((int)bbin + 1);
}

static inline mi_chunkbin_t mi_chunkbin_dec(mi_chunkbin_t bbin) {
  mi_assert_internal(bbin > MI_CBIN_NONE);
  return (mi_chunkbin_t)((int)bbin - 1);
}

static inline mi_chunkbin_t mi_chunkbin_of(size_t slice_count) {
  if (slice_count==1) return MI_CBIN_SMALL;
  if (slice_count==8) return MI_CBIN_MEDIUM;
  #if MI_ENABLE_LARGE_PAGES
  if (slice_count==MI_BFIELD_BITS) return MI_CBIN_LARGE;
  #endif
  return MI_CBIN_OTHER;
}

// An atomic "binned" bitmap for the free slices where we keep chunks reserved for particalar size classes
typedef mi_decl_bchunk_align struct mi_bbitmap_s {
  _Atomic(size_t)  chunk_count;         // total count of chunks (0 < N <= MI_BCHUNKMAP_BITS)
  _Atomic(size_t)  chunk_max_accessed;  // max chunk index that was once cleared or set
  #if (MI_BCHUNK_SIZE / MI_SIZE_SIZE) > 2
  size_t           _padding[MI_BCHUNK_SIZE/MI_SIZE_SIZE - 2];    // suppress warning on msvc by aligning manually
  #endif
  mi_bchunkmap_t   chunkmap;                                    
  mi_bchunkmap_t   chunkmap_bins[MI_CBIN_COUNT - 1];             // chunkmaps with bit set if the chunk is in that size class (excluding MI_CBIN_NONE)  
  mi_bchunk_t      chunks[MI_BITMAP_DEFAULT_CHUNK_COUNT];        // usually dynamic MI_BITMAP_MAX_CHUNK_COUNT
} mi_bbitmap_t;


static inline size_t mi_bbitmap_chunk_count(const mi_bbitmap_t* bbitmap) {
  return mi_atomic_load_relaxed(&((mi_bbitmap_t*)bbitmap)->chunk_count);
}

static inline size_t mi_bbitmap_max_bits(const mi_bbitmap_t* bbitmap) {
  return (mi_bbitmap_chunk_count(bbitmap) * MI_BCHUNK_BITS);
}

mi_chunkbin_t mi_bbitmap_debug_get_bin(const mi_bchunk_t* chunkmap_bins, size_t chunk_idx);

size_t mi_bbitmap_size(size_t bit_count, size_t* chunk_count);


// Initialize a bitmap to all clear; avoid a mem_zero if `already_zero` is true
// returns the size of the bitmap.
size_t mi_bbitmap_init(mi_bbitmap_t* bbitmap, size_t bit_count, bool already_zero);

// Set/clear a sequence of `n` bits in the bitmap (and can cross chunks).
// Not atomic so only use if still local to a thread.
void mi_bbitmap_unsafe_setN(mi_bbitmap_t* bbitmap, size_t idx, size_t n);


// Set a sequence of `n` bits in the bbitmap; returns `true` if atomically transitioned from all 0's to 1's
// `n` cannot cross chunk boundaries (and `n <= MI_BCHUNK_BITS`)!
bool mi_bbitmap_setN(mi_bbitmap_t* bbitmap, size_t idx, size_t n);


// Is a sequence of n bits already all set/cleared?
bool mi_bbitmap_is_xsetN(mi_xset_t set, mi_bbitmap_t* bbitmap, size_t idx, size_t n);

// Is a sequence of n bits already set?
// (Used to check if a memory range is already committed)
static inline bool mi_bbitmap_is_setN(mi_bbitmap_t* bbitmap, size_t idx, size_t n) {
  return mi_bbitmap_is_xsetN(MI_BIT_SET, bbitmap, idx, n);
}

// Is a sequence of n bits already clear?
static inline bool mi_bbitmap_is_clearN(mi_bbitmap_t* bbitmap, size_t idx, size_t n) {
  return mi_bbitmap_is_xsetN(MI_BIT_CLEAR, bbitmap, idx, n);
}


// Try to atomically transition `n` bits from all set to all clear. Returns `true` on succes.
// `n` cannot cross chunk boundaries, where `n <= MI_CHUNK_BITS`.
bool mi_bbitmap_try_clearN(mi_bbitmap_t* bbitmap, size_t idx, size_t n);

// Specialized versions for common bit sequence sizes
bool mi_bbitmap_try_find_and_clear(mi_bbitmap_t* bbitmap, size_t tseq, size_t* pidx);  // 1-bit
bool mi_bbitmap_try_find_and_clear8(mi_bbitmap_t* bbitmap, size_t tseq, size_t* pidx); // 8-bits
// bool mi_bbitmap_try_find_and_clearX(mi_bbitmap_t* bbitmap, size_t tseq, size_t* pidx); // MI_BFIELD_BITS
bool mi_bbitmap_try_find_and_clearNX(mi_bbitmap_t* bbitmap, size_t n, size_t tseq, size_t* pidx); // < MI_BFIELD_BITS
bool mi_bbitmap_try_find_and_clearN_(mi_bbitmap_t* bbitmap, size_t n, size_t tseq, size_t* pidx); // > MI_BFIELD_BITS <= MI_BCHUNK_BITS

// Find a sequence of `n` bits in the bbitmap with all bits set, and try to atomically clear all.
// Returns true on success, and in that case sets the index: `0 <= *pidx <= MI_BITMAP_MAX_BITS-n`.
mi_decl_nodiscard static inline bool mi_bbitmap_try_find_and_clearN(mi_bbitmap_t* bbitmap, size_t n, size_t tseq, size_t* pidx) {
  if (n==1) return mi_bbitmap_try_find_and_clear(bbitmap, tseq, pidx);               // small pages
  if (n==8) return mi_bbitmap_try_find_and_clear8(bbitmap, tseq, pidx);              // medium pages
  // if (n==MI_BFIELD_BITS) return mi_bbitmap_try_find_and_clearX(bbitmap, tseq, pidx); // large pages
  if (n==0 || n>MI_BCHUNK_BITS) return false;  // cannot be more than a chunk
  if (n<=MI_BFIELD_BITS) return mi_bbitmap_try_find_and_clearNX(bbitmap, tseq, n, pidx);
  return mi_bbitmap_try_find_and_clearN_(bbitmap, tseq, n, pidx);
}


#endif // MI_BITMAP_H
