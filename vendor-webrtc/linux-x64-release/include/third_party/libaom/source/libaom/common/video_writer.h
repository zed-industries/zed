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

#ifndef AOM_COMMON_VIDEO_WRITER_H_
#define AOM_COMMON_VIDEO_WRITER_H_

#include "common/video_common.h"

enum { kContainerIVF } UENUM1BYTE(AvxContainer);

struct AvxVideoWriterStruct;
typedef struct AvxVideoWriterStruct AvxVideoWriter;

#ifdef __cplusplus
extern "C" {
#endif

// Finds and opens writer for specified container format.
// Returns an opaque AvxVideoWriter* upon success, or NULL upon failure.
// Right now only IVF format is supported.
AvxVideoWriter *aom_video_writer_open(const char *filename,
                                      AvxContainer container,
                                      const AvxVideoInfo *info);

// Frees all resources associated with AvxVideoWriter* returned from
// aom_video_writer_open() call.
void aom_video_writer_close(AvxVideoWriter *writer);

// Writes frame bytes to the file.
int aom_video_writer_write_frame(AvxVideoWriter *writer, const uint8_t *buffer,
                                 size_t size, int64_t pts);
// Set fourcc.
void aom_video_writer_set_fourcc(AvxVideoWriter *writer, uint32_t fourcc);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_COMMON_VIDEO_WRITER_H_
