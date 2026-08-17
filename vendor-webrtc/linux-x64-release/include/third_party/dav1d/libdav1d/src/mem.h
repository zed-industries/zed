/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef DAV1D_SRC_MEM_H
#define DAV1D_SRC_MEM_H

#define TRACK_HEAP_ALLOCATIONS 0

#include <stdlib.h>

#if defined(_WIN32) || HAVE_MEMALIGN
#include <malloc.h>
#endif

#include "dav1d/dav1d.h"

#include "common/attributes.h"

#include "src/thread.h"

enum AllocationType {
    ALLOC_BLOCK,
    ALLOC_CDEF,
    ALLOC_CDF,
    ALLOC_COEF,
    ALLOC_COMMON_CTX,
    ALLOC_DAV1DDATA,
    ALLOC_IPRED,
    ALLOC_LF,
    ALLOC_LR,
    ALLOC_OBU_HDR,
    ALLOC_OBU_META,
    ALLOC_PAL,
    ALLOC_PIC,
    ALLOC_PIC_CTX,
    ALLOC_REFMVS,
    ALLOC_SEGMAP,
    ALLOC_THREAD_CTX,
    ALLOC_TILE,
    N_ALLOC_TYPES,
};

typedef struct Dav1dMemPoolBuffer {
    void *data;
    struct Dav1dMemPoolBuffer *next;
} Dav1dMemPoolBuffer;

typedef struct Dav1dMemPool {
    pthread_mutex_t lock;
    Dav1dMemPoolBuffer *buf;
    int ref_cnt;
    int end;
#if TRACK_HEAP_ALLOCATIONS
    enum AllocationType type;
#endif
} Dav1dMemPool;

// TODO: Move this to a common location?
#define ROUND_UP(x,a) (((x)+((a)-1)) & ~((a)-1))

/*
 * Allocate align-byte aligned memory. The return value can be released
 * by calling the dav1d_free_aligned() function.
 */
static inline void *dav1d_alloc_aligned_internal(const size_t sz, const size_t align) {
    assert(!(align & (align - 1)));
#ifdef _WIN32
    return _aligned_malloc(sz, align);
#elif HAVE_POSIX_MEMALIGN
    void *ptr;
    if (posix_memalign(&ptr, align, sz)) return NULL;
    return ptr;
#elif HAVE_MEMALIGN
    return memalign(align, sz);
#elif HAVE_ALIGNED_ALLOC
    // The C11 standard specifies that the size parameter
    // must be an integral multiple of alignment.
    return aligned_alloc(align, ROUND_UP(sz, align));
#else
#error No aligned allocation functions are available
#endif
}

static inline void dav1d_free_aligned_internal(void *ptr) {
#ifdef _WIN32
    _aligned_free(ptr);
#else
    free(ptr);
#endif
}

#if TRACK_HEAP_ALLOCATIONS
void *dav1d_malloc(enum AllocationType type, size_t sz);
void *dav1d_realloc(enum AllocationType type, void *ptr, size_t sz);
void *dav1d_alloc_aligned(enum AllocationType type, size_t sz, size_t align);
void dav1d_free(void *ptr);
void dav1d_free_aligned(void *ptr);
void dav1d_log_alloc_stats(Dav1dContext *c);
#else
#define dav1d_mem_pool_init(type, pool) dav1d_mem_pool_init(pool)
#define dav1d_malloc(type, sz) malloc(sz)
#define dav1d_realloc(type, ptr, sz) realloc(ptr, sz)
#define dav1d_alloc_aligned(type, sz, align) dav1d_alloc_aligned_internal(sz, align)
#define dav1d_free(ptr) free(ptr)
#define dav1d_free_aligned(ptr) dav1d_free_aligned_internal(ptr)
#endif /* TRACK_HEAP_ALLOCATIONS */

void dav1d_mem_pool_push(Dav1dMemPool *pool, Dav1dMemPoolBuffer *buf);
Dav1dMemPoolBuffer *dav1d_mem_pool_pop(Dav1dMemPool *pool, size_t size);
int dav1d_mem_pool_init(enum AllocationType type, Dav1dMemPool **pool);
void dav1d_mem_pool_end(Dav1dMemPool *pool);

static inline void dav1d_freep_aligned(void *ptr) {
    void **mem = (void **) ptr;
    if (*mem) {
        dav1d_free_aligned(*mem);
        *mem = NULL;
    }
}

#endif /* DAV1D_SRC_MEM_H */
