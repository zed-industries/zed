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

#ifndef DAV1D_SRC_REF_H
#define DAV1D_SRC_REF_H

#include "dav1d/dav1d.h"

#include "src/mem.h"
#include "src/thread.h"

#include <stdatomic.h>
#include <stddef.h>

struct Dav1dRef {
    void *data;
    const void *const_data;
    atomic_int ref_cnt;
    int free_ref;
    void (*free_callback)(const uint8_t *data, void *user_data);
    void *user_data;
};

#if !TRACK_HEAP_ALLOCATIONS
#define dav1d_ref_create(type, size) dav1d_ref_create(size)
#endif

Dav1dRef *dav1d_ref_create(enum AllocationType type, size_t size);
Dav1dRef *dav1d_ref_create_using_pool(Dav1dMemPool *pool, size_t size);
void dav1d_ref_dec(Dav1dRef **ref);

static inline Dav1dRef *dav1d_ref_init(Dav1dRef *const ref, const void *const ptr,
                                       void (*const free_callback)(const uint8_t *data, void *user_data),
                                       void *const user_data, const int free_ref)
{
    ref->data = NULL;
    ref->const_data = ptr;
    atomic_init(&ref->ref_cnt, 1);
    ref->free_ref = free_ref;
    ref->free_callback = free_callback;
    ref->user_data = user_data;
    return ref;
}

static inline void dav1d_ref_inc(Dav1dRef *const ref) {
    atomic_fetch_add_explicit(&ref->ref_cnt, 1, memory_order_relaxed);
}

static inline int dav1d_ref_is_writable(Dav1dRef *const ref) {
    return atomic_load(&ref->ref_cnt) == 1 && ref->data;
}

#endif /* DAV1D_SRC_REF_H */
