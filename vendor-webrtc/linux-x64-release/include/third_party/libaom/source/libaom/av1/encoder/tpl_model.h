/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_TPL_MODEL_H_
#define AOM_AV1_ENCODER_TPL_MODEL_H_

#ifdef __cplusplus
extern "C" {
#endif

/*!\cond */

struct AV1_PRIMARY;
struct AV1_COMP;
struct AV1_SEQ_CODING_TOOLS;
struct EncodeFrameParams;
struct EncodeFrameInput;
struct GF_GROUP;
struct ThreadData;
struct TPL_INFO;

#include "config/aom_config.h"

#include "aom_scale/yv12config.h"
#include "aom_util/aom_pthread.h"

#include "av1/common/mv.h"
#include "av1/common/scale.h"
#include "av1/encoder/block.h"
#include "av1/encoder/lookahead.h"
#include "av1/encoder/ratectrl.h"

static inline BLOCK_SIZE convert_length_to_bsize(int length) {
  switch (length) {
    case 64: return BLOCK_64X64;
    case 32: return BLOCK_32X32;
    case 16: return BLOCK_16X16;
    case 8: return BLOCK_8X8;
    case 4: return BLOCK_4X4;
    default:
      assert(0 && "Invalid block size for tpl model");
      return BLOCK_16X16;
  }
}

typedef struct AV1TplRowMultiThreadSync {
#if CONFIG_MULTITHREAD
  // Synchronization objects for top-right dependency.
  pthread_mutex_t *mutex_;
  pthread_cond_t *cond_;
#endif
  // Buffer to store the macroblock whose encoding is complete.
  // num_finished_cols[i] stores the number of macroblocks which finished
  // encoding in the ith macroblock row.
  int *num_finished_cols;
  // Number of extra macroblocks of the top row to be complete for encoding
  // of the current macroblock to start. A value of 1 indicates top-right
  // dependency.
  int sync_range;
  // Number of macroblock rows.
  int rows;
  // Number of threads processing the current tile.
  int num_threads_working;
} AV1TplRowMultiThreadSync;

typedef struct AV1TplRowMultiThreadInfo {
  // Initialized to false, set to true by the worker thread that encounters an
  // error in order to abort the processing of other worker threads.
  bool tpl_mt_exit;
#if CONFIG_MULTITHREAD
  // Mutex lock object used for error handling.
  pthread_mutex_t *mutex_;
#endif
  // Row synchronization related function pointers.
  void (*sync_read_ptr)(AV1TplRowMultiThreadSync *tpl_mt_sync, int r, int c);
  void (*sync_write_ptr)(AV1TplRowMultiThreadSync *tpl_mt_sync, int r, int c,
                         int cols);
} AV1TplRowMultiThreadInfo;

// TODO(jingning): This needs to be cleaned up next.

// TPL stats buffers are prepared for every frame in the GOP,
// including (internal) overlays and (internal) arfs.
// In addition, frames in the lookahead that are outside of the GOP
// are also used.
// Thus it should use
// (gop_length) + (# overlays) + (MAX_LAG_BUFFERS - gop_len) =
// MAX_LAG_BUFFERS + (# overlays)
// 2 * MAX_LAG_BUFFERS is therefore a safe estimate.
// TODO(bohanli): test setting it to 1.5 * MAX_LAG_BUFFER
#define MAX_TPL_FRAME_IDX (2 * MAX_LAG_BUFFERS)
// The first REF_FRAMES + 1 buffers are reserved.
// tpl_data->tpl_frame starts after REF_FRAMES + 1
#define MAX_LENGTH_TPL_FRAME_STATS (MAX_TPL_FRAME_IDX + REF_FRAMES + 1)
#define TPL_DEP_COST_SCALE_LOG2 4

#define TPL_EPSILON 0.0000001

typedef struct TplTxfmStats {
  int ready;                  // Whether abs_coeff_mean is ready
  double abs_coeff_sum[256];  // Assume we are using 16x16 transform block
  double abs_coeff_mean[256];
  int txfm_block_count;
  int coeff_num;
} TplTxfmStats;

typedef struct {
  uint8_t *predictor8;
  int16_t *src_diff;
  tran_low_t *coeff;
  tran_low_t *qcoeff;
  tran_low_t *dqcoeff;
} TplBuffers;

typedef struct TplDepStats {
  int64_t srcrf_sse;
  int64_t srcrf_dist;
  int64_t recrf_sse;
  int64_t recrf_dist;
  int64_t intra_sse;
  int64_t intra_dist;
  int64_t cmp_recrf_dist[2];
  int64_t mc_dep_rate;
  int64_t mc_dep_dist;
  int64_t pred_error[INTER_REFS_PER_FRAME];
  int32_t intra_cost;
  int32_t inter_cost;
  int32_t srcrf_rate;
  int32_t recrf_rate;
  int32_t intra_rate;
  int32_t cmp_recrf_rate[2];
  int_mv mv[INTER_REFS_PER_FRAME];
  int8_t ref_frame_index[2];
} TplDepStats;

typedef struct TplDepFrame {
  uint8_t is_valid;
  TplDepStats *tpl_stats_ptr;
  const YV12_BUFFER_CONFIG *gf_picture;
  YV12_BUFFER_CONFIG *rec_picture;
  int ref_map_index[REF_FRAMES];
  int stride;
  int width;
  int height;
  int mi_rows;
  int mi_cols;
  int base_rdmult;
  uint32_t frame_display_index;
  // When set, SAD metric is used for intra and inter mode decision.
  int use_pred_sad;
} TplDepFrame;

/*!\endcond */
/*!
 * \brief Params related to temporal dependency model.
 */
typedef struct TplParams {
  /*!
   * Whether the tpl stats is ready.
   */
  int ready;

  /*!
   * Block granularity of tpl score storage.
   */
  uint8_t tpl_stats_block_mis_log2;

  /*!
   * Tpl motion estimation block 1d size. tpl_bsize_1d >= 16.
   */
  uint8_t tpl_bsize_1d;

  /*!
   * Buffer to store the frame level tpl information for each frame in a gf
   * group. tpl_stats_buffer[i] stores the tpl information of ith frame in a gf
   * group
   */
  TplDepFrame tpl_stats_buffer[MAX_LENGTH_TPL_FRAME_STATS];

  /*!
   * Buffer to store tpl stats at block granularity.
   * tpl_stats_pool[i][j] stores the tpl stats of jth block of ith frame in a gf
   * group.
   */
  TplDepStats *tpl_stats_pool[MAX_LAG_BUFFERS];

  /*!
   * Pointer to the buffer which stores tpl transform stats per frame.
   * txfm_stats_list[i] stores the TplTxfmStats of the ith frame in a gf group.
   * Memory is allocated dynamically for MAX_LENGTH_TPL_FRAME_STATS frames when
   * tpl is enabled.
   */
  TplTxfmStats *txfm_stats_list;

  /*!
   * Buffer to store tpl reconstructed frame.
   * tpl_rec_pool[i] stores the reconstructed frame of ith frame in a gf group.
   */
  YV12_BUFFER_CONFIG tpl_rec_pool[MAX_LAG_BUFFERS];

  /*!
   * Pointer to tpl_stats_buffer.
   */
  TplDepFrame *tpl_frame;

  /*!
   * Scale factors for the current frame.
   */
  struct scale_factors sf;

  /*!
   * GF group index of the current frame.
   */
  int frame_idx;

  /*!
   * Array of pointers to the frame buffers holding the source frame.
   * src_ref_frame[i] stores the pointer to the source frame of the ith
   * reference frame type.
   */
  const YV12_BUFFER_CONFIG *src_ref_frame[INTER_REFS_PER_FRAME];

  /*!
   * Array of pointers to the frame buffers holding the tpl reconstructed frame.
   * ref_frame[i] stores the pointer to the tpl reconstructed frame of the ith
   * reference frame type.
   */
  const YV12_BUFFER_CONFIG *ref_frame[INTER_REFS_PER_FRAME];

  /*!
   * Parameters related to synchronization for top-right dependency in row based
   * multi-threading of tpl
   */
  AV1TplRowMultiThreadSync tpl_mt_sync;

  /*!
   * Frame border for tpl frame.
   */
  int border_in_pixels;

  /*!
   * Factor to adjust r0 if TPL uses a subset of frames in the gf group.
   */
  double r0_adjust_factor;
} TplParams;

#if CONFIG_BITRATE_ACCURACY || CONFIG_RATECTRL_LOG
#define VBR_RC_INFO_MAX_FRAMES 500
#endif  //  CONFIG_BITRATE_ACCURACY || CONFIG_RATECTRL_LOG

#if CONFIG_BITRATE_ACCURACY

/*!
 * \brief This structure stores information needed for bitrate accuracy
 * experiment.
 */
typedef struct {
  int ready;
  double total_bit_budget;  // The total bit budget of the entire video
  int show_frame_count;     // Number of show frames in the entire video

  int gop_showframe_count;  // The number of show frames in the current gop
  double gop_bit_budget;    // The bitbudget for the current gop
  double scale_factors[FRAME_UPDATE_TYPES];     // Scale factors to improve the
                                                // budget estimation
  double mv_scale_factors[FRAME_UPDATE_TYPES];  // Scale factors to improve
                                                // MV entropy estimation

  // === Below this line are GOP related data that will be updated per GOP ===
  int base_q_index;  // Stores the base q index.
  int q_index_list_ready;
  int q_index_list[VBR_RC_INFO_MAX_FRAMES];  // q indices for the current
                                             // GOP

  // Array to store qstep_ratio for each frame in a GOP
  double qstep_ratio_list[VBR_RC_INFO_MAX_FRAMES];

#if CONFIG_THREE_PASS
  TplTxfmStats txfm_stats_list[VBR_RC_INFO_MAX_FRAMES];
  FRAME_UPDATE_TYPE update_type_list[VBR_RC_INFO_MAX_FRAMES];
  int gop_start_idx_list[VBR_RC_INFO_MAX_FRAMES];
  int gop_length_list[VBR_RC_INFO_MAX_FRAMES];
  int cur_gop_idx;
  int total_frame_count;
  int gop_count;
#endif  // CONFIG_THREE_PASS
} VBR_RATECTRL_INFO;

static inline void vbr_rc_reset_gop_data(VBR_RATECTRL_INFO *vbr_rc_info) {
  vbr_rc_info->q_index_list_ready = 0;
  av1_zero(vbr_rc_info->q_index_list);
}

void av1_vbr_rc_init(VBR_RATECTRL_INFO *vbr_rc_info, double total_bit_budget,
                     int show_frame_count);

int av1_vbr_rc_frame_coding_idx(const VBR_RATECTRL_INFO *vbr_rc_info,
                                int gf_frame_index);

void av1_vbr_rc_append_tpl_info(VBR_RATECTRL_INFO *vbr_rc_info,
                                const struct TPL_INFO *tpl_info);

void av1_vbr_rc_set_gop_bit_budget(VBR_RATECTRL_INFO *vbr_rc_info,
                                   int gop_showframe_count);

void av1_vbr_rc_compute_q_indices(int base_q_index, int frame_count,
                                  const double *qstep_ratio_list,
                                  aom_bit_depth_t bit_depth, int *q_index_list);

/*!\brief Update q_index_list in vbr_rc_info based on tpl stats
 *
 * \param[out]      vbr_rc_info    Rate control info for BITRATE_ACCURACY
 *                                 experiment
 * \param[in]       tpl_data       TPL struct
 * \param[in]       gf_group       GOP struct
 * \param[in]       bit_depth      bit depth
 */
void av1_vbr_rc_update_q_index_list(VBR_RATECTRL_INFO *vbr_rc_info,
                                    const TplParams *tpl_data,
                                    const struct GF_GROUP *gf_group,
                                    aom_bit_depth_t bit_depth);
/*
 *!\brief Compute the number of bits needed to encode a GOP
 *
 * \param[in]    base_q_index              base layer q_index
 * \param[in]    bit_depth                 bit depth
 * \param[in]    update_type_scale_factors array of scale factors for each
 *                                         update_type
 * \param[in]    frame_count               size of update_type_list,
 *                                         qstep_ratio_list stats_list,
 *                                         q_index_list and
 *                                         estimated_bitrate_byframe
 * \param[in]    update_type_list          array of update_type, one per frame
 * \param[in]    qstep_ratio_list          array of qstep_ratio, one per frame
 * \param[in]    stats_list                array of transform stats, one per
 *                                         frame
 * \param[out]   q_index_list              array of q_index, one per frame
 * \param[out]   estimated_bitrate_byframe array to keep track of frame
 *                                         bitrate
 *
 * \return The estimated GOP bitrate.
 *
 */
double av1_vbr_rc_info_estimate_gop_bitrate(
    int base_q_index, aom_bit_depth_t bit_depth,
    const double *update_type_scale_factors, int frame_count,
    const FRAME_UPDATE_TYPE *update_type_list, const double *qstep_ratio_list,
    const TplTxfmStats *stats_list, int *q_index_list,
    double *estimated_bitrate_byframe);

/*!\brief Estimate the optimal base q index for a GOP.
 *
 * This function uses a binary search to find base layer q index to
 * achieve the specified bit budget.
 *
 * \param[in]    bit_budget        target bit budget
 * \param[in]    bit_depth         bit depth
 * \param[in]    update_type_scale_factors array of scale factors for each
 *                                 update_type
 * \param[in]    frame_count       size of update_type_list, qstep_ratio_list
 *                                 stats_list, q_index_list and
 *                                 estimated_bitrate_byframe
 * \param[in]    update_type_list  array of update_type, one per frame
 * \param[in]    qstep_ratio_list  array of qstep_ratio, one per frame
 * \param[in]    stats_list        array of transform stats, one per frame
 * \param[out]   q_index_list      array of q_index, one per frame
 * \param[out]   estimated_bitrate_byframe Array to keep track of frame
 * bitrate
 *
 * \return Returns the optimal base q index to use.
 */
int av1_vbr_rc_info_estimate_base_q(
    double bit_budget, aom_bit_depth_t bit_depth,
    const double *update_type_scale_factors, int frame_count,
    const FRAME_UPDATE_TYPE *update_type_list, const double *qstep_ratio_list,
    const TplTxfmStats *stats_list, int *q_index_list,
    double *estimated_bitrate_byframe);

#endif  // CONFIG_BITRATE_ACCURACY

#if CONFIG_RD_COMMAND
typedef enum {
  RD_OPTION_NONE,
  RD_OPTION_SET_Q,
  RD_OPTION_SET_Q_RDMULT
} RD_OPTION;

typedef struct RD_COMMAND {
  RD_OPTION option_ls[MAX_LENGTH_TPL_FRAME_STATS];
  int q_index_ls[MAX_LENGTH_TPL_FRAME_STATS];
  int rdmult_ls[MAX_LENGTH_TPL_FRAME_STATS];
  int frame_count;
  int frame_index;
} RD_COMMAND;

void av1_read_rd_command(const char *filepath, RD_COMMAND *rd_command);
#endif  // CONFIG_RD_COMMAND

/*!\brief Allocate buffers used by tpl model
 *
 * \param[in]    Top-level encode/decode structure
 * \param[in]    lag_in_frames  number of lookahead frames
 *
 * \param[out]   tpl_data  tpl data structure
 */

void av1_setup_tpl_buffers(struct AV1_PRIMARY *const ppi,
                           CommonModeInfoParams *const mi_params, int width,
                           int height, int byte_alignment, int lag_in_frames);

static inline void tpl_dealloc_temp_buffers(TplBuffers *tpl_tmp_buffers) {
  aom_free(tpl_tmp_buffers->predictor8);
  tpl_tmp_buffers->predictor8 = NULL;
  aom_free(tpl_tmp_buffers->src_diff);
  tpl_tmp_buffers->src_diff = NULL;
  aom_free(tpl_tmp_buffers->coeff);
  tpl_tmp_buffers->coeff = NULL;
  aom_free(tpl_tmp_buffers->qcoeff);
  tpl_tmp_buffers->qcoeff = NULL;
  aom_free(tpl_tmp_buffers->dqcoeff);
  tpl_tmp_buffers->dqcoeff = NULL;
}

static inline bool tpl_alloc_temp_buffers(TplBuffers *tpl_tmp_buffers,
                                          uint8_t tpl_bsize_1d) {
  // Number of pixels in a tpl block
  const int tpl_block_pels = tpl_bsize_1d * tpl_bsize_1d;

  // Allocate temporary buffers used in mode estimation.
  tpl_tmp_buffers->predictor8 = (uint8_t *)aom_memalign(
      32, tpl_block_pels * 2 * sizeof(*tpl_tmp_buffers->predictor8));
  tpl_tmp_buffers->src_diff = (int16_t *)aom_memalign(
      32, tpl_block_pels * sizeof(*tpl_tmp_buffers->src_diff));
  tpl_tmp_buffers->coeff = (tran_low_t *)aom_memalign(
      32, tpl_block_pels * sizeof(*tpl_tmp_buffers->coeff));
  tpl_tmp_buffers->qcoeff = (tran_low_t *)aom_memalign(
      32, tpl_block_pels * sizeof(*tpl_tmp_buffers->qcoeff));
  tpl_tmp_buffers->dqcoeff = (tran_low_t *)aom_memalign(
      32, tpl_block_pels * sizeof(*tpl_tmp_buffers->dqcoeff));

  if (!(tpl_tmp_buffers->predictor8 && tpl_tmp_buffers->src_diff &&
        tpl_tmp_buffers->coeff && tpl_tmp_buffers->qcoeff &&
        tpl_tmp_buffers->dqcoeff)) {
    tpl_dealloc_temp_buffers(tpl_tmp_buffers);
    return false;
  }
  return true;
}

/*!\brief Implements temporal dependency modelling for a GOP (GF/ARF
 * group) and selects between 16 and 32 frame GOP structure.
 *
 *\ingroup tpl_modelling
 *
 * \param[in]    cpi           Top - level encoder instance structure
 * \param[in]    gop_eval      Flag if it is in the GOP length decision stage
 * \param[in]    frame_params  Per frame encoding parameters
 *
 * \return Indicates whether or not we should use a longer GOP length.
 */
int av1_tpl_setup_stats(struct AV1_COMP *cpi, int gop_eval,
                        const struct EncodeFrameParams *const frame_params);

/*!\cond */

void av1_tpl_preload_rc_estimate(
    struct AV1_COMP *cpi, const struct EncodeFrameParams *const frame_params);

int av1_tpl_ptr_pos(int mi_row, int mi_col, int stride, uint8_t right_shift);

void av1_init_tpl_stats(TplParams *const tpl_data);

int av1_tpl_stats_ready(const TplParams *tpl_data, int gf_frame_index);

void av1_tpl_rdmult_setup(struct AV1_COMP *cpi);

void av1_tpl_rdmult_setup_sb(struct AV1_COMP *cpi, MACROBLOCK *const x,
                             BLOCK_SIZE sb_size, int mi_row, int mi_col);

void av1_mc_flow_dispenser_row(struct AV1_COMP *cpi,
                               TplTxfmStats *tpl_txfm_stats,
                               TplBuffers *tpl_tmp_buffers, MACROBLOCK *x,
                               int mi_row, BLOCK_SIZE bsize, TX_SIZE tx_size);

/*!\brief  Compute the entropy of an exponential probability distribution
 * function (pdf) subjected to uniform quantization.
 *
 * pdf(x) = b*exp(-b*x)
 *
 *\ingroup tpl_modelling
 *
 * \param[in]    q_step        quantizer step size
 * \param[in]    b             parameter of exponential distribution
 *
 * \return entropy cost
 */
double av1_exponential_entropy(double q_step, double b);

/*!\brief  Compute the entropy of a Laplace probability distribution
 * function (pdf) subjected to non-uniform quantization.
 *
 * pdf(x) = 0.5*b*exp(-0.5*b*|x|)
 *
 *\ingroup tpl_modelling
 *
 * \param[in]    q_step          quantizer step size for non-zero bins
 * \param[in]    b               parameter of Laplace distribution
 * \param[in]    zero_bin_ratio  zero bin's size is zero_bin_ratio * q_step
 *
 * \return entropy cost
 */
double av1_laplace_entropy(double q_step, double b, double zero_bin_ratio);

#if CONFIG_BITRATE_ACCURACY
/*!\brief  Compute the frame rate using transform block stats
 *
 * Assume each position i in the transform block is of Laplace distribution
 * with mean absolute deviation abs_coeff_mean[i]
 *
 * Then we can use av1_laplace_entropy() to compute the expected frame
 * rate.
 *
 *\ingroup tpl_modelling
 *
 * \param[in]    q_index         quantizer index
 * \param[in]    block_count     number of transform blocks
 * \param[in]    abs_coeff_mean  array of mean absolute deviation
 * \param[in]    coeff_num       number of coefficients per transform block
 *
 * \return expected frame rate
 */
double av1_laplace_estimate_frame_rate(int q_index, int block_count,
                                       const double *abs_coeff_mean,
                                       int coeff_num);
#endif  // CONFIG_BITRATE_ACCURACY

/*
 *!\brief Init TplTxfmStats
 *
 * \param[in]    tpl_txfm_stats  a structure for storing transform stats
 *
 */
void av1_init_tpl_txfm_stats(TplTxfmStats *tpl_txfm_stats);

#if CONFIG_BITRATE_ACCURACY
/*
 *!\brief Accumulate TplTxfmStats
 *
 * \param[in]  sub_stats          a structure for storing sub transform stats
 * \param[out] accumulated_stats  a structure for storing accumulated
 *transform stats
 *
 */
void av1_accumulate_tpl_txfm_stats(const TplTxfmStats *sub_stats,
                                   TplTxfmStats *accumulated_stats);

/*
 *!\brief Record a transform block into  TplTxfmStats
 *
 * \param[in]  tpl_txfm_stats     A structure for storing transform stats
 * \param[out] coeff              An array of transform coefficients. Its size
 *                                should equal to tpl_txfm_stats.coeff_num.
 *
 */
void av1_record_tpl_txfm_block(TplTxfmStats *tpl_txfm_stats,
                               const tran_low_t *coeff);

/*
 *!\brief Update abs_coeff_mean and ready of txfm_stats
 * If txfm_block_count > 0, this function will use abs_coeff_sum and
 * txfm_block_count to compute abs_coeff_mean. Moreover, reday flag
 * will be set to one.
 *
 * \param[in]  txfm_stats     A structure for storing transform stats
 */
void av1_tpl_txfm_stats_update_abs_coeff_mean(TplTxfmStats *txfm_stats);
#endif  // CONFIG_BITRATE_ACCURACY

/*!\brief  Estimate coefficient entropy using Laplace dsitribution
 *
 *\ingroup tpl_modelling
 *
 * This function is equivalent to -log2(laplace_prob()), where laplace_prob()
 *is defined in tpl_model_test.cc
 *
 * \param[in]    q_step          quantizer step size without any scaling
 * \param[in]    b               mean absolute deviation of Laplace
 *distribution \param[in]    zero_bin_ratio  zero bin's size is zero_bin_ratio
 ** q_step \param[in]    qcoeff          quantized coefficient
 *
 * \return estimated coefficient entropy
 *
 */
double av1_estimate_coeff_entropy(double q_step, double b,
                                  double zero_bin_ratio, int qcoeff);

// TODO(angiebird): Add doxygen description here.
int64_t av1_delta_rate_cost(int64_t delta_rate, int64_t recrf_dist,
                            int64_t srcrf_dist, int pix_num);

/*!\brief  Compute the overlap area between two blocks with the same size
 *
 *\ingroup tpl_modelling
 *
 * If there is no overlap, this function should return zero.
 *
 * \param[in]    row_a  row position of the first block
 * \param[in]    col_a  column position of the first block
 * \param[in]    row_b  row position of the second block
 * \param[in]    col_b  column position of the second block
 * \param[in]    width  width shared by the two blocks
 * \param[in]    height height shared by the two blocks
 *
 * \return overlap area of the two blocks
 */
int av1_get_overlap_area(int row_a, int col_a, int row_b, int col_b, int width,
                         int height);

/*!\brief Get current frame's q_index from tpl stats and leaf_qindex
 *
 * \param[in]       tpl_data          TPL struct
 * \param[in]       gf_frame_index    current frame index in the GOP
 * \param[in]       leaf_qindex       q index of leaf frame
 * \param[in]       bit_depth         bit depth
 *
 * \return q_index
 */
int av1_tpl_get_q_index(const TplParams *tpl_data, int gf_frame_index,
                        int leaf_qindex, aom_bit_depth_t bit_depth);

/*!\brief Compute the ratio between arf q step and the leaf q step based on
 * TPL stats
 *
 * \param[in]       tpl_data          TPL struct
 * \param[in]       gf_frame_index    current frame index in the GOP
 * \param[in]       leaf_qindex       q index of leaf frame
 * \param[in]       bit_depth         bit depth
 *
 * \return qstep_ratio
 */
double av1_tpl_get_qstep_ratio(const TplParams *tpl_data, int gf_frame_index);

/*!\brief Find a q index whose step size is near qstep_ratio * leaf_qstep
 *
 * \param[in]       leaf_qindex       q index of leaf frame
 * \param[in]       qstep_ratio       step ratio between target q index and
 * leaf q index \param[in]       bit_depth         bit depth
 *
 * \return q_index
 */
int av1_get_q_index_from_qstep_ratio(int leaf_qindex, double qstep_ratio,
                                     aom_bit_depth_t bit_depth);

/*!\brief Improve the motion vector estimation by taking neighbors into
 * account.
 *
 * Use the upper and left neighbor block as the reference MVs.
 * Compute the minimum difference between current MV and reference MV.
 *
 * \param[in]       tpl_frame         Tpl frame struct
 * \param[in]       row               Current row
 * \param[in]       col               Current column
 * \param[in]       step              Step parameter for av1_tpl_ptr_pos
 * \param[in]       tpl_stride        Stride parameter for av1_tpl_ptr_pos
 * \param[in]       right_shift       Right shift parameter for
 * av1_tpl_ptr_pos
 */
int_mv av1_compute_mv_difference(const TplDepFrame *tpl_frame, int row, int col,
                                 int step, int tpl_stride, int right_shift);

/*!\brief Compute the entropy of motion vectors for a single frame.
 *
 * \param[in]       tpl_frame         TPL frame struct
 * \param[in]       right_shift       right shift value for step
 *
 * \return Bits used by the motion vectors for one frame.
 */
double av1_tpl_compute_frame_mv_entropy(const TplDepFrame *tpl_frame,
                                        uint8_t right_shift);

#if CONFIG_RATECTRL_LOG
typedef struct {
  int coding_frame_count;
  int base_q_index;

  // Encode decision
  int q_index_list[VBR_RC_INFO_MAX_FRAMES];
  double qstep_ratio_list[VBR_RC_INFO_MAX_FRAMES];
  FRAME_UPDATE_TYPE update_type_list[VBR_RC_INFO_MAX_FRAMES];

  // Frame stats
  TplTxfmStats txfm_stats_list[VBR_RC_INFO_MAX_FRAMES];

  // Estimated encode results
  double est_coeff_rate_list[VBR_RC_INFO_MAX_FRAMES];

  // Actual encode results
  double act_rate_list[VBR_RC_INFO_MAX_FRAMES];
  double act_coeff_rate_list[VBR_RC_INFO_MAX_FRAMES];
} RATECTRL_LOG;

static inline void rc_log_init(RATECTRL_LOG *rc_log) { av1_zero(*rc_log); }

static inline void rc_log_frame_stats(RATECTRL_LOG *rc_log, int coding_index,
                                      const TplTxfmStats *txfm_stats) {
  rc_log->txfm_stats_list[coding_index] = *txfm_stats;
}

#if CONFIG_RATECTRL_LOG && CONFIG_THREE_PASS && CONFIG_BITRATE_ACCURACY
static inline void rc_log_frame_encode_param(RATECTRL_LOG *rc_log,
                                             int coding_index,
                                             double qstep_ratio, int q_index,
                                             FRAME_UPDATE_TYPE update_type) {
  rc_log->qstep_ratio_list[coding_index] = qstep_ratio;
  rc_log->q_index_list[coding_index] = q_index;
  rc_log->update_type_list[coding_index] = update_type;
  const TplTxfmStats *txfm_stats = &rc_log->txfm_stats_list[coding_index];
  rc_log->est_coeff_rate_list[coding_index] = 0;
  if (txfm_stats->ready) {
    rc_log->est_coeff_rate_list[coding_index] = av1_laplace_estimate_frame_rate(
        q_index, txfm_stats->txfm_block_count, txfm_stats->abs_coeff_mean,
        txfm_stats->coeff_num);
  }
}
#endif  // CONFIG_RATECTRL_LOG && CONFIG_THREE_PASS && CONFIG_BITRATE_ACCURACY

static inline void rc_log_frame_entropy(RATECTRL_LOG *rc_log, int coding_index,
                                        double act_rate,
                                        double act_coeff_rate) {
  rc_log->act_rate_list[coding_index] = act_rate;
  rc_log->act_coeff_rate_list[coding_index] = act_coeff_rate;
}

static inline void rc_log_record_chunk_info(RATECTRL_LOG *rc_log,
                                            int base_q_index,
                                            int coding_frame_count) {
  rc_log->base_q_index = base_q_index;
  rc_log->coding_frame_count = coding_frame_count;
}

static inline void rc_log_show(const RATECTRL_LOG *rc_log) {
  printf("= chunk 1\n");
  printf("coding_frame_count %d base_q_index %d\n", rc_log->coding_frame_count,
         rc_log->base_q_index);
  printf("= frame %d\n", rc_log->coding_frame_count);
  for (int coding_idx = 0; coding_idx < rc_log->coding_frame_count;
       coding_idx++) {
    printf(
        "coding_idx %d update_type %d q %d qstep_ratio %f est_coeff_rate %f "
        "act_coeff_rate %f act_rate %f\n",
        coding_idx, rc_log->update_type_list[coding_idx],
        rc_log->q_index_list[coding_idx], rc_log->qstep_ratio_list[coding_idx],
        rc_log->est_coeff_rate_list[coding_idx],
        rc_log->act_coeff_rate_list[coding_idx],
        rc_log->act_rate_list[coding_idx]);
  }
}
#endif  // CONFIG_RATECTRL_LOG

/*!\endcond */
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_TPL_MODEL_H_
