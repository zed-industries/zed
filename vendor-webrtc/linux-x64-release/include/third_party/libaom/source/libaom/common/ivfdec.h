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
#ifndef AOM_COMMON_IVFDEC_H_
#define AOM_COMMON_IVFDEC_H_

#include "aom/aom_codec.h"
#include "common/tools_common.h"

#ifdef __cplusplus
extern "C" {
#endif

int file_is_ivf(struct AvxInputContext *input);
int ivf_read_frame(struct AvxInputContext *input_ctx, uint8_t **buffer,
                   size_t *bytes_read, size_t *buffer_size,
                   aom_codec_pts_t *pts);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif  // AOM_COMMON_IVFDEC_H_
