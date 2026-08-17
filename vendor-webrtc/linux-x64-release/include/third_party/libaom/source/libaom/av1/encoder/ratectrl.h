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

#ifndef AOM_AV1_ENCODER_RATECTRL_H_
#define AOM_AV1_ENCODER_RATECTRL_H_

#include "aom/aom_codec.h"
#include "aom/aom_integer.h"

#include "aom_ports/mem.h"

#include "av1/common/av1_common_int.h"
#include "av1/common/blockd.h"

#ifdef __cplusplus
extern "C" {
#endif

/*!\cond */

// Bits Per MB at different Q (Multiplied by 512)
#define BPER_MB_NORMBITS 9

// Use this macro to turn on/off use of alt-refs in one-pass mode.
#define USE_ALTREF_FOR_ONE_PASS 1

// Threshold used to define if a KF group is static (e.g. a slide show).
// Essentially, this means that no frame in the group has more than 1% of MBs
// that are not marked as coded with 0,0 motion in the first pass.
#define STATIC_KF_GROUP_THRESH 99
#define STATIC_KF_GROUP_FLOAT_THRESH 0.99

// The maximum duration of a GF group that is static (e.g. a slide show).
#define MAX_STATIC_GF_GROUP_LENGTH 250

#define MIN_GF_INTERVAL 4
#define MAX_GF_INTERVAL 32
#define FIXED_GF_INTERVAL 16
#define MAX_GF_LENGTH_LAP 16

#define FIXED_GF_INTERVAL_RT 80
#define MAX_GF_INTERVAL_RT 160

#define MAX_NUM_GF_INTERVALS 15

#define MAX_ARF_LAYERS 6
// #define STRICT_RC

#define DEFAULT_KF_BOOST_RT 2300
#define DEFAULT_GF_BOOST_RT 2000

// A passive rate control strategy for screen content type in real-time mode.
// When it is turned on, the compression performance is improved by
// 7.8% (overall_psnr), 5.0% (VMAF) on average. Some clips see gains
// over 20% on metric.
// The downside is that it does not guarantee frame size.
// Since RT mode has a tight restriction on buffer overflow control, we
// turn it off by default.
#define RT_PASSIVE_STRATEGY 0
#define MAX_Q_HISTORY 1000

typedef struct {
  int resize_width;
  int resize_height;
  uint8_t superres_denom;
} size_params_type;

enum {
  INTER_NORMAL,
  GF_ARF_LOW,
  GF_ARF_STD,
  KF_STD,
  RATE_FACTOR_LEVELS
} UENUM1BYTE(RATE_FACTOR_LEVEL);

enum {
  KF_UPDATE,
  LF_UPDATE,
  GF_UPDATE,
  ARF_UPDATE,
  OVERLAY_UPDATE,
  INTNL_OVERLAY_UPDATE,  // Internal Overlay Frame
  INTNL_ARF_UPDATE,      // Internal Altref Frame
  FRAME_UPDATE_TYPES
} UENUM1BYTE(FRAME_UPDATE_TYPE);

enum {
  REFBUF_RESET,   // Clear reference frame buffer
  REFBUF_UPDATE,  // Refresh reference frame buffer
  REFBUF_STATES
} UENUM1BYTE(REFBUF_STATE);

typedef enum {
  NO_RESIZE = 0,
  DOWN_THREEFOUR = 1,  // From orig to 3/4.
  DOWN_ONEHALF = 2,    // From orig or 3/4 to 1/2.
  UP_THREEFOUR = -1,   // From 1/2 to 3/4.
  UP_ORIG = -2,        // From 1/2 or 3/4 to orig.
} RESIZE_ACTION;

typedef enum { ORIG = 0, THREE_QUARTER = 1, ONE_HALF = 2 } RESIZE_STATE;

#define MAX_FIRSTPASS_ANALYSIS_FRAMES 150
typedef enum region_types {
  STABLE_REGION = 0,
  HIGH_VAR_REGION = 1,
  SCENECUT_REGION = 2,
  BLENDING_REGION = 3,
} REGION_TYPES;

typedef struct regions {
  int start;
  int last;
  double avg_noise_var;
  double avg_cor_coeff;
  double avg_sr_fr_ratio;
  double avg_intra_err;
  double avg_coded_err;
  REGION_TYPES type;
} REGIONS;

/*!\endcond */
/*!
 * \brief  Rate Control parameters and status
 */
typedef struct {
  // Rate targetting variables

  /*!
   * Baseline target rate for frame before adjustment for previous under or
   * over shoot.
   */
  int base_frame_target;
  /*!
   * Target rate for frame after adjustment for previous under or over shoot.
   */
  int this_frame_target;  // Actual frame target after rc adjustment.

  /*!
   * Projected size for current frame
   */
  int projected_frame_size;

  /*!
   * Bit size of transform coefficient for current frame.
   */
  int coefficient_size;

  /*!
   * Super block rate target used with some adaptive quantization strategies.
   */
  int sb64_target_rate;

  /*!
   * Number of frames since the last ARF / GF.
   */
  int frames_since_golden;

  /*!
   * Number of frames till the next ARF / GF is due.
   */
  int frames_till_gf_update_due;

  /*!
   * Number of determined gf groups left
   */
  int intervals_till_gf_calculate_due;

  /*!\cond */
  int min_gf_interval;
  int max_gf_interval;
  int static_scene_max_gf_interval;
  /*!\endcond */
  /*!
   * Frames before the next key frame
   */
  int frames_to_key;
  /*!\cond */
  int frames_since_key;
  int frames_to_fwd_kf;
  int is_src_frame_alt_ref;
  int sframe_due;

  int high_source_sad;
  int high_motion_content_screen_rtc;
  uint64_t avg_source_sad;
  uint64_t prev_avg_source_sad;
  uint64_t frame_source_sad;
  uint64_t frame_spatial_variance;
  int static_since_last_scene_change;
  int last_encoded_size_keyframe;
  int last_target_size_keyframe;
  int frames_since_scene_change;
  int perc_spatial_flat_blocks;
  int num_col_blscroll_last_tl0;
  int num_row_blscroll_last_tl0;

  int avg_frame_bandwidth;  // Average frame size target for clip
  int min_frame_bandwidth;  // Minimum allocation used for any frame
  int max_frame_bandwidth;  // Maximum burst rate allowed for a frame.
  int prev_avg_frame_bandwidth;

  int ni_av_qi;
  int ni_tot_qi;

  int decimation_factor;
  int decimation_count;
  int prev_frame_is_dropped;
  int drop_count_consec;
  int max_consec_drop;
  int force_max_q;
  int postencode_drop;

  /*!
   * Frame number for encoded frames (non-dropped).
   * Use for setting the rtc reference structure.
   */
  unsigned int frame_number_encoded;

  /*!\endcond */
  /*!
   * User specified maximum Q allowed for current frame
   */
  int worst_quality;
  /*!
   * User specified minimum Q allowed for current frame
   */
  int best_quality;

  /*!\cond */

  // rate control history for last frame(1) and the frame before(2).
  // -1: overshoot
  //  1: undershoot
  //  0: not initialized.
  int rc_1_frame;
  int rc_2_frame;
  int q_1_frame;
  int q_2_frame;

  /*!\endcond */
  /*!
   * Proposed maximum allowed Q for current frame
   */
  int active_worst_quality;

  /*!\cond */
  // Track amount of low motion in scene
  int avg_frame_low_motion;
  int cnt_zeromv;

  // signals if number of blocks with motion is high
  int percent_blocks_with_motion;

  // signals percentage of 16x16 blocks that are inactive, via active_maps
  int percent_blocks_inactive;

  // Maximum value of source sad across all blocks of frame.
  uint64_t max_block_source_sad;

  // For dynamic resize, 1 pass cbr.
  RESIZE_STATE resize_state;
  int resize_avg_qp;
  int resize_buffer_underflow;
  int resize_count;

  // Flag to disable content related qp adjustment.
  int rtc_external_ratectrl;

  // Stores fast_extra_bits of the current frame.
  int frame_level_fast_extra_bits;

  double frame_level_rate_correction_factors[RATE_FACTOR_LEVELS];

  int frame_num_last_gf_refresh;

  int prev_coded_width;
  int prev_coded_height;

  // The ratio used for inter frames in bit estimation.
  // TODO(yunqing): if golden frame is treated differently (e.g. gf_cbr_boost_
  // pct > THR), consider to add bit_est_ratio_g for golden frames.
  int bit_est_ratio;

  // Whether to use a fixed qp for the frame, bypassing internal rate control.
  // This flag will reset to 0 after every frame.
  int use_external_qp_one_pass;
  /*!\endcond */
} RATE_CONTROL;

/*!
 * \brief  Primary Rate Control parameters and status
 */
typedef struct {
  // Sub-gop level Rate targetting variables

  /*!
   * Target bit budget for the current GF / ARF group of frame.
   */
  int64_t gf_group_bits;

  /*!
   * Boost factor used to calculate the extra bits allocated to the key frame
   */
  int kf_boost;

  /*!
   * Boost factor used to calculate the extra bits allocated to ARFs and GFs
   */
  int gfu_boost;

  /*!
   * Stores the determined gf group lengths for a set of gf groups
   */
  int gf_intervals[MAX_NUM_GF_INTERVALS];

  /*!
   * The current group's index into gf_intervals[]
   */
  int cur_gf_index;

  /*!\cond */
  int num_regions;

  REGIONS regions[MAX_FIRSTPASS_ANALYSIS_FRAMES];
  int regions_offset;  // offset of regions from the last keyframe
  int frames_till_regions_update;

  int baseline_gf_interval;

  int constrained_gf_group;

  int this_key_frame_forced;

  int next_key_frame_forced;
  /*!\endcond */

  /*!
   * Initial buffuer level in ms for CBR / low delay encoding
   */
  int64_t starting_buffer_level;

  /*!
   * Optimum / target buffuer level in ms for CBR / low delay encoding
   */
  int64_t optimal_buffer_level;

  /*!
   * Maximum target buffuer level in ms for CBR / low delay encoding
   */
  int64_t maximum_buffer_size;

  /*!
   * Q index used for ALT frame
   */
  int arf_q;

  /*!\cond */
  float_t arf_boost_factor;

  int base_layer_qp;

  // Total number of stats used only for kf_boost calculation.
  int num_stats_used_for_kf_boost;

  // Total number of stats used only for gfu_boost calculation.
  int num_stats_used_for_gfu_boost;

  // Total number of stats required by gfu_boost calculation.
  int num_stats_required_for_gfu_boost;

  int enable_scenecut_detection;

  int use_arf_in_this_kf_group;

  int ni_frames;

  double tot_q;
  /*!\endcond */

  /*!
   * Q used for last boosted (non leaf) frame
   */
  int last_kf_qindex;

  /*!
   * Average of q index of previous encoded frames in a sequence.
   */
  int avg_frame_qindex[FRAME_TYPES];

#if CONFIG_FPMT_TEST
  /*!
   * Temporary variable used in simulating the delayed update of
   * active_best_quality.
   */
  int temp_active_best_quality[MAX_ARF_LAYERS + 1];

  /*!
   * Temporary variable used in simulating the delayed update of
   * last_boosted_qindex.
   */
  int temp_last_boosted_qindex;

  /*!
   * Temporary variable used in simulating the delayed update of
   * avg_q.
   */
  double temp_avg_q;

  /*!
   * Temporary variable used in simulating the delayed update of
   * last_q.
   */
  int temp_last_q[FRAME_TYPES];

  /*!
   * Temporary variable used in simulating the delayed update of
   * projected_frame_size.
   */
  int temp_projected_frame_size;

  /*!
   * Temporary variable used in simulating the delayed update of
   * total_actual_bits.
   */
  int64_t temp_total_actual_bits;

  /*!
   * Temporary variable used in simulating the delayed update of
   * buffer_level.
   */
  int64_t temp_buffer_level;

  /*!
   * Temporary variable used in simulating the delayed update of
   * vbr_bits_off_target.
   */
  int64_t temp_vbr_bits_off_target;

  /*!
   * Temporary variable used in simulating the delayed update of
   * vbr_bits_off_target_fast.
   */
  int64_t temp_vbr_bits_off_target_fast;

  /*!
   * Temporary variable used in simulating the delayed update of
   * rate_correction_factors.
   */
  double temp_rate_correction_factors[RATE_FACTOR_LEVELS];

  /*!
   * Temporary variable used in simulating the delayed update of
   * rate_error_estimate.
   */
  int temp_rate_error_estimate;

  /*!
   * Temporary variable used in simulating the delayed update of
   * rolling_arf_group_target_bits.
   */
  int temp_rolling_arf_group_target_bits;

  /*!
   * Temporary variable used in simulating the delayed update of
   * rolling_arf_group_actual_bits;.
   */
  int temp_rolling_arf_group_actual_bits;

  /*!
   * Temporary variable used in simulating the delayed update of
   * bits_left;.
   */
  int64_t temp_bits_left;

  /*!
   * Temporary variable used in simulating the delayed update of
   * extend_minq.
   */
  int temp_extend_minq;

  /*!
   * Temporary variable used in simulating the delayed update of
   * extend_maxq.
   */
  int temp_extend_maxq;

#endif
  /*!
   * Proposed minimum allowed Q different layers in a coding pyramid
   */
  int active_best_quality[MAX_ARF_LAYERS + 1];

  /*!
   * Q used for last boosted (non leaf) frame (GF/KF/ARF)
   */
  int last_boosted_qindex;

  /*!
   * Average Q value of previous inter frames
   */
  double avg_q;

  /*!
   * Q used on last encoded frame of the given type.
   */
  int last_q[FRAME_TYPES];

  /*!
   * Correction factors used to adjust the q estimate for a given target rate
   * in the encode loop.
   */
  double rate_correction_factors[RATE_FACTOR_LEVELS];

  /*!
   * Current total consumed bits.
   */
  int64_t total_actual_bits;

  /*!
   * Current total target bits.
   */
  int64_t total_target_bits;

  /*!
   * Current buffer level.
   */
  int64_t buffer_level;

  /*!
   * PCT rc error.
   */
  int rate_error_estimate;

  /*!
   * Error bits available from previously encoded frames.
   */
  int64_t vbr_bits_off_target;

  /*!
   * Error bits available from previously encoded frames undershoot.
   */
  int64_t vbr_bits_off_target_fast;

  /*!
   * Total bits deviated from the average frame target, from previously
   * encoded frames.
   */
  int64_t bits_off_target;

  /*!
   * Rolling monitor target bits updated based on current frame target size.
   */
  int rolling_target_bits;

  /*!
   * Rolling monitor actual bits updated based on current frame final projected
   * size.
   */
  int rolling_actual_bits;

  /*!
   * The history of qindex for each frame.
   * Only used when RT_PASSIVE_STRATEGY = 1.
   */
  int q_history[MAX_Q_HISTORY];
} PRIMARY_RATE_CONTROL;

/*!\cond */

struct AV1_COMP;
struct AV1EncoderConfig;
struct GF_GROUP;

void av1_primary_rc_init(const struct AV1EncoderConfig *oxcf,
                         PRIMARY_RATE_CONTROL *p_rc);

void av1_rc_init(const struct AV1EncoderConfig *oxcf, RATE_CONTROL *rc);

int av1_estimate_bits_at_q(const struct AV1_COMP *cpi, int q,
                           double correction_factor);

double av1_convert_qindex_to_q(int qindex, aom_bit_depth_t bit_depth);

// Converts a Q value to a qindex.
int av1_convert_q_to_qindex(double q, aom_bit_depth_t bit_depth);

void av1_rc_init_minq_luts(void);

int av1_rc_get_default_min_gf_interval(int width, int height, double framerate);

// Generally at the high level, the following flow is expected
// to be enforced for rate control:
// First call per frame, one of:
//   av1_get_one_pass_rt_params()
//   av1_get_second_pass_params()
// depending on the usage to set the rate control encode parameters desired.
//
// Then, call encode_frame_to_data_rate() to perform the
// actual encode. This function will in turn call encode_frame()
// one or more times, followed by:
//   av1_rc_postencode_update_drop_frame()
//
// The majority of rate control parameters are only expected
// to be set in the av1_get_..._params() functions and
// updated during the av1_rc_postencode_update...() functions.
// The only exceptions are av1_rc_drop_frame() and
// av1_rc_update_rate_correction_factors() functions.

// Functions to set parameters for encoding before the actual
// encode_frame_to_data_rate() function.
struct EncodeFrameInput;

// Post encode update of the rate control parameters based
// on bytes used
void av1_rc_postencode_update(struct AV1_COMP *cpi, uint64_t bytes_used);
// Post encode update of the rate control parameters for dropped frames
void av1_rc_postencode_update_drop_frame(struct AV1_COMP *cpi);

/*!\endcond */
/*!\brief Updates the rate correction factor linking Q to output bits
 *
 * This function updates the Q rate correction factor after an encode
 * cycle depending on whether we overshot or undershot the target rate.
 *
 * \ingroup rate_control
 * \param[in]   cpi                   Top level encoder instance structure
 * \param[in]   is_encode_stage       Indicates if recode loop or post-encode
 * \param[in]   width                 Frame width
 * \param[in]   height                Frame height
 *
 * \remark Updates the relevant rate correction factor in cpi->rc
 */
void av1_rc_update_rate_correction_factors(struct AV1_COMP *cpi,
                                           int is_encode_stage, int width,
                                           int height);
/*!\cond */

// Decide if we should drop this frame: For 1-pass CBR.
// Changes only the decimation count in the rate control structure
int av1_rc_drop_frame(struct AV1_COMP *cpi);

// Computes frame size bounds.
void av1_rc_compute_frame_size_bounds(const struct AV1_COMP *cpi,
                                      int this_frame_target,
                                      int *frame_under_shoot_limit,
                                      int *frame_over_shoot_limit);

/*!\endcond */

/*!\brief Picks q and q bounds given the rate control parameters in \c cpi->rc.
 *
 * \ingroup rate_control
 * \param[in]       cpi          Top level encoder structure
 * \param[in]       width        Coded frame width
 * \param[in]       height       Coded frame height
 * \param[in]       gf_index     Index of this frame in the golden frame group
 * \param[out]      bottom_index Bottom bound for q index (best quality)
 * \param[out]      top_index    Top bound for q index (worst quality)
 * \return Returns selected q index to be used for encoding this frame.
 * Also, updates \c rc->arf_q.
 */
int av1_rc_pick_q_and_bounds(struct AV1_COMP *cpi, int width, int height,
                             int gf_index, int *bottom_index, int *top_index);

/*!\brief Estimates q to achieve a target bits per frame
 *
 * \ingroup rate_control
 * \param[in]   cpi                   Top level encoder instance structure
 * \param[in]   target_bits_per_frame Frame rate target
 * \param[in]   active_worst_quality  Max Q allowed
 * \param[in]   active_best_quality   Min Q allowed
 * \param[in]   width                 Frame width
 * \param[in]   height                Frame height
 *
 * \return Returns a q index value
 */
int av1_rc_regulate_q(const struct AV1_COMP *cpi, int target_bits_per_frame,
                      int active_best_quality, int active_worst_quality,
                      int width, int height);

/*!\cond */
// Estimates bits per mb for a given qindex and correction factor.
int av1_rc_bits_per_mb(const struct AV1_COMP *cpi, FRAME_TYPE frame_type,
                       int qindex, double correction_factor,
                       int accurate_estimate);

// Find q_index corresponding to desired_q, within [best_qindex, worst_qindex].
// To be precise, 'q_index' is the smallest integer, for which the corresponding
// q >= desired_q.
// If no such q index is found, returns 'worst_qindex'.
int av1_find_qindex(double desired_q, aom_bit_depth_t bit_depth,
                    int best_qindex, int worst_qindex);

// Computes a q delta (in "q index" terms) to get from a starting q value
// to a target q value
int av1_compute_qdelta(const RATE_CONTROL *rc, double qstart, double qtarget,
                       aom_bit_depth_t bit_depth);

// Computes a q delta (in "q index" terms) to get from a starting q value
// to a value that should equate to the given rate ratio.
int av1_compute_qdelta_by_rate(const struct AV1_COMP *cpi,
                               FRAME_TYPE frame_type, int qindex,
                               double rate_target_ratio);

void av1_rc_update_framerate(struct AV1_COMP *cpi, int width, int height);

void av1_set_target_rate(struct AV1_COMP *cpi, int width, int height);

int av1_resize_one_pass_cbr(struct AV1_COMP *cpi);

void av1_rc_set_frame_target(struct AV1_COMP *cpi, int target, int width,
                             int height);

void av1_adjust_gf_refresh_qp_one_pass_rt(struct AV1_COMP *cpi);

void av1_set_rtc_reference_structure_one_layer(struct AV1_COMP *cpi,
                                               int gf_update);

/*!\endcond */
/*!\brief Calculates how many bits to use for a P frame in one pass vbr
 *
 * \ingroup rate_control
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi                 Top level encoder structure
 * \param[in]       frame_update_type   Type of frame
 *
 * \return	Returns the target number of bits for this frame.
 */
int av1_calc_pframe_target_size_one_pass_vbr(
    const struct AV1_COMP *const cpi, FRAME_UPDATE_TYPE frame_update_type);

/*!\brief Calculates how many bits to use for an i frame in one pass vbr
 *
 * \ingroup rate_control
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi  Top level encoder structure
 *
 * \return	Returns the target number of bits for this frame.
 */
int av1_calc_iframe_target_size_one_pass_vbr(const struct AV1_COMP *const cpi);

/*!\brief Calculates how many bits to use for a P frame in one pass cbr
 *
 * \ingroup rate_control
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi                 Top level encoder structure
 * \param[in]       frame_update_type   Type of frame
 *
 * \return  Returns the target number of bits for this frame.
 */
int av1_calc_pframe_target_size_one_pass_cbr(
    const struct AV1_COMP *cpi, FRAME_UPDATE_TYPE frame_update_type);

/*!\brief Calculates how many bits to use for an i frame in one pass cbr
 *
 * \ingroup rate_control
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi  Top level encoder structure
 *
 * \return  Returns the target number of bits for this frame.
 */
int av1_calc_iframe_target_size_one_pass_cbr(const struct AV1_COMP *cpi);

/*!\brief Setup the rate control parameters for 1 pass real-time mode.
 *
 * - Sets the frame type and target frame size.
 * - Sets the GF update.
 * - Checks for scene change.
 * - Sets the reference prediction structure for 1 layers (non-SVC).
 * - Resets and updates are done for SVC.
 *
 * \ingroup rate_control
 * \param[in]       cpi          Top level encoder structure
 * \param[in]       frame_type   Encoder frame type
 * \param[in]       frame_input  Current and last input source frames
 * \param[in]       frame_flags  Encoder frame flags
 *
 * \remark Nothing is returned. Instead the settings computed in this
 * function are set in: \c frame_params, \c cpi->common, \c cpi->rc,
 * \c cpi->svc.
 */
void av1_get_one_pass_rt_params(struct AV1_COMP *cpi,
                                FRAME_TYPE *const frame_type,
                                const struct EncodeFrameInput *frame_input,
                                unsigned int frame_flags);

/*!\brief Increase q on expected encoder overshoot, for CBR mode.
 *
 *  Handles the case when encoder is expected to create a large frame:
 *  - q is increased to value closer to \c cpi->rc.worst_quality
 *  - avg_frame_qindex is reset
 *  - buffer levels are reset
 *  - rate correction factor is adjusted
 *
 * \ingroup rate_control
 * \param[in]       cpi          Top level encoder structure
 * \param[in]        q           Current q index
 *
 * \return q is returned, and updates are done to \c cpi->rc.
 */
int av1_encodedframe_overshoot_cbr(struct AV1_COMP *cpi, int *q);

/*!\brief Check if frame should be dropped, for RTC mode.
 *
 * \ingroup rate_control
 * \param[in]       cpi          Top level encoder structure
 * \param[in,out]       size         Size of encoded frame
 *
 * \return 1 if frame is to be dropped, 0 otherwise (no drop).
 * Set cpi->rc.force_max_q if frame is to be dropped, and updates are
 * made to rate control parameters. *size is set to 0 when this
 * function returns 1 (frame is dropped).
 */
int av1_postencode_drop_cbr(struct AV1_COMP *cpi, size_t *size);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_RATECTRL_H_
