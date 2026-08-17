/* ----------------------------------------------------------------------------
Copyright (c) 2018-2023, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
#pragma once
#ifndef MI_INTERNAL_H
#define MI_INTERNAL_H

// --------------------------------------------------------------------------
// This file contains the internal API's of mimalloc and various utility
// functions and macros.
// --------------------------------------------------------------------------

#include "types.h"
#include "track.h"
#include "bits.h"


// --------------------------------------------------------------------------
// Compiler defines
// --------------------------------------------------------------------------

#if (MI_DEBUG>0)
#define mi_trace_message(...)  _mi_trace_message(__VA_ARGS__)
#else
#define mi_trace_message(...)
#endif

#define mi_decl_cache_align     mi_decl_align(64)

#if defined(_MSC_VER)
#pragma warning(disable:4127)   // suppress constant conditional warning (due to MI_SECURE paths)
#pragma warning(disable:26812)  // unscoped enum warning
#define mi_decl_noinline        __declspec(noinline)
#define mi_decl_thread          __declspec(thread)
#define mi_decl_align(a)        __declspec(align(a))
#define mi_decl_noreturn        __declspec(noreturn)
#define mi_decl_weak
#define mi_decl_hidden
#define mi_decl_cold
#elif (defined(__GNUC__) && (__GNUC__ >= 3)) || defined(__clang__) // includes clang and icc
#define mi_decl_noinline        __attribute__((noinline))
#define mi_decl_thread          __thread
#define mi_decl_align(a)        __attribute__((aligned(a)))
#define mi_decl_noreturn        __attribute__((noreturn))
#define mi_decl_weak            __attribute__((weak))
#define mi_decl_hidden          __attribute__((visibility("hidden")))
#if (__GNUC__ >= 4) || defined(__clang__)
#define mi_decl_cold            __attribute__((cold))
#else
#define mi_decl_cold
#endif
#elif __cplusplus >= 201103L    // c++11
#define mi_decl_noinline
#define mi_decl_thread          thread_local
#define mi_decl_align(a)        alignas(a)
#define mi_decl_noreturn        [[noreturn]]
#define mi_decl_weak
#define mi_decl_hidden
#define mi_decl_cold
#else
#define mi_decl_noinline
#define mi_decl_thread          __thread        // hope for the best :-)
#define mi_decl_align(a)
#define mi_decl_noreturn
#define mi_decl_weak
#define mi_decl_hidden
#define mi_decl_cold
#endif

#if defined(__GNUC__) || defined(__clang__)
#define mi_unlikely(x)     (__builtin_expect(!!(x),false))
#define mi_likely(x)       (__builtin_expect(!!(x),true))
#elif (defined(__cplusplus) && (__cplusplus >= 202002L)) || (defined(_MSVC_LANG) && _MSVC_LANG >= 202002L)
#define mi_unlikely(x)     (x) [[unlikely]]
#define mi_likely(x)       (x) [[likely]]
#else
#define mi_unlikely(x)     (x)
#define mi_likely(x)       (x)
#endif

#ifndef __has_builtin
#define __has_builtin(x)    0
#endif

#if defined(__cplusplus)
#define mi_decl_externc     extern "C"
#else
#define mi_decl_externc
#endif

#if (defined(__GNUC__) && (__GNUC__ >= 7)) || defined(__clang__) // includes clang and icc
#define mi_decl_maybe_unused    __attribute__((unused))
#elif __cplusplus >= 201703L    // c++17
#define mi_decl_maybe_unused    [[maybe_unused]]
#else
#define mi_decl_maybe_unused
#endif

#if defined(__cplusplus)
#define mi_decl_externc         extern "C"
#else
#define mi_decl_externc
#endif


#if defined(__EMSCRIPTEN__) && !defined(__wasi__)
#define __wasi__
#endif


// --------------------------------------------------------------------------
// Internal functions
// --------------------------------------------------------------------------


// "libc.c"
#include <stdarg.h>
int           _mi_vsnprintf(char* buf, size_t bufsize, const char* fmt, va_list args);
int           _mi_snprintf(char* buf, size_t buflen, const char* fmt, ...);
char          _mi_toupper(char c);
int           _mi_strnicmp(const char* s, const char* t, size_t n);
void          _mi_strlcpy(char* dest, const char* src, size_t dest_size);
void          _mi_strlcat(char* dest, const char* src, size_t dest_size);
size_t        _mi_strlen(const char* s);
size_t        _mi_strnlen(const char* s, size_t max_len);
bool          _mi_getenv(const char* name, char* result, size_t result_size);

// "options.c"
void          _mi_fputs(mi_output_fun* out, void* arg, const char* prefix, const char* message);
void          _mi_fprintf(mi_output_fun* out, void* arg, const char* fmt, ...);
void          _mi_raw_message(const char* fmt, ...);
void          _mi_message(const char* fmt, ...);
void          _mi_warning_message(const char* fmt, ...);
void          _mi_verbose_message(const char* fmt, ...);
void          _mi_trace_message(const char* fmt, ...);
void          _mi_options_init(void);
long          _mi_option_get_fast(mi_option_t option);
void          _mi_error_message(int err, const char* fmt, ...);

// random.c
void          _mi_random_init(mi_random_ctx_t* ctx);
void          _mi_random_init_weak(mi_random_ctx_t* ctx);
void          _mi_random_reinit_if_weak(mi_random_ctx_t * ctx);
void          _mi_random_split(mi_random_ctx_t* ctx, mi_random_ctx_t* new_ctx);
uintptr_t     _mi_random_next(mi_random_ctx_t* ctx);
uintptr_t     _mi_heap_random_next(mi_heap_t* heap);
uintptr_t     _mi_os_random_weak(uintptr_t extra_seed);
static inline uintptr_t _mi_random_shuffle(uintptr_t x);

// init.c
extern mi_decl_hidden mi_decl_cache_align const mi_page_t  _mi_page_empty;
void          _mi_auto_process_init(void);
void mi_cdecl _mi_auto_process_done(void) mi_attr_noexcept;
bool          _mi_is_redirected(void);
bool          _mi_allocator_init(const char** message);
void          _mi_allocator_done(void);
bool          _mi_is_main_thread(void);
size_t        _mi_current_thread_count(void);
bool          _mi_preloading(void);           // true while the C runtime is not initialized yet
void          _mi_thread_done(mi_heap_t* heap);

mi_subproc_t* _mi_subproc(void);
mi_subproc_t* _mi_subproc_main(void);
mi_subproc_t* _mi_subproc_from_id(mi_subproc_id_t subproc_id);
mi_threadid_t _mi_thread_id(void) mi_attr_noexcept;
size_t        _mi_thread_seq_id(void) mi_attr_noexcept;
mi_tld_t*     _mi_thread_tld(void) mi_attr_noexcept;
void          _mi_heap_guarded_init(mi_heap_t* heap);
mi_heap_t*    _mi_heap_main_get(void);

// os.c
void          _mi_os_init(void);                                            // called from process init
void*         _mi_os_alloc(size_t size, mi_memid_t* memid);
void*         _mi_os_zalloc(size_t size, mi_memid_t* memid);
void          _mi_os_free(void* p, size_t size, mi_memid_t memid);
void          _mi_os_free_ex(void* p, size_t size, bool still_committed, mi_memid_t memid, mi_subproc_t* subproc );

size_t        _mi_os_page_size(void);
size_t        _mi_os_guard_page_size(void);
size_t        _mi_os_good_alloc_size(size_t size);
bool          _mi_os_has_overcommit(void);
bool          _mi_os_has_virtual_reserve(void);
size_t        _mi_os_virtual_address_bits(void);

bool          _mi_os_reset(void* addr, size_t size);
bool          _mi_os_decommit(void* addr, size_t size);
void          _mi_os_reuse(void* p, size_t size);
mi_decl_nodiscard bool _mi_os_commit(void* p, size_t size, bool* is_zero);
mi_decl_nodiscard bool _mi_os_commit_ex(void* addr, size_t size, bool* is_zero, size_t stat_size);
mi_decl_nodiscard bool _mi_os_protect(void* addr, size_t size);
bool          _mi_os_unprotect(void* addr, size_t size);
bool          _mi_os_purge(void* p, size_t size);
bool          _mi_os_purge_ex(void* p, size_t size, bool allow_reset, size_t stats_size, mi_commit_fun_t* commit_fun, void* commit_fun_arg);

size_t        _mi_os_secure_guard_page_size(void);
bool          _mi_os_secure_guard_page_set_at(void* addr, mi_memid_t memid);
bool          _mi_os_secure_guard_page_set_before(void* addr, mi_memid_t memid);
bool          _mi_os_secure_guard_page_reset_at(void* addr, mi_memid_t memid);
bool          _mi_os_secure_guard_page_reset_before(void* addr, mi_memid_t memid);

int           _mi_os_numa_node(void);
int           _mi_os_numa_node_count(void);

void*         _mi_os_alloc_aligned(size_t size, size_t alignment, bool commit, bool allow_large, mi_memid_t* memid);
void*         _mi_os_alloc_aligned_at_offset(size_t size, size_t alignment, size_t align_offset, bool commit, bool allow_large, mi_memid_t* memid);

void*         _mi_os_get_aligned_hint(size_t try_alignment, size_t size);
bool          _mi_os_use_large_page(size_t size, size_t alignment);
size_t        _mi_os_large_page_size(void);
void*         _mi_os_alloc_huge_os_pages(size_t pages, int numa_node, mi_msecs_t max_secs, size_t* pages_reserved, size_t* psize, mi_memid_t* memid);


// arena.c
mi_arena_id_t _mi_arena_id_none(void);
mi_arena_t*   _mi_arena_from_id(mi_arena_id_t id);
bool          _mi_arena_memid_is_suitable(mi_memid_t memid, mi_arena_t* request_arena);

void*         _mi_arenas_alloc(mi_subproc_t* subproc, size_t size, bool commit, bool allow_pinned, mi_arena_t* req_arena, size_t tseq, int numa_node, mi_memid_t* memid);
void*         _mi_arenas_alloc_aligned(mi_subproc_t* subproc, size_t size, size_t alignment, size_t align_offset, bool commit, bool allow_pinned, mi_arena_t* req_arena, size_t tseq, int numa_node, mi_memid_t* memid);
void          _mi_arenas_free(void* p, size_t size, mi_memid_t memid);
bool          _mi_arenas_contain(const void* p);
void          _mi_arenas_collect(bool force_purge, bool visit_all, mi_tld_t* tld);
void          _mi_arenas_unsafe_destroy_all(mi_tld_t* tld);

mi_page_t*    _mi_arenas_page_alloc(mi_heap_t* heap, size_t block_size, size_t page_alignment);
void          _mi_arenas_page_free(mi_page_t* page, mi_tld_t* tld);
void          _mi_arenas_page_abandon(mi_page_t* page, mi_tld_t* tld);
void          _mi_arenas_page_unabandon(mi_page_t* page);
bool          _mi_arenas_page_try_reabandon_to_mapped(mi_page_t* page);

// arena-meta.c
void*         _mi_meta_zalloc( size_t size, mi_memid_t* memid );
void          _mi_meta_free(void* p, size_t size, mi_memid_t memid);
bool          _mi_meta_is_meta_page(void* p);

// "page-map.c"
bool          _mi_page_map_init(void);
void          _mi_page_map_register(mi_page_t* page);
void          _mi_page_map_unregister(mi_page_t* page);
void          _mi_page_map_unregister_range(void* start, size_t size);
mi_page_t*    _mi_safe_ptr_page(const void* p);
void          _mi_page_map_unsafe_destroy(mi_subproc_t* subproc);

// "page.c"
void*         _mi_malloc_generic(mi_heap_t* heap, size_t size, bool zero, size_t huge_alignment)  mi_attr_noexcept mi_attr_malloc;

void          _mi_page_retire(mi_page_t* page) mi_attr_noexcept;       // free the page if there are no other pages with many free blocks
void          _mi_page_unfull(mi_page_t* page);
void          _mi_page_free(mi_page_t* page, mi_page_queue_t* pq);     // free the page
void          _mi_page_abandon(mi_page_t* page, mi_page_queue_t* pq);  // abandon the page, to be picked up by another thread...
void          _mi_heap_collect_retired(mi_heap_t* heap, bool force);

size_t        _mi_page_queue_append(mi_heap_t* heap, mi_page_queue_t* pq, mi_page_queue_t* append);
void          _mi_deferred_free(mi_heap_t* heap, bool force);

void          _mi_page_free_collect(mi_page_t* page, bool force);
void          _mi_page_free_collect_partly(mi_page_t* page, mi_block_t* head);
mi_decl_nodiscard bool _mi_page_init(mi_heap_t* heap, mi_page_t* page);
bool          _mi_page_queue_is_valid(mi_heap_t* heap, const mi_page_queue_t* pq);

size_t        _mi_page_bin(const mi_page_t* page); // for stats
size_t        _mi_bin_size(size_t bin);            // for stats
size_t        _mi_bin(size_t size);                // for stats

// "heap.c"
mi_heap_t*    _mi_heap_create(int heap_tag, bool allow_destroy, mi_arena_id_t arena_id, mi_tld_t* tld);
void          _mi_heap_init(mi_heap_t* heap, mi_arena_id_t arena_id, bool noreclaim, uint8_t tag, mi_tld_t* tld);
void          _mi_heap_destroy_pages(mi_heap_t* heap);
void          _mi_heap_collect_abandon(mi_heap_t* heap);
void          _mi_heap_set_default_direct(mi_heap_t* heap);
bool          _mi_heap_memid_is_suitable(mi_heap_t* heap, mi_memid_t memid);
void          _mi_heap_unsafe_destroy_all(mi_heap_t* heap);
mi_heap_t*    _mi_heap_by_tag(mi_heap_t* heap, uint8_t tag);
void          _mi_heap_area_init(mi_heap_area_t* area, mi_page_t* page);
bool          _mi_heap_area_visit_blocks(const mi_heap_area_t* area, mi_page_t* page, mi_block_visit_fun* visitor, void* arg);
void          _mi_heap_page_reclaim(mi_heap_t* heap, mi_page_t* page);

// "stats.c"
void          _mi_stats_init(void);
void          _mi_stats_done(mi_stats_t* stats);
void          _mi_stats_print(mi_stats_t* stats, mi_output_fun* out, void* arg) mi_attr_noexcept;
void          _mi_stats_merge_thread(mi_tld_t* tld);
void          _mi_stats_merge_from(mi_stats_t* to, mi_stats_t* from);
mi_msecs_t    _mi_clock_now(void);
mi_msecs_t    _mi_clock_end(mi_msecs_t start);
mi_msecs_t    _mi_clock_start(void);

// "alloc.c"
void*         _mi_page_malloc_zero(mi_heap_t* heap, mi_page_t* page, size_t size, bool zero) mi_attr_noexcept;  // called from `_mi_malloc_generic`
void*         _mi_page_malloc(mi_heap_t* heap, mi_page_t* page, size_t size) mi_attr_noexcept;                  // called from `_mi_heap_malloc_aligned`
void*         _mi_page_malloc_zeroed(mi_heap_t* heap, mi_page_t* page, size_t size) mi_attr_noexcept;           // called from `_mi_heap_malloc_aligned`
void*         _mi_heap_malloc_zero(mi_heap_t* heap, size_t size, bool zero) mi_attr_noexcept;
void*         _mi_heap_malloc_zero_ex(mi_heap_t* heap, size_t size, bool zero, size_t huge_alignment) mi_attr_noexcept;     // called from `_mi_heap_malloc_aligned`
void*         _mi_heap_realloc_zero(mi_heap_t* heap, void* p, size_t newsize, bool zero) mi_attr_noexcept;
mi_block_t*   _mi_page_ptr_unalign(const mi_page_t* page, const void* p);
void          _mi_padding_shrink(const mi_page_t* page, const mi_block_t* block, const size_t min_size);

#if MI_DEBUG>1
bool          _mi_page_is_valid(mi_page_t* page);
#endif


// ------------------------------------------------------
// Assertions
// ------------------------------------------------------

#if (MI_DEBUG)
// use our own assertion to print without memory allocation
mi_decl_noreturn mi_decl_cold void _mi_assert_fail(const char* assertion, const char* fname, unsigned int line, const char* func) mi_attr_noexcept;
#define mi_assert(expr)     ((expr) ? (void)0 : _mi_assert_fail(#expr,__FILE__,__LINE__,__func__))
#else
#define mi_assert(x)
#endif

#if (MI_DEBUG>1)
#define mi_assert_internal    mi_assert
#else
#define mi_assert_internal(x)
#endif

#if (MI_DEBUG>2)
#define mi_assert_expensive   mi_assert
#else
#define mi_assert_expensive(x)
#endif


/* -----------------------------------------------------------
  Statistics (in `stats.c`)
----------------------------------------------------------- */

// add to stat keeping track of the peak
void __mi_stat_increase(mi_stat_count_t* stat, size_t amount);
void __mi_stat_decrease(mi_stat_count_t* stat, size_t amount);
void __mi_stat_increase_mt(mi_stat_count_t* stat, size_t amount);
void __mi_stat_decrease_mt(mi_stat_count_t* stat, size_t amount);

// adjust stat in special cases to compensate for double counting (and does not adjust peak values and can decrease the total)
void __mi_stat_adjust_increase(mi_stat_count_t* stat, size_t amount);
void __mi_stat_adjust_decrease(mi_stat_count_t* stat, size_t amount);
void __mi_stat_adjust_increase_mt(mi_stat_count_t* stat, size_t amount);
void __mi_stat_adjust_decrease_mt(mi_stat_count_t* stat, size_t amount);

// counters can just be increased
void __mi_stat_counter_increase(mi_stat_counter_t* stat, size_t amount);
void __mi_stat_counter_increase_mt(mi_stat_counter_t* stat, size_t amount);

#define mi_subproc_stat_counter_increase(subproc,stat,amount)   __mi_stat_counter_increase_mt( &(subproc)->stats.stat, amount)
#define mi_subproc_stat_increase(subproc,stat,amount)           __mi_stat_increase_mt( &(subproc)->stats.stat, amount)
#define mi_subproc_stat_decrease(subproc,stat,amount)           __mi_stat_decrease_mt( &(subproc)->stats.stat, amount)
#define mi_subproc_stat_adjust_increase(subproc,stat,amnt)      __mi_stat_adjust_increase_mt( &(subproc)->stats.stat, amnt)
#define mi_subproc_stat_adjust_decrease(subproc,stat,amnt)      __mi_stat_adjust_decrease_mt( &(subproc)->stats.stat, amnt)

#define mi_tld_stat_counter_increase(tld,stat,amount)           __mi_stat_counter_increase( &(tld)->stats.stat, amount)
#define mi_tld_stat_increase(tld,stat,amount)                   __mi_stat_increase( &(tld)->stats.stat, amount)
#define mi_tld_stat_decrease(tld,stat,amount)                   __mi_stat_decrease( &(tld)->stats.stat, amount)
#define mi_tld_stat_adjust_increase(tld,stat,amnt)              __mi_stat_adjust_increase( &(tld)->stats.stat, amnt)
#define mi_tld_stat_adjust_decrease(tld,stat,amnt)              __mi_stat_adjust_decrease( &(tld)->stats.stat, amnt)

#define mi_os_stat_counter_increase(stat,amount)                mi_subproc_stat_counter_increase(_mi_subproc(),stat,amount)
#define mi_os_stat_increase(stat,amount)                        mi_subproc_stat_increase(_mi_subproc(),stat,amount)
#define mi_os_stat_decrease(stat,amount)                        mi_subproc_stat_decrease(_mi_subproc(),stat,amount)

#define mi_heap_stat_counter_increase(heap,stat,amount)         mi_tld_stat_counter_increase(heap->tld, stat, amount)
#define mi_heap_stat_increase(heap,stat,amount)                 mi_tld_stat_increase( heap->tld, stat, amount)
#define mi_heap_stat_decrease(heap,stat,amount)                 mi_tld_stat_decrease( heap->tld, stat, amount)
#define mi_heap_stat_adjust_decrease(heap,stat,amount)          mi_tld_stat_adjust_decrease( heap->tld, stat, amount)

/* -----------------------------------------------------------
  Options (exposed for the debugger)
----------------------------------------------------------- */
typedef enum mi_option_init_e {
  MI_OPTION_UNINIT,       // not yet initialized
  MI_OPTION_DEFAULTED,    // not found in the environment, use default value
  MI_OPTION_INITIALIZED   // found in environment or set explicitly
} mi_option_init_t;

typedef struct mi_option_desc_s {
  long              value;  // the value
  mi_option_init_t  init;   // is it initialized yet? (from the environment)
  mi_option_t       option; // for debugging: the option index should match the option
  const char*       name;   // option name without `mimalloc_` prefix
  const char*       legacy_name; // potential legacy option name
} mi_option_desc_t;



/* -----------------------------------------------------------
  Inlined definitions
----------------------------------------------------------- */
#define MI_UNUSED(x)     (void)(x)
#if (MI_DEBUG>0)
#define MI_UNUSED_RELEASE(x)
#else
#define MI_UNUSED_RELEASE(x)  MI_UNUSED(x)
#endif

#define MI_INIT4(x)   x(),x(),x(),x()
#define MI_INIT8(x)   MI_INIT4(x),MI_INIT4(x)
#define MI_INIT16(x)  MI_INIT8(x),MI_INIT8(x)
#define MI_INIT32(x)  MI_INIT16(x),MI_INIT16(x)
#define MI_INIT64(x)  MI_INIT32(x),MI_INIT32(x)
#define MI_INIT128(x) MI_INIT64(x),MI_INIT64(x)
#define MI_INIT256(x) MI_INIT128(x),MI_INIT128(x)

#define MI_INIT74(x)  MI_INIT64(x),MI_INIT8(x),x(),x()
#define MI_INIT5(x)   MI_INIT4(x),x()

#include <string.h>
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
#define _mi_memzero_var(x)  memset(&x,0,sizeof(x))

// Is `x` a power of two? (0 is considered a power of two)
static inline bool _mi_is_power_of_two(uintptr_t x) {
  return ((x & (x - 1)) == 0);
}

// Is a pointer aligned?
static inline bool _mi_is_aligned(void* p, size_t alignment) {
  mi_assert_internal(alignment != 0);
  return (((uintptr_t)p % alignment) == 0);
}

// Align upwards
static inline uintptr_t _mi_align_up(uintptr_t sz, size_t alignment) {
  mi_assert_internal(alignment != 0);
  uintptr_t mask = alignment - 1;
  if ((alignment & mask) == 0) {  // power of two?
    return ((sz + mask) & ~mask);
  }
  else {
    return (((sz + mask)/alignment)*alignment);
  }
}


// Align a pointer upwards
static inline uint8_t* _mi_align_up_ptr(void* p, size_t alignment) {
  return (uint8_t*)_mi_align_up((uintptr_t)p, alignment);
}


static inline uintptr_t _mi_align_down(uintptr_t sz, size_t alignment) {
  mi_assert_internal(alignment != 0);
  uintptr_t mask = alignment - 1;
  if ((alignment & mask) == 0) { // power of two?
    return (sz & ~mask);
  }
  else {
    return ((sz / alignment) * alignment);
  }
}

static inline void* mi_align_down_ptr(void* p, size_t alignment) {
  return (void*)_mi_align_down((uintptr_t)p, alignment);
}

// Divide upwards: `s <= _mi_divide_up(s,d)*d < s+d`.
static inline uintptr_t _mi_divide_up(uintptr_t size, size_t divider) {
  mi_assert_internal(divider != 0);
  return (divider == 0 ? size : ((size + divider - 1) / divider));
}


// clamp an integer
static inline size_t _mi_clamp(size_t sz, size_t min, size_t max) {
  if (sz < min) return min;
  else if (sz > max) return max;
  else return sz;
}

// Is memory zero initialized?
static inline bool mi_mem_is_zero(const void* p, size_t size) {
  for (size_t i = 0; i < size; i++) {
    if (((uint8_t*)p)[i] != 0) return false;
  }
  return true;
}

// Align a byte size to a size in _machine words_,
// i.e. byte size == `wsize*sizeof(void*)`.
static inline size_t _mi_wsize_from_size(size_t size) {
  mi_assert_internal(size <= SIZE_MAX - sizeof(uintptr_t));
  return (size + sizeof(uintptr_t) - 1) / sizeof(uintptr_t);
}

// Overflow detecting multiply
#if __has_builtin(__builtin_umul_overflow) || (defined(__GNUC__) && (__GNUC__ >= 5))
#include <limits.h>      // UINT_MAX, ULONG_MAX
#if defined(_CLOCK_T)    // for Illumos
#undef _CLOCK_T
#endif
static inline bool mi_mul_overflow(size_t count, size_t size, size_t* total) {
  #if (SIZE_MAX == ULONG_MAX)
    return __builtin_umull_overflow(count, size, (unsigned long *)total);
  #elif (SIZE_MAX == UINT_MAX)
    return __builtin_umul_overflow(count, size, (unsigned int *)total);
  #else
    return __builtin_umulll_overflow(count, size, (unsigned long long *)total);
  #endif
}
#else /* __builtin_umul_overflow is unavailable */
static inline bool mi_mul_overflow(size_t count, size_t size, size_t* total) {
  #define MI_MUL_COULD_OVERFLOW ((size_t)1 << (4*sizeof(size_t)))  // sqrt(SIZE_MAX)
  *total = count * size;
  // note: gcc/clang optimize this to directly check the overflow flag
  return ((size >= MI_MUL_COULD_OVERFLOW || count >= MI_MUL_COULD_OVERFLOW) && size > 0 && (SIZE_MAX / size) < count);
}
#endif

// Safe multiply `count*size` into `total`; return `true` on overflow.
static inline bool mi_count_size_overflow(size_t count, size_t size, size_t* total) {
  if (count==1) {  // quick check for the case where count is one (common for C++ allocators)
    *total = size;
    return false;
  }
  else if mi_unlikely(mi_mul_overflow(count, size, total)) {
    #if MI_DEBUG > 0
    _mi_error_message(EOVERFLOW, "allocation request is too large (%zu * %zu bytes)\n", count, size);
    #endif
    *total = SIZE_MAX;
    return true;
  }
  else return false;
}


/*----------------------------------------------------------------------------------------
  Heap functions
------------------------------------------------------------------------------------------- */

extern mi_decl_hidden const mi_heap_t _mi_heap_empty;  // read-only empty heap, initial value of the thread local default heap

static inline bool mi_heap_is_backing(const mi_heap_t* heap) {
  return (heap->tld->heap_backing == heap);
}

static inline bool mi_heap_is_initialized(const mi_heap_t* heap) {
  mi_assert_internal(heap != NULL);
  return (heap != NULL && heap != &_mi_heap_empty);
}

static inline mi_page_t* _mi_heap_get_free_small_page(mi_heap_t* heap, size_t size) {
  mi_assert_internal(size <= (MI_SMALL_SIZE_MAX + MI_PADDING_SIZE));
  const size_t idx = _mi_wsize_from_size(size);
  mi_assert_internal(idx < MI_PAGES_DIRECT);
  return heap->pages_free_direct[idx];
}


//static inline uintptr_t _mi_ptr_cookie(const void* p) {
//  extern mi_heap_t _mi_heap_main;
//  mi_assert_internal(_mi_heap_main.cookie != 0);
//  return ((uintptr_t)p ^ _mi_heap_main.cookie);
//}


/* -----------------------------------------------------------
  The page map maps addresses to `mi_page_t` pointers
----------------------------------------------------------- */

#if MI_PAGE_MAP_FLAT

// flat page-map committed on demand, using one byte per slice (64 KiB).
// single indirection and low commit, but large initial virtual reserve (4 GiB with 48 bit virtual addresses)
// used by default on <= 40 bit virtual address spaces.
extern mi_decl_hidden uint8_t* _mi_page_map;

static inline size_t _mi_page_map_index(const void* p) {
  return (size_t)((uintptr_t)p >> MI_ARENA_SLICE_SHIFT);
}

static inline mi_page_t* _mi_ptr_page_ex(const void* p, bool* valid) {
  const size_t idx = _mi_page_map_index(p);
  const size_t ofs = _mi_page_map[idx];
  if (valid != NULL) { *valid = (ofs != 0); }
  return (mi_page_t*)((((uintptr_t)p >> MI_ARENA_SLICE_SHIFT) + 1 - ofs) << MI_ARENA_SLICE_SHIFT);
}

static inline mi_page_t* _mi_checked_ptr_page(const void* p) {
  bool valid;
  mi_page_t* const page = _mi_ptr_page_ex(p, &valid);
  return (valid ? page : NULL);
}

static inline mi_page_t* _mi_unchecked_ptr_page(const void* p) {
  return _mi_ptr_page_ex(p, NULL);
}

#else

// 2-level page map:
// double indirection, but low commit and low virtual reserve.
//
// the page-map is usually 4 MiB (for 48 bit virtual addresses) and points to sub maps of 64 KiB.
// the page-map is committed on-demand (in 64 KiB parts) (and sub-maps are committed on-demand as well)
// one sub page-map = 64 KiB => covers 2^(16-3) * 2^16 = 2^29 = 512 MiB address space
// the page-map needs 48-(16+13) = 19 bits => 2^19 sub map pointers = 2^22 bytes = 4 MiB reserved size.
#define MI_PAGE_MAP_SUB_SHIFT     (13)
#define MI_PAGE_MAP_SUB_COUNT     (MI_ZU(1) << MI_PAGE_MAP_SUB_SHIFT)
#define MI_PAGE_MAP_SHIFT         (MI_MAX_VABITS - MI_PAGE_MAP_SUB_SHIFT - MI_ARENA_SLICE_SHIFT)
#define MI_PAGE_MAP_COUNT         (MI_ZU(1) << MI_PAGE_MAP_SHIFT)

extern mi_decl_hidden _Atomic(mi_page_t**)* _mi_page_map;

static inline size_t _mi_page_map_index(const void* p, size_t* sub_idx) {
  const size_t u = (size_t)((uintptr_t)p / MI_ARENA_SLICE_SIZE);
  if (sub_idx != NULL) { *sub_idx = u % MI_PAGE_MAP_SUB_COUNT; }
  return (u / MI_PAGE_MAP_SUB_COUNT);
}

static inline mi_page_t** _mi_page_map_at(size_t idx) {
  return mi_atomic_load_ptr_relaxed(mi_page_t*, &_mi_page_map[idx]);
}

static inline mi_page_t* _mi_unchecked_ptr_page(const void* p) {
  size_t sub_idx;
  const size_t idx = _mi_page_map_index(p, &sub_idx);
  return (_mi_page_map_at(idx))[sub_idx];  // NULL if p==NULL
}

static inline mi_page_t* _mi_checked_ptr_page(const void* p) {
  size_t sub_idx;
  const size_t idx = _mi_page_map_index(p, &sub_idx);
  mi_page_t** const sub = _mi_page_map_at(idx);
  if mi_unlikely(sub == NULL) return NULL;
  return sub[sub_idx];
}

#endif


static inline mi_page_t* _mi_ptr_page(const void* p) {
  mi_assert_internal(p==NULL || mi_is_in_heap_region(p));
  #if MI_DEBUG || MI_SECURE || defined(__APPLE__)
  return _mi_checked_ptr_page(p);
  #else
  return _mi_unchecked_ptr_page(p);
  #endif
}


// Get the block size of a page
static inline size_t mi_page_block_size(const mi_page_t* page) {
  mi_assert_internal(page->block_size > 0);
  return page->block_size;
}

// Page start
static inline uint8_t* mi_page_start(const mi_page_t* page) {
  return page->page_start;
}

static inline size_t mi_page_size(const mi_page_t* page) {
  return mi_page_block_size(page) * page->reserved;
}

static inline uint8_t* mi_page_area(const mi_page_t* page, size_t* size) {
  if (size) { *size = mi_page_size(page); }
  return mi_page_start(page);
}

static inline size_t mi_page_info_size(void) {
  return _mi_align_up(sizeof(mi_page_t), MI_MAX_ALIGN_SIZE);
}

static inline bool mi_page_contains_address(const mi_page_t* page, const void* p) {
  size_t psize;
  uint8_t* start = mi_page_area(page, &psize);
  return (start <= (uint8_t*)p && (uint8_t*)p < start + psize);
}

static inline bool mi_page_is_in_arena(const mi_page_t* page) {
  return (page->memid.memkind == MI_MEM_ARENA);
}

static inline bool mi_page_is_singleton(const mi_page_t* page) {
  return (page->reserved == 1);
}

// Get the usable block size of a page without fixed padding.
// This may still include internal padding due to alignment and rounding up size classes.
static inline size_t mi_page_usable_block_size(const mi_page_t* page) {
  return mi_page_block_size(page) - MI_PADDING_SIZE;
}

// This may change if we locate page info outside the page data slices
static inline uint8_t* mi_page_slice_start(const mi_page_t* page) {
  return (uint8_t*)page;
}

// This gives the offset relative to the start slice of a page. This may change if we ever
// locate page info outside the page-data itself.
static inline size_t mi_page_slice_offset_of(const mi_page_t* page, size_t offset_relative_to_page_start) {
  return (page->page_start - mi_page_slice_start(page)) + offset_relative_to_page_start;
}

static inline size_t mi_page_committed(const mi_page_t* page) {
  return (page->slice_committed == 0 ? mi_page_size(page) : page->slice_committed - (page->page_start - mi_page_slice_start(page)));
}

static inline mi_heap_t* mi_page_heap(const mi_page_t* page) {
  return page->heap;
}


// are all blocks in a page freed?
// note: needs up-to-date used count, (as the `xthread_free` list may not be empty). see `_mi_page_collect_free`.
static inline bool mi_page_all_free(const mi_page_t* page) {
  mi_assert_internal(page != NULL);
  return (page->used == 0);
}

// are there immediately available blocks, i.e. blocks available on the free list.
static inline bool mi_page_immediate_available(const mi_page_t* page) {
  mi_assert_internal(page != NULL);
  return (page->free != NULL);
}


// is the page not yet used up to its reserved space?
static inline bool mi_page_is_expandable(const mi_page_t* page) {
  mi_assert_internal(page != NULL);
  mi_assert_internal(page->capacity <= page->reserved);
  return (page->capacity < page->reserved);
}


static inline bool mi_page_is_full(mi_page_t* page) {
  bool full = (page->reserved == page->used);
  mi_assert_internal(!full || page->free == NULL);
  return full;
}

// is more than 7/8th of a page in use?
static inline bool mi_page_is_mostly_used(const mi_page_t* page) {
  if (page==NULL) return true;
  uint16_t frac = page->reserved / 8U;
  return (page->reserved - page->used <= frac);
}

// is more than (n-1)/n'th of a page in use?
static inline bool mi_page_is_used_at_frac(const mi_page_t* page, uint16_t n) {
  if (page==NULL) return true;
  uint16_t frac = page->reserved / n;
  return (page->reserved - page->used <= frac);
}


static inline bool mi_page_is_huge(const mi_page_t* page) {
  return (mi_page_is_singleton(page) &&
          (page->block_size > MI_LARGE_MAX_OBJ_SIZE ||
           (mi_memkind_is_os(page->memid.memkind) && page->memid.mem.os.base < (void*)page)));
}

static inline mi_page_queue_t* mi_page_queue(const mi_heap_t* heap, size_t size) {
  mi_page_queue_t* const pq = &((mi_heap_t*)heap)->pages[_mi_bin(size)];
  if (size <= MI_LARGE_MAX_OBJ_SIZE) { mi_assert_internal(pq->block_size <= MI_LARGE_MAX_OBJ_SIZE); }
  return pq;
}


//-----------------------------------------------------------
// Page thread id and flags
//-----------------------------------------------------------

// Thread id of thread that owns this page (with flags in the bottom 2 bits)
static inline mi_threadid_t mi_page_xthread_id(const mi_page_t* page) {
  return mi_atomic_load_relaxed(&((mi_page_t*)page)->xthread_id);
}

// Plain thread id of the thread that owns this page
static inline mi_threadid_t mi_page_thread_id(const mi_page_t* page) {
  return (mi_page_xthread_id(page) & ~MI_PAGE_FLAG_MASK);
}

static inline mi_page_flags_t mi_page_flags(const mi_page_t* page) {
  return (mi_page_xthread_id(page) & MI_PAGE_FLAG_MASK);
}

static inline void mi_page_flags_set(mi_page_t* page, bool set, mi_page_flags_t newflag) {
  if (set) { mi_atomic_or_relaxed(&page->xthread_id, newflag); }
      else { mi_atomic_and_relaxed(&page->xthread_id, ~newflag); }
}

static inline bool mi_page_is_in_full(const mi_page_t* page) {
  return ((mi_page_flags(page) & MI_PAGE_IN_FULL_QUEUE) != 0);
}

static inline void mi_page_set_in_full(mi_page_t* page, bool in_full) {
  mi_page_flags_set(page, in_full, MI_PAGE_IN_FULL_QUEUE);
}

static inline bool mi_page_has_aligned(const mi_page_t* page) {
  return ((mi_page_flags(page) & MI_PAGE_HAS_ALIGNED) != 0);
}

static inline void mi_page_set_has_aligned(mi_page_t* page, bool has_aligned) {
  mi_page_flags_set(page, has_aligned, MI_PAGE_HAS_ALIGNED);
}

static inline void mi_page_set_heap(mi_page_t* page, mi_heap_t* heap) {
  // mi_assert_internal(!mi_page_is_in_full(page));  // can happen when destroying pages on heap_destroy
  const mi_threadid_t tid = (heap == NULL ? MI_THREADID_ABANDONED : heap->tld->thread_id) | mi_page_flags(page);
  if (heap != NULL) {
    page->heap = heap;
    page->heap_tag = heap->tag;
  }
  else {
    page->heap = NULL;
  }
  mi_atomic_store_release(&page->xthread_id, tid);
}

static inline bool mi_page_is_abandoned(const mi_page_t* page) {
  // note: the xheap field of an abandoned heap is set to the subproc (for fast reclaim-on-free)
  return (mi_page_thread_id(page) <= MI_THREADID_ABANDONED_MAPPED);
}

static inline bool mi_page_is_abandoned_mapped(const mi_page_t* page) {
  return (mi_page_thread_id(page) == MI_THREADID_ABANDONED_MAPPED);
}

static inline void mi_page_set_abandoned_mapped(mi_page_t* page) {
  mi_assert_internal(mi_page_is_abandoned(page));
  mi_atomic_or_relaxed(&page->xthread_id, MI_THREADID_ABANDONED_MAPPED);
}

static inline void mi_page_clear_abandoned_mapped(mi_page_t* page) {
  mi_assert_internal(mi_page_is_abandoned_mapped(page));
  mi_atomic_and_relaxed(&page->xthread_id, MI_PAGE_FLAG_MASK);
}

//-----------------------------------------------------------
// Thread free list and ownership
//-----------------------------------------------------------

// Thread free flag helpers
static inline mi_block_t* mi_tf_block(mi_thread_free_t tf) {
  return (mi_block_t*)(tf & ~1);
}
static inline bool mi_tf_is_owned(mi_thread_free_t tf) {
  return ((tf & 1) == 1);
}
static inline mi_thread_free_t mi_tf_create(mi_block_t* block, bool owned) {
  return (mi_thread_free_t)((uintptr_t)block | (owned ? 1 : 0));
}

// Thread free access
static inline mi_block_t* mi_page_thread_free(const mi_page_t* page) {
  return mi_tf_block(mi_atomic_load_relaxed(&((mi_page_t*)page)->xthread_free));
}

// are there any available blocks?
static inline bool mi_page_has_any_available(const mi_page_t* page) {
  mi_assert_internal(page != NULL && page->reserved > 0);
  return (page->used < page->reserved || (mi_page_thread_free(page) != NULL));
}


// Owned?
static inline bool mi_page_is_owned(const mi_page_t* page) {
  return mi_tf_is_owned(mi_atomic_load_relaxed(&((mi_page_t*)page)->xthread_free));
}

// Unown a page that is currently owned
static inline void _mi_page_unown_unconditional(mi_page_t* page) {
  mi_assert_internal(mi_page_is_owned(page));
  mi_assert_internal(mi_page_thread_id(page)==0);
  const uintptr_t old = mi_atomic_and_acq_rel(&page->xthread_free, ~((uintptr_t)1));
  mi_assert_internal((old&1)==1); MI_UNUSED(old);
}

// get ownership if it is not yet owned
static inline bool mi_page_try_claim_ownership(mi_page_t* page) {
  const uintptr_t old = mi_atomic_or_acq_rel(&page->xthread_free, 1);
  return ((old&1)==0);
}

// release ownership of a page. This may free the page if all blocks were concurrently
// freed in the meantime. Returns true if the page was freed.
static inline bool _mi_page_unown(mi_page_t* page) {
  mi_assert_internal(mi_page_is_owned(page));
  mi_assert_internal(mi_page_is_abandoned(page));
  mi_thread_free_t tf_new;
  mi_thread_free_t tf_old = mi_atomic_load_relaxed(&page->xthread_free);
  do {
    mi_assert_internal(mi_tf_is_owned(tf_old));
    while mi_unlikely(mi_tf_block(tf_old) != NULL) {
      _mi_page_free_collect(page, false);  // update used
      if (mi_page_all_free(page)) {        // it may become free just before unowning it
        _mi_arenas_page_unabandon(page);
        _mi_arenas_page_free(page,NULL);
        return true;
      }
      tf_old = mi_atomic_load_relaxed(&page->xthread_free);
    }
    mi_assert_internal(mi_tf_block(tf_old)==NULL);
    tf_new = mi_tf_create(NULL, false);
  } while (!mi_atomic_cas_weak_acq_rel(&page->xthread_free, &tf_old, tf_new));
  return false;
}


/* -------------------------------------------------------------------
  Guarded objects
------------------------------------------------------------------- */
#if MI_GUARDED

// we always align guarded pointers in a block at an offset
// the block `next` field is then used as a tag to distinguish regular offset aligned blocks from guarded ones
#define MI_BLOCK_TAG_ALIGNED   ((mi_encoded_t)(0))
#define MI_BLOCK_TAG_GUARDED   (~MI_BLOCK_TAG_ALIGNED)

static inline bool mi_block_ptr_is_guarded(const mi_block_t* block, const void* p) {
  const ptrdiff_t offset = (uint8_t*)p - (uint8_t*)block;
  return (offset >= (ptrdiff_t)(sizeof(mi_block_t)) && block->next == MI_BLOCK_TAG_GUARDED);
}

static inline bool mi_heap_malloc_use_guarded(mi_heap_t* heap, size_t size) {
  // this code is written to result in fast assembly as it is on the hot path for allocation
  const size_t count = heap->guarded_sample_count - 1;  // if the rate was 0, this will underflow and count for a long time..
  if mi_likely(count != 0) {
    // no sample
    heap->guarded_sample_count = count;
    return false;
  }
  else if (size >= heap->guarded_size_min && size <= heap->guarded_size_max) {
    // use guarded allocation
    heap->guarded_sample_count = heap->guarded_sample_rate;  // reset
    return (heap->guarded_sample_rate != 0);
  }
  else {
    // failed size criteria, rewind count (but don't write to an empty heap)
    if (heap->guarded_sample_rate != 0) { heap->guarded_sample_count = 1; }
    return false;
  }
}

mi_decl_restrict void* _mi_heap_malloc_guarded(mi_heap_t* heap, size_t size, bool zero) mi_attr_noexcept;

#endif


/* -------------------------------------------------------------------
Encoding/Decoding the free list next pointers

This is to protect against buffer overflow exploits where the
free list is mutated. Many hardened allocators xor the next pointer `p`
with a secret key `k1`, as `p^k1`. This prevents overwriting with known
values but might be still too weak: if the attacker can guess
the pointer `p` this  can reveal `k1` (since `p^k1^p == k1`).
Moreover, if multiple blocks can be read as well, the attacker can
xor both as `(p1^k1) ^ (p2^k1) == p1^p2` which may reveal a lot
about the pointers (and subsequently `k1`).

Instead mimalloc uses an extra key `k2` and encodes as `((p^k2)<<<k1)+k1`.
Since these operations are not associative, the above approaches do not
work so well any more even if the `p` can be guesstimated. For example,
for the read case we can subtract two entries to discard the `+k1` term,
but that leads to `((p1^k2)<<<k1) - ((p2^k2)<<<k1)` at best.
We include the left-rotation since xor and addition are otherwise linear
in the lowest bit. Finally, both keys are unique per page which reduces
the re-use of keys by a large factor.

We also pass a separate `null` value to be used as `NULL` or otherwise
`(k2<<<k1)+k1` would appear (too) often as a sentinel value.
------------------------------------------------------------------- */

static inline bool mi_is_in_same_page(const void* p, const void* q) {
  mi_page_t* page = _mi_ptr_page(p);
  return mi_page_contains_address(page,q);
  // return (_mi_ptr_page(p) == _mi_ptr_page(q));
}

static inline void* mi_ptr_decode(const void* null, const mi_encoded_t x, const uintptr_t* keys) {
  void* p = (void*)(mi_rotr(x - keys[0], keys[0]) ^ keys[1]);
  return (p==null ? NULL : p);
}

static inline mi_encoded_t mi_ptr_encode(const void* null, const void* p, const uintptr_t* keys) {
  uintptr_t x = (uintptr_t)(p==NULL ? null : p);
  return mi_rotl(x ^ keys[1], keys[0]) + keys[0];
}

static inline uint32_t mi_ptr_encode_canary(const void* null, const void* p, const uintptr_t* keys) {
  const uint32_t x = (uint32_t)(mi_ptr_encode(null,p,keys));
  // make the lowest byte 0 to prevent spurious read overflows which could be a security issue (issue #951)
  #if MI_BIG_ENDIAN
  return (x & 0x00FFFFFF);
  #else
  return (x & 0xFFFFFF00);
  #endif
}

static inline mi_block_t* mi_block_nextx( const void* null, const mi_block_t* block, const uintptr_t* keys ) {
  mi_track_mem_defined(block,sizeof(mi_block_t));
  mi_block_t* next;
  #ifdef MI_ENCODE_FREELIST
  next = (mi_block_t*)mi_ptr_decode(null, block->next, keys);
  #else
  MI_UNUSED(keys); MI_UNUSED(null);
  next = (mi_block_t*)block->next;
  #endif
  mi_track_mem_noaccess(block,sizeof(mi_block_t));
  return next;
}

static inline void mi_block_set_nextx(const void* null, mi_block_t* block, const mi_block_t* next, const uintptr_t* keys) {
  mi_track_mem_undefined(block,sizeof(mi_block_t));
  #ifdef MI_ENCODE_FREELIST
  block->next = mi_ptr_encode(null, next, keys);
  #else
  MI_UNUSED(keys); MI_UNUSED(null);
  block->next = (mi_encoded_t)next;
  #endif
  mi_track_mem_noaccess(block,sizeof(mi_block_t));
}

static inline mi_block_t* mi_block_next(const mi_page_t* page, const mi_block_t* block) {
  #ifdef MI_ENCODE_FREELIST
  mi_block_t* next = mi_block_nextx(page,block,page->keys);
  // check for free list corruption: is `next` at least in the same page?
  // TODO: check if `next` is `page->block_size` aligned?
  if mi_unlikely(next!=NULL && !mi_is_in_same_page(block, next)) {
    _mi_error_message(EFAULT, "corrupted free list entry of size %zub at %p: value 0x%zx\n", mi_page_block_size(page), block, (uintptr_t)next);
    next = NULL;
  }
  return next;
  #else
  MI_UNUSED(page);
  return mi_block_nextx(page,block,NULL);
  #endif
}

static inline void mi_block_set_next(const mi_page_t* page, mi_block_t* block, const mi_block_t* next) {
  #ifdef MI_ENCODE_FREELIST
  mi_block_set_nextx(page,block,next, page->keys);
  #else
  MI_UNUSED(page);
  mi_block_set_nextx(page,block,next,NULL);
  #endif
}

/* -----------------------------------------------------------
  arena blocks
----------------------------------------------------------- */

// Blocks needed for a given byte size
static inline size_t mi_slice_count_of_size(size_t size) {
  return _mi_divide_up(size, MI_ARENA_SLICE_SIZE);
}

// Byte size of a number of blocks
static inline size_t mi_size_of_slices(size_t bcount) {
  return (bcount * MI_ARENA_SLICE_SIZE);
}


/* -----------------------------------------------------------
  memory id's
----------------------------------------------------------- */

static inline mi_memid_t _mi_memid_create(mi_memkind_t memkind) {
  mi_memid_t memid;
  _mi_memzero_var(memid);
  memid.memkind = memkind;
  return memid;
}

static inline mi_memid_t _mi_memid_none(void) {
  return _mi_memid_create(MI_MEM_NONE);
}

static inline mi_memid_t _mi_memid_create_os(void* base, size_t size, bool committed, bool is_zero, bool is_large) {
  mi_memid_t memid = _mi_memid_create(MI_MEM_OS);
  memid.mem.os.base = base;
  memid.mem.os.size = size;
  memid.initially_committed = committed;
  memid.initially_zero = is_zero;
  memid.is_pinned = is_large;
  return memid;
}

static inline mi_memid_t _mi_memid_create_meta(void* mpage, size_t block_idx, size_t block_count) {
  mi_memid_t memid = _mi_memid_create(MI_MEM_META);
  memid.mem.meta.meta_page = mpage;
  memid.mem.meta.block_index = (uint32_t)block_idx;
  memid.mem.meta.block_count = (uint32_t)block_count;
  memid.initially_committed = true;
  memid.initially_zero = true;
  memid.is_pinned = true;
  return memid;
}


// -------------------------------------------------------------------
// Fast "random" shuffle
// -------------------------------------------------------------------

static inline uintptr_t _mi_random_shuffle(uintptr_t x) {
  if (x==0) { x = 17; }   // ensure we don't get stuck in generating zeros
#if (MI_INTPTR_SIZE>=8)
  // by Sebastiano Vigna, see: <http://xoshiro.di.unimi.it/splitmix64.c>
  x ^= x >> 30;
  x *= 0xbf58476d1ce4e5b9UL;
  x ^= x >> 27;
  x *= 0x94d049bb133111ebUL;
  x ^= x >> 31;
#elif (MI_INTPTR_SIZE==4)
  // by Chris Wellons, see: <https://nullprogram.com/blog/2018/07/31/>
  x ^= x >> 16;
  x *= 0x7feb352dUL;
  x ^= x >> 15;
  x *= 0x846ca68bUL;
  x ^= x >> 16;
#endif
  return x;
}


// ---------------------------------------------------------------------------------
// Provide our own `_mi_memcpy` for potential performance optimizations.
//
// For now, only on Windows with msvc/clang-cl we optimize to `rep movsb` if
// we happen to run on x86/x64 cpu's that have "fast short rep movsb" (FSRM) support
// (AMD Zen3+ (~2020) or Intel Ice Lake+ (~2017). See also issue #201 and pr #253.
// ---------------------------------------------------------------------------------

#if !MI_TRACK_ENABLED && defined(_WIN32) && (MI_ARCH_X64 || MI_ARCH_X86)
extern mi_decl_hidden bool _mi_cpu_has_fsrm;
extern mi_decl_hidden bool _mi_cpu_has_erms;

static inline void _mi_memcpy(void* dst, const void* src, size_t n) {
  if ((_mi_cpu_has_fsrm && n <= 128) || (_mi_cpu_has_erms && n > 128)) {
    __movsb((unsigned char*)dst, (const unsigned char*)src, n);
  }
  else {
    memcpy(dst, src, n);
  }
}
static inline void _mi_memset(void* dst, int val, size_t n) {
  if ((_mi_cpu_has_fsrm && n <= 128) || (_mi_cpu_has_erms && n > 128)) {
    __stosb((unsigned char*)dst, (uint8_t)val, n);
  }
  else {
    memset(dst, val, n);
  }
}
#else
static inline void _mi_memcpy(void* dst, const void* src, size_t n) {
  memcpy(dst, src, n);
}
static inline void _mi_memset(void* dst, int val, size_t n) {
  memset(dst, val, n);
}
#endif

// -------------------------------------------------------------------------------
// The `_mi_memcpy_aligned` can be used if the pointers are machine-word aligned
// This is used for example in `mi_realloc`.
// -------------------------------------------------------------------------------

#if (defined(__GNUC__) && (__GNUC__ >= 4)) || defined(__clang__)
// On GCC/CLang we provide a hint that the pointers are word aligned.
static inline void _mi_memcpy_aligned(void* dst, const void* src, size_t n) {
  mi_assert_internal(((uintptr_t)dst % MI_INTPTR_SIZE == 0) && ((uintptr_t)src % MI_INTPTR_SIZE == 0));
  void* adst = __builtin_assume_aligned(dst, MI_INTPTR_SIZE);
  const void* asrc = __builtin_assume_aligned(src, MI_INTPTR_SIZE);
  _mi_memcpy(adst, asrc, n);
}

static inline void _mi_memset_aligned(void* dst, int val, size_t n) {
  mi_assert_internal((uintptr_t)dst % MI_INTPTR_SIZE == 0);
  void* adst = __builtin_assume_aligned(dst, MI_INTPTR_SIZE);
  _mi_memset(adst, val, n);
}
#else
// Default fallback on `_mi_memcpy`
static inline void _mi_memcpy_aligned(void* dst, const void* src, size_t n) {
  mi_assert_internal(((uintptr_t)dst % MI_INTPTR_SIZE == 0) && ((uintptr_t)src % MI_INTPTR_SIZE == 0));
  _mi_memcpy(dst, src, n);
}

static inline void _mi_memset_aligned(void* dst, int val, size_t n) {
  mi_assert_internal((uintptr_t)dst % MI_INTPTR_SIZE == 0);
  _mi_memset(dst, val, n);
}
#endif

static inline void _mi_memzero(void* dst, size_t n) {
  _mi_memset(dst, 0, n);
}

static inline void _mi_memzero_aligned(void* dst, size_t n) {
  _mi_memset_aligned(dst, 0, n);
}


#endif  // MI_INTERNAL_H
