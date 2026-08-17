/* ----------------------------------------------------------------------------
Copyright (c) 2019-2024 Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/

/* ----------------------------------------------------------------------------
Concurrent bitmap that can set/reset sequences of bits atomically
---------------------------------------------------------------------------- */

#include "mimalloc.h"
#include "mimalloc/internal.h"
#include "mimalloc/bits.h"
#include "bitmap.h"

#ifndef MI_OPT_SIMD
#define MI_OPT_SIMD   0
#endif

/* --------------------------------------------------------------------------------
  bfields
-------------------------------------------------------------------------------- */

static inline size_t mi_bfield_ctz(mi_bfield_t x) {
  return mi_ctz(x);
}

static inline size_t mi_bfield_clz(mi_bfield_t x) {
  return mi_clz(x);
}

static inline size_t mi_bfield_popcount(mi_bfield_t x) {
  return mi_popcount(x);
}

static inline mi_bfield_t mi_bfield_clear_least_bit(mi_bfield_t x) {
  return (x & (x-1));
}

// find the least significant bit that is set (i.e. count trailing zero's)
// return false if `x==0` (with `*idx` undefined) and true otherwise,
// with the `idx` is set to the bit index (`0 <= *idx < MI_BFIELD_BITS`).
static inline bool mi_bfield_find_least_bit(mi_bfield_t x, size_t* idx) {
  return mi_bsf(x,idx);
}

// find the most significant bit that is set.
// return false if `x==0` (with `*idx` undefined) and true otherwise,
// with the `idx` is set to the bit index (`0 <= *idx < MI_BFIELD_BITS`).
static inline bool mi_bfield_find_highest_bit(mi_bfield_t x, size_t* idx) {
  return mi_bsr(x, idx);
}



// find each set bit in a bit field `x` and clear it, until it becomes zero.
static inline bool mi_bfield_foreach_bit(mi_bfield_t* x, size_t* idx) {
  const bool found = mi_bfield_find_least_bit(*x, idx);
  *x = mi_bfield_clear_least_bit(*x);
  return found;
}

static inline mi_bfield_t mi_bfield_zero(void) {
  return 0;
}

static inline mi_bfield_t mi_bfield_one(void) {
  return 1;
}

static inline mi_bfield_t mi_bfield_all_set(void) {
  return ~((mi_bfield_t)0);
}

// mask of `bit_count` bits set shifted to the left by `shiftl`
static inline mi_bfield_t mi_bfield_mask(size_t bit_count, size_t shiftl) {
  mi_assert_internal(bit_count > 0);
  mi_assert_internal(bit_count + shiftl <= MI_BFIELD_BITS);
  const mi_bfield_t mask0 = (bit_count < MI_BFIELD_BITS ? (mi_bfield_one() << bit_count)-1 : mi_bfield_all_set());
  return (mask0 << shiftl);
}


// ------- mi_bfield_atomic_set ---------------------------------------
// the `_set` functions return also the count of bits that were already set (for commit statistics)
// the `_clear` functions return also whether the new bfield is all clear or not (for the chunk_map)

// Set a bit atomically. Returns `true` if the bit transitioned from 0 to 1
static inline bool mi_bfield_atomic_set(_Atomic(mi_bfield_t)*b, size_t idx) {
  mi_assert_internal(idx < MI_BFIELD_BITS);
  const mi_bfield_t mask = mi_bfield_mask(1, idx);;
  const mi_bfield_t old = mi_atomic_or_acq_rel(b, mask);
  return ((old&mask) == 0);
}

// Clear a bit atomically. Returns `true` if the bit transitioned from 1 to 0.
// `all_clear` is set if the new bfield is zero.
static inline bool mi_bfield_atomic_clear(_Atomic(mi_bfield_t)*b, size_t idx, bool* all_clear) {
  mi_assert_internal(idx < MI_BFIELD_BITS);
  const mi_bfield_t mask = mi_bfield_mask(1, idx);;
  mi_bfield_t old = mi_atomic_and_acq_rel(b, ~mask);
  if (all_clear != NULL) { *all_clear = ((old&~mask)==0); }
  return ((old&mask) == mask);
}

// Clear a bit but only when/once it is set. This is used by concurrent free's while
// the page is abandoned and mapped. This can incure a busy wait :-( but it should
// happen almost never (and is accounted for in the stats)
static inline void mi_bfield_atomic_clear_once_set(_Atomic(mi_bfield_t)*b, size_t idx) {
  mi_assert_internal(idx < MI_BFIELD_BITS);
  const mi_bfield_t mask = mi_bfield_mask(1, idx);;
  mi_bfield_t old = mi_atomic_load_relaxed(b);
  do {
    if mi_unlikely((old&mask) == 0) {
      old = mi_atomic_load_acquire(b);
      if ((old&mask)==0) {
        mi_subproc_stat_counter_increase(_mi_subproc(), pages_unabandon_busy_wait, 1);
      }
      while ((old&mask)==0) { // busy wait
        mi_atomic_yield();
        old = mi_atomic_load_acquire(b);
      }
    }
  } while (!mi_atomic_cas_weak_acq_rel(b,&old, (old&~mask)));
  mi_assert_internal((old&mask)==mask);  // we should only clear when it was set
}

// Set a mask set of bits atomically, and return true of the mask bits transitioned from all 0's to 1's.
// `already_set` contains the count of bits that were already set (used when committing ranges to account
// statistics correctly).
static inline bool mi_bfield_atomic_set_mask(_Atomic(mi_bfield_t)*b, mi_bfield_t mask, size_t* already_set) {
  mi_assert_internal(mask != 0);
  mi_bfield_t old = mi_atomic_load_relaxed(b);
  while (!mi_atomic_cas_weak_acq_rel(b, &old, old|mask)) {};  // try to atomically set the mask bits until success
  if (already_set!=NULL) { *already_set = mi_bfield_popcount(old&mask); }
  return ((old&mask) == 0);
}

// Clear a mask set of bits atomically, and return true of the mask bits transitioned from all 1's to 0's
// `all_clear` is set to `true` if the new bfield became zero.
static inline bool mi_bfield_atomic_clear_mask(_Atomic(mi_bfield_t)*b, mi_bfield_t mask, bool* all_clear) {
  mi_assert_internal(mask != 0);
  mi_bfield_t old = mi_atomic_load_relaxed(b);
  while (!mi_atomic_cas_weak_acq_rel(b, &old, old&~mask)) {};  // try to atomically clear the mask bits until success
  if (all_clear != NULL) { *all_clear = ((old&~mask)==0); }
  return ((old&mask) == mask);
}

static inline bool mi_bfield_atomic_setX(_Atomic(mi_bfield_t)*b, size_t* already_set) {
  const mi_bfield_t old = mi_atomic_exchange_release(b, mi_bfield_all_set());
  if (already_set!=NULL) { *already_set = mi_bfield_popcount(old); }
  return (old==0);
}

// static inline bool mi_bfield_atomic_clearX(_Atomic(mi_bfield_t)*b, bool* all_clear) {
//   const mi_bfield_t old = mi_atomic_exchange_release(b, mi_bfield_zero());
//   if (all_clear!=NULL) { *all_clear = true; }
//   return (~old==0);
// }

// ------- mi_bfield_atomic_try_clear ---------------------------------------


// Tries to clear a mask atomically, and returns true if the mask bits atomically transitioned from mask to 0
// and false otherwise (leaving the bit field as is).
// `all_clear` is set to `true` if the new bfield became zero.
static inline bool mi_bfield_atomic_try_clear_mask_of(_Atomic(mi_bfield_t)*b, mi_bfield_t mask, mi_bfield_t expect, bool* all_clear) {
  mi_assert_internal(mask != 0);
  // try to atomically clear the mask bits
  do {
    if ((expect & mask) != mask) {  // are all bits still set?
      if (all_clear != NULL) { *all_clear = (expect == 0); }
      return false;
    }
  } while (!mi_atomic_cas_weak_acq_rel(b, &expect, expect & ~mask));
  if (all_clear != NULL) { *all_clear = ((expect & ~mask) == 0);  }
  return true;
}

static inline bool mi_bfield_atomic_try_clear_mask(_Atomic(mi_bfield_t)* b, mi_bfield_t mask, bool* all_clear) {
  mi_assert_internal(mask != 0);
  const mi_bfield_t expect = mi_atomic_load_relaxed(b);
  return mi_bfield_atomic_try_clear_mask_of(b, mask, expect, all_clear);
}

// Tries to clear a bit atomically. Returns `true` if the bit transitioned from 1 to 0
// and `false` otherwise leaving the bfield `b` as-is.
// `all_clear` is set to true if the new bfield became zero (and false otherwise)
mi_decl_maybe_unused static inline bool mi_bfield_atomic_try_clear(_Atomic(mi_bfield_t)* b, size_t idx, bool* all_clear) {
  mi_assert_internal(idx < MI_BFIELD_BITS);
  const mi_bfield_t mask = mi_bfield_one()<<idx;
  return mi_bfield_atomic_try_clear_mask(b, mask, all_clear);
}

// Tries to clear a byte atomically, and returns true if the byte atomically transitioned from 0xFF to 0
// `all_clear` is set to true if the new bfield became zero (and false otherwise)
mi_decl_maybe_unused static inline bool mi_bfield_atomic_try_clear8(_Atomic(mi_bfield_t)*b, size_t idx, bool* all_clear) {
  mi_assert_internal(idx < MI_BFIELD_BITS);
  mi_assert_internal((idx%8)==0);
  const mi_bfield_t mask = ((mi_bfield_t)0xFF)<<idx;
  return mi_bfield_atomic_try_clear_mask(b, mask, all_clear);
}

// Try to clear a full field of bits atomically, and return true all bits transitioned from all 1's to 0's.
// and false otherwise leaving the bit field as-is.
// `all_clear` is set to true if the new bfield became zero (which is always the case if successful).
static inline bool mi_bfield_atomic_try_clearX(_Atomic(mi_bfield_t)*b, bool* all_clear) {
  mi_bfield_t old = mi_bfield_all_set();
  if (mi_atomic_cas_strong_acq_rel(b, &old, mi_bfield_zero())) {
    if (all_clear != NULL) { *all_clear = true; }
    return true;
  }
  else return false;
}


// ------- mi_bfield_atomic_is_set ---------------------------------------

// Check if a bit is set
static inline bool mi_bfield_atomic_is_set(const _Atomic(mi_bfield_t)*b, const size_t idx) {
  const mi_bfield_t x = mi_atomic_load_relaxed(b);
  return ((x & mi_bfield_mask(1,idx)) != 0);
}

// Check if a bit is clear
static inline bool mi_bfield_atomic_is_clear(const _Atomic(mi_bfield_t)*b, const size_t idx) {
  const mi_bfield_t x = mi_atomic_load_relaxed(b);
  return ((x & mi_bfield_mask(1, idx)) == 0);
}

// Check if a bit is xset
static inline bool mi_bfield_atomic_is_xset(mi_xset_t set, const _Atomic(mi_bfield_t)*b, const size_t idx) {
  if (set) return mi_bfield_atomic_is_set(b, idx);
      else return mi_bfield_atomic_is_clear(b, idx);
}

// Check if all bits corresponding to a mask are set.
static inline bool mi_bfield_atomic_is_set_mask(const _Atomic(mi_bfield_t)* b, mi_bfield_t mask) {
  mi_assert_internal(mask != 0);
  const mi_bfield_t x = mi_atomic_load_relaxed(b);
  return ((x & mask) == mask);
}

// Check if all bits corresponding to a mask are clear.
static inline bool mi_bfield_atomic_is_clear_mask(const _Atomic(mi_bfield_t)* b, mi_bfield_t mask) {
  mi_assert_internal(mask != 0);
  const mi_bfield_t x = mi_atomic_load_relaxed(b);
  return ((x & mask) == 0);
}

// Check if all bits corresponding to a mask are set/cleared.
static inline bool mi_bfield_atomic_is_xset_mask(mi_xset_t set, const _Atomic(mi_bfield_t)* b, mi_bfield_t mask) {
  mi_assert_internal(mask != 0);
  if (set) return mi_bfield_atomic_is_set_mask(b, mask);
      else return mi_bfield_atomic_is_clear_mask(b, mask);
}

// Count bits in a mask
static inline size_t mi_bfield_atomic_popcount_mask(_Atomic(mi_bfield_t)*b, mi_bfield_t mask) {
  const mi_bfield_t x = mi_atomic_load_relaxed(b);
  return mi_bfield_popcount(x & mask);
}


/* --------------------------------------------------------------------------------
 bitmap chunks
-------------------------------------------------------------------------------- */

// ------- mi_bchunk_set ---------------------------------------

// Set a single bit
static inline bool mi_bchunk_set(mi_bchunk_t* chunk, size_t cidx, size_t* already_set) {
  mi_assert_internal(cidx < MI_BCHUNK_BITS);
  const size_t i = cidx / MI_BFIELD_BITS;
  const size_t idx = cidx % MI_BFIELD_BITS;
  const bool was_clear = mi_bfield_atomic_set(&chunk->bfields[i], idx);
  if (already_set != NULL) { *already_set = (was_clear ? 0 : 1); }
  return was_clear;
}

// Set `0 < n <= MI_BFIELD_BITS`, and return true of the mask bits transitioned from all 0's to 1's.
// `already_set` contains the count of bits that were already set (used when committing ranges to account
// statistics correctly).
// Can cross over two bfields.
static inline bool mi_bchunk_setNX(mi_bchunk_t* chunk, size_t cidx, size_t n, size_t* already_set) {
  mi_assert_internal(cidx < MI_BCHUNK_BITS);
  mi_assert_internal(n > 0 && n <= MI_BFIELD_BITS);
  const size_t i = cidx / MI_BFIELD_BITS;
  const size_t idx = cidx % MI_BFIELD_BITS;
  if mi_likely(idx + n <= MI_BFIELD_BITS) {
    // within one field
    return mi_bfield_atomic_set_mask(&chunk->bfields[i], mi_bfield_mask(n,idx), already_set);
  }
  else {
    // spanning two fields
    const size_t m = MI_BFIELD_BITS - idx;  // bits to clear in the first field
    mi_assert_internal(m < n);
    mi_assert_internal(i < MI_BCHUNK_FIELDS - 1);
    mi_assert_internal(idx + m <= MI_BFIELD_BITS);
    size_t already_set1;
    const bool all_set1 = mi_bfield_atomic_set_mask(&chunk->bfields[i], mi_bfield_mask(m, idx), &already_set1);
    mi_assert_internal(n - m > 0);
    mi_assert_internal(n - m < MI_BFIELD_BITS);
    size_t already_set2;
    const bool all_set2 = mi_bfield_atomic_set_mask(&chunk->bfields[i+1], mi_bfield_mask(n - m, 0), &already_set2);
    if (already_set != NULL) { *already_set = already_set1 + already_set2; }
    return (all_set1 && all_set2);
  }
}

// Set a sequence of `n` bits within a chunk.
// Returns true if all bits transitioned from 0 to 1 (or 1 to 0).
mi_decl_noinline static bool mi_bchunk_xsetN_(mi_xset_t set, mi_bchunk_t* chunk, size_t cidx, size_t n, size_t* palready_set, bool* pmaybe_all_clear) {
  mi_assert_internal(cidx + n <= MI_BCHUNK_BITS);
  mi_assert_internal(n>0);
  bool all_transition = true;
  bool maybe_all_clear = true;
  size_t total_already_set = 0;
  size_t idx   = cidx % MI_BFIELD_BITS;
  size_t field = cidx / MI_BFIELD_BITS;
  while (n > 0) {
    size_t m = MI_BFIELD_BITS - idx;   // m is the bits to xset in this field
    if (m > n) { m = n; }
    mi_assert_internal(idx + m <= MI_BFIELD_BITS);
    mi_assert_internal(field < MI_BCHUNK_FIELDS);
    const mi_bfield_t mask = mi_bfield_mask(m, idx);
    size_t already_set = 0;
    bool all_clear = false;
    const bool transition = (set ? mi_bfield_atomic_set_mask(&chunk->bfields[field], mask, &already_set)
                                 : mi_bfield_atomic_clear_mask(&chunk->bfields[field], mask, &all_clear));
    mi_assert_internal((transition && already_set == 0) || (!transition && already_set > 0));
    all_transition = all_transition && transition;
    total_already_set += already_set;
    maybe_all_clear = maybe_all_clear && all_clear;
    // next field
    field++;
    idx = 0;
    mi_assert_internal(m <= n);
    n -= m;
  }
  if (palready_set!=NULL) { *palready_set = total_already_set; }
  if (pmaybe_all_clear!=NULL) { *pmaybe_all_clear = maybe_all_clear; }
  return all_transition;
}

static inline bool mi_bchunk_setN(mi_bchunk_t* chunk, size_t cidx, size_t n, size_t* already_set) {
  mi_assert_internal(n>0 && n <= MI_BCHUNK_BITS);
  if (n==1) return mi_bchunk_set(chunk, cidx, already_set);
  // if (n==8 && (cidx%8) == 0) return mi_bchunk_set8(chunk, cidx, already_set);
  // if (n==MI_BFIELD_BITS) return mi_bchunk_setX(chunk, cidx, already_set);
  if (n<=MI_BFIELD_BITS) return mi_bchunk_setNX(chunk, cidx, n, already_set);
  return mi_bchunk_xsetN_(MI_BIT_SET, chunk, cidx, n, already_set, NULL);
}

// ------- mi_bchunk_clear ---------------------------------------

static inline bool mi_bchunk_clear(mi_bchunk_t* chunk, size_t cidx, bool* all_clear) {
  mi_assert_internal(cidx < MI_BCHUNK_BITS);
  const size_t i = cidx / MI_BFIELD_BITS;
  const size_t idx = cidx % MI_BFIELD_BITS;
  return mi_bfield_atomic_clear(&chunk->bfields[i], idx, all_clear);
}

static inline bool mi_bchunk_clearN(mi_bchunk_t* chunk, size_t cidx, size_t n, bool* maybe_all_clear) {
  mi_assert_internal(n>0 && n <= MI_BCHUNK_BITS);
  if (n==1) return mi_bchunk_clear(chunk, cidx, maybe_all_clear);
  // if (n==8) return mi_bchunk_clear8(chunk, cidx, maybe_all_clear);
  // if (n==MI_BFIELD_BITS) return mi_bchunk_clearX(chunk, cidx, maybe_all_clear);
  // TODO: implement mi_bchunk_xsetNX instead of setNX
  return mi_bchunk_xsetN_(MI_BIT_CLEAR, chunk, cidx, n, NULL, maybe_all_clear);
}

// Check if a sequence of `n` bits within a chunk are all set/cleared.
// This can cross bfield's
mi_decl_noinline static size_t mi_bchunk_popcountN_(mi_bchunk_t* chunk, size_t field_idx, size_t idx, size_t n) {
  mi_assert_internal((field_idx*MI_BFIELD_BITS) + idx + n <= MI_BCHUNK_BITS);
  size_t count = 0;
  while (n > 0) {
    size_t m = MI_BFIELD_BITS - idx;   // m is the bits to xset in this field
    if (m > n) { m = n; }
    mi_assert_internal(idx + m <= MI_BFIELD_BITS);
    mi_assert_internal(field_idx < MI_BCHUNK_FIELDS);
    const size_t mask = mi_bfield_mask(m, idx);
    count += mi_bfield_atomic_popcount_mask(&chunk->bfields[field_idx], mask);
    // next field
    field_idx++;
    idx = 0;
    n -= m;
  }
  return count;
}

// Count set bits a sequence of `n` bits.
static inline size_t mi_bchunk_popcountN(mi_bchunk_t* chunk, size_t cidx, size_t n) {
  mi_assert_internal(cidx + n <= MI_BCHUNK_BITS);
  mi_assert_internal(n>0);
  if (n==0) return 0;
  const size_t i = cidx / MI_BFIELD_BITS;
  const size_t idx = cidx % MI_BFIELD_BITS;
  if (n==1) { return (mi_bfield_atomic_is_set(&chunk->bfields[i], idx) ? 1 : 0); }
  if (idx + n <= MI_BFIELD_BITS) { return mi_bfield_atomic_popcount_mask(&chunk->bfields[i], mi_bfield_mask(n, idx)); }
  return mi_bchunk_popcountN_(chunk, i, idx, n);
}


// ------- mi_bchunk_is_xset ---------------------------------------

// Check if a sequence of `n` bits within a chunk are all set/cleared.
// This can cross bfield's
mi_decl_noinline static bool mi_bchunk_is_xsetN_(mi_xset_t set, const mi_bchunk_t* chunk, size_t field_idx, size_t idx, size_t n) {
  mi_assert_internal((field_idx*MI_BFIELD_BITS) + idx + n <= MI_BCHUNK_BITS);
  while (n > 0) {
    size_t m = MI_BFIELD_BITS - idx;   // m is the bits to xset in this field
    if (m > n) { m = n; }
    mi_assert_internal(idx + m <= MI_BFIELD_BITS);
    mi_assert_internal(field_idx < MI_BCHUNK_FIELDS);
    const size_t mask = mi_bfield_mask(m, idx);
    if (!mi_bfield_atomic_is_xset_mask(set, &chunk->bfields[field_idx], mask)) {
      return false;
    }
    // next field
    field_idx++;
    idx = 0;
    n -= m;
  }
  return true;
}

// Check if a sequence of `n` bits within a chunk are all set/cleared.
static inline bool mi_bchunk_is_xsetN(mi_xset_t set, const mi_bchunk_t* chunk, size_t cidx, size_t n) {
  mi_assert_internal(cidx + n <= MI_BCHUNK_BITS);
  mi_assert_internal(n>0);
  if (n==0) return true;
  const size_t i = cidx / MI_BFIELD_BITS;
  const size_t idx = cidx % MI_BFIELD_BITS;
  if (n==1) { return mi_bfield_atomic_is_xset(set, &chunk->bfields[i], idx); }
  if (idx + n <= MI_BFIELD_BITS) { return mi_bfield_atomic_is_xset_mask(set, &chunk->bfields[i], mi_bfield_mask(n, idx)); }
  return mi_bchunk_is_xsetN_(set, chunk, i, idx, n);
}


// ------- mi_bchunk_try_clear  ---------------------------------------

// Clear `0 < n <= MI_BITFIELD_BITS`. Can cross over a bfield boundary.
static inline bool mi_bchunk_try_clearNX(mi_bchunk_t* chunk, size_t cidx, size_t n, bool* pmaybe_all_clear) {
  mi_assert_internal(cidx < MI_BCHUNK_BITS);
  mi_assert_internal(n <= MI_BFIELD_BITS);
  const size_t i = cidx / MI_BFIELD_BITS;
  const size_t idx = cidx % MI_BFIELD_BITS;
  if mi_likely(idx + n <= MI_BFIELD_BITS) {
    // within one field
    return mi_bfield_atomic_try_clear_mask(&chunk->bfields[i], mi_bfield_mask(n, idx), pmaybe_all_clear);
  }
  else {
    // spanning two fields (todo: use double-word atomic ops?)
    const size_t m = MI_BFIELD_BITS - idx;  // bits to clear in the first field
    mi_assert_internal(m < n);
    mi_assert_internal(i < MI_BCHUNK_FIELDS - 1);
    bool field1_is_clear;
    if (!mi_bfield_atomic_try_clear_mask(&chunk->bfields[i], mi_bfield_mask(m, idx), &field1_is_clear)) return false;
    // try the second field as well
    mi_assert_internal(n - m > 0);
    mi_assert_internal(n - m < MI_BFIELD_BITS);
    bool field2_is_clear;
    if (!mi_bfield_atomic_try_clear_mask(&chunk->bfields[i+1], mi_bfield_mask(n - m, 0), &field2_is_clear)) {
      // we failed to clear the second field, restore the first one
      mi_bfield_atomic_set_mask(&chunk->bfields[i], mi_bfield_mask(m, idx), NULL);
      return false;
    }
    if (pmaybe_all_clear != NULL) { *pmaybe_all_clear = field1_is_clear && field2_is_clear;  }
    return true;
  }
}

// Clear a full aligned bfield.
// static inline bool mi_bchunk_try_clearX(mi_bchunk_t* chunk, size_t cidx, bool* pmaybe_all_clear) {
//   mi_assert_internal(cidx < MI_BCHUNK_BITS);
//   mi_assert_internal((cidx%MI_BFIELD_BITS) == 0);
//   const size_t i = cidx / MI_BFIELD_BITS;
//   return mi_bfield_atomic_try_clearX(&chunk->bfields[i], pmaybe_all_clear);
// }

// Try to atomically clear a sequence of `n` bits within a chunk.
// Returns true if all bits transitioned from 1 to 0,
// and false otherwise leaving all bit fields as is.
// Note: this is the complex one as we need to unwind partial atomic operations if we fail halfway..
// `maybe_all_clear` is set to `true` if all the bfields involved become zero.
mi_decl_noinline static bool mi_bchunk_try_clearN_(mi_bchunk_t* chunk, size_t cidx, size_t n, bool* pmaybe_all_clear) {
  mi_assert_internal(cidx + n <= MI_BCHUNK_BITS);
  mi_assert_internal(n>0);
  if (pmaybe_all_clear != NULL) { *pmaybe_all_clear = true; }
  if (n==0) return true;

  // first field
  const size_t start_idx = cidx % MI_BFIELD_BITS;
  const size_t start_field = cidx / MI_BFIELD_BITS;
  size_t field = start_field;
  size_t m = MI_BFIELD_BITS - start_idx;   // m are the bits to clear in this field
  if (m > n) { m = n; }
  mi_assert_internal(start_idx + m <= MI_BFIELD_BITS);
  mi_assert_internal(start_field < MI_BCHUNK_FIELDS);
  const mi_bfield_t mask_start = mi_bfield_mask(m, start_idx);
  bool maybe_all_clear;
  if (!mi_bfield_atomic_try_clear_mask(&chunk->bfields[field], mask_start, &maybe_all_clear)) return false;

  // done?
  mi_assert_internal(m <= n);
  n -= m;

  // continue with mid fields and last field: if these fail we need to recover by unsetting previous fields
  // mid fields?
  while (n >= MI_BFIELD_BITS) {
    field++;
    mi_assert_internal(field < MI_BCHUNK_FIELDS);
    bool field_is_clear;
    if (!mi_bfield_atomic_try_clearX(&chunk->bfields[field], &field_is_clear)) goto restore;
    maybe_all_clear = maybe_all_clear && field_is_clear;
    n -= MI_BFIELD_BITS;
  }

  // last field?
  if (n > 0) {
    mi_assert_internal(n < MI_BFIELD_BITS);
    field++;
    mi_assert_internal(field < MI_BCHUNK_FIELDS);
    const mi_bfield_t mask_end = mi_bfield_mask(n, 0);
    bool field_is_clear;
    if (!mi_bfield_atomic_try_clear_mask(&chunk->bfields[field], mask_end, &field_is_clear)) goto restore;
    maybe_all_clear = maybe_all_clear && field_is_clear;
  }

  if (pmaybe_all_clear != NULL) { *pmaybe_all_clear = maybe_all_clear; }
  return true;

restore:
  // `field` is the index of the field that failed to set atomically; we need to restore all previous fields
  mi_assert_internal(field > start_field);
  while( field > start_field) {
    field--;
    if (field == start_field) {
      mi_bfield_atomic_set_mask(&chunk->bfields[field], mask_start, NULL);
    }
    else {
      mi_bfield_atomic_setX(&chunk->bfields[field], NULL);  // mid-field: set all bits again
    }
  }
  return false;
}


static inline bool mi_bchunk_try_clearN(mi_bchunk_t* chunk, size_t cidx, size_t n, bool* maybe_all_clear) {
  mi_assert_internal(n>0);
  // if (n==MI_BFIELD_BITS) return mi_bchunk_try_clearX(chunk, cidx, maybe_all_clear);
  if (n<=MI_BFIELD_BITS) return mi_bchunk_try_clearNX(chunk, cidx, n, maybe_all_clear);
  return mi_bchunk_try_clearN_(chunk, cidx, n, maybe_all_clear);
}


// ------- mi_bchunk_try_find_and_clear ---------------------------------------

#if MI_OPT_SIMD && defined(__AVX2__)
mi_decl_maybe_unused static inline __m256i mi_mm256_zero(void) {
  return _mm256_setzero_si256();
}
mi_decl_maybe_unused static inline __m256i mi_mm256_ones(void) {
  return _mm256_set1_epi64x(~0);
}
mi_decl_maybe_unused static inline bool mi_mm256_is_ones(__m256i vec) {
  return _mm256_testc_si256(vec, _mm256_cmpeq_epi32(vec, vec));
}
mi_decl_maybe_unused static inline bool mi_mm256_is_zero( __m256i vec) {
  return _mm256_testz_si256(vec,vec);
}
#endif

static inline bool mi_bchunk_try_find_and_clear_at(mi_bchunk_t* chunk, size_t chunk_idx, size_t* pidx) {
  mi_assert_internal(chunk_idx < MI_BCHUNK_FIELDS);
  // note: this must be acquire (and not relaxed), or otherwise the AVX code below can loop forever
  // as the compiler won't reload the registers vec1 and vec2 from memory again.
  const mi_bfield_t b = mi_atomic_load_acquire(&chunk->bfields[chunk_idx]);
  size_t idx;
  if (mi_bfield_find_least_bit(b, &idx)) {           // find the least bit
    if mi_likely(mi_bfield_atomic_try_clear_mask_of(&chunk->bfields[chunk_idx], mi_bfield_mask(1,idx), b, NULL)) {  // clear it atomically
      *pidx = (chunk_idx*MI_BFIELD_BITS) + idx;
      mi_assert_internal(*pidx < MI_BCHUNK_BITS);
      return true;
    }
  }
  return false;
}

// Find least 1-bit in a chunk and try to clear it atomically
// set `*pidx` to the bit index (0 <= *pidx < MI_BCHUNK_BITS) on success.
// This is used to find free slices and abandoned pages and should be efficient.
// todo: try neon version
static inline bool mi_bchunk_try_find_and_clear(mi_bchunk_t* chunk, size_t* pidx) {
  #if MI_OPT_SIMD && defined(__AVX2__) && (MI_BCHUNK_BITS==256)
  while (true) {
    const __m256i vec = _mm256_load_si256((const __m256i*)chunk->bfields);
    const __m256i vcmp = _mm256_cmpeq_epi64(vec, mi_mm256_zero()); // (elem64 == 0 ? 0xFF  : 0)
    const uint32_t mask = ~_mm256_movemask_epi8(vcmp);  // mask of most significant bit of each byte (so each 8 bits are all set or clear)
    // mask is inverted, so each 8-bits is 0xFF iff the corresponding elem64 has a bit set (and thus can be cleared)
    if (mask==0) return false;
    mi_assert_internal((_tzcnt_u32(mask)%8) == 0); // tzcnt == 0, 8, 16, or 24
    const size_t chunk_idx = _tzcnt_u32(mask) / 8;
    if (mi_bchunk_try_find_and_clear_at(chunk, chunk_idx, pidx)) return true;
    // try again
    // note: there must be an atomic release/acquire in between or otherwise the registers may not be reloaded
  }
  #elif MI_OPT_SIMD && defined(__AVX2__) && (MI_BCHUNK_BITS==512)
  while (true) {
    size_t chunk_idx = 0;
    #if 0
    // one vector at a time
    __m256i vec = _mm256_load_si256((const __m256i*)chunk->bfields);
    if (mi_mm256_is_zero(vec)) {
      chunk_idx += 4;
      vec = _mm256_load_si256(((const __m256i*)chunk->bfields) + 1);
    }
    const __m256i vcmp = _mm256_cmpeq_epi64(vec, mi_mm256_zero()); // (elem64 == 0 ? 0xFF  : 0)
    const uint32_t mask = ~_mm256_movemask_epi8(vcmp);  // mask of most significant bit of each byte (so each 8 bits are all set or clear)
    // mask is inverted, so each 8-bits is 0xFF iff the corresponding elem64 has a bit set (and thus can be cleared)
    if (mask==0) return false;
    mi_assert_internal((_tzcnt_u32(mask)%8) == 0); // tzcnt == 0, 8, 16, or 24
    chunk_idx += _tzcnt_u32(mask) / 8;
    #else
    // a cache line is 64b so we can just as well load all at the same time
    const __m256i vec1  = _mm256_load_si256((const __m256i*)chunk->bfields);
    const __m256i vec2  = _mm256_load_si256(((const __m256i*)chunk->bfields)+1);
    const __m256i cmpv  = mi_mm256_zero();
    const __m256i vcmp1 = _mm256_cmpeq_epi64(vec1, cmpv); // (elem64 == 0 ? 0xFF  : 0)
    const __m256i vcmp2 = _mm256_cmpeq_epi64(vec2, cmpv); // (elem64 == 0 ? 0xFF  : 0)
    const uint32_t mask1 = ~_mm256_movemask_epi8(vcmp1);  // mask of most significant bit of each byte (so each 8 bits are all set or clear)
    const uint32_t mask2 = ~_mm256_movemask_epi8(vcmp2);  // mask of most significant bit of each byte (so each 8 bits are all set or clear)
    const uint64_t mask = ((uint64_t)mask2 << 32) | mask1;
    // mask is inverted, so each 8-bits is 0xFF iff the corresponding elem64 has a bit set (and thus can be cleared)
    if (mask==0) return false;
    mi_assert_internal((_tzcnt_u64(mask)%8) == 0); // tzcnt == 0, 8, 16, 24 , ..
    chunk_idx = mi_ctz(mask) / 8;
    #endif
    if (mi_bchunk_try_find_and_clear_at(chunk, chunk_idx, pidx)) return true;
    // try again
    // note: there must be an atomic release/acquire in between or otherwise the registers may not be reloaded
  }
  #elif MI_OPT_SIMD && (MI_BCHUNK_BITS==512) && MI_ARCH_ARM64
  while(true) {
    // a cache line is 64b so we can just as well load all at the same time (?)
    const uint64x2_t vzero1_lo = vceqzq_u64(vld1q_u64((uint64_t*)chunk->bfields));        // 2x64 bit is_zero
    const uint64x2_t vzero1_hi = vceqzq_u64(vld1q_u64((uint64_t*)chunk->bfields + 2));    // 2x64 bit is_zero
    const uint64x2_t vzero2_lo = vceqzq_u64(vld1q_u64((uint64_t*)chunk->bfields + 4));    // 2x64 bit is_zero
    const uint64x2_t vzero2_hi = vceqzq_u64(vld1q_u64((uint64_t*)chunk->bfields + 6));    // 2x64 bit is_zero
    const uint32x4_t vzero1    = vuzp1q_u32(vreinterpretq_u32_u64(vzero1_lo),vreinterpretq_u32_u64(vzero1_hi)); // unzip even elements: narrow to 4x32 bit is_zero ()
    const uint32x4_t vzero2    = vuzp1q_u32(vreinterpretq_u32_u64(vzero2_lo),vreinterpretq_u32_u64(vzero2_hi)); // unzip even elements: narrow to 4x32 bit is_zero ()
    const uint32x4_t vzero1x   = vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(vzero1), 24));        // shift-right 2x32bit elem by 24: lo 16 bits contain the 2 lo bytes
    const uint32x4_t vzero2x   = vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(vzero2), 24));
    const uint16x8_t vzero12   = vreinterpretq_u16_u32(vuzp1q_u32(vzero1x,vzero2x));                           // unzip even 32-bit elements into one vector
    const uint8x8_t  vzero     = vmovn_u32(vzero12);                                                           // narrow the bottom 16-bits
    const uint64_t mask = ~vget_lane_u64(vreinterpret_u64_u8(vzero), 0);  // 1 byte for each bfield (0xFF => bfield has a bit set)
    if (mask==0) return false;
    mi_assert_internal((mi_ctz(mask)%8) == 0); // tzcnt == 0, 8, 16, 24 , ..
    const size_t chunk_idx = mi_ctz(mask) / 8;
    if (mi_bchunk_try_find_and_clear_at(chunk, chunk_idx, pidx)) return true;
    // try again
    // note: there must be an atomic release/acquire in between or otherwise the registers may not be reloaded
  }
  #else
  for (int i = 0; i < MI_BCHUNK_FIELDS; i++) {
    if (mi_bchunk_try_find_and_clear_at(chunk, i, pidx)) return true;
  }
  return false;
  #endif
}

static inline bool mi_bchunk_try_find_and_clear_1(mi_bchunk_t* chunk, size_t n, size_t* pidx) {
  mi_assert_internal(n==1); MI_UNUSED(n);
  return mi_bchunk_try_find_and_clear(chunk, pidx);
}

mi_decl_maybe_unused static inline bool mi_bchunk_try_find_and_clear8_at(mi_bchunk_t* chunk, size_t chunk_idx, size_t* pidx) {
  const mi_bfield_t b = mi_atomic_load_relaxed(&chunk->bfields[chunk_idx]);
  // has_set8 has low bit in each byte set if the byte in x == 0xFF
  const mi_bfield_t has_set8 =
    ((~b - MI_BFIELD_LO_BIT8) &      // high bit set if byte in x is 0xFF or < 0x7F
     (b  & MI_BFIELD_HI_BIT8))       // high bit set if byte in x is >= 0x80
     >> 7;                           // shift high bit to low bit
  size_t idx;
  if (mi_bfield_find_least_bit(has_set8, &idx)) { // find least 1-bit
    mi_assert_internal(idx <= (MI_BFIELD_BITS - 8));
    mi_assert_internal((idx%8)==0);
    if mi_likely(mi_bfield_atomic_try_clear_mask_of(&chunk->bfields[chunk_idx], (mi_bfield_t)0xFF << idx, b, NULL)) {  // unset the byte atomically
      *pidx = (chunk_idx*MI_BFIELD_BITS) + idx;
      mi_assert_internal(*pidx + 8 <= MI_BCHUNK_BITS);
      return true;
    }
  }
  return false;
}

// find least aligned byte in a chunk with all bits set, and try unset it atomically
// set `*pidx` to its bit index (0 <= *pidx < MI_BCHUNK_BITS) on success.
// Used to find medium size pages in the free blocks.
// todo: try neon version
static mi_decl_noinline bool mi_bchunk_try_find_and_clear8(mi_bchunk_t* chunk, size_t* pidx) {
  #if MI_OPT_SIMD && defined(__AVX2__) && (MI_BCHUNK_BITS==512)
  while (true) {
    // since a cache-line is 64b, load all at once
    const __m256i vec1 = _mm256_load_si256((const __m256i*)chunk->bfields);
    const __m256i vec2 = _mm256_load_si256((const __m256i*)chunk->bfields+1);
    const __m256i cmpv = mi_mm256_ones();
    const __m256i vcmp1 = _mm256_cmpeq_epi8(vec1, cmpv); // (byte == ~0 ? 0xFF : 0)
    const __m256i vcmp2 = _mm256_cmpeq_epi8(vec2, cmpv); // (byte == ~0 ? 0xFF : 0)
    const uint32_t mask1 = _mm256_movemask_epi8(vcmp1);    // mask of most significant bit of each byte
    const uint32_t mask2 = _mm256_movemask_epi8(vcmp2);    // mask of most significant bit of each byte
    const uint64_t mask = ((uint64_t)mask2 << 32) | mask1;
    // mask is inverted, so each bit is 0xFF iff the corresponding byte has a bit set (and thus can be cleared)
    if (mask==0) return false;
    const size_t bidx = _tzcnt_u64(mask);          // byte-idx of the byte in the chunk
    const size_t chunk_idx = bidx / 8;
    const size_t idx = (bidx % 8)*8;
    mi_assert_internal(chunk_idx < MI_BCHUNK_FIELDS);
    if mi_likely(mi_bfield_atomic_try_clear8(&chunk->bfields[chunk_idx], idx, NULL)) {  // clear it atomically
      *pidx = (chunk_idx*MI_BFIELD_BITS) + idx;
      mi_assert_internal(*pidx + 8 <= MI_BCHUNK_BITS);
      return true;
    }
    // try again
    // note: there must be an atomic release/acquire in between or otherwise the registers may not be reloaded  }
  }
  #else
    for (int i = 0; i < MI_BCHUNK_FIELDS; i++) {
      if (mi_bchunk_try_find_and_clear8_at(chunk, i, pidx)) return true;
    }
    return false;
  #endif
}

static inline bool mi_bchunk_try_find_and_clear_8(mi_bchunk_t* chunk, size_t n, size_t* pidx) {
  mi_assert_internal(n==8); MI_UNUSED(n);
  return mi_bchunk_try_find_and_clear8(chunk, pidx);
}


// find least aligned bfield in a chunk with all bits set, and try unset it atomically
// set `*pidx` to its bit index (0 <= *pidx < MI_BCHUNK_BITS) on success.
// Used to find large size pages in the free blocks.
// todo: try neon version
/*
static mi_decl_noinline  bool mi_bchunk_try_find_and_clearX(mi_bchunk_t* chunk, size_t* pidx) {
  #if MI_OPT_SIMD && defined(__AVX2__) && (MI_BCHUNK_BITS==512)
  while (true) {
    // since a cache-line is 64b, load all at once
    const __m256i vec1 = _mm256_load_si256((const __m256i*)chunk->bfields);
    const __m256i vec2 = _mm256_load_si256((const __m256i*)chunk->bfields+1);
    const __m256i cmpv = mi_mm256_ones();
    const __m256i vcmp1 = _mm256_cmpeq_epi64(vec1, cmpv); // (bfield == ~0 ? -1 : 0)
    const __m256i vcmp2 = _mm256_cmpeq_epi64(vec2, cmpv); // (bfield == ~0 ? -1 : 0)
    const uint32_t mask1 = _mm256_movemask_epi8(vcmp1);    // mask of most significant bit of each byte
    const uint32_t mask2 = _mm256_movemask_epi8(vcmp2);    // mask of most significant bit of each byte
    const uint64_t mask = ((uint64_t)mask2 << 32) | mask1;
    // mask is inverted, so each 8-bits are set iff the corresponding elem64 has all bits set (and thus can be cleared)
    if (mask==0) return false;
    mi_assert_internal((_tzcnt_u64(mask)%8) == 0); // tzcnt == 0, 8, 16, 24 , ..
    const size_t chunk_idx = _tzcnt_u64(mask) / 8;
    mi_assert_internal(chunk_idx < MI_BCHUNK_FIELDS);
    if mi_likely(mi_bfield_atomic_try_clearX(&chunk->bfields[chunk_idx],NULL)) {
      *pidx = chunk_idx*MI_BFIELD_BITS;
      mi_assert_internal(*pidx + MI_BFIELD_BITS <= MI_BCHUNK_BITS);
      return true;
    }
    // try again
    // note: there must be an atomic release/acquire in between or otherwise the registers may not be reloaded
  }
#else
  for (int i = 0; i < MI_BCHUNK_FIELDS; i++) {
    const mi_bfield_t b = mi_atomic_load_relaxed(&chunk->bfields[i]);
    if (~b==0 && mi_bfield_atomic_try_clearX(&chunk->bfields[i], NULL)) {
      *pidx = i*MI_BFIELD_BITS;
      mi_assert_internal(*pidx + MI_BFIELD_BITS <= MI_BCHUNK_BITS);
      return true;
    }
  }
  return false;
#endif
}

static inline bool mi_bchunk_try_find_and_clear_X(mi_bchunk_t* chunk, size_t n, size_t* pidx) {
  mi_assert_internal(n==MI_BFIELD_BITS); MI_UNUSED(n);
  return mi_bchunk_try_find_and_clearX(chunk, pidx);
}
*/

// find a sequence of `n` bits in a chunk with `0 < n <= MI_BFIELD_BITS` with all bits set,
// and try to clear them atomically.
// set `*pidx` to its bit index (0 <= *pidx <= MI_BCHUNK_BITS - n) on success.
// will cross bfield boundaries.
mi_decl_noinline static bool mi_bchunk_try_find_and_clearNX(mi_bchunk_t* chunk, size_t n, size_t* pidx) {
  if (n == 0 || n > MI_BFIELD_BITS) return false;
  const mi_bfield_t mask = mi_bfield_mask(n, 0);
  // for all fields in the chunk
  for (int i = 0; i < MI_BCHUNK_FIELDS; i++) {
    mi_bfield_t b0 = mi_atomic_load_relaxed(&chunk->bfields[i]);
    mi_bfield_t b = b0;
    size_t idx;

    // is there a range inside the field?
    while (mi_bfield_find_least_bit(b, &idx)) { // find least 1-bit
      if (idx + n > MI_BFIELD_BITS) break; // too short: maybe cross over, or continue with the next field

      const size_t bmask = mask<<idx;
      mi_assert_internal(bmask>>idx == mask);
      if ((b&bmask) == bmask) { // found a match with all bits set, try clearing atomically
        if mi_likely(mi_bfield_atomic_try_clear_mask_of(&chunk->bfields[i], bmask, b0, NULL)) {
          *pidx = (i*MI_BFIELD_BITS) + idx;
          mi_assert_internal(*pidx < MI_BCHUNK_BITS);
          mi_assert_internal(*pidx + n <= MI_BCHUNK_BITS);
          return true;
        }
        else {
          // if we failed to atomically commit, reload b and try again from the start
          b = b0 = mi_atomic_load_acquire(&chunk->bfields[i]);
        }
      }
      else {
        // advance by clearing the least run of ones, for example, with n>=4, idx=2:
        // b             = 1111 1101 1010 1100
        // .. + (1<<idx) = 1111 1101 1011 0000
        // .. & b        = 1111 1101 1010 0000
        b = b & (b + (mi_bfield_one() << idx));
      }
    }

    // check if we can cross into the next bfield
    if (b!=0 && i < MI_BCHUNK_FIELDS-1) {
      const size_t post = mi_bfield_clz(~b);
      if (post > 0) {
        const size_t pre = mi_bfield_ctz(~mi_atomic_load_relaxed(&chunk->bfields[i+1]));
        if (post + pre >= n) {
          // it fits -- try to claim it atomically
          const size_t cidx = (i*MI_BFIELD_BITS) + (MI_BFIELD_BITS - post);
          if (mi_bchunk_try_clearNX(chunk, cidx, n, NULL)) {
            // we cleared all atomically
            *pidx = cidx;
            mi_assert_internal(*pidx < MI_BCHUNK_BITS);
            mi_assert_internal(*pidx + n <= MI_BCHUNK_BITS);
            return true;
          }
        }
      }
    }
  }
  return false;
}

// find a sequence of `n` bits in a chunk with `n < MI_BCHUNK_BITS` with all bits set,
// and try to clear them atomically.
// set `*pidx` to its bit index (0 <= *pidx <= MI_BCHUNK_BITS - n) on success.
// This can cross bfield boundaries.
static mi_decl_noinline bool mi_bchunk_try_find_and_clearN_(mi_bchunk_t* chunk, size_t n, size_t* pidx) {
  if (n == 0 || n > MI_BCHUNK_BITS) return false;  // cannot be more than a chunk

  // we first scan ahead to see if there is a range of `n` set bits, and only then try to clear atomically
  mi_assert_internal(n>0);
  const size_t skip_count = (n-1)/MI_BFIELD_BITS;
  size_t cidx;
  for (size_t i = 0; i < MI_BCHUNK_FIELDS - skip_count; i++)
  {
    size_t m = n;   // bits to go

    // first field
    mi_bfield_t b = mi_atomic_load_relaxed(&chunk->bfields[i]);
    size_t ones = mi_bfield_clz(~b);
    cidx = (i*MI_BFIELD_BITS) + (MI_BFIELD_BITS - ones);  // start index
    if (ones >= m) {
      // we found enough bits!
      m = 0;
    }
    else {
      m -= ones;

      // keep scanning further fields?
      size_t j = 1;   // field count from i
      while (i+j < MI_BCHUNK_FIELDS) {
        mi_assert_internal(m > 0);
        b = mi_atomic_load_relaxed(&chunk->bfields[i+j]);
        ones = mi_bfield_ctz(~b);
        if (ones >= m) {
          // we found enough bits
          m = 0;
          break;
        }
        else if (ones == MI_BFIELD_BITS) {
          // not enough yet, proceed to the next field
          j++;
          m -= MI_BFIELD_BITS;
        }
        else {
          // the range was not enough, start from scratch
          i = i + j - 1;  // no need to re-scan previous fields, except the last one (with clz this time)
          mi_assert_internal(m>0);
          break;
        }
      }
    }

    // did we find a range?
    if (m==0) {
      if (mi_bchunk_try_clearN(chunk, cidx, n, NULL)) {
        // we cleared all atomically
        *pidx = cidx;
        mi_assert_internal(*pidx < MI_BCHUNK_BITS);
        mi_assert_internal(*pidx + n <= MI_BCHUNK_BITS);
        return true;
      }
      // note: if we fail for a small `n` on the first field, we don't rescan that field (as `i` is incremented)
    }
    // otherwise continue searching
  }
  return false;
}



// ------- mi_bchunk_clear_once_set ---------------------------------------

static inline void mi_bchunk_clear_once_set(mi_bchunk_t* chunk, size_t cidx) {
  mi_assert_internal(cidx < MI_BCHUNK_BITS);
  const size_t i = cidx / MI_BFIELD_BITS;
  const size_t idx = cidx % MI_BFIELD_BITS;
  mi_bfield_atomic_clear_once_set(&chunk->bfields[i], idx);
}


// ------- mi_bitmap_all_are_clear ---------------------------------------


// are all bits in a bitmap chunk clear?
static inline bool mi_bchunk_all_are_clear_relaxed(mi_bchunk_t* chunk) {
  #if MI_OPT_SIMD && defined(__AVX2__) && (MI_BCHUNK_BITS==256)
  const __m256i vec = _mm256_load_si256((const __m256i*)chunk->bfields);
  return mi_mm256_is_zero(vec);
  #elif MI_OPT_SIMD &&  defined(__AVX2__) && (MI_BCHUNK_BITS==512)
  // a 64b cache-line contains the entire chunk anyway so load both at once
  const __m256i vec1 = _mm256_load_si256((const __m256i*)chunk->bfields);
  const __m256i vec2 = _mm256_load_si256(((const __m256i*)chunk->bfields)+1);
  return (mi_mm256_is_zero(_mm256_or_si256(vec1,vec2)));
  #elif MI_OPT_SIMD && (MI_BCHUNK_BITS==512) && MI_ARCH_ARM64
  const uint64x2_t v0 = vld1q_u64((uint64_t*)chunk->bfields);
  const uint64x2_t v1 = vld1q_u64((uint64_t*)chunk->bfields + 2);
  const uint64x2_t v2 = vld1q_u64((uint64_t*)chunk->bfields + 4);
  const uint64x2_t v3 = vld1q_u64((uint64_t*)chunk->bfields + 6);
  const uint64x2_t v  = vorrq_u64(vorrq_u64(v0,v1),vorrq_u64(v2,v3));
  return (vmaxvq_u32(vreinterpretq_u32_u64(v)) == 0);
  #else
  for (int i = 0; i < MI_BCHUNK_FIELDS; i++) {
    if (mi_atomic_load_relaxed(&chunk->bfields[i]) != 0) return false;
  }
  return true;
  #endif
}

// are all bits in a bitmap chunk set?
static inline bool mi_bchunk_all_are_set_relaxed(mi_bchunk_t* chunk) {
#if MI_OPT_SIMD && defined(__AVX2__) && (MI_BCHUNK_BITS==256)
  const __m256i vec = _mm256_load_si256((const __m256i*)chunk->bfields);
  return mi_mm256_is_ones(vec);
#elif MI_OPT_SIMD &&  defined(__AVX2__) && (MI_BCHUNK_BITS==512)
  // a 64b cache-line contains the entire chunk anyway so load both at once
  const __m256i vec1 = _mm256_load_si256((const __m256i*)chunk->bfields);
  const __m256i vec2 = _mm256_load_si256(((const __m256i*)chunk->bfields)+1);
  return (mi_mm256_is_ones(_mm256_and_si256(vec1, vec2)));
#elif MI_OPT_SIMD && (MI_BCHUNK_BITS==512) && MI_ARCH_ARM64
  const uint64x2_t v0 = vld1q_u64((uint64_t*)chunk->bfields);
  const uint64x2_t v1 = vld1q_u64((uint64_t*)chunk->bfields + 2);
  const uint64x2_t v2 = vld1q_u64((uint64_t*)chunk->bfields + 4);
  const uint64x2_t v3 = vld1q_u64((uint64_t*)chunk->bfields + 6);
  const uint64x2_t v  = vandq_u64(vandq_u64(v0,v1),vandq_u64(v2,v3));
  return (vminvq_u32(vreinterpretq_u32_u64(v)) == 0xFFFFFFFFUL);
#else
  for (int i = 0; i < MI_BCHUNK_FIELDS; i++) {
    if (~mi_atomic_load_relaxed(&chunk->bfields[i]) != 0) return false;
  }
  return true;
#endif
}


static bool mi_bchunk_bsr(mi_bchunk_t* chunk, size_t* pidx) {
  for (size_t i = MI_BCHUNK_FIELDS; i > 0; ) {
    i--;
    mi_bfield_t b = mi_atomic_load_relaxed(&chunk->bfields[i]);
    size_t idx;
    if (mi_bsr(b, &idx)) {
      *pidx = (i*MI_BFIELD_BITS) + idx;
      return true;
    }
  }
  return false;
}

static size_t mi_bchunk_popcount(mi_bchunk_t* chunk) {
  size_t popcount = 0;
  for (size_t i = 0; i < MI_BCHUNK_FIELDS; i++) {
    const mi_bfield_t b = mi_atomic_load_relaxed(&chunk->bfields[i]);
    popcount += mi_bfield_popcount(b);
  }
  return popcount;
}


/* --------------------------------------------------------------------------------
 bitmap chunkmap
-------------------------------------------------------------------------------- */

static void mi_bitmap_chunkmap_set(mi_bitmap_t* bitmap, size_t chunk_idx) {
  mi_assert(chunk_idx < mi_bitmap_chunk_count(bitmap));
  mi_bchunk_set(&bitmap->chunkmap, chunk_idx, NULL);
}

static bool mi_bitmap_chunkmap_try_clear(mi_bitmap_t* bitmap, size_t chunk_idx) {
  mi_assert(chunk_idx < mi_bitmap_chunk_count(bitmap));
  // check if the corresponding chunk is all clear
  if (!mi_bchunk_all_are_clear_relaxed(&bitmap->chunks[chunk_idx])) return false;
  // clear the chunkmap bit
  mi_bchunk_clear(&bitmap->chunkmap, chunk_idx, NULL);
  // .. but a concurrent set may have happened in between our all-clear test and the clearing of the
  // bit in the mask. We check again to catch this situation.
  if (!mi_bchunk_all_are_clear_relaxed(&bitmap->chunks[chunk_idx])) {
    mi_bchunk_set(&bitmap->chunkmap, chunk_idx, NULL);
    return false;
  }
  return true;
}


/* --------------------------------------------------------------------------------
  bitmap
-------------------------------------------------------------------------------- */

size_t mi_bitmap_size(size_t bit_count, size_t* pchunk_count) {
  mi_assert_internal((bit_count % MI_BCHUNK_BITS) == 0);
  bit_count = _mi_align_up(bit_count, MI_BCHUNK_BITS);
  mi_assert_internal(bit_count <= MI_BITMAP_MAX_BIT_COUNT);
  mi_assert_internal(bit_count > 0);
  const size_t chunk_count = bit_count / MI_BCHUNK_BITS;
  mi_assert_internal(chunk_count >= 1);
  const size_t size = offsetof(mi_bitmap_t,chunks) + (chunk_count * MI_BCHUNK_SIZE);
  mi_assert_internal( (size%MI_BCHUNK_SIZE) == 0 );
  if (pchunk_count != NULL) { *pchunk_count = chunk_count;  }
  return size;
}


// initialize a bitmap to all unset; avoid a mem_zero if `already_zero` is true
// returns the size of the bitmap
size_t mi_bitmap_init(mi_bitmap_t* bitmap, size_t bit_count, bool already_zero) {
  size_t chunk_count;
  const size_t size = mi_bitmap_size(bit_count, &chunk_count);
  if (!already_zero) {
    _mi_memzero_aligned(bitmap, size);
  }
  mi_atomic_store_release(&bitmap->chunk_count, chunk_count);
  mi_assert_internal(mi_atomic_load_relaxed(&bitmap->chunk_count) <= MI_BITMAP_MAX_CHUNK_COUNT);
  return size;
}


// Set a sequence of `n` bits in the bitmap (and can cross chunks). Not atomic so only use if local to a thread.
static void mi_bchunks_unsafe_setN(mi_bchunk_t* chunks, mi_bchunkmap_t* cmap, size_t idx, size_t n) {
  mi_assert_internal(n>0);

  // start chunk and index
  size_t chunk_idx = idx / MI_BCHUNK_BITS;
  const size_t cidx = idx % MI_BCHUNK_BITS;
  const size_t ccount = _mi_divide_up(n, MI_BCHUNK_BITS);

  // first update the chunkmap
  mi_bchunk_setN(cmap, chunk_idx, ccount, NULL);

  // first chunk
  size_t m = MI_BCHUNK_BITS - cidx;
  if (m > n) { m = n; }
  mi_bchunk_setN(&chunks[chunk_idx], cidx, m, NULL);

  // n can be large so use memset for efficiency for all in-between chunks
  chunk_idx++;
  n -= m;
  const size_t mid_chunks = n / MI_BCHUNK_BITS;
  if (mid_chunks > 0) {
    _mi_memset(&chunks[chunk_idx], ~0, mid_chunks * MI_BCHUNK_SIZE);
    chunk_idx += mid_chunks;
    n -= (mid_chunks * MI_BCHUNK_BITS);
  }

  // last chunk
  if (n > 0) {
    mi_assert_internal(n < MI_BCHUNK_BITS);
    mi_assert_internal(chunk_idx < MI_BCHUNK_FIELDS);
    mi_bchunk_setN(&chunks[chunk_idx], 0, n, NULL);
  }
}

// Set a sequence of `n` bits in the bitmap (and can cross chunks). Not atomic so only use if local to a thread.
void mi_bitmap_unsafe_setN(mi_bitmap_t* bitmap, size_t idx, size_t n) {
  mi_assert_internal(n>0);
  mi_assert_internal(idx + n <= mi_bitmap_max_bits(bitmap));
  mi_bchunks_unsafe_setN(&bitmap->chunks[0], &bitmap->chunkmap, idx, n);
}




// ------- mi_bitmap_xset ---------------------------------------

// Set a sequence of `n` bits in the bitmap; returns `true` if atomically transitioned from 0's to 1's (or 1's to 0's).
// `n` cannot cross chunk boundaries (and `n <= MI_BCHUNK_BITS`)!
bool mi_bitmap_setN(mi_bitmap_t* bitmap, size_t idx, size_t n, size_t* already_set) {
  mi_assert_internal(n>0);
  mi_assert_internal(n<=MI_BCHUNK_BITS);

  const size_t chunk_idx = idx / MI_BCHUNK_BITS;
  const size_t cidx = idx % MI_BCHUNK_BITS;
  mi_assert_internal(cidx + n <= MI_BCHUNK_BITS);  // don't cross chunks (for now)
  mi_assert_internal(chunk_idx < mi_bitmap_chunk_count(bitmap));
  if (cidx + n > MI_BCHUNK_BITS) { n = MI_BCHUNK_BITS - cidx; }  // paranoia

  const bool were_allclear = mi_bchunk_setN(&bitmap->chunks[chunk_idx], cidx, n, already_set);
  mi_bitmap_chunkmap_set(bitmap, chunk_idx);   // set afterwards
  return were_allclear;
}

// Clear a sequence of `n` bits in the bitmap; returns `true` if atomically transitioned from 1's to 0's.
// `n` cannot cross chunk boundaries (and `n <= MI_BCHUNK_BITS`)!
bool mi_bitmap_clearN(mi_bitmap_t* bitmap, size_t idx, size_t n) {
  mi_assert_internal(n>0);
  mi_assert_internal(n<=MI_BCHUNK_BITS);

  const size_t chunk_idx = idx / MI_BCHUNK_BITS;
  const size_t cidx = idx % MI_BCHUNK_BITS;
  mi_assert_internal(cidx + n <= MI_BCHUNK_BITS);  // don't cross chunks (for now)
  mi_assert_internal(chunk_idx < mi_bitmap_chunk_count(bitmap));
  if (cidx + n > MI_BCHUNK_BITS) { n = MI_BCHUNK_BITS - cidx; }  // paranoia

  bool maybe_all_clear;
  const bool were_allset = mi_bchunk_clearN(&bitmap->chunks[chunk_idx], cidx, n, &maybe_all_clear);
  if (maybe_all_clear) { mi_bitmap_chunkmap_try_clear(bitmap, chunk_idx); }
  return were_allset;
}

// Count bits set in a range of `n` bits.
// `n` cannot cross chunk boundaries (and `n <= MI_BCHUNK_BITS`)!
size_t mi_bitmap_popcountN( mi_bitmap_t* bitmap, size_t idx, size_t n) {
  mi_assert_internal(n>0);
  mi_assert_internal(n<=MI_BCHUNK_BITS);

  const size_t chunk_idx = idx / MI_BCHUNK_BITS;
  const size_t cidx = idx % MI_BCHUNK_BITS;
  mi_assert_internal(cidx + n <= MI_BCHUNK_BITS);  // don't cross chunks (for now)
  mi_assert_internal(chunk_idx < mi_bitmap_chunk_count(bitmap));
  if (cidx + n > MI_BCHUNK_BITS) { n = MI_BCHUNK_BITS - cidx; }  // paranoia
  return mi_bchunk_popcountN(&bitmap->chunks[chunk_idx], cidx, n);
}


// Set/clear a bit in the bitmap; returns `true` if atomically transitioned from 0 to 1 (or 1 to 0)
bool mi_bitmap_set(mi_bitmap_t* bitmap, size_t idx) {
  return mi_bitmap_setN(bitmap, idx, 1, NULL);
}

bool mi_bitmap_clear(mi_bitmap_t* bitmap, size_t idx) {
  return mi_bitmap_clearN(bitmap, idx, 1);
}



// ------- mi_bitmap_is_xset ---------------------------------------

// Is a sequence of n bits already all set/cleared?
bool mi_bitmap_is_xsetN(mi_xset_t set, mi_bitmap_t* bitmap, size_t idx, size_t n) {
  mi_assert_internal(n>0);
  mi_assert_internal(n<=MI_BCHUNK_BITS);
  mi_assert_internal(idx + n <= mi_bitmap_max_bits(bitmap));

  const size_t chunk_idx = idx / MI_BCHUNK_BITS;
  const size_t cidx = idx % MI_BCHUNK_BITS;
  mi_assert_internal(cidx + n <= MI_BCHUNK_BITS);  // don't cross chunks (for now)
  mi_assert_internal(chunk_idx < mi_bitmap_chunk_count(bitmap));
  if (cidx + n > MI_BCHUNK_BITS) { n = MI_BCHUNK_BITS - cidx; }  // paranoia

  return mi_bchunk_is_xsetN(set, &bitmap->chunks[chunk_idx], cidx, n);
}




/* --------------------------------------------------------------------------------
  Iterate through a bfield
-------------------------------------------------------------------------------- */

// Cycle iteration through a bitfield. This is used to space out threads
// so there is less chance of contention. When searching for a free page we
// like to first search only the accessed part (so we reuse better). This
// high point is called the `cycle`.
//
// We then iterate through the bitfield as:
// first: [start, cycle>
// then : [0, start>
// then : [cycle, MI_BFIELD_BITS>
//
// The start is determined usually as `tseq % cycle` to have each thread
// start at a different spot.
// - We use `popcount` to improve branch prediction (maybe not needed? can we simplify?)
// - The `cycle_mask` is the part `[start, cycle>`.
#define mi_bfield_iterate(bfield,start,cycle,name_idx,SUF) { \
  mi_assert_internal(start <= cycle); \
  mi_assert_internal(start < MI_BFIELD_BITS); \
  mi_assert_internal(cycle <= MI_BFIELD_BITS); \
  mi_bfield_t _cycle_mask##SUF = mi_bfield_mask(cycle - start, start); \
  size_t _bcount##SUF = mi_bfield_popcount(bfield); \
  mi_bfield_t _b##SUF = bfield & _cycle_mask##SUF; /* process [start, cycle> first*/\
  while(_bcount##SUF > 0) { \
    _bcount##SUF--;\
    if (_b##SUF==0) { _b##SUF = bfield & ~_cycle_mask##SUF; } /* process [0,start> + [cycle, MI_BFIELD_BITS> next */ \
    /* size_t name_idx; */ \
    bool _found##SUF = mi_bfield_find_least_bit(_b##SUF,&name_idx); \
    mi_assert_internal(_found##SUF); MI_UNUSED(_found##SUF); \
    { \

#define mi_bfield_iterate_end(SUF) \
    } \
    _b##SUF = mi_bfield_clear_least_bit(_b##SUF); \
  } \
}

#define mi_bfield_cycle_iterate(bfield,tseq,cycle,name_idx,SUF) { \
  const size_t _start##SUF = (uint32_t)(tseq) % (uint32_t)(cycle); /* or: 0 to always search from the start? */\
  mi_bfield_iterate(bfield,_start##SUF,cycle,name_idx,SUF)

#define mi_bfield_cycle_iterate_end(SUF) \
  mi_bfield_iterate_end(SUF); }


/* --------------------------------------------------------------------------------
  mi_bitmap_find
  (used to find free pages)
-------------------------------------------------------------------------------- */

typedef bool (mi_bitmap_visit_fun_t)(mi_bitmap_t* bitmap, size_t chunk_idx, size_t n, size_t* idx, void* arg1, void* arg2);

// Go through the bitmap and for every sequence of `n` set bits, call the visitor function.
// If it returns `true` stop the search.
static inline bool mi_bitmap_find(mi_bitmap_t* bitmap, size_t tseq, size_t n, size_t* pidx, mi_bitmap_visit_fun_t* on_find, void* arg1, void* arg2)
{
  const size_t chunkmap_max = _mi_divide_up(mi_bitmap_chunk_count(bitmap), MI_BFIELD_BITS);
  for (size_t i = 0; i < chunkmap_max; i++) {
    // and for each chunkmap entry we iterate over its bits to find the chunks
    const mi_bfield_t cmap_entry = mi_atomic_load_relaxed(&bitmap->chunkmap.bfields[i]);
    size_t hi;
    if (mi_bfield_find_highest_bit(cmap_entry, &hi)) {
      size_t eidx = 0;
      mi_bfield_cycle_iterate(cmap_entry, tseq%8, hi+1, eidx, Y) // reduce the tseq to 8 bins to reduce using extra memory (see `mstress`)
      {
        mi_assert_internal(eidx <= MI_BFIELD_BITS);
        const size_t chunk_idx = i*MI_BFIELD_BITS + eidx;
        mi_assert_internal(chunk_idx < mi_bitmap_chunk_count(bitmap));
        if ((*on_find)(bitmap, chunk_idx, n, pidx, arg1, arg2)) {
          return true;
        }
      }
      mi_bfield_cycle_iterate_end(Y);
    }
  }
  return false;
}


/* --------------------------------------------------------------------------------
  Bitmap: try_find_and_claim  -- used to allocate abandoned pages
  note: the compiler will fully inline the indirect function call
-------------------------------------------------------------------------------- */

typedef struct mi_claim_fun_data_s {
  mi_arena_t*   arena;
  mi_heaptag_t  heap_tag;
} mi_claim_fun_data_t;

static bool mi_bitmap_try_find_and_claim_visit(mi_bitmap_t* bitmap, size_t chunk_idx, size_t n, size_t* pidx, void* arg1, void* arg2)
{
  mi_assert_internal(n==1); MI_UNUSED(n);
  mi_claim_fun_t* claim_fun = (mi_claim_fun_t*)arg1;
  mi_claim_fun_data_t* claim_data = (mi_claim_fun_data_t*)arg2;
  size_t cidx;
  if mi_likely(mi_bchunk_try_find_and_clear(&bitmap->chunks[chunk_idx], &cidx)) {
    const size_t slice_index = (chunk_idx * MI_BCHUNK_BITS) + cidx;
    mi_assert_internal(slice_index < mi_bitmap_max_bits(bitmap));
    bool keep_set = true;
    if ((*claim_fun)(slice_index, claim_data->arena, claim_data->heap_tag, &keep_set)) {
      // success!
      mi_assert_internal(!keep_set);
      *pidx = slice_index;
      return true;
    }
    else {
      // failed to claim it, set abandoned mapping again (unless the page was freed)
      if (keep_set) {
        const bool wasclear = mi_bchunk_set(&bitmap->chunks[chunk_idx], cidx, NULL);
        mi_assert_internal(wasclear); MI_UNUSED(wasclear);
      }
    }
  }
  else {
    // we may find that all are cleared only on a second iteration but that is ok as
    // the chunkmap is a conservative approximation.
    mi_bitmap_chunkmap_try_clear(bitmap, chunk_idx);
  }
  return false;
}

// Find a set bit in the bitmap and try to atomically clear it and claim it.
// (Used to find pages in the pages_abandoned bitmaps.)
mi_decl_nodiscard bool mi_bitmap_try_find_and_claim(mi_bitmap_t* bitmap, size_t tseq, size_t* pidx,
  mi_claim_fun_t* claim, mi_arena_t* arena, mi_heaptag_t heap_tag)
{
  mi_claim_fun_data_t claim_data = { arena, heap_tag };
  return mi_bitmap_find(bitmap, tseq, 1, pidx, &mi_bitmap_try_find_and_claim_visit, (void*)claim, &claim_data);
}


bool mi_bitmap_bsr(mi_bitmap_t* bitmap, size_t* idx) {
  const size_t chunkmap_max = _mi_divide_up(mi_bitmap_chunk_count(bitmap), MI_BFIELD_BITS);
  for (size_t i = chunkmap_max; i > 0; ) {
    i--;
    mi_bfield_t cmap = mi_atomic_load_relaxed(&bitmap->chunkmap.bfields[i]);
    size_t cmap_idx;
    if (mi_bsr(cmap,&cmap_idx)) {
      // highest chunk
      const size_t chunk_idx = i*MI_BFIELD_BITS + cmap_idx;
      size_t cidx;
      if (mi_bchunk_bsr(&bitmap->chunks[chunk_idx], &cidx)) {
        *idx = (chunk_idx * MI_BCHUNK_BITS) + cidx;
        return true;
      }
    }
  }
  return false;
}

// Return count of all set bits in a bitmap.
size_t mi_bitmap_popcount(mi_bitmap_t* bitmap) {
  // for all chunkmap entries
  size_t popcount = 0;
  const size_t chunkmap_max = _mi_divide_up(mi_bitmap_chunk_count(bitmap), MI_BFIELD_BITS);
  for (size_t i = 0; i < chunkmap_max; i++) {
    mi_bfield_t cmap_entry = mi_atomic_load_relaxed(&bitmap->chunkmap.bfields[i]);
    size_t cmap_idx;
    // for each chunk (corresponding to a set bit in a chunkmap entry)
    while (mi_bfield_foreach_bit(&cmap_entry, &cmap_idx)) {
      const size_t chunk_idx = i*MI_BFIELD_BITS + cmap_idx;
      // count bits in a chunk
      popcount += mi_bchunk_popcount(&bitmap->chunks[chunk_idx]);
    }
  }
  return popcount;
}



// Clear a bit once it is set.
void mi_bitmap_clear_once_set(mi_bitmap_t* bitmap, size_t idx) {
  mi_assert_internal(idx < mi_bitmap_max_bits(bitmap));
  const size_t chunk_idx = idx / MI_BCHUNK_BITS;
  const size_t cidx = idx % MI_BCHUNK_BITS;
  mi_assert_internal(chunk_idx < mi_bitmap_chunk_count(bitmap));
  mi_bchunk_clear_once_set(&bitmap->chunks[chunk_idx], cidx);
}


// Visit all set bits in a bitmap.
// todo: optimize further? maybe use avx512 to directly get all indices using a mask_compressstore?
bool _mi_bitmap_forall_set(mi_bitmap_t* bitmap, mi_forall_set_fun_t* visit, mi_arena_t* arena, void* arg) {
  // for all chunkmap entries
  const size_t chunkmap_max = _mi_divide_up(mi_bitmap_chunk_count(bitmap), MI_BFIELD_BITS);
  for(size_t i = 0; i < chunkmap_max; i++) {
    mi_bfield_t cmap_entry = mi_atomic_load_relaxed(&bitmap->chunkmap.bfields[i]);
    size_t cmap_idx;
    // for each chunk (corresponding to a set bit in a chunkmap entry)
    while (mi_bfield_foreach_bit(&cmap_entry, &cmap_idx)) {
      const size_t chunk_idx = i*MI_BFIELD_BITS + cmap_idx;
      // for each chunk field
      mi_bchunk_t* const chunk = &bitmap->chunks[chunk_idx];
      for (size_t j = 0; j < MI_BCHUNK_FIELDS; j++) {
        const size_t base_idx = (chunk_idx*MI_BCHUNK_BITS) + (j*MI_BFIELD_BITS);
        mi_bfield_t b = mi_atomic_load_relaxed(&chunk->bfields[j]);
        size_t bidx;
        while (mi_bfield_foreach_bit(&b, &bidx)) {
          const size_t idx = base_idx + bidx;
          if (!visit(idx, 1, arena, arg)) return false;
        }
      }
    }
  }
  return true;
}

// Visit all set bits in a bitmap but try to return ranges (within bfields) if possible.
// Also clear those ranges atomically.
// Used by purging to purge larger ranges when possible
// todo: optimize further? maybe use avx512 to directly get all indices using a mask_compressstore?
bool _mi_bitmap_forall_setc_ranges(mi_bitmap_t* bitmap, mi_forall_set_fun_t* visit, mi_arena_t* arena, void* arg) {
  // for all chunkmap entries
  const size_t chunkmap_max = _mi_divide_up(mi_bitmap_chunk_count(bitmap), MI_BFIELD_BITS);
  for (size_t i = 0; i < chunkmap_max; i++) {
    mi_bfield_t cmap_entry = mi_atomic_load_relaxed(&bitmap->chunkmap.bfields[i]);
    size_t cmap_idx;
    // for each chunk (corresponding to a set bit in a chunkmap entry)
    while (mi_bfield_foreach_bit(&cmap_entry, &cmap_idx)) {
      const size_t chunk_idx = i*MI_BFIELD_BITS + cmap_idx;
      // for each chunk field
      mi_bchunk_t* const chunk = &bitmap->chunks[chunk_idx];
      for (size_t j = 0; j < MI_BCHUNK_FIELDS; j++) {
        const size_t base_idx = (chunk_idx*MI_BCHUNK_BITS) + (j*MI_BFIELD_BITS);
        mi_bfield_t b = mi_atomic_exchange_acq_rel(&chunk->bfields[j], 0); // can be relaxed?
        #if MI_DEBUG > 1
        const size_t bpopcount = mi_popcount(b);
        size_t rngcount = 0;
        #endif
        size_t bidx;
        while (mi_bfield_find_least_bit(b, &bidx)) {
          const size_t rng = mi_ctz(~(b>>bidx)); // all the set bits from bidx
          #if MI_DEBUG > 1
          rngcount += rng;
          #endif
          mi_assert_internal(rng>=1 && rng<=MI_BFIELD_BITS);
          const size_t idx = base_idx + bidx;
          mi_assert_internal((idx % MI_BFIELD_BITS) + rng <= MI_BFIELD_BITS);
          mi_assert_internal((idx / MI_BCHUNK_BITS) < mi_bitmap_chunk_count(bitmap));
          if (!visit(idx, rng, arena, arg)) return false;
          // clear rng bits in b
          b = b & ~mi_bfield_mask(rng, bidx);
        }
        mi_assert_internal(rngcount == bpopcount);
      }
    }
  }
  return true;
}



/* --------------------------------------------------------------------------------
  binned bitmap's
-------------------------------------------------------------------------------- */


size_t mi_bbitmap_size(size_t bit_count, size_t* pchunk_count) {
  // mi_assert_internal((bit_count % MI_BCHUNK_BITS) == 0);
  bit_count = _mi_align_up(bit_count, MI_BCHUNK_BITS);
  mi_assert_internal(bit_count <= MI_BITMAP_MAX_BIT_COUNT);
  mi_assert_internal(bit_count > 0);
  const size_t chunk_count = bit_count / MI_BCHUNK_BITS;
  mi_assert_internal(chunk_count >= 1);
  const size_t size = offsetof(mi_bbitmap_t,chunks) + (chunk_count * MI_BCHUNK_SIZE);
  mi_assert_internal( (size%MI_BCHUNK_SIZE) == 0 );
  if (pchunk_count != NULL) { *pchunk_count = chunk_count;  }
  return size;
}

// initialize a bitmap to all unset; avoid a mem_zero if `already_zero` is true
// returns the size of the bitmap
size_t mi_bbitmap_init(mi_bbitmap_t* bbitmap, size_t bit_count, bool already_zero) {
  size_t chunk_count;
  const size_t size = mi_bbitmap_size(bit_count, &chunk_count);
  if (!already_zero) {
    _mi_memzero_aligned(bbitmap, size);
  }
  mi_atomic_store_release(&bbitmap->chunk_count, chunk_count);
  mi_assert_internal(mi_atomic_load_relaxed(&bbitmap->chunk_count) <= MI_BITMAP_MAX_CHUNK_COUNT);
  return size;
}

void mi_bbitmap_unsafe_setN(mi_bbitmap_t* bbitmap, size_t idx, size_t n) {
  mi_assert_internal(n>0);
  mi_assert_internal(idx + n <= mi_bbitmap_max_bits(bbitmap));
  mi_bchunks_unsafe_setN(&bbitmap->chunks[0], &bbitmap->chunkmap, idx, n);
}



/* --------------------------------------------------------------------------------
 binned bitmap used to track free slices
-------------------------------------------------------------------------------- */

// Assign a specific size bin to a chunk
static void mi_bbitmap_set_chunk_bin(mi_bbitmap_t* bbitmap, size_t chunk_idx, mi_chunkbin_t bin) {
  mi_assert_internal(chunk_idx < mi_bbitmap_chunk_count(bbitmap));
  for (mi_chunkbin_t ibin = MI_CBIN_SMALL; ibin < MI_CBIN_NONE; ibin = mi_chunkbin_inc(ibin)) {
    if (ibin == bin) {
      const bool was_clear = mi_bchunk_set(& bbitmap->chunkmap_bins[ibin], chunk_idx, NULL);
      if (was_clear) { mi_os_stat_increase(chunk_bins[ibin],1); }
    }
    else {
      const bool was_set = mi_bchunk_clear(&bbitmap->chunkmap_bins[ibin], chunk_idx, NULL);
      if (was_set) { mi_os_stat_decrease(chunk_bins[ibin],1); }
    }
  }  
}

mi_chunkbin_t mi_bbitmap_debug_get_bin(const mi_bchunkmap_t* chunkmap_bins, size_t chunk_idx) {
  for (mi_chunkbin_t ibin = MI_CBIN_SMALL; ibin < MI_CBIN_NONE; ibin = mi_chunkbin_inc(ibin)) {
    if (mi_bchunk_is_xsetN(MI_BIT_SET, &chunkmap_bins[ibin], chunk_idx, 1)) {
      return ibin;
    }
  }
  return MI_CBIN_NONE;
}

// Track the index of the highest chunk that is accessed.
static void mi_bbitmap_chunkmap_set_max(mi_bbitmap_t* bbitmap, size_t chunk_idx) {
  size_t oldmax = mi_atomic_load_relaxed(&bbitmap->chunk_max_accessed);
  if mi_unlikely(chunk_idx > oldmax) {
    mi_atomic_cas_strong_relaxed(&bbitmap->chunk_max_accessed, &oldmax, chunk_idx);
  }
}

// Set a bit in the chunkmap
static void mi_bbitmap_chunkmap_set(mi_bbitmap_t* bbitmap, size_t chunk_idx, bool check_all_set) {
  mi_assert(chunk_idx < mi_bbitmap_chunk_count(bbitmap));
  if (check_all_set) {
    if (mi_bchunk_all_are_set_relaxed(&bbitmap->chunks[chunk_idx])) {
      // all slices are free in this chunk: return back to the NONE bin
      mi_bbitmap_set_chunk_bin(bbitmap, chunk_idx, MI_CBIN_NONE);
    }
  }
  mi_bchunk_set(&bbitmap->chunkmap, chunk_idx, NULL);
  mi_bbitmap_chunkmap_set_max(bbitmap, chunk_idx);
}

static bool mi_bbitmap_chunkmap_try_clear(mi_bbitmap_t* bbitmap, size_t chunk_idx) {
  mi_assert(chunk_idx < mi_bbitmap_chunk_count(bbitmap));
  // check if the corresponding chunk is all clear
  if (!mi_bchunk_all_are_clear_relaxed(&bbitmap->chunks[chunk_idx])) return false;
  // clear the chunkmap bit
  mi_bchunk_clear(&bbitmap->chunkmap, chunk_idx, NULL);
  // .. but a concurrent set may have happened in between our all-clear test and the clearing of the
  // bit in the mask. We check again to catch this situation. (note: mi_bchunk_clear must be acq-rel)
  if (!mi_bchunk_all_are_clear_relaxed(&bbitmap->chunks[chunk_idx])) {
    mi_bchunk_set(&bbitmap->chunkmap, chunk_idx, NULL);
    return false;
  }
  mi_bbitmap_chunkmap_set_max(bbitmap, chunk_idx);
  return true;
}


/* --------------------------------------------------------------------------------
  mi_bbitmap_setN, try_clearN, and is_xsetN
  (used to find free pages)
-------------------------------------------------------------------------------- */

// Set a sequence of `n` bits in the bitmap; returns `true` if atomically transitioned from 0's to 1's (or 1's to 0's).
// `n` cannot cross chunk boundaries (and `n <= MI_BCHUNK_BITS`)!
bool mi_bbitmap_setN(mi_bbitmap_t* bbitmap, size_t idx, size_t n) {
  mi_assert_internal(n>0);
  mi_assert_internal(n<=MI_BCHUNK_BITS);

  const size_t chunk_idx = idx / MI_BCHUNK_BITS;
  const size_t cidx = idx % MI_BCHUNK_BITS;
  mi_assert_internal(cidx + n <= MI_BCHUNK_BITS);  // don't cross chunks (for now)
  mi_assert_internal(chunk_idx < mi_bbitmap_chunk_count(bbitmap));
  if (cidx + n > MI_BCHUNK_BITS) { n = MI_BCHUNK_BITS - cidx; }  // paranoia

  const bool were_allclear = mi_bchunk_setN(&bbitmap->chunks[chunk_idx], cidx, n, NULL);
  mi_bbitmap_chunkmap_set(bbitmap, chunk_idx, true);   // set after
  return were_allclear;
}


// ------- mi_bbitmap_try_clearN ---------------------------------------

bool mi_bbitmap_try_clearN(mi_bbitmap_t* bbitmap, size_t idx, size_t n) {
  mi_assert_internal(n>0);
  mi_assert_internal(n<=MI_BCHUNK_BITS);
  mi_assert_internal(idx + n <= mi_bbitmap_max_bits(bbitmap));

  const size_t chunk_idx = idx / MI_BCHUNK_BITS;
  const size_t cidx = idx % MI_BCHUNK_BITS;
  mi_assert_internal(cidx + n <= MI_BCHUNK_BITS);  // don't cross chunks (for now)
  mi_assert_internal(chunk_idx < mi_bbitmap_chunk_count(bbitmap));
  if (cidx + n > MI_BCHUNK_BITS) return false;
  bool maybe_all_clear;
  const bool cleared = mi_bchunk_try_clearN(&bbitmap->chunks[chunk_idx], cidx, n, &maybe_all_clear);
  if (cleared && maybe_all_clear) { mi_bbitmap_chunkmap_try_clear(bbitmap, chunk_idx); }
  // note: we don't set the size class for an explicit try_clearN (only used by purging)
  return cleared;
}


// ------- mi_bbitmap_is_xset ---------------------------------------

// Is a sequence of n bits already all set/cleared?
bool mi_bbitmap_is_xsetN(mi_xset_t set, mi_bbitmap_t* bbitmap, size_t idx, size_t n) {
  mi_assert_internal(n>0);
  mi_assert_internal(n<=MI_BCHUNK_BITS);
  mi_assert_internal(idx + n <= mi_bbitmap_max_bits(bbitmap));

  const size_t chunk_idx = idx / MI_BCHUNK_BITS;
  const size_t cidx = idx % MI_BCHUNK_BITS;
  mi_assert_internal(cidx + n <= MI_BCHUNK_BITS);  // don't cross chunks (for now)
  mi_assert_internal(chunk_idx < mi_bbitmap_chunk_count(bbitmap));
  if (cidx + n > MI_BCHUNK_BITS) { n = MI_BCHUNK_BITS - cidx; }  // paranoia

  return mi_bchunk_is_xsetN(set, &bbitmap->chunks[chunk_idx], cidx, n);
}




/* --------------------------------------------------------------------------------
  mi_bbitmap_find
  (used to find free pages)
-------------------------------------------------------------------------------- */

typedef bool (mi_bchunk_try_find_and_clear_fun_t)(mi_bchunk_t* chunk, size_t n, size_t* idx);

// Go through the bbitmap and for every sequence of `n` set bits, call the visitor function.
// If it returns `true` stop the search.
//
// This is used for finding free blocks and it is important to be efficient (with 2-level bitscan)
// but also reduce fragmentation (through size bins).
static inline bool mi_bbitmap_try_find_and_clear_generic(mi_bbitmap_t* bbitmap, size_t tseq, size_t n, size_t* pidx, mi_bchunk_try_find_and_clear_fun_t* on_find)
{
  // we space out threads to reduce contention
  const size_t cmap_max_count  = _mi_divide_up(mi_bbitmap_chunk_count(bbitmap),MI_BFIELD_BITS);
  const size_t chunk_acc       = mi_atomic_load_relaxed(&bbitmap->chunk_max_accessed);
  const size_t cmap_acc        = chunk_acc / MI_BFIELD_BITS;
  const size_t cmap_acc_bits   = 1 + (chunk_acc % MI_BFIELD_BITS);

  // create a mask over the chunkmap entries to iterate over them efficiently
  mi_assert_internal(MI_BFIELD_BITS >= MI_BCHUNK_FIELDS);
  const mi_bfield_t cmap_mask  = mi_bfield_mask(cmap_max_count,0);
  const size_t cmap_cycle      = cmap_acc+1;
  const mi_chunkbin_t bbin = mi_chunkbin_of(n); 
  // visit each cmap entry
  size_t cmap_idx = 0;
  mi_bfield_cycle_iterate(cmap_mask, tseq, cmap_cycle, cmap_idx, X)
  {
    // and for each chunkmap entry we iterate over its bits to find the chunks
    const mi_bfield_t cmap_entry = mi_atomic_load_relaxed(&bbitmap->chunkmap.bfields[cmap_idx]);
    const size_t cmap_entry_cycle = (cmap_idx != cmap_acc ? MI_BFIELD_BITS : cmap_acc_bits);
    if (cmap_entry == 0) continue;

    // get size bin masks
    mi_bfield_t cmap_bins[MI_CBIN_COUNT] = { 0 };
    cmap_bins[MI_CBIN_NONE] = cmap_entry;
    for (mi_chunkbin_t ibin = MI_CBIN_SMALL; ibin < MI_CBIN_NONE; ibin = mi_chunkbin_inc(ibin)) {
      const mi_bfield_t cmap_bin = mi_atomic_load_relaxed(&bbitmap->chunkmap_bins[ibin].bfields[cmap_idx]);
      cmap_bins[ibin] = cmap_bin & cmap_entry;
      cmap_bins[MI_CBIN_NONE] &= ~cmap_bin;      // clear bits that are in an assigned size bin
    }

    // consider only chunks for a particular size bin at a time    
    // this picks the best bin only within a cmap entry (~ 1GiB address space), but avoids multiple
    // iterations through all entries.
    mi_assert_internal(bbin < MI_CBIN_NONE);
    for (mi_chunkbin_t ibin = MI_CBIN_SMALL; ibin <= MI_CBIN_NONE;
          // skip from bbin to NONE (so, say, a SMALL will never be placed in a OTHER, MEDIUM, or LARGE chunk to reduce fragmentation)
          ibin = (ibin == bbin ? MI_CBIN_NONE : mi_chunkbin_inc(ibin)))
    {
      mi_assert_internal(ibin < MI_CBIN_COUNT);
      const mi_bfield_t cmap_bin = cmap_bins[ibin];      
      size_t eidx = 0;
      mi_bfield_cycle_iterate(cmap_bin, tseq, cmap_entry_cycle, eidx, Y)  
      {
        // assertion doesn't quite hold as the max_accessed may be out-of-date
        // mi_assert_internal(cmap_entry_cycle > eidx || ibin == MI_CBIN_NONE);

        // get the chunk 
        const size_t chunk_idx = cmap_idx*MI_BFIELD_BITS + eidx;
        mi_bchunk_t* chunk = &bbitmap->chunks[chunk_idx];

        size_t cidx;
        if ((*on_find)(chunk, n, &cidx)) {
          if (cidx==0 && ibin == MI_CBIN_NONE) { // only the first block determines the size bin
            // this chunk is now reserved for the `bbin` size class
            mi_bbitmap_set_chunk_bin(bbitmap, chunk_idx, bbin);
          }
          *pidx = (chunk_idx * MI_BCHUNK_BITS) + cidx;
          mi_assert_internal(*pidx + n <= mi_bbitmap_max_bits(bbitmap));
          return true;
        }
        else {
          // todo: should _on_find_ return a boolen if there is a chance all are clear to avoid calling `try_clear?`
          // we may find that all are cleared only on a second iteration but that is ok as the chunkmap is a conservative approximation.
          mi_bbitmap_chunkmap_try_clear(bbitmap, chunk_idx);
        }
      }
      mi_bfield_cycle_iterate_end(Y);
    }
  }
  mi_bfield_cycle_iterate_end(X);
  return false;
}


/* --------------------------------------------------------------------------------
  mi_bbitmap_try_find_and_clear -- used to find free pages
  note: the compiler will fully inline the indirect function calls
-------------------------------------------------------------------------------- */

bool mi_bbitmap_try_find_and_clear(mi_bbitmap_t* bbitmap, size_t tseq, size_t* pidx) {
  return mi_bbitmap_try_find_and_clear_generic(bbitmap, tseq, 1, pidx, &mi_bchunk_try_find_and_clear_1);
}

bool mi_bbitmap_try_find_and_clear8(mi_bbitmap_t* bbitmap, size_t tseq, size_t* pidx) {
  return mi_bbitmap_try_find_and_clear_generic(bbitmap, tseq, 8, pidx, &mi_bchunk_try_find_and_clear_8);
}

// bool mi_bbitmap_try_find_and_clearX(mi_bbitmap_t* bbitmap, size_t tseq, size_t* pidx) {
//   return mi_bbitmap_try_find_and_clear_generic(bbitmap, tseq, MI_BFIELD_BITS, pidx, &mi_bchunk_try_find_and_clear_X);
// }

bool mi_bbitmap_try_find_and_clearNX(mi_bbitmap_t* bbitmap, size_t tseq, size_t n, size_t* pidx) {
  mi_assert_internal(n<=MI_BFIELD_BITS);
  return mi_bbitmap_try_find_and_clear_generic(bbitmap, tseq, n, pidx, &mi_bchunk_try_find_and_clearNX);
}

bool mi_bbitmap_try_find_and_clearN_(mi_bbitmap_t* bbitmap, size_t tseq, size_t n, size_t* pidx) {
  mi_assert_internal(n<=MI_BCHUNK_BITS);
  return mi_bbitmap_try_find_and_clear_generic(bbitmap, tseq, n, pidx, &mi_bchunk_try_find_and_clearN_);
}
