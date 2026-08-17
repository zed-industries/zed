/*
 * Copyright (c) 2024, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_COMMON_DEBUGMODES_H_
#define AOM_AV1_COMMON_DEBUGMODES_H_

#include "av1/common/av1_common_int.h"
#include "av1/common/blockd.h"
#include "av1/common/enums.h"

void av1_print_modes_and_motion_vectors(AV1_COMMON *cm, const char *file);
void av1_print_uncompressed_frame_header(const uint8_t *data, int size,
                                         const char *filename);
void av1_print_frame_contexts(const FRAME_CONTEXT *fc, const char *filename);

#endif  // AOM_AV1_COMMON_DEBUGMODES_H_
