/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_RECENTER_H_
#define AOM_AOM_DSP_RECENTER_H_

#include "config/aom_config.h"

#include "aom/aom_integer.h"

// Inverse recenters a non-negative literal v around a reference r
static inline uint16_t inv_recenter_nonneg(uint16_t r, uint16_t v) {
  if (v > (r << 1))
    return v;
  else if ((v & 1) == 0)
    return (v >> 1) + r;
  else
    return r - ((v + 1) >> 1);
}

// Inverse recenters a non-negative literal v in [0, n-1] around a
// reference r also in [0, n-1]
static inline uint16_t inv_recenter_finite_nonneg(uint16_t n, uint16_t r,
                                                  uint16_t v) {
  if ((r << 1) <= n) {
    return inv_recenter_nonneg(r, v);
  } else {
    return n - 1 - inv_recenter_nonneg(n - 1 - r, v);
  }
}

// Recenters a non-negative literal v around a reference r
static inline uint16_t recenter_nonneg(uint16_t r, uint16_t v) {
  if (v > (r << 1))
    return v;
  else if (v >= r)
    return ((v - r) << 1);
  else
    return ((r - v) << 1) - 1;
}

// Recenters a non-negative literal v in [0, n-1] around a
// reference r also in [0, n-1]
static inline uint16_t recenter_finite_nonneg(uint16_t n, uint16_t r,
                                              uint16_t v) {
  if ((r << 1) <= n) {
    return recenter_nonneg(r, v);
  } else {
    return recenter_nonneg(n - 1 - r, n - 1 - v);
  }
}

#endif  // AOM_AOM_DSP_RECENTER_H_
