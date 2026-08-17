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
 * \brief Declares functions used in palette search.
 */
#ifndef AOM_AV1_ENCODER_PALETTE_H_
#define AOM_AV1_ENCODER_PALETTE_H_

#include "av1/common/blockd.h"

#ifdef __cplusplus
extern "C" {
#endif

struct AV1_COMP;
struct PICK_MODE_CONTEXT;
struct macroblock;

/*!\cond */
#define AV1_K_MEANS_RENAME(func, dim) func##_dim##dim

void AV1_K_MEANS_RENAME(av1_k_means, 1)(const int16_t *data, int16_t *centroids,
                                        uint8_t *indices, int n, int k,
                                        int max_itr);
void AV1_K_MEANS_RENAME(av1_k_means, 2)(const int16_t *data, int16_t *centroids,
                                        uint8_t *indices, int n, int k,
                                        int max_itr);
/*!\endcond */

/*!\brief Calculates the cluster to which each data point belong.
 *
 * \ingroup palette_mode_search
 * \param[in]    data               The data points whose cluster indices are
 *                                  to be computed. The data layout is
 *                                  NUM_DATA_POINTS X DATA_DIM.
 * \param[in]    centroids          Pointer to the centroids. The data layout
 *                                  is NUM_CENTROIDS X DATA_DIM.
 * \param[in]    indices            Pointer to store the computed indices.
 * \param[in]    n                  Number of data points.
 * \param[in]    k                  Number of clusters.
 * \param[in]    dim                Data dimension.
 *
 * \remark Returns nothing, but saves each data's cluster index in \a indices.
 */
static inline void av1_calc_indices(const int16_t *data,
                                    const int16_t *centroids, uint8_t *indices,
                                    int n, int k, int dim) {
  assert(n > 0);
  assert(k > 0);
  if (dim == 1) {
    av1_calc_indices_dim1(data, centroids, indices, /*total_dist=*/NULL, n, k);
  } else if (dim == 2) {
    av1_calc_indices_dim2(data, centroids, indices, /*total_dist=*/NULL, n, k);
  } else {
    assert(0 && "Untemplated k means dimension");
  }
}

/*!\brief Performs k-means cluster on the data.
 *
 * \ingroup palette_mode_search
 * \param[in]    data               The data points to be clustered. The data
 *                                  layout is NUM_DATA_POINTS X DATA_DIM.
 * \param[in]    centroids          Pointer to store the computed centroids.
 *                                  The data layout is
 *                                  NUM_CENTROIDS X DATA_DIM.
 * \param[in]    indices            Pointer to store the computed indices. For
 *                                  each training data.
 * \param[in]    n                  Number of data points.
 * \param[in]    k                  Number of clusters.
 * \param[in]    dim                Data dimension.
 * \param[in]    max_itr            Maximum number of iterations to run.
 *
 * \remark Returns nothing, but saves each cluster's centroid in centroids and
 * each data's cluster index in \a indices.
 *
 * \attention The output centroids are rounded off to nearest integers.
 */
static inline void av1_k_means(const int16_t *data, int16_t *centroids,
                               uint8_t *indices, int n, int k, int dim,
                               int max_itr) {
  assert(n > 0);
  assert(k > 0);
  if (dim == 1) {
    AV1_K_MEANS_RENAME(av1_k_means, 1)(data, centroids, indices, n, k, max_itr);
  } else if (dim == 2) {
    AV1_K_MEANS_RENAME(av1_k_means, 2)(data, centroids, indices, n, k, max_itr);
  } else {
    assert(0 && "Untemplated k means dimension");
  }
}

/*!\brief Checks what colors are in the color cache.
 *
 * \ingroup palette_mode_search
 * \param[in]    color_cache          A cache of colors.
 * \param[in]    n_cache              Number of colors in the cache.
 * \param[in]    colors               New base colors.
 * \param[in]    n_colors             Number of new colors.
 * \param[in]    cache_color_found    Stores what cached colors are presented in
 *                                    colors.
 * \param[in]    out_cache_colors     Stores what colors are not in the cache.
 *
 * \return Returns the number of colors that are not in cache. In addition,
 * records whether each cache color is presented in colors in cache_color_found,
 * and stores and stores the out of cache colors in out_cache_colors.
 */
int av1_index_color_cache(const uint16_t *color_cache, int n_cache,
                          const uint16_t *colors, int n_colors,
                          uint8_t *cache_color_found, int *out_cache_colors);

/*!\brief Gets the rate cost for each delta-encoding v palette.
 *
 * \ingroup palette_mode_search
 * \param[in]    pmi                  Struct that stores the palette mode info.
 * \param[in]    bit_depth            Pixel bitdepth of the sequence.
 * \param[in]    zero_count           Stores the number of zero deltas.
 * \param[in]    min_bits             Minimum bits for the deltas. Sets to
 *                                    bit_depth - 4.
 *
 * \return Returns the number of bits used to transmit each v palette color
 * delta and assigns zero_count with the number of deltas being 0.
 */
int av1_get_palette_delta_bits_v(const PALETTE_MODE_INFO *const pmi,
                                 int bit_depth, int *zero_count, int *min_bits);

/*!\brief Gets the rate cost for transmitting luma palette color values.
 *
 * \ingroup palette_mode_search
 * \param[in]    pmi                  Struct that stores the palette mode info.
 * \param[in]    color_cache          Color cache presented at the decoder.
 * \param[in]    n_cache              Number of colors in the cache.
 * \param[in]    bit_depth            Pixel bitdepth of the sequence.
 *
 * \return Returns the rate needed to transmit the palette. Note that this does
 * not include the cost of transmitted the color map.
 */
int av1_palette_color_cost_y(const PALETTE_MODE_INFO *const pmi,
                             const uint16_t *color_cache, int n_cache,
                             int bit_depth);

/*!\brief Gets the rate cost for transmitting luma palette chroma values.
 *
 * \ingroup palette_mode_search
 * \param[in]    pmi                  Struct that stores the palette mode info.
 * \param[in]    color_cache          Color cache presented at the decoder.
 * \param[in]    n_cache              Number of colors in the cache.
 * \param[in]    bit_depth            Pixel bitdepth of the sequence.
 *
 * \return Returns the rate needed to transmit the palette. Note that this does
 * not include the cost of transmitted the color map.
 */
int av1_palette_color_cost_uv(const PALETTE_MODE_INFO *const pmi,
                              const uint16_t *color_cache, int n_cache,
                              int bit_depth);

/*!\brief Search for the best palette in the luma plane.
 *
 * \ingroup palette_mode_search
 * \callergraph
 * This function is used in both inter and intra frame coding.
 */
void av1_rd_pick_palette_intra_sby(
    const struct AV1_COMP *cpi, struct macroblock *x, BLOCK_SIZE bsize,
    int dc_mode_cost, MB_MODE_INFO *best_mbmi, uint8_t *best_palette_color_map,
    int64_t *best_rd, int *rate, int *rate_tokenonly, int64_t *distortion,
    uint8_t *skippable, int *beat_best_rd, struct PICK_MODE_CONTEXT *ctx,
    uint8_t *best_blk_skip, uint8_t *tx_type_map);

/*!\brief Search for the best palette in the chroma plane.
 *
 * \ingroup palette_mode_search
 * \callergraph
 * This function is used in both inter and intra frame coding.
 */
void av1_rd_pick_palette_intra_sbuv(const struct AV1_COMP *cpi,
                                    struct macroblock *x, int dc_mode_cost,
                                    uint8_t *best_palette_color_map,
                                    MB_MODE_INFO *const best_mbmi,
                                    int64_t *best_rd, int *rate,
                                    int *rate_tokenonly, int64_t *distortion,
                                    uint8_t *skippable);

/*!\brief Resets palette color map for chroma channels.
 */
void av1_restore_uv_color_map(const struct AV1_COMP *cpi, struct macroblock *x);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_PALETTE_H_
