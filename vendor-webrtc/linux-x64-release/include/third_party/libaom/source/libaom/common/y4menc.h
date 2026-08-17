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

#ifndef AOM_COMMON_Y4MENC_H_
#define AOM_COMMON_Y4MENC_H_

#include "aom/aom_decoder.h"
#include "common/md5_utils.h"
#include "common/tools_common.h"

#ifdef __cplusplus
extern "C" {
#endif

#define Y4M_BUFFER_SIZE 256

int y4m_write_file_header(char *buf, size_t len, int width, int height,
                          const struct AvxRational *framerate, int monochrome,
                          aom_chroma_sample_position_t csp, aom_img_fmt_t fmt,
                          unsigned int bit_depth, aom_color_range_t range);
int y4m_write_frame_header(char *buf, size_t len);
void y4m_write_image_file(const aom_image_t *img, const int *planes,
                          FILE *file);
void y4m_update_image_md5(const aom_image_t *img, const int *planes,
                          MD5Context *md5);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_COMMON_Y4MENC_H_
