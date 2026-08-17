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
#ifndef AOM_COMMON_WEBMDEC_H_
#define AOM_COMMON_WEBMDEC_H_

#include "common/tools_common.h"

#ifdef __cplusplus
extern "C" {
#endif

struct AvxInputContext;

struct WebmInputContext {
  void *reader;
  void *segment;
  uint8_t *buffer;
  const void *cluster;
  const void *block_entry;
  const void *block;
  int block_frame_index;
  int video_track_index;
  int64_t timestamp_ns;
  int is_key_frame;
  int reached_eos;
};

// Checks if the input is a WebM file. If so, initializes WebMInputContext so
// that webm_read_frame can be called to retrieve a video frame.
// Returns 1 on success and 0 on failure or input is not WebM file.
// TODO(vigneshv): Refactor this function into two smaller functions specific
// to their task.
int file_is_webm(struct WebmInputContext *webm_ctx,
                 struct AvxInputContext *aom_ctx);

// Reads a WebM Video Frame. Memory for the buffer is created, owned and managed
// by this function. For the first call, |buffer| should be NULL and
// |*buffer_size| should be 0. Once all the frames are read and used,
// webm_free() should be called, otherwise there will be a leak.
// Parameters:
//      webm_ctx - WebmInputContext object
//      buffer - pointer where the frame data will be filled.
//      bytes_read - pointer to bytes read.
//      buffer_size - pointer to buffer size.
// Return values:
//      0 - Success
//      1 - End of Stream
//     -1 - Error
int webm_read_frame(struct WebmInputContext *webm_ctx, uint8_t **buffer,
                    size_t *bytes_read, size_t *buffer_size);

// Guesses the frame rate of the input file based on the container timestamps.
int webm_guess_framerate(struct WebmInputContext *webm_ctx,
                         struct AvxInputContext *aom_ctx);

// Resets the WebMInputContext.
void webm_free(struct WebmInputContext *webm_ctx);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_COMMON_WEBMDEC_H_
