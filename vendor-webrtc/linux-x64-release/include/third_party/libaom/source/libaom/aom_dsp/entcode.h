/*
 * Copyright (c) 2001-2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_ENTCODE_H_
#define AOM_AOM_DSP_ENTCODE_H_

#include <limits.h>
#include <stddef.h>
#include "aom_dsp/odintrin.h"
#include "aom_dsp/prob.h"

#define EC_PROB_SHIFT 6
#define EC_MIN_PROB 4  // must be <= (1<<EC_PROB_SHIFT)/16

/*OPT: od_ec_window must be at least 32 bits, but if you have fast arithmetic
   on a larger type, you can speed up the decoder by using it here.*/
typedef uint32_t od_ec_window;

/*The size in bits of od_ec_window.*/
#define OD_EC_WINDOW_SIZE ((int)sizeof(od_ec_window) * CHAR_BIT)

/*The resolution of fractional-precision bit usage measurements, i.e.,
   3 => 1/8th bits.*/
#define OD_BITRES (3)

#define OD_ICDF AOM_ICDF

/*See entcode.c for further documentation.*/

OD_WARN_UNUSED_RESULT uint32_t od_ec_tell_frac(uint32_t nbits_total,
                                               uint32_t rng);

#endif  // AOM_AOM_DSP_ENTCODE_H_
