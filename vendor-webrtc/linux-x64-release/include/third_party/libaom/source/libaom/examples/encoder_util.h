/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Utility functions used by encoder binaries.

#ifndef AOM_EXAMPLES_ENCODER_UTIL_H_
#define AOM_EXAMPLES_ENCODER_UTIL_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "aom/aom_image.h"

// Returns mismatch location (?loc[0],?loc[1]) and the values at that location
// in img1 (?loc[2]) and img2 (?loc[3]).
void aom_find_mismatch_high(const aom_image_t *const img1,
                            const aom_image_t *const img2, int yloc[4],
                            int uloc[4], int vloc[4]);

void aom_find_mismatch(const aom_image_t *const img1,
                       const aom_image_t *const img2, int yloc[4], int uloc[4],
                       int vloc[4]);

// Returns 1 if the two images match.
int aom_compare_img(const aom_image_t *const img1,
                    const aom_image_t *const img2);

#ifdef __cplusplus
}
#endif
#endif  // AOM_EXAMPLES_ENCODER_UTIL_H_
