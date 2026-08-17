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

#ifndef AOM_AV1_ENCODER_TEMPORAL_FILTER_H_
#define AOM_AV1_ENCODER_TEMPORAL_FILTER_H_

#include <stdbool.h>

#include "aom_util/aom_pthread.h"

#ifdef __cplusplus
extern "C" {
#endif
/*!\cond */
struct AV1_COMP;
struct AV1EncoderConfig;
struct ThreadData;
// TODO(wtc): These two variables are only used in avx2, sse2, neon
// implementations, where the block size is still hard coded to TF_BLOCK_SIZE.
// This should be fixed to align with the c implementation.
#define BH 32
#define BW 32

// Block size used in temporal filtering.
#define TF_BLOCK_SIZE BLOCK_32X32

// Window size for temporal filtering.
#define TF_WINDOW_LENGTH 5

// A constant number, sqrt(pi / 2),  used for noise estimation.
static const double SQRT_PI_BY_2 = 1.25331413732;

// Hyper-parameters used to compute filtering weight. These hyper-parameters can
// be tuned for a better performance.
// 0. A scale factor used in temporal filtering to raise the filter weight from
//    `double` with range [0, 1] to `int` with range [0, 1000].
#define TF_WEIGHT_SCALE 1000
// 1. Weight factor used to balance the weighted-average between window error
//    and block error. The weight is for window error while the weight for block
//    error is always set as 1.
#define TF_WINDOW_BLOCK_BALANCE_WEIGHT 5
// 2. Threshold for using q to adjust the filtering weight. Concretely, when
//    using a small q (high bitrate), we would like to reduce the filtering
//    strength such that more detailed information can be preserved. Hence, when
//    q is smaller than this threshold, we will adjust the filtering weight
//    based on the q-value.
#define TF_Q_DECAY_THRESHOLD 20
// 3. Normalization factor used to normalize the motion search error. Since the
//    motion search error can be large and uncontrollable, we will simply
//    normalize it before using it to compute the filtering weight.
#define TF_SEARCH_ERROR_NORM_WEIGHT 20
// 4. Threshold for using `arnr_strength` to adjust the filtering strength.
//    Concretely, users can use `arnr_strength` arguments to control the
//    strength of temporal filtering. When `arnr_strength` is small enough (
//    i.e., smaller than this threshold), we will adjust the filtering weight
//    based on the strength value.
#define TF_STRENGTH_THRESHOLD 4
// 5. Threshold for using motion search distance to adjust the filtering weight.
//    Concretely, larger motion search vector leads to a higher probability of
//    unreliable search. Hence, we would like to reduce the filtering strength
//    when the distance is large enough. Considering that the distance actually
//    relies on the frame size, this threshold is also a resolution-based
//    threshold. Taking 720p videos as an instance, if this field equals to 0.1,
//    then the actual threshold will be 720 * 0.1 = 72. Similarly, the threshold
//    for 360p videos will be 360 * 0.1 = 36.
#define TF_SEARCH_DISTANCE_THRESHOLD 0.1
// 6. Threshold to identify if the q is in a relative high range.
//    Above this cutoff q, a stronger filtering is applied.
//    For a high q, the quantization throws away more information, and thus a
//    stronger filtering is less likely to distort the encoded quality, while a
//    stronger filtering could reduce bit rates.
//    Ror a low q, more details are expected to be retained. Filtering is thus
//    more conservative.
#define TF_QINDEX_CUTOFF 128

#define NOISE_ESTIMATION_EDGE_THRESHOLD 50

// Sum and SSE source vs filtered frame difference returned by
// temporal filter.
typedef struct {
  int64_t sum;
  int64_t sse;
} FRAME_DIFF;

/*!\endcond */

/*!
 * \brief Parameters related to temporal filtering.
 */
typedef struct {
  /*!
   * Frame buffers used for temporal filtering.
   */
  YV12_BUFFER_CONFIG *frames[MAX_LAG_BUFFERS];
  /*!
   * Number of frames in the frame buffer.
   */
  int num_frames;

  /*!
   * Output filtered frame
   */
  YV12_BUFFER_CONFIG *output_frame;

  /*!
   * Index of the frame to be filtered.
   */
  int filter_frame_idx;
  /*!
   * Whether to accumulate diff for show existing condition check.
   */
  int compute_frame_diff;
  /*!
   * Frame scaling factor.
   */
  struct scale_factors sf;
  /*!
   * Estimated noise levels for each plane in the frame.
   */
  double noise_levels[MAX_MB_PLANE];
  /*!
   * Number of pixels in the temporal filtering block across all planes.
   */
  int num_pels;
  /*!
   * Number of temporal filtering block rows.
   */
  int mb_rows;
  /*!
   * Number of temporal filtering block columns.
   */
  int mb_cols;
  /*!
   * Whether the frame is high-bitdepth or not.
   */
  int is_highbitdepth;
  /*!
   * Quantization factor used in temporal filtering.
   */
  int q_factor;
} TemporalFilterCtx;

/*!
 * buffer count in TEMPORAL_FILTER_INFO
 * Currently we only apply filtering on KEY and ARF after
 * define_gf_group(). Hence, the count is two.
 */
#define TF_INFO_BUF_COUNT 2

/*!
 * \brief Temporal filter info for a gop
 */
typedef struct TEMPORAL_FILTER_INFO {
  /*!
   * A flag indicate whether temporal filter shoud be applied.
   * This flag will stored the result of
   * av1_is_temporal_filter_on()
   */
  int is_temporal_filter_on;
  /*!
   * buffers used for temporal filtering in a GOP
   * index 0 for key frame and index 1 for ARF
   */
  YV12_BUFFER_CONFIG tf_buf[TF_INFO_BUF_COUNT];

  /*!
   * buffers used for temporal filtering for
   * INTNL_ARF_UPDATE
   * Check av1_gop_is_second_arf() for the
   * definition of second_arf in detail
   */
  YV12_BUFFER_CONFIG tf_buf_second_arf;
  /*!
   * whether to show the buffer directly or not.
   */
  FRAME_DIFF frame_diff[TF_INFO_BUF_COUNT];
  /*!
   * the corresponding gf_index for the buffer.
   */
  int tf_buf_gf_index[TF_INFO_BUF_COUNT];
  /*!
   * the display_index offset between next show frame and the frames in the GOP
   */
  int tf_buf_display_index_offset[TF_INFO_BUF_COUNT];
  /*!
   * whether the buf is valid or not.
   */
  int tf_buf_valid[TF_INFO_BUF_COUNT];
} TEMPORAL_FILTER_INFO;

/*!\brief Check whether we should apply temporal filter at all.
 * \param[in]   oxcf           AV1 encoder config
 *
 * \return 1: temporal filter is on 0: temporal is off
 */
int av1_is_temporal_filter_on(const struct AV1EncoderConfig *oxcf);

/*!\brief Allocate buffers for TEMPORAL_FILTER_INFO
 * \param[in,out]   tf_info           Temporal filter info for a gop
 * \param[in,out]   cpi               Top level encoder instance structure
 *
 * \return True on success, false on memory allocation failure.
 */
bool av1_tf_info_alloc(TEMPORAL_FILTER_INFO *tf_info,
                       const struct AV1_COMP *cpi);

/*!\brief Free buffers for TEMPORAL_FILTER_INFO
 * \param[in,out]   tf_info           Temporal filter info for a gop
 */
void av1_tf_info_free(TEMPORAL_FILTER_INFO *tf_info);

/*!\brief Reset validity of tf_buf in TEMPORAL_FILTER_INFO
 * \param[in,out]   tf_info           Temporal filter info for a gop
 */
void av1_tf_info_reset(TEMPORAL_FILTER_INFO *tf_info);

/*!\brief Apply temporal filter for key frame and ARF in a gop
 * \param[in,out]   tf_info           Temporal filter info for a gop
 * \param[in,out]   cpi               Top level encoder instance structure
 * \param[in]       gf_group          GF/ARF group data structure
 */
void av1_tf_info_filtering(TEMPORAL_FILTER_INFO *tf_info, struct AV1_COMP *cpi,
                           const GF_GROUP *gf_group);

/*!\brief Get a filtered buffer from TEMPORAL_FILTER_INFO
 * \param[in,out]   tf_info           Temporal filter info for a gop
 * \param[in]       gf_index          gf_index for the target buffer
 * \param[out]      show_tf_buf       whether the target buffer can be shown
 * directly
 */
YV12_BUFFER_CONFIG *av1_tf_info_get_filtered_buf(TEMPORAL_FILTER_INFO *tf_info,
                                                 int gf_index,
                                                 FRAME_DIFF *frame_diff);

/*!\cond */

// Data related to temporal filtering.
typedef struct {
  // Source vs filtered frame error.
  FRAME_DIFF diff;
  // Pointer to temporary block info used to store state in temporal filtering
  // process.
  MB_MODE_INFO *tmp_mbmi;
  // Pointer to accumulator buffer used in temporal filtering process.
  uint32_t *accum;
  // Pointer to count buffer used in temporal filtering process.
  uint16_t *count;
  // Pointer to predictor used in temporal filtering process.
  uint8_t *pred;
} TemporalFilterData;

// Data related to temporal filter multi-thread synchronization.
typedef struct {
#if CONFIG_MULTITHREAD
  // Mutex lock used for dispatching jobs.
  pthread_mutex_t *mutex_;
#endif  // CONFIG_MULTITHREAD
  // Next temporal filter block row to be filtered.
  int next_tf_row;
  // Initialized to false, set to true by the worker thread that encounters an
  // error in order to abort the processing of other worker threads.
  bool tf_mt_exit;
} AV1TemporalFilterSync;

// Estimates noise level from a given frame using a single plane (Y, U, or V).
// This is an adaptation of the mehtod in the following paper:
// Shen-Chuan Tai, Shih-Ming Yang, "A fast method for image noise
// estimation using Laplacian operator and adaptive edge detection",
// Proc. 3rd International Symposium on Communications, Control and
// Signal Processing, 2008, St Julians, Malta.
// Inputs:
//   frame: Pointer to the frame to estimate noise level from.
//   noise_level: Pointer to store the estimated noise.
//   plane_from: Index of the starting plane used for noise estimation.
//               Commonly, 0 for Y-plane, 1 for U-plane, and 2 for V-plane.
//   plane_to: Index of the end plane used for noise estimation.
//   bit_depth: Actual bit-depth instead of the encoding bit-depth of the frame.
//   edge_thresh: Edge threshold.
void av1_estimate_noise_level(const YV12_BUFFER_CONFIG *frame,
                              double *noise_level, int plane_from, int plane_to,
                              int bit_depth, int edge_thresh);
/*!\endcond */

/*!\brief Does temporal filter for a given macroblock row.
*
* \ingroup src_frame_proc
* \param[in]   cpi                   Top level encoder instance structure
* \param[in]   td                    Pointer to thread data
* \param[in]   mb_row                Macroblock row to be filtered
filtering
*
* \remark Nothing will be returned, but the contents of td->diff will be
modified.
*/
void av1_tf_do_filtering_row(struct AV1_COMP *cpi, struct ThreadData *td,
                             int mb_row);

/*!\brief Performs temporal filtering if needed on a source frame.
 * For example to create a filtered alternate reference frame (ARF)
 *
 * In this function, the lookahead index is different from the 0-based
 * real index. For example, if we want to filter the first frame in the
 * pre-fetched buffer `cpi->lookahead`, the lookahead index will be -1 instead
 * of 0. More concretely, 0 indicates the first LOOKAHEAD frame, which is the
 * second frame in the pre-fetched buffer. Another example: if we want to filter
 * the 17-th frame, which is an ARF, the lookahead index is 15 instead of 16.
 * Futhermore, negative number is used for key frame in one-pass mode, where key
 * frame is filtered with the frames before it instead of after it. For example,
 * -15 means to filter the 17-th frame, which is a key frame in one-pass mode.
 *
 * \ingroup src_frame_proc
 * \param[in]      cpi                        Top level encoder instance
 *                                            structure
 * \param[in]      filter_frame_lookahead_idx The index of the
 *                                            to-filter frame in the lookahead
 *                                            buffer cpi->lookahead.
 * \param[in]      gf_frame_index             Index of GOP
 * \param[in,out]  frame_diff                 structure of sse and sum of the
 *                                            filtered frame.
 * \param[out]     output_frame               Ouput filtered frame.
 */
void av1_temporal_filter(struct AV1_COMP *cpi,
                         const int filter_frame_lookahead_idx,
                         int gf_frame_index, FRAME_DIFF *frame_diff,
                         YV12_BUFFER_CONFIG *output_frame);

/*!\brief Check whether a filtered frame can be show directly
 *
 * This function will use the filtered frame's sse and current q index
 * to make decision.
 *
 * \ingroup src_frame_proc
 * \param[in]  frame        filtered frame's buffer
 * \param[in]  frame_diff   structure of sse and sum of the
 *                          filtered frame.
 * \param[in]  q_index      q_index used for this frame
 * \param[in]  bit_depth    bit depth
 * \return     return 1 if this frame can be shown directly, otherwise
 *             return 0
 */
int av1_check_show_filtered_frame(const YV12_BUFFER_CONFIG *frame,
                                  const FRAME_DIFF *frame_diff, int q_index,
                                  aom_bit_depth_t bit_depth);

/*!\cond */
// Allocates memory for members of TemporalFilterData.
// Inputs:
//   tf_data: Pointer to the structure containing temporal filter related data.
//   num_pels: Number of pixels in the block across all planes.
//   is_high_bitdepth: Whether the frame is high-bitdepth or not.
// Returns:
//   True if allocation is successful and false otherwise.
static inline bool tf_alloc_and_reset_data(TemporalFilterData *tf_data,
                                           int num_pels, int is_high_bitdepth) {
  tf_data->tmp_mbmi = (MB_MODE_INFO *)aom_calloc(1, sizeof(*tf_data->tmp_mbmi));
  tf_data->accum =
      (uint32_t *)aom_memalign(16, num_pels * sizeof(*tf_data->accum));
  tf_data->count =
      (uint16_t *)aom_memalign(16, num_pels * sizeof(*tf_data->count));
  if (is_high_bitdepth)
    tf_data->pred = CONVERT_TO_BYTEPTR(
        aom_memalign(32, num_pels * 2 * sizeof(*tf_data->pred)));
  else
    tf_data->pred =
        (uint8_t *)aom_memalign(32, num_pels * sizeof(*tf_data->pred));
  // In case of an allocation failure, other successfully allocated buffers will
  // be freed by the tf_dealloc_data() call in encoder_destroy().
  if (!(tf_data->tmp_mbmi && tf_data->accum && tf_data->count && tf_data->pred))
    return false;
  memset(&tf_data->diff, 0, sizeof(tf_data->diff));
  return true;
}

// Setup macroblockd params for temporal filtering process.
// Inputs:
//   mbd: Pointer to the block for filtering.
//   tf_data: Pointer to the structure containing temporal filter related data.
//   scale: Scaling factor.
// Returns:
//   Nothing will be returned. Contents of mbd will be modified.
static inline void tf_setup_macroblockd(MACROBLOCKD *mbd,
                                        TemporalFilterData *tf_data,
                                        const struct scale_factors *scale) {
  mbd->block_ref_scale_factors[0] = scale;
  mbd->block_ref_scale_factors[1] = scale;
  mbd->mi = &tf_data->tmp_mbmi;
  mbd->mi[0]->motion_mode = SIMPLE_TRANSLATION;
}

// Deallocates the memory allocated for members of TemporalFilterData.
// Inputs:
//   tf_data: Pointer to the structure containing temporal filter related data.
//   is_high_bitdepth: Whether the frame is high-bitdepth or not.
// Returns:
//   Nothing will be returned.
static inline void tf_dealloc_data(TemporalFilterData *tf_data,
                                   int is_high_bitdepth) {
  if (is_high_bitdepth)
    tf_data->pred = (uint8_t *)CONVERT_TO_SHORTPTR(tf_data->pred);
  aom_free(tf_data->tmp_mbmi);
  tf_data->tmp_mbmi = NULL;
  aom_free(tf_data->accum);
  tf_data->accum = NULL;
  aom_free(tf_data->count);
  tf_data->count = NULL;
  aom_free(tf_data->pred);
  tf_data->pred = NULL;
}

// Saves the state prior to temporal filter process.
// Inputs:
//   mbd: Pointer to the block for filtering.
//   input_mbmi: Backup block info to save input state.
//   input_buffer: Backup buffer pointer to save input state.
//   num_planes: Number of planes.
// Returns:
//   Nothing will be returned. Contents of input_mbmi and input_buffer will be
//   modified.
static inline void tf_save_state(MACROBLOCKD *mbd, MB_MODE_INFO ***input_mbmi,
                                 uint8_t **input_buffer, int num_planes) {
  for (int i = 0; i < num_planes; i++) {
    input_buffer[i] = mbd->plane[i].pre[0].buf;
  }
  *input_mbmi = mbd->mi;
}

// Restores the initial state after temporal filter process.
// Inputs:
//   mbd: Pointer to the block for filtering.
//   input_mbmi: Backup block info from where input state is restored.
//   input_buffer: Backup buffer pointer from where input state is restored.
//   num_planes: Number of planes.
// Returns:
//   Nothing will be returned. Contents of mbd will be modified.
static inline void tf_restore_state(MACROBLOCKD *mbd, MB_MODE_INFO **input_mbmi,
                                    uint8_t **input_buffer, int num_planes) {
  for (int i = 0; i < num_planes; i++) {
    mbd->plane[i].pre[0].buf = input_buffer[i];
  }
  mbd->mi = input_mbmi;
}

/*!\endcond */
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_TEMPORAL_FILTER_H_
