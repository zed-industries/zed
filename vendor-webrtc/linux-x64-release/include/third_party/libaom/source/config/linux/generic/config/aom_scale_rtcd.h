/*
 * Copyright (c) 2025, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// This file is generated. Do not edit.
#ifndef AOM_SCALE_RTCD_H_
#define AOM_SCALE_RTCD_H_

#ifdef RTCD_C
#define RTCD_EXTERN
#else
#define RTCD_EXTERN extern
#endif

#include <stdbool.h>

struct yv12_buffer_config;

#ifdef __cplusplus
extern "C" {
#endif

void aom_extend_frame_borders_c(struct yv12_buffer_config *ybf, int num_planes);
#define aom_extend_frame_borders aom_extend_frame_borders_c

void aom_extend_frame_borders_plane_row_c(const struct yv12_buffer_config *ybf, int plane, int v_start, int v_end);
#define aom_extend_frame_borders_plane_row aom_extend_frame_borders_plane_row_c

void aom_yv12_copy_frame_c(const struct yv12_buffer_config *src_bc, struct yv12_buffer_config *dst_bc, const int num_planes);
#define aom_yv12_copy_frame aom_yv12_copy_frame_c

void aom_yv12_copy_u_c(const struct yv12_buffer_config *src_bc, struct yv12_buffer_config *dst_bc, int use_crop);
#define aom_yv12_copy_u aom_yv12_copy_u_c

void aom_yv12_copy_v_c(const struct yv12_buffer_config *src_bc, struct yv12_buffer_config *dst_bc, int use_crop);
#define aom_yv12_copy_v aom_yv12_copy_v_c

void aom_yv12_copy_y_c(const struct yv12_buffer_config *src_ybc, struct yv12_buffer_config *dst_ybc, int use_crop);
#define aom_yv12_copy_y aom_yv12_copy_y_c

void aom_yv12_extend_frame_borders_c(struct yv12_buffer_config *ybf, const int num_planes);
#define aom_yv12_extend_frame_borders aom_yv12_extend_frame_borders_c

void aom_yv12_partial_coloc_copy_u_c(const struct yv12_buffer_config *src_bc, struct yv12_buffer_config *dst_bc, int hstart, int hend, int vstart, int vend);
#define aom_yv12_partial_coloc_copy_u aom_yv12_partial_coloc_copy_u_c

void aom_yv12_partial_coloc_copy_v_c(const struct yv12_buffer_config *src_bc, struct yv12_buffer_config *dst_bc, int hstart, int hend, int vstart, int vend);
#define aom_yv12_partial_coloc_copy_v aom_yv12_partial_coloc_copy_v_c

void aom_yv12_partial_coloc_copy_y_c(const struct yv12_buffer_config *src_ybc, struct yv12_buffer_config *dst_ybc, int hstart, int hend, int vstart, int vend);
#define aom_yv12_partial_coloc_copy_y aom_yv12_partial_coloc_copy_y_c

void aom_yv12_partial_copy_u_c(const struct yv12_buffer_config *src_bc, int hstart1, int hend1, int vstart1, int vend1, struct yv12_buffer_config *dst_bc, int hstart2, int vstart2);
#define aom_yv12_partial_copy_u aom_yv12_partial_copy_u_c

void aom_yv12_partial_copy_v_c(const struct yv12_buffer_config *src_bc, int hstart1, int hend1, int vstart1, int vend1, struct yv12_buffer_config *dst_bc, int hstart2, int vstart2);
#define aom_yv12_partial_copy_v aom_yv12_partial_copy_v_c

void aom_yv12_partial_copy_y_c(const struct yv12_buffer_config *src_ybc, int hstart1, int hend1, int vstart1, int vend1, struct yv12_buffer_config *dst_ybc, int hstart2, int vstart2);
#define aom_yv12_partial_copy_y aom_yv12_partial_copy_y_c

int aom_yv12_realloc_with_new_border_c(struct yv12_buffer_config *ybf, int new_border, int byte_alignment, bool alloc_pyramid, int num_planes);
#define aom_yv12_realloc_with_new_border aom_yv12_realloc_with_new_border_c

void aom_scale_rtcd(void);

#include "config/aom_config.h"

#ifdef RTCD_C
static void setup_rtcd_internal(void)
{
}
#endif

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_SCALE_RTCD_H_
