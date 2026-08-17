/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef plarena_h___
#define plarena_h___
/*
 * Lifetime-based fast allocation, inspired by much prior art, including
 * "Fast Allocation and Deallocation of Memory Based on Object Lifetimes"
 * David R. Hanson, Software -- Practice and Experience, Vol. 20(1).
 *
 * Also supports LIFO allocation (PL_ARENA_MARK/PL_ARENA_RELEASE).
 */
#include "prtypes.h"

PR_BEGIN_EXTERN_C

typedef struct PLArena          PLArena;

struct PLArena {
    PLArena     *next;          /* next arena for this lifetime */
    PRUword     base;           /* aligned base address, follows this header */
    PRUword     limit;          /* one beyond last byte in arena */
    PRUword     avail;          /* points to next available byte */
};

#ifdef PL_ARENAMETER
typedef struct PLArenaStats PLArenaStats;

struct PLArenaStats {
    PLArenaStats  *next;        /* next in arenaStats list */
    char          *name;        /* name for debugging */
    PRUint32      narenas;      /* number of arenas in pool */
    PRUint32      nallocs;      /* number of PL_ARENA_ALLOCATE() calls */
    PRUint32      nreclaims;    /* number of reclaims from freeArenas */
    PRUint32      nmallocs;     /* number of malloc() calls */
    PRUint32      ndeallocs;    /* number of lifetime deallocations */
    PRUint32      ngrows;       /* number of PL_ARENA_GROW() calls */
    PRUint32      ninplace;     /* number of in-place growths */
    PRUint32      nreleases;    /* number of PL_ARENA_RELEASE() calls */
    PRUint32      nfastrels;    /* number of "fast path" releases */
    PRUint32      nbytes;       /* total bytes allocated */
    PRUint32      maxalloc;     /* maximum allocation size in bytes */
    PRFloat64     variance;     /* size variance accumulator */
};
#endif

typedef struct PLArenaPool      PLArenaPool;

struct PLArenaPool {
    PLArena     first;          /* first arena in pool list */
    PLArena     *current;       /* arena from which to allocate space */
    PRUint32    arenasize;      /* net exact size of a new arena */
    PRUword     mask;           /* alignment mask (power-of-2 - 1) */
#ifdef PL_ARENAMETER
    PLArenaStats stats;
#endif
};

/*
 * WARNING: The PL_MAKE_MEM_ macros are for internal use by NSPR. Do NOT use
 * them in your code.
 *
 * NOTE: Valgrind support to be added.
 *
 * The PL_MAKE_MEM_ macros are modeled after the MOZ_MAKE_MEM_ macros in
 * Mozilla's mfbt/MemoryChecking.h. Only AddressSanitizer is supported now.
 *
 * Provides a common interface to the ASan (AddressSanitizer) and Valgrind
 * functions used to mark memory in certain ways. In detail, the following
 * three macros are provided:
 *
 *   PL_MAKE_MEM_NOACCESS  - Mark memory as unsafe to access (e.g. freed)
 *   PL_MAKE_MEM_UNDEFINED - Mark memory as accessible, with content undefined
 *   PL_MAKE_MEM_DEFINED - Mark memory as accessible, with content defined
 *
 * With Valgrind in use, these directly map to the three respective Valgrind
 * macros. With ASan in use, the NOACCESS macro maps to poisoning the memory,
 * while the UNDEFINED/DEFINED macros unpoison memory.
 *
 * With no memory checker available, all macros expand to the empty statement.
 */

/* WARNING: PL_SANITIZE_ADDRESS is for internal use by this header. Do NOT
 * define or test this macro in your code.
 */
#if defined(__has_feature)
#if __has_feature(address_sanitizer)
#define PL_SANITIZE_ADDRESS 1
#endif
#elif defined(__SANITIZE_ADDRESS__)
#define PL_SANITIZE_ADDRESS 1
#endif

#if defined(PL_SANITIZE_ADDRESS)

#if defined(_MSC_VER)
/* We can't use dllimport due to DLL linkage mismatch with
 * sanitizer/asan_interface.h.
 */
#define PL_ASAN_VISIBILITY(type_) type_
#else
#define PL_ASAN_VISIBILITY(type_) PR_IMPORT(type_)
#endif

/* These definitions are usually provided through the
 * sanitizer/asan_interface.h header installed by ASan.
 * See https://github.com/google/sanitizers/wiki/AddressSanitizerManualPoisoning
 */

PL_ASAN_VISIBILITY(void) __asan_poison_memory_region(
    void const volatile *addr, size_t size);
PL_ASAN_VISIBILITY(void) __asan_unpoison_memory_region(
    void const volatile *addr, size_t size);

#define PL_MAKE_MEM_NOACCESS(addr, size) \
    __asan_poison_memory_region((addr), (size))

#define PL_MAKE_MEM_UNDEFINED(addr, size) \
    __asan_unpoison_memory_region((addr), (size))

#define PL_MAKE_MEM_DEFINED(addr, size) \
    __asan_unpoison_memory_region((addr), (size))

#else

#define PL_MAKE_MEM_NOACCESS(addr, size)
#define PL_MAKE_MEM_UNDEFINED(addr, size)
#define PL_MAKE_MEM_DEFINED(addr, size)

#endif

/*
 * If the including .c file uses only one power-of-2 alignment, it may define
 * PL_ARENA_CONST_ALIGN_MASK to the alignment mask and save a few instructions
 * per ALLOCATE and GROW.
 */
#ifdef PL_ARENA_CONST_ALIGN_MASK
#define PL_ARENA_ALIGN(pool, n) (((PRUword)(n) + PL_ARENA_CONST_ALIGN_MASK) \
                                & ~PL_ARENA_CONST_ALIGN_MASK)

#define PL_INIT_ARENA_POOL(pool, name, size) \
        PL_InitArenaPool(pool, name, size, PL_ARENA_CONST_ALIGN_MASK + 1)
#else
#define PL_ARENA_ALIGN(pool, n) (((PRUword)(n) + (pool)->mask) & ~(pool)->mask)
#endif

#define PL_ARENA_ALLOCATE(p, pool, nb) \
    PR_BEGIN_MACRO \
        PLArena *_a = (pool)->current; \
        PRUint32 _nb = PL_ARENA_ALIGN(pool, (PRUint32)nb); \
        PRUword _p = _a->avail; \
        if (_nb < (PRUint32)nb) { \
            _p = 0; \
        } else if (_nb > (_a->limit - _a->avail)) { \
            _p = (PRUword)PL_ArenaAllocate(pool, _nb); \
        } else { \
            _a->avail += _nb; \
        } \
        p = (void *)_p; \
        if (p) { \
            PL_MAKE_MEM_UNDEFINED(p, (PRUint32)nb); \
            PL_ArenaCountAllocation(pool, (PRUint32)nb); \
        } \
    PR_END_MACRO

#define PL_ARENA_GROW(p, pool, size, incr) \
    PR_BEGIN_MACRO \
        PLArena *_a = (pool)->current; \
        PRUint32 _incr = PL_ARENA_ALIGN(pool, (PRUint32)incr); \
        if (_incr < (PRUint32)incr) { \
            p = NULL; \
        } else if (_a->avail == (PRUword)(p) + PL_ARENA_ALIGN(pool, size) && \
            _incr <= (_a->limit - _a->avail)) { \
            PL_MAKE_MEM_UNDEFINED((unsigned char *)(p) + size, (PRUint32)incr); \
            _a->avail += _incr; \
            PL_ArenaCountInplaceGrowth(pool, size, (PRUint32)incr); \
        } else { \
            p = PL_ArenaGrow(pool, p, size, (PRUint32)incr); \
        } \
        if (p) {\
            PL_ArenaCountGrowth(pool, size, (PRUint32)incr); \
        } \
    PR_END_MACRO

#define PL_ARENA_MARK(pool) ((void *) (pool)->current->avail)
#define PR_UPTRDIFF(p,q) ((PRUword)(p) - (PRUword)(q))

#define PL_CLEAR_UNUSED_PATTERN(a, pattern) \
    PR_BEGIN_MACRO \
        PR_ASSERT((a)->avail <= (a)->limit); \
        PL_MAKE_MEM_UNDEFINED((void*)(a)->avail, (a)->limit - (a)->avail); \
        memset((void*)(a)->avail, (pattern), (a)->limit - (a)->avail); \
    PR_END_MACRO
#ifdef DEBUG
#define PL_FREE_PATTERN 0xDA
#define PL_CLEAR_UNUSED(a) PL_CLEAR_UNUSED_PATTERN((a), PL_FREE_PATTERN)
#define PL_CLEAR_ARENA(a) \
    PR_BEGIN_MACRO \
        PL_MAKE_MEM_UNDEFINED((void*)(a), (a)->limit - (PRUword)(a)); \
        memset((void*)(a), PL_FREE_PATTERN, (a)->limit - (PRUword)(a)); \
    PR_END_MACRO
#else
#define PL_CLEAR_UNUSED(a)
#define PL_CLEAR_ARENA(a)
#endif

#define PL_ARENA_RELEASE(pool, mark) \
    PR_BEGIN_MACRO \
        char *_m = (char *)(mark); \
        PLArena *_a = (pool)->current; \
        if (PR_UPTRDIFF(_m, _a->base) <= PR_UPTRDIFF(_a->avail, _a->base)) { \
            _a->avail = (PRUword)PL_ARENA_ALIGN(pool, _m); \
            PL_CLEAR_UNUSED(_a); \
            PL_MAKE_MEM_NOACCESS((void*)_a->avail, _a->limit - _a->avail); \
            PL_ArenaCountRetract(pool, _m); \
        } else { \
            PL_ArenaRelease(pool, _m); \
        } \
        PL_ArenaCountRelease(pool, _m); \
    PR_END_MACRO

#ifdef PL_ARENAMETER
#define PL_COUNT_ARENA(pool,op) ((pool)->stats.narenas op)
#else
#define PL_COUNT_ARENA(pool,op)
#endif

#define PL_ARENA_DESTROY(pool, a, pnext) \
    PR_BEGIN_MACRO \
        PL_COUNT_ARENA(pool,--); \
        if ((pool)->current == (a)) (pool)->current = &(pool)->first; \
        *(pnext) = (a)->next; \
        PL_CLEAR_ARENA(a); \
        free(a); \
        (a) = 0; \
    PR_END_MACRO

/*
** Initialize an arena pool with the given name for debugging and metering,
** with a minimum gross size per arena of size bytes.  The net size per arena
** is smaller than the gross size by a header of four pointers plus any
** necessary padding for alignment.
**
** Note: choose a gross size that's a power of two to avoid the heap allocator
** rounding the size up.
**/
PR_EXTERN(void) PL_InitArenaPool(
    PLArenaPool *pool, const char *name, PRUint32 size, PRUint32 align);

/*
** Finish using arenas, freeing all memory associated with them.
** NOTE: this function is now a no-op. If you want to free a single
** PLArenaPoolUse use PL_FreeArenaPool() or PL_FinishArenaPool().
**/
PR_EXTERN(void) PL_ArenaFinish(void);

/*
** Free the arenas in pool.  The user may continue to allocate from pool
** after calling this function.  There is no need to call PL_InitArenaPool()
** again unless PL_FinishArenaPool(pool) has been called.
**/
PR_EXTERN(void) PL_FreeArenaPool(PLArenaPool *pool);

/*
** Free the arenas in pool and finish using it altogether.
**/
PR_EXTERN(void) PL_FinishArenaPool(PLArenaPool *pool);

/*
** Compact all of the arenas in a pool so that no space is wasted.
** NOT IMPLEMENTED.  Do not use.
**/
PR_EXTERN(void) PL_CompactArenaPool(PLArenaPool *pool);

/*
** Friend functions used by the PL_ARENA_*() macros.
**
** WARNING: do not call these functions directly. Always use the
** PL_ARENA_*() macros.
**/
PR_EXTERN(void *) PL_ArenaAllocate(PLArenaPool *pool, PRUint32 nb);

PR_EXTERN(void *) PL_ArenaGrow(
    PLArenaPool *pool, void *p, PRUint32 size, PRUint32 incr);

PR_EXTERN(void) PL_ArenaRelease(PLArenaPool *pool, char *mark);

/*
** memset contents of all arenas in pool to pattern
*/
PR_EXTERN(void) PL_ClearArenaPool(PLArenaPool *pool, PRInt32 pattern);

/*
** A function like malloc_size() or malloc_usable_size() that measures the
** size of a heap block.
*/
typedef size_t (*PLMallocSizeFn)(const void *ptr);

/*
** Measure all memory used by a PLArenaPool, excluding the PLArenaPool
** structure.
*/
PR_EXTERN(size_t) PL_SizeOfArenaPoolExcludingPool(
    const PLArenaPool *pool, PLMallocSizeFn mallocSizeOf);

#ifdef PL_ARENAMETER

#include <stdio.h>

PR_EXTERN(void) PL_ArenaCountAllocation(PLArenaPool *pool, PRUint32 nb);

PR_EXTERN(void) PL_ArenaCountInplaceGrowth(
    PLArenaPool *pool, PRUint32 size, PRUint32 incr);

PR_EXTERN(void) PL_ArenaCountGrowth(
    PLArenaPool *pool, PRUint32 size, PRUint32 incr);

PR_EXTERN(void) PL_ArenaCountRelease(PLArenaPool *pool, char *mark);

PR_EXTERN(void) PL_ArenaCountRetract(PLArenaPool *pool, char *mark);

PR_EXTERN(void) PL_DumpArenaStats(FILE *fp);

#else  /* !PL_ARENAMETER */

#define PL_ArenaCountAllocation(ap, nb)                 /* nothing */
#define PL_ArenaCountInplaceGrowth(ap, size, incr)      /* nothing */
#define PL_ArenaCountGrowth(ap, size, incr)             /* nothing */
#define PL_ArenaCountRelease(ap, mark)                  /* nothing */
#define PL_ArenaCountRetract(ap, mark)                  /* nothing */

#endif /* !PL_ARENAMETER */

PR_END_EXTERN_C

#endif /* plarena_h___ */
