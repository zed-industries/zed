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

#ifndef DAV1D_SRC_CDEF_H
#define DAV1D_SRC_CDEF_H

#include <stddef.h>
#include <stdint.h>

#include "common/bitdepth.h"

enum CdefEdgeFlags {
    CDEF_HAVE_LEFT = 1 << 0,
    CDEF_HAVE_RIGHT = 1 << 1,
    CDEF_HAVE_TOP = 1 << 2,
    CDEF_HAVE_BOTTOM = 1 << 3,
};

#ifdef BITDEPTH
typedef const pixel (*const_left_pixel_row_2px)[2];
#else
typedef const void *const_left_pixel_row_2px;
#endif

// CDEF operates entirely on pre-filter data; if bottom/right edges are
// present (according to $edges), then the pre-filter data is located in
// $dst. However, the edge pixels above $dst may be post-filter, so in
// order to get access to pre-filter top pixels, use $top.
#define decl_cdef_fn(name) \
void (name)(pixel *dst, ptrdiff_t stride, const_left_pixel_row_2px left, \
            const pixel *top, const pixel *bottom, \
            int pri_strength, int sec_strength, \
            int dir, int damping, enum CdefEdgeFlags edges HIGHBD_DECL_SUFFIX)
typedef decl_cdef_fn(*cdef_fn);

#define decl_cdef_dir_fn(name) \
int (name)(const pixel *dst, ptrdiff_t dst_stride, unsigned *var HIGHBD_DECL_SUFFIX)
typedef decl_cdef_dir_fn(*cdef_dir_fn);

typedef struct Dav1dCdefDSPContext {
    cdef_dir_fn dir;
    cdef_fn fb[3 /* 444/luma, 422, 420 */];
} Dav1dCdefDSPContext;

bitfn_decls(void dav1d_cdef_dsp_init, Dav1dCdefDSPContext *c);

#endif /* DAV1D_SRC_CDEF_H */
