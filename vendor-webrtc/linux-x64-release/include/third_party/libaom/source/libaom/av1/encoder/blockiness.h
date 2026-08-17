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

#ifndef AOM_AV1_ENCODER_BLOCKINESS_H_
#define AOM_AV1_ENCODER_BLOCKINESS_H_

double av1_get_blockiness(const unsigned char *img1, int img1_pitch,
                          const unsigned char *img2, int img2_pitch, int width,
                          int height);

#endif  // AOM_AV1_ENCODER_BLOCKINESS_H_
