/*
 * Copyright (c) 2022, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_AV1_CX_IFACE_H_
#define AOM_AV1_AV1_CX_IFACE_H_
#include "av1/encoder/encoder.h"
#include "aom/aom_encoder.h"

#ifdef __cplusplus
extern "C" {
#endif

AV1EncoderConfig av1_get_encoder_config(const aom_codec_enc_cfg_t *cfg);

aom_codec_err_t av1_create_context_and_bufferpool(AV1_PRIMARY *ppi,
                                                  AV1_COMP **p_cpi,
                                                  BufferPool **p_buffer_pool,
                                                  const AV1EncoderConfig *oxcf,
                                                  COMPRESSOR_STAGE stage,
                                                  int lap_lag_in_frames);

void av1_destroy_context_and_bufferpool(AV1_COMP *cpi,
                                        BufferPool **p_buffer_pool);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_AV1_CX_IFACE_H_
