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

#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "av1/common/blockd.h"
#include "av1/encoder/palette.h"
#include "av1/encoder/random.h"

#ifndef AV1_K_MEANS_DIM
#error "This template requires AV1_K_MEANS_DIM to be defined"
#endif

#define RENAME_(x, y) AV1_K_MEANS_RENAME(x, y)
#define RENAME(x) RENAME_(x, AV1_K_MEANS_DIM)
#define K_MEANS_RENAME_C(x, y) x##_dim##y##_c
#define RENAME_C_(x, y) K_MEANS_RENAME_C(x, y)
#define RENAME_C(x) RENAME_C_(x, AV1_K_MEANS_DIM)

// Though we want to compute the smallest L2 norm, in 1 dimension,
// it is equivalent to find the smallest L1 norm and then square it.
// This is preferrable for speed, especially on the SIMD side.
static int RENAME(calc_dist)(const int16_t *p1, const int16_t *p2) {
#if AV1_K_MEANS_DIM == 1
  return abs(p1[0] - p2[0]);
#else
  int dist = 0;
  for (int i = 0; i < AV1_K_MEANS_DIM; ++i) {
    const int diff = p1[i] - p2[i];
    dist += diff * diff;
  }
  return dist;
#endif
}

void RENAME_C(av1_calc_indices)(const int16_t *data, const int16_t *centroids,
                                uint8_t *indices, int64_t *dist, int n, int k) {
  if (dist) {
    *dist = 0;
  }
  for (int i = 0; i < n; ++i) {
    int min_dist = RENAME(calc_dist)(data + i * AV1_K_MEANS_DIM, centroids);
    indices[i] = 0;
    for (int j = 1; j < k; ++j) {
      const int this_dist = RENAME(calc_dist)(data + i * AV1_K_MEANS_DIM,
                                              centroids + j * AV1_K_MEANS_DIM);
      if (this_dist < min_dist) {
        min_dist = this_dist;
        indices[i] = j;
      }
    }
    if (dist) {
#if AV1_K_MEANS_DIM == 1
      *dist += min_dist * min_dist;
#else
      *dist += min_dist;
#endif
    }
  }
}

static void RENAME(calc_centroids)(const int16_t *data, int16_t *centroids,
                                   const uint8_t *indices, int n, int k) {
  int i, j;
  int count[PALETTE_MAX_SIZE] = { 0 };
  int centroids_sum[AV1_K_MEANS_DIM * PALETTE_MAX_SIZE];
  unsigned int rand_state = (unsigned int)data[0];
  assert(n <= 32768);
  memset(centroids_sum, 0, sizeof(centroids_sum[0]) * k * AV1_K_MEANS_DIM);

  for (i = 0; i < n; ++i) {
    const int index = indices[i];
    assert(index < k);
    ++count[index];
    for (j = 0; j < AV1_K_MEANS_DIM; ++j) {
      centroids_sum[index * AV1_K_MEANS_DIM + j] +=
          data[i * AV1_K_MEANS_DIM + j];
    }
  }

  for (i = 0; i < k; ++i) {
    if (count[i] == 0) {
      memcpy(centroids + i * AV1_K_MEANS_DIM,
             data + (lcg_rand16(&rand_state) % n) * AV1_K_MEANS_DIM,
             sizeof(centroids[0]) * AV1_K_MEANS_DIM);
    } else {
      for (j = 0; j < AV1_K_MEANS_DIM; ++j) {
        centroids[i * AV1_K_MEANS_DIM + j] =
            DIVIDE_AND_ROUND(centroids_sum[i * AV1_K_MEANS_DIM + j], count[i]);
      }
    }
  }
}

void RENAME(av1_k_means)(const int16_t *data, int16_t *centroids,
                         uint8_t *indices, int n, int k, int max_itr) {
  int16_t centroids_tmp[AV1_K_MEANS_DIM * PALETTE_MAX_SIZE];
  uint8_t indices_tmp[MAX_PALETTE_BLOCK_WIDTH * MAX_PALETTE_BLOCK_HEIGHT];
  int16_t *meta_centroids[2] = { centroids, centroids_tmp };
  uint8_t *meta_indices[2] = { indices, indices_tmp };
  int i, l = 0, prev_l, best_l = 0;
  int64_t this_dist;

  assert(n <= MAX_PALETTE_BLOCK_WIDTH * MAX_PALETTE_BLOCK_HEIGHT);

#if AV1_K_MEANS_DIM == 1
  av1_calc_indices_dim1(data, centroids, indices, &this_dist, n, k);
#else
  av1_calc_indices_dim2(data, centroids, indices, &this_dist, n, k);
#endif

  for (i = 0; i < max_itr; ++i) {
    const int64_t prev_dist = this_dist;
    prev_l = l;
    l = (l == 1) ? 0 : 1;

    RENAME(calc_centroids)(data, meta_centroids[l], meta_indices[prev_l], n, k);
    if (!memcmp(meta_centroids[l], meta_centroids[prev_l],
                sizeof(centroids[0]) * k * AV1_K_MEANS_DIM)) {
      break;
    }
#if AV1_K_MEANS_DIM == 1
    av1_calc_indices_dim1(data, meta_centroids[l], meta_indices[l], &this_dist,
                          n, k);
#else
    av1_calc_indices_dim2(data, meta_centroids[l], meta_indices[l], &this_dist,
                          n, k);
#endif

    if (this_dist > prev_dist) {
      best_l = prev_l;
      break;
    }
  }
  if (i == max_itr) best_l = l;
  if (best_l != 0) {
    memcpy(centroids, meta_centroids[1],
           sizeof(centroids[0]) * k * AV1_K_MEANS_DIM);
    memcpy(indices, meta_indices[1], sizeof(indices[0]) * n);
  }
}
#undef RENAME_
#undef RENAME
#undef K_MEANS_RENAME_C
#undef RENAME_C_
#undef RENAME_C
