/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_COMMON_FRAME_BUFFERS_H_
#define AOM_AV1_COMMON_FRAME_BUFFERS_H_

#include "aom/aom_frame_buffer.h"
#include "aom/aom_integer.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct InternalFrameBuffer {
  uint8_t *data;
  size_t size;
  int in_use;
} InternalFrameBuffer;

typedef struct InternalFrameBufferList {
  int num_internal_frame_buffers;
  InternalFrameBuffer *int_fb;
} InternalFrameBufferList;

// Initializes |list|. Returns 0 on success.
int av1_alloc_internal_frame_buffers(InternalFrameBufferList *list);

// Free any data allocated to the frame buffers.
void av1_free_internal_frame_buffers(InternalFrameBufferList *list);

// Zeros all unused internal frame buffers. In particular, this zeros the
// frame borders. Call this function after a sequence header change to
// re-initialize the frame borders for the different width, height, or bit
// depth.
void av1_zero_unused_internal_frame_buffers(InternalFrameBufferList *list);

// Callback used by libaom to request an external frame buffer. |cb_priv|
// Callback private data, which points to an InternalFrameBufferList.
// |min_size| is the minimum size in bytes needed to decode the next frame.
// |fb| pointer to the frame buffer.
int av1_get_frame_buffer(void *cb_priv, size_t min_size,
                         aom_codec_frame_buffer_t *fb);

// Callback used by libaom when there are no references to the frame buffer.
// |cb_priv| is not used. |fb| pointer to the frame buffer.
int av1_release_frame_buffer(void *cb_priv, aom_codec_frame_buffer_t *fb);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_COMMON_FRAME_BUFFERS_H_
