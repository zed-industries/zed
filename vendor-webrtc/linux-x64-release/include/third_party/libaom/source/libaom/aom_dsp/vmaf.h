/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_VMAF_H_
#define AOM_AOM_DSP_VMAF_H_

#include <libvmaf/libvmaf.h>
#include <stdbool.h>

#include "aom_scale/yv12config.h"

void aom_init_vmaf_context(VmafContext **vmaf_context, VmafModel *vmaf_model,
                           bool cal_vmaf_neg);
void aom_close_vmaf_context(VmafContext *vmaf_context);

void aom_init_vmaf_model(VmafModel **vmaf_model, const char *model_path);
void aom_close_vmaf_model(VmafModel *vmaf_model);

void aom_calc_vmaf(VmafModel *vmaf_model, const YV12_BUFFER_CONFIG *source,
                   const YV12_BUFFER_CONFIG *distorted, int bit_depth,
                   bool cal_vmaf_neg, double *vmaf);

void aom_read_vmaf_image(VmafContext *vmaf_context,
                         const YV12_BUFFER_CONFIG *source,
                         const YV12_BUFFER_CONFIG *distorted, int bit_depth,
                         int frame_index);

double aom_calc_vmaf_at_index(VmafContext *vmaf_context, VmafModel *vmaf_model,
                              int frame_index);

void aom_flush_vmaf_context(VmafContext *vmaf_context);

#endif  // AOM_AOM_DSP_VMAF_H_
