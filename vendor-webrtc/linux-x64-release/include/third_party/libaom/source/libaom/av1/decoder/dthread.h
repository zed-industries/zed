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

#ifndef AOM_AV1_DECODER_DTHREAD_H_
#define AOM_AV1_DECODER_DTHREAD_H_

#include "config/aom_config.h"

#include "aom/internal/aom_codec_internal.h"

#ifdef __cplusplus
extern "C" {
#endif

struct AV1Common;
struct AV1Decoder;
struct ThreadData;

typedef struct DecWorkerData {
  struct ThreadData *td;
  const uint8_t *data_end;
  struct aom_internal_error_info error_info;
} DecWorkerData;

// WorkerData for the FrameWorker thread. It contains all the information of
// the worker and decode structures for decoding a frame.
typedef struct FrameWorkerData {
  struct AV1Decoder *pbi;
  const uint8_t *data;
  const uint8_t *data_end;
  size_t data_size;
  void *user_priv;
  int received_frame;
  int frame_context_ready;  // Current frame's context is ready to read.
  int frame_decoded;        // Finished decoding current frame.
} FrameWorkerData;

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_DECODER_DTHREAD_H_
