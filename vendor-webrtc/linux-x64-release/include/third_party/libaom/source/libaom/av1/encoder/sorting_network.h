/*
 * Copyright (c) 2021, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

/*! \file
 * This file contains several utility functions used to sort small arrays with
 * sorting networks.
 *
 * Sorting network is a (potentially branch-less) way to quickly sort small
 * arrays with known size. For more details, consult
 * (https://en.wikipedia.org/wiki/Sorting_network).
 */
#ifndef AOM_AV1_ENCODER_SORTING_NETWORK_H_
#define AOM_AV1_ENCODER_SORTING_NETWORK_H_

#include "aom/aom_integer.h"

#define SWAP(i, j)                                   \
  do {                                               \
    const float maxf = (k[i] >= k[j]) ? k[i] : k[j]; \
    const float minf = (k[i] >= k[j]) ? k[j] : k[i]; \
    const int maxi = (k[i] >= k[j]) ? v[i] : v[j];   \
    const int mini = (k[i] >= k[j]) ? v[j] : v[i];   \
    k[i] = maxf;                                     \
    k[j] = minf;                                     \
    v[i] = maxi;                                     \
    v[j] = mini;                                     \
  } while (0)

/*!\brief Sorts two size-16 arrays of keys and values in descending order of
 * keys.
 *
 * \param[in,out]    k          An length-16 array of float serves as the keys.
 * \param[in,out]    v          An length-16 array of int32 serves as the
 *                              value.
 */
static inline void av1_sort_fi32_16(float k[], int32_t v[]) {
  SWAP(0, 1);
  SWAP(2, 3);
  SWAP(4, 5);
  SWAP(6, 7);
  SWAP(8, 9);
  SWAP(10, 11);
  SWAP(12, 13);
  SWAP(14, 15);
  SWAP(0, 2);
  SWAP(1, 3);
  SWAP(4, 6);
  SWAP(5, 7);
  SWAP(8, 10);
  SWAP(9, 11);
  SWAP(12, 14);
  SWAP(13, 15);
  SWAP(1, 2);
  SWAP(5, 6);
  SWAP(0, 4);
  SWAP(3, 7);
  SWAP(9, 10);
  SWAP(13, 14);
  SWAP(8, 12);
  SWAP(11, 15);
  SWAP(1, 5);
  SWAP(2, 6);
  SWAP(9, 13);
  SWAP(10, 14);
  SWAP(0, 8);
  SWAP(7, 15);
  SWAP(1, 4);
  SWAP(3, 6);
  SWAP(9, 12);
  SWAP(11, 14);
  SWAP(2, 4);
  SWAP(3, 5);
  SWAP(10, 12);
  SWAP(11, 13);
  SWAP(1, 9);
  SWAP(6, 14);
  SWAP(3, 4);
  SWAP(11, 12);
  SWAP(1, 8);
  SWAP(2, 10);
  SWAP(5, 13);
  SWAP(7, 14);
  SWAP(3, 11);
  SWAP(2, 8);
  SWAP(4, 12);
  SWAP(7, 13);
  SWAP(3, 10);
  SWAP(5, 12);
  SWAP(3, 9);
  SWAP(6, 12);
  SWAP(3, 8);
  SWAP(7, 12);
  SWAP(5, 9);
  SWAP(6, 10);
  SWAP(4, 8);
  SWAP(7, 11);
  SWAP(5, 8);
  SWAP(7, 10);
  SWAP(6, 8);
  SWAP(7, 9);
  SWAP(7, 8);
}

/*!\brief Sorts two size-8 arrays of keys and values in descending order of
 * keys.
 *
 * \param[in,out]    k          An length-8 array of float serves as the keys.
 * \param[in,out]    v          An length-8 array of int32 serves as the values.
 */
static inline void av1_sort_fi32_8(float k[], int32_t v[]) {
  SWAP(0, 1);
  SWAP(2, 3);
  SWAP(4, 5);
  SWAP(6, 7);
  SWAP(0, 2);
  SWAP(1, 3);
  SWAP(4, 6);
  SWAP(5, 7);
  SWAP(1, 2);
  SWAP(5, 6);
  SWAP(0, 4);
  SWAP(3, 7);
  SWAP(1, 5);
  SWAP(2, 6);
  SWAP(1, 4);
  SWAP(3, 6);
  SWAP(2, 4);
  SWAP(3, 5);
  SWAP(3, 4);
}
#undef SWAP
#endif  // AOM_AV1_ENCODER_SORTING_NETWORK_H_
