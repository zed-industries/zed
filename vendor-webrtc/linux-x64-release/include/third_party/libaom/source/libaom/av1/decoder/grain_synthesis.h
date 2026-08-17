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

/*!\file
 * \brief Describes film grain synthesis
 *
 */
#ifndef AOM_AV1_DECODER_GRAIN_SYNTHESIS_H_
#define AOM_AV1_DECODER_GRAIN_SYNTHESIS_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

#include "aom_dsp/grain_params.h"
#include "aom/aom_image.h"

/*!\brief Add film grain
 *
 * Add film grain to an image
 *
 * Returns 0 for success, -1 for failure
 *
 * \param[in]    grain_params     Grain parameters
 * \param[in]    src              Source image
 * \param[out]   dst              Resulting image with grain
 */
int av1_add_film_grain(const aom_film_grain_t *grain_params,
                       const aom_image_t *src, aom_image_t *dst);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_DECODER_GRAIN_SYNTHESIS_H_
