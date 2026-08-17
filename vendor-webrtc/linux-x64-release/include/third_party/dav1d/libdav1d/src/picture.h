/*
 * Copyright © 2018-2021, VideoLAN and dav1d authors
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

#ifndef DAV1D_SRC_PICTURE_H
#define DAV1D_SRC_PICTURE_H

#include <stdatomic.h>

#include "src/thread.h"
#include "dav1d/picture.h"

#include "src/thread_data.h"
#include "src/ref.h"

enum PlaneType {
    PLANE_TYPE_Y,
    PLANE_TYPE_UV,
    PLANE_TYPE_BLOCK,
    PLANE_TYPE_ALL,
};

enum PictureFlags {
    PICTURE_FLAG_NEW_SEQUENCE =       1 << 0,
    PICTURE_FLAG_NEW_OP_PARAMS_INFO = 1 << 1,
    PICTURE_FLAG_NEW_TEMPORAL_UNIT  = 1 << 2,
};

typedef struct Dav1dThreadPicture {
    Dav1dPicture p;
    int visible;
    // This can be set for inter frames, non-key intra frames, or for invisible
    // keyframes that have not yet been made visible using the show-existing-frame
    // mechanism.
    int showable;
    enum PictureFlags flags;
    // [0] block data (including segmentation map and motion vectors)
    // [1] pixel data
    atomic_uint *progress;
} Dav1dThreadPicture;

typedef struct Dav1dPictureBuffer {
    void *data;
    struct Dav1dPictureBuffer *next;
} Dav1dPictureBuffer;

/*
 * Allocate a picture with custom border size.
 */
int dav1d_thread_picture_alloc(Dav1dContext *c, Dav1dFrameContext *f, const int bpc);

/**
 * Allocate a picture with identical metadata to an existing picture.
 * The width is a separate argument so this function can be used for
 * super-res, where the width changes, but everything else is the same.
 * For the more typical use case of allocating a new image of the same
 * dimensions, use src->p.w as width.
 */
int dav1d_picture_alloc_copy(Dav1dContext *c, Dav1dPicture *dst, const int w,
                             const Dav1dPicture *src);

/**
 * Create a copy of a picture.
 */
void dav1d_picture_ref(Dav1dPicture *dst, const Dav1dPicture *src);
void dav1d_thread_picture_ref(Dav1dThreadPicture *dst,
                              const Dav1dThreadPicture *src);
void dav1d_thread_picture_move_ref(Dav1dThreadPicture *dst,
                                   Dav1dThreadPicture *src);
void dav1d_thread_picture_unref(Dav1dThreadPicture *p);

/**
 * Move a picture reference.
 */
void dav1d_picture_move_ref(Dav1dPicture *dst, Dav1dPicture *src);

int dav1d_default_picture_alloc(Dav1dPicture *p, void *cookie);
void dav1d_default_picture_release(Dav1dPicture *p, void *cookie);
void dav1d_picture_unref_internal(Dav1dPicture *p);

struct itut_t35_ctx_context {
    Dav1dITUTT35 *itut_t35;
    size_t n_itut_t35;
    Dav1dRef ref;
};

void dav1d_picture_free_itut_t35(const uint8_t *data, void *user_data);
void dav1d_picture_copy_props(Dav1dPicture *p,
                              Dav1dContentLightLevel *content_light, Dav1dRef *content_light_ref,
                              Dav1dMasteringDisplay *mastering_display, Dav1dRef *mastering_display_ref,
                              Dav1dITUTT35 *itut_t35, Dav1dRef *itut_t35_ref, size_t n_itut_t35,
                              const Dav1dDataProps *props);

/**
 * Get event flags from picture flags.
 */
enum Dav1dEventFlags dav1d_picture_get_event_flags(const Dav1dThreadPicture *p);

#endif /* DAV1D_SRC_PICTURE_H */
