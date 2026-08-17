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
 * \brief Describes film grain parameters
 *
 */
#ifndef AOM_AOM_DSP_GRAIN_PARAMS_H_
#define AOM_AOM_DSP_GRAIN_PARAMS_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <string.h>

#include "config/aom_config.h"

/*!\brief Structure containing film grain synthesis parameters for a frame
 *
 * This structure contains input parameters for film grain synthesis
 */
typedef struct {
  // This structure is compared element-by-element in the function
  // aom_check_grain_params_equiv: this function must be updated if any changes
  // are made to this structure.
  int apply_grain;

  int update_parameters;

  // 8 bit values
  int scaling_points_y[14][2];
  int num_y_points;  // value: 0..14

  // 8 bit values
  int scaling_points_cb[10][2];
  int num_cb_points;  // value: 0..10

  // 8 bit values
  int scaling_points_cr[10][2];
  int num_cr_points;  // value: 0..10

  int scaling_shift;  // values : 8..11

  int ar_coeff_lag;  // values:  0..3

  // 8 bit values
  int ar_coeffs_y[24];
  int ar_coeffs_cb[25];
  int ar_coeffs_cr[25];

  // Shift value: AR coeffs range
  // 6: [-2, 2)
  // 7: [-1, 1)
  // 8: [-0.5, 0.5)
  // 9: [-0.25, 0.25)
  int ar_coeff_shift;  // values : 6..9

  int cb_mult;       // 8 bits
  int cb_luma_mult;  // 8 bits
  int cb_offset;     // 9 bits

  int cr_mult;       // 8 bits
  int cr_luma_mult;  // 8 bits
  int cr_offset;     // 9 bits

  int overlap_flag;

  int clip_to_restricted_range;

  unsigned int bit_depth;  // video bit depth

  int chroma_scaling_from_luma;

  int grain_scale_shift;

  uint16_t random_seed;
  // This structure is compared element-by-element in the function
  // aom_check_grain_params_equiv: this function must be updated if any changes
  // are made to this structure.
} aom_film_grain_t;

/*!\brief Check if two film grain parameters structs are equivalent
 *
 * Check if two film grain parameters are equal, except for the
 * update_parameters and random_seed elements which are ignored.
 *
 * \param[in]    pa               The first set of parameters to compare
 * \param[in]    pb               The second set of parameters to compare
 * \return       Returns 1 if the params are equivalent, 0 otherwise
 */
static inline int aom_check_grain_params_equiv(
    const aom_film_grain_t *const pa, const aom_film_grain_t *const pb) {
  if (pa->apply_grain != pb->apply_grain) return 0;
  // Don't compare update_parameters

  if (pa->num_y_points != pb->num_y_points) return 0;
  if (memcmp(pa->scaling_points_y, pb->scaling_points_y,
             pa->num_y_points * 2 * sizeof(*pa->scaling_points_y)) != 0)
    return 0;

  if (pa->num_cb_points != pb->num_cb_points) return 0;
  if (memcmp(pa->scaling_points_cb, pb->scaling_points_cb,
             pa->num_cb_points * 2 * sizeof(*pa->scaling_points_cb)) != 0)
    return 0;

  if (pa->num_cr_points != pb->num_cr_points) return 0;
  if (memcmp(pa->scaling_points_cr, pb->scaling_points_cr,
             pa->num_cr_points * 2 * sizeof(*pa->scaling_points_cr)) != 0)
    return 0;

  if (pa->scaling_shift != pb->scaling_shift) return 0;
  if (pa->ar_coeff_lag != pb->ar_coeff_lag) return 0;

  const int num_pos = 2 * pa->ar_coeff_lag * (pa->ar_coeff_lag + 1);
  if (memcmp(pa->ar_coeffs_y, pb->ar_coeffs_y,
             num_pos * sizeof(*pa->ar_coeffs_y)) != 0)
    return 0;
  if (memcmp(pa->ar_coeffs_cb, pb->ar_coeffs_cb,
             num_pos * sizeof(*pa->ar_coeffs_cb)) != 0)
    return 0;
  if (memcmp(pa->ar_coeffs_cr, pb->ar_coeffs_cr,
             num_pos * sizeof(*pa->ar_coeffs_cr)) != 0)
    return 0;

  if (pa->ar_coeff_shift != pb->ar_coeff_shift) return 0;

  if (pa->cb_mult != pb->cb_mult) return 0;
  if (pa->cb_luma_mult != pb->cb_luma_mult) return 0;
  if (pa->cb_offset != pb->cb_offset) return 0;

  if (pa->cr_mult != pb->cr_mult) return 0;
  if (pa->cr_luma_mult != pb->cr_luma_mult) return 0;
  if (pa->cr_offset != pb->cr_offset) return 0;

  if (pa->overlap_flag != pb->overlap_flag) return 0;
  if (pa->clip_to_restricted_range != pb->clip_to_restricted_range) return 0;
  if (pa->bit_depth != pb->bit_depth) return 0;
  if (pa->chroma_scaling_from_luma != pb->chroma_scaling_from_luma) return 0;
  if (pa->grain_scale_shift != pb->grain_scale_shift) return 0;

  return 1;
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_DSP_GRAIN_PARAMS_H_
