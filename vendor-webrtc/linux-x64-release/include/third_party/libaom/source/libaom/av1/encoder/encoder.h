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
 * \brief Declares top-level encoder structures and functions.
 */
#ifndef AOM_AV1_ENCODER_ENCODER_H_
#define AOM_AV1_ENCODER_ENCODER_H_

#include <stdbool.h>
#include <stdio.h>

#include "config/aom_config.h"

#include "aom/aomcx.h"
#include "aom_util/aom_pthread.h"

#include "av1/common/alloccommon.h"
#include "av1/common/av1_common_int.h"
#include "av1/common/blockd.h"
#include "av1/common/entropymode.h"
#include "av1/common/enums.h"
#include "av1/common/reconintra.h"
#include "av1/common/resize.h"
#include "av1/common/thread_common.h"
#include "av1/common/timing.h"

#include "av1/encoder/aq_cyclicrefresh.h"
#include "av1/encoder/av1_quantize.h"
#include "av1/encoder/block.h"
#include "av1/encoder/context_tree.h"
#include "av1/encoder/enc_enums.h"
#include "av1/encoder/encodemb.h"
#include "av1/encoder/external_partition.h"
#include "av1/encoder/firstpass.h"
#include "av1/encoder/global_motion.h"
#include "av1/encoder/level.h"
#include "av1/encoder/lookahead.h"
#include "av1/encoder/mcomp.h"
#include "av1/encoder/pickcdef.h"
#include "av1/encoder/ratectrl.h"
#include "av1/encoder/rd.h"
#include "av1/encoder/speed_features.h"
#include "av1/encoder/svc_layercontext.h"
#include "av1/encoder/temporal_filter.h"
#if CONFIG_THREE_PASS
#include "av1/encoder/thirdpass.h"
#endif
#include "av1/encoder/tokenize.h"
#include "av1/encoder/tpl_model.h"
#include "av1/encoder/av1_noise_estimate.h"
#include "av1/encoder/bitstream.h"

#if CONFIG_INTERNAL_STATS
#include "aom_dsp/ssim.h"
#endif
#include "aom_dsp/variance.h"
#if CONFIG_DENOISE
#include "aom_dsp/noise_model.h"
#endif
#if CONFIG_TUNE_VMAF
#include "av1/encoder/tune_vmaf.h"
#endif
#if CONFIG_AV1_TEMPORAL_DENOISING
#include "av1/encoder/av1_temporal_denoiser.h"
#endif
#if CONFIG_TUNE_BUTTERAUGLI
#include "av1/encoder/tune_butteraugli.h"
#endif

#include "aom/internal/aom_codec_internal.h"

#ifdef __cplusplus
extern "C" {
#endif

// TODO(yunqing, any): Added suppression tag to quiet Doxygen warnings. Need to
// adjust it while we work on documentation.
/*!\cond */
// Number of frames required to test for scene cut detection
#define SCENE_CUT_KEY_TEST_INTERVAL 16

// Lookahead index threshold to enable temporal filtering for second arf.
#define TF_LOOKAHEAD_IDX_THR 7

#define HDR_QP_LEVELS 10
#define CHROMA_CB_QP_SCALE 1.04
#define CHROMA_CR_QP_SCALE 1.04
#define CHROMA_QP_SCALE -0.46
#define CHROMA_QP_OFFSET 9.26
#define QP_SCALE_FACTOR 2.0
#define DISABLE_HDR_LUMA_DELTAQ 1

// Rational number with an int64 numerator
// This structure holds a fractional value
typedef struct aom_rational64 {
  int64_t num;       // fraction numerator
  int den;           // fraction denominator
} aom_rational64_t;  // alias for struct aom_rational

enum {
  // Good Quality Fast Encoding. The encoder balances quality with the amount of
  // time it takes to encode the output. Speed setting controls how fast.
  GOOD,
  // Realtime Fast Encoding. Will force some restrictions on bitrate
  // constraints.
  REALTIME,
  // All intra mode. All the frames are coded as intra frames.
  ALLINTRA
} UENUM1BYTE(MODE);

enum {
  FRAMEFLAGS_KEY = 1 << 0,
  FRAMEFLAGS_GOLDEN = 1 << 1,
  FRAMEFLAGS_BWDREF = 1 << 2,
  // TODO(zoeliu): To determine whether a frame flag is needed for ALTREF2_FRAME
  FRAMEFLAGS_ALTREF = 1 << 3,
  FRAMEFLAGS_INTRAONLY = 1 << 4,
  FRAMEFLAGS_SWITCH = 1 << 5,
  FRAMEFLAGS_ERROR_RESILIENT = 1 << 6,
} UENUM1BYTE(FRAMETYPE_FLAGS);

#if CONFIG_FPMT_TEST
enum {
  PARALLEL_ENCODE = 0,
  PARALLEL_SIMULATION_ENCODE,
  NUM_FPMT_TEST_ENCODES
} UENUM1BYTE(FPMT_TEST_ENC_CFG);
#endif  // CONFIG_FPMT_TEST
// 0 level frames are sometimes used for rate control purposes, but for
// reference mapping purposes, the minimum level should be 1.
#define MIN_PYR_LEVEL 1
static inline int get_true_pyr_level(int frame_level, int frame_order,
                                     int max_layer_depth) {
  if (frame_order == 0) {
    // Keyframe case
    return MIN_PYR_LEVEL;
  } else if (frame_level == MAX_ARF_LAYERS) {
    // Leaves
    return max_layer_depth;
  } else if (frame_level == (MAX_ARF_LAYERS + 1)) {
    // Altrefs
    return MIN_PYR_LEVEL;
  }
  return AOMMAX(MIN_PYR_LEVEL, frame_level);
}

enum {
  NO_AQ = 0,
  VARIANCE_AQ = 1,
  COMPLEXITY_AQ = 2,
  CYCLIC_REFRESH_AQ = 3,
  AQ_MODE_COUNT  // This should always be the last member of the enum
} UENUM1BYTE(AQ_MODE);
enum {
  NO_DELTA_Q = 0,
  DELTA_Q_OBJECTIVE = 1,      // Modulation to improve objective quality
  DELTA_Q_PERCEPTUAL = 2,     // Modulation to improve video perceptual quality
  DELTA_Q_PERCEPTUAL_AI = 3,  // Perceptual quality opt for all intra mode
  DELTA_Q_USER_RATING_BASED = 4,  // User rating based delta q mode
  DELTA_Q_HDR = 5,  // QP adjustment based on HDR block pixel average
  DELTA_Q_VARIANCE_BOOST =
      6,              // Variance Boost style modulation for all intra mode
  DELTA_Q_MODE_COUNT  // This should always be the last member of the enum
} UENUM1BYTE(DELTAQ_MODE);

enum {
  RESIZE_NONE = 0,     // No frame resizing allowed.
  RESIZE_FIXED = 1,    // All frames are coded at the specified scale.
  RESIZE_RANDOM = 2,   // All frames are coded at a random scale.
  RESIZE_DYNAMIC = 3,  // Frames coded at lower scale based on rate control.
  RESIZE_MODES
} UENUM1BYTE(RESIZE_MODE);

enum {
  SS_CFG_SRC = 0,
  SS_CFG_LOOKAHEAD = 1,
  SS_CFG_FPF = 2,
  SS_CFG_TOTAL = 3
} UENUM1BYTE(SS_CFG_OFFSET);

enum {
  DISABLE_SCENECUT,        // For LAP, lag_in_frames < 19
  ENABLE_SCENECUT_MODE_1,  // For LAP, lag_in_frames >=19 and < 33
  ENABLE_SCENECUT_MODE_2   // For twopass and LAP - lag_in_frames >=33
} UENUM1BYTE(SCENECUT_MODE);

#define MAX_VBR_CORPUS_COMPLEXITY 10000

typedef enum {
  MOD_FP,           // First pass
  MOD_TF,           // Temporal filtering
  MOD_TPL,          // TPL
  MOD_GME,          // Global motion estimation
  MOD_ENC,          // Encode stage
  MOD_LPF,          // Deblocking loop filter
  MOD_CDEF_SEARCH,  // CDEF search
  MOD_CDEF,         // CDEF frame
  MOD_LR,           // Loop restoration filtering
  MOD_PACK_BS,      // Pack bitstream
  MOD_FRAME_ENC,    // Frame Parallel encode
  MOD_AI,           // All intra
  NUM_MT_MODULES
} MULTI_THREADED_MODULES;

/*!\endcond */

/*!\enum COST_UPDATE_TYPE
 * \brief    This enum controls how often the entropy costs should be updated.
 * \warning  In case of any modifications/additions done to the enum
 * COST_UPDATE_TYPE, the enum INTERNAL_COST_UPDATE_TYPE needs to be updated as
 * well.
 */
typedef enum {
  COST_UPD_SB,           /*!< Update every sb. */
  COST_UPD_SBROW,        /*!< Update every sb rows inside a tile. */
  COST_UPD_TILE,         /*!< Update every tile. */
  COST_UPD_OFF,          /*!< Turn off cost updates. */
  NUM_COST_UPDATE_TYPES, /*!< Number of cost update types. */
} COST_UPDATE_TYPE;

/*!\enum LOOPFILTER_CONTROL
 * \brief This enum controls to which frames loopfilter is applied.
 */
typedef enum {
  LOOPFILTER_NONE = 0,      /*!< Disable loopfilter on all frames. */
  LOOPFILTER_ALL = 1,       /*!< Enable loopfilter for all frames. */
  LOOPFILTER_REFERENCE = 2, /*!< Disable loopfilter on non reference frames. */
  LOOPFILTER_SELECTIVELY =
      3, /*!< Disable loopfilter on frames with low motion. */
} LOOPFILTER_CONTROL;

/*!\enum SKIP_APPLY_POSTPROC_FILTER
 * \brief This enum controls the application of post-processing filters on a
 * reconstructed frame.
 */
typedef enum {
  SKIP_APPLY_RESTORATION = 1 << 0,
  SKIP_APPLY_SUPERRES = 1 << 1,
  SKIP_APPLY_CDEF = 1 << 2,
  SKIP_APPLY_LOOPFILTER = 1 << 3,
} SKIP_APPLY_POSTPROC_FILTER;

/*!
 * \brief Encoder config related to resize.
 */
typedef struct {
  /*!
   * Indicates the frame resize mode to be used by the encoder.
   */
  RESIZE_MODE resize_mode;
  /*!
   * Indicates the denominator for resize of inter frames, assuming 8 as the
   *  numerator. Its value ranges between 8-16.
   */
  uint8_t resize_scale_denominator;
  /*!
   * Indicates the denominator for resize of key frames, assuming 8 as the
   * numerator. Its value ranges between 8-16.
   */
  uint8_t resize_kf_scale_denominator;
} ResizeCfg;

/*!
 * \brief Encoder config for coding block partitioning.
 */
typedef struct {
  /*!
   * Flag to indicate if rectanguar partitions should be enabled.
   */
  bool enable_rect_partitions;
  /*!
   * Flag to indicate if AB partitions should be enabled.
   */
  bool enable_ab_partitions;
  /*!
   * Flag to indicate if 1:4 / 4:1 partitions should be enabled.
   */
  bool enable_1to4_partitions;
  /*!
   * Indicates the minimum partition size that should be allowed. Both width and
   * height of a partition cannot be smaller than the min_partition_size.
   */
  BLOCK_SIZE min_partition_size;
  /*!
   * Indicates the maximum partition size that should be allowed. Both width and
   * height of a partition cannot be larger than the max_partition_size.
   */
  BLOCK_SIZE max_partition_size;
} PartitionCfg;

/*!
 * \brief Encoder flags for intra prediction.
 */
typedef struct {
  /*!
   * Flag to indicate if intra edge filtering process should be enabled.
   */
  bool enable_intra_edge_filter;
  /*!
   * Flag to indicate if recursive filtering based intra prediction should be
   * enabled.
   */
  bool enable_filter_intra;
  /*!
   * Flag to indicate if smooth intra prediction modes should be enabled.
   */
  bool enable_smooth_intra;
  /*!
   * Flag to indicate if PAETH intra prediction mode should be enabled.
   */
  bool enable_paeth_intra;
  /*!
   * Flag to indicate if CFL uv intra mode should be enabled.
   */
  bool enable_cfl_intra;
  /*!
   * Flag to indicate if directional modes should be enabled.
   */
  bool enable_directional_intra;
  /*!
   * Flag to indicate if the subset of directional modes from D45 to D203 intra
   * should be enabled. Has no effect if directional modes are disabled.
   */
  bool enable_diagonal_intra;
  /*!
   * Flag to indicate if delta angles for directional intra prediction should be
   * enabled.
   */
  bool enable_angle_delta;
  /*!
   * Flag to indicate whether to automatically turn off several intral coding
   * tools.
   * This flag is only used when "--deltaq-mode=3" is true.
   * When set to 1, the encoder will analyze the reconstruction quality
   * as compared to the source image in the preprocessing pass.
   * If the recontruction quality is considered high enough, we disable
   * the following intra coding tools, for better encoding speed:
   * "--enable_smooth_intra",
   * "--enable_paeth_intra",
   * "--enable_cfl_intra",
   * "--enable_diagonal_intra".
   */
  bool auto_intra_tools_off;
} IntraModeCfg;

/*!
 * \brief Encoder flags for transform sizes and types.
 */
typedef struct {
  /*!
   * Flag to indicate if 64-pt transform should be enabled.
   */
  bool enable_tx64;
  /*!
   * Flag to indicate if flip and identity transform types should be enabled.
   */
  bool enable_flip_idtx;
  /*!
   * Flag to indicate if rectangular transform should be enabled.
   */
  bool enable_rect_tx;
  /*!
   * Flag to indicate whether or not to use a default reduced set for ext-tx
   * rather than the potential full set of 16 transforms.
   */
  bool reduced_tx_type_set;
  /*!
   * Flag to indicate if transform type for intra blocks should be limited to
   * DCT_DCT.
   */
  bool use_intra_dct_only;
  /*!
   * Flag to indicate if transform type for inter blocks should be limited to
   * DCT_DCT.
   */
  bool use_inter_dct_only;
  /*!
   * Flag to indicate if intra blocks should use default transform type
   * (mode-dependent) only.
   */
  bool use_intra_default_tx_only;
  /*!
   * Flag to indicate if transform size search should be enabled.
   */
  bool enable_tx_size_search;
} TxfmSizeTypeCfg;

/*!
 * \brief Encoder flags for compound prediction modes.
 */
typedef struct {
  /*!
   * Flag to indicate if distance-weighted compound type should be enabled.
   */
  bool enable_dist_wtd_comp;
  /*!
   * Flag to indicate if masked (wedge/diff-wtd) compound type should be
   * enabled.
   */
  bool enable_masked_comp;
  /*!
   * Flag to indicate if smooth interintra mode should be enabled.
   */
  bool enable_smooth_interintra;
  /*!
   * Flag to indicate if difference-weighted compound type should be enabled.
   */
  bool enable_diff_wtd_comp;
  /*!
   * Flag to indicate if inter-inter wedge compound type should be enabled.
   */
  bool enable_interinter_wedge;
  /*!
   * Flag to indicate if inter-intra wedge compound type should be enabled.
   */
  bool enable_interintra_wedge;
} CompoundTypeCfg;

/*!
 * \brief Encoder config related to frame super-resolution.
 */
typedef struct {
  /*!
   * Indicates the qindex based threshold to be used when AOM_SUPERRES_QTHRESH
   * mode is used for inter frames.
   */
  int superres_qthresh;
  /*!
   * Indicates the qindex based threshold to be used when AOM_SUPERRES_QTHRESH
   * mode is used for key frames.
   */
  int superres_kf_qthresh;
  /*!
   * Indicates the denominator of the fraction that specifies the ratio between
   * the superblock width before and after upscaling for inter frames. The
   * numerator of this fraction is equal to the constant SCALE_NUMERATOR.
   */
  uint8_t superres_scale_denominator;
  /*!
   * Indicates the denominator of the fraction that specifies the ratio between
   * the superblock width before and after upscaling for key frames. The
   * numerator of this fraction is equal to the constant SCALE_NUMERATOR.
   */
  uint8_t superres_kf_scale_denominator;
  /*!
   * Indicates the Super-resolution mode to be used by the encoder.
   */
  aom_superres_mode superres_mode;
  /*!
   * Flag to indicate if super-resolution should be enabled for the sequence.
   */
  bool enable_superres;
} SuperResCfg;

/*!
 * \brief Encoder config related to the coding of key frames.
 */
typedef struct {
  /*!
   * Indicates the minimum distance to a key frame.
   */
  int key_freq_min;

  /*!
   * Indicates the maximum distance to a key frame.
   */
  int key_freq_max;

  /*!
   * Indicates if temporal filtering should be applied on keyframe.
   */
  int enable_keyframe_filtering;

  /*!
   * Indicates the number of frames after which a frame may be coded as an
   * S-Frame.
   */
  int sframe_dist;

  /*!
   * Indicates how an S-Frame should be inserted.
   * 1: the considered frame will be made into an S-Frame only if it is an
   * altref frame. 2: the next altref frame will be made into an S-Frame.
   */
  int sframe_mode;

  /*!
   * Indicates if encoder should autodetect cut scenes and set the keyframes.
   */
  bool auto_key;

  /*!
   * Indicates the forward key frame distance.
   */
  int fwd_kf_dist;

  /*!
   * Indicates if forward keyframe reference should be enabled.
   */
  bool fwd_kf_enabled;

  /*!
   * Indicates if S-Frames should be enabled for the sequence.
   */
  bool enable_sframe;

  /*!
   * Indicates if intra block copy prediction mode should be enabled or not.
   */
  bool enable_intrabc;
} KeyFrameCfg;

/*!
 * \brief Encoder rate control configuration parameters
 */
typedef struct {
  /*!\cond */
  // BUFFERING PARAMETERS
  /*!\endcond */
  /*!
   * Indicates the amount of data that will be buffered by the decoding
   * application prior to beginning playback, and is expressed in units of
   * time(milliseconds).
   */
  int64_t starting_buffer_level_ms;
  /*!
   * Indicates the amount of data that the encoder should try to maintain in the
   * decoder's buffer, and is expressed in units of time(milliseconds).
   */
  int64_t optimal_buffer_level_ms;
  /*!
   * Indicates the maximum amount of data that may be buffered by the decoding
   * application, and is expressed in units of time(milliseconds).
   */
  int64_t maximum_buffer_size_ms;

  /*!
   * Indicates the bandwidth to be used in bits per second.
   */
  int64_t target_bandwidth;

  /*!
   * Indicates average complexity of the corpus in single pass vbr based on
   * LAP. 0 indicates that corpus complexity vbr mode is disabled.
   */
  unsigned int vbr_corpus_complexity_lap;
  /*!
   * Indicates the maximum allowed bitrate for any intra frame as % of bitrate
   * target.
   */
  unsigned int max_intra_bitrate_pct;
  /*!
   * Indicates the maximum allowed bitrate for any inter frame as % of bitrate
   * target.
   */
  unsigned int max_inter_bitrate_pct;
  /*!
   * Indicates the percentage of rate boost for golden frame in CBR mode.
   */
  unsigned int gf_cbr_boost_pct;
  /*!
   * min_cr / 100 indicates the target minimum compression ratio for each
   * frame.
   */
  unsigned int min_cr;
  /*!
   * Indicates the frame drop threshold.
   */
  int drop_frames_water_mark;
  /*!
   * under_shoot_pct indicates the tolerance of the VBR algorithm to
   * undershoot and is used as a trigger threshold for more aggressive
   * adaptation of Q. It's value can range from 0-100.
   */
  int under_shoot_pct;
  /*!
   * over_shoot_pct indicates the tolerance of the VBR algorithm to overshoot
   * and is used as a trigger threshold for more aggressive adaptation of Q.
   * It's value can range from 0-1000.
   */
  int over_shoot_pct;
  /*!
   * Indicates the maximum qindex that can be used by the quantizer i.e. the
   * worst quality qindex.
   */
  int worst_allowed_q;
  /*!
   * Indicates the minimum qindex that can be used by the quantizer i.e. the
   * best quality qindex.
   */
  int best_allowed_q;
  /*!
   * Indicates the Constant/Constrained Quality level.
   */
  int cq_level;
  /*!
   * Indicates if the encoding mode is vbr, cbr, constrained quality or
   * constant quality.
   */
  enum aom_rc_mode mode;
  /*!
   * Indicates the bias (expressed on a scale of 0 to 100) for determining
   * target size for the current frame. The value 0 indicates the optimal CBR
   * mode value should be used, and 100 indicates the optimal VBR mode value
   * should be used.
   */
  int vbrbias;
  /*!
   * Indicates the minimum bitrate to be used for a single frame as a percentage
   * of the target bitrate.
   */
  int vbrmin_section;
  /*!
   * Indicates the maximum bitrate to be used for a single frame as a percentage
   * of the target bitrate.
   */
  int vbrmax_section;

  /*!
   * Indicates the maximum consecutive amount of frame drops, in units of time
   * (milliseconds). This is converted to frame units internally. Only used in
   * CBR mode.
   */
  int max_consec_drop_ms;
} RateControlCfg;

/*!\cond */
typedef struct {
  // Indicates the number of frames lag before encoding is started.
  int lag_in_frames;
  // Indicates the minimum gf/arf interval to be used.
  int min_gf_interval;
  // Indicates the maximum gf/arf interval to be used.
  int max_gf_interval;
  // Indicates the minimum height for GF group pyramid structure to be used.
  int gf_min_pyr_height;
  // Indicates the maximum height for GF group pyramid structure to be used.
  int gf_max_pyr_height;
  // Indicates if automatic set and use of altref frames should be enabled.
  bool enable_auto_arf;
  // Indicates if automatic set and use of (b)ackward (r)ef (f)rames should be
  // enabled.
  bool enable_auto_brf;
} GFConfig;

typedef struct {
  // Indicates the number of tile groups.
  unsigned int num_tile_groups;
  // Indicates the MTU size for a tile group. If mtu is non-zero,
  // num_tile_groups is set to DEFAULT_MAX_NUM_TG.
  unsigned int mtu;
  // Indicates the number of tile columns in log2.
  int tile_columns;
  // Indicates the number of tile rows in log2.
  int tile_rows;
  // Indicates the number of widths in the tile_widths[] array.
  int tile_width_count;
  // Indicates the number of heights in the tile_heights[] array.
  int tile_height_count;
  // Indicates the tile widths, and may be empty.
  int tile_widths[MAX_TILE_COLS];
  // Indicates the tile heights, and may be empty.
  int tile_heights[MAX_TILE_ROWS];
  // Indicates if large scale tile coding should be used.
  bool enable_large_scale_tile;
  // Indicates if single tile decoding mode should be enabled.
  bool enable_single_tile_decoding;
  // Indicates if EXT_TILE_DEBUG should be enabled.
  bool enable_ext_tile_debug;
} TileConfig;

typedef struct {
  // Indicates the width of the input frame.
  int width;
  // Indicates the height of the input frame.
  int height;
  // If forced_max_frame_width is non-zero then it is used to force the maximum
  // frame width written in write_sequence_header().
  int forced_max_frame_width;
  // If forced_max_frame_width is non-zero then it is used to force the maximum
  // frame height written in write_sequence_header().
  int forced_max_frame_height;
  // Indicates the frame width after applying both super-resolution and resize
  // to the coded frame.
  int render_width;
  // Indicates the frame height after applying both super-resolution and resize
  // to the coded frame.
  int render_height;
} FrameDimensionCfg;

typedef struct {
  // Indicates if warped motion should be enabled.
  bool enable_warped_motion;
  // Indicates if warped motion should be evaluated or not.
  bool allow_warped_motion;
  // Indicates if OBMC motion should be enabled.
  bool enable_obmc;
} MotionModeCfg;

typedef struct {
  // Timing info for each frame.
  aom_timing_info_t timing_info;
  // Indicates the number of time units of a decoding clock.
  uint32_t num_units_in_decoding_tick;
  // Indicates if decoder model information is present in the coded sequence
  // header.
  bool decoder_model_info_present_flag;
  // Indicates if display model information is present in the coded sequence
  // header.
  bool display_model_info_present_flag;
  // Indicates if timing info for each frame is present.
  bool timing_info_present;
} DecoderModelCfg;

typedef struct {
  // Indicates the update frequency for coeff costs.
  COST_UPDATE_TYPE coeff;
  // Indicates the update frequency for mode costs.
  COST_UPDATE_TYPE mode;
  // Indicates the update frequency for mv costs.
  COST_UPDATE_TYPE mv;
  // Indicates the update frequency for dv costs.
  COST_UPDATE_TYPE dv;
} CostUpdateFreq;

typedef struct {
  // Indicates the maximum number of reference frames allowed per frame.
  unsigned int max_reference_frames;
  // Indicates if the reduced set of references should be enabled.
  bool enable_reduced_reference_set;
  // Indicates if one-sided compound should be enabled.
  bool enable_onesided_comp;
} RefFrameCfg;

typedef struct {
  // Indicates the color space that should be used.
  aom_color_primaries_t color_primaries;
  // Indicates the characteristics of transfer function to be used.
  aom_transfer_characteristics_t transfer_characteristics;
  // Indicates the matrix coefficients to be used for the transfer function.
  aom_matrix_coefficients_t matrix_coefficients;
  // Indicates the chroma 4:2:0 sample position info.
  aom_chroma_sample_position_t chroma_sample_position;
  // Indicates if a limited color range or full color range should be used.
  aom_color_range_t color_range;
} ColorCfg;

typedef struct {
  // Indicates if extreme motion vector unit test should be enabled or not.
  unsigned int motion_vector_unit_test;
  // Indicates if superblock multipass unit test should be enabled or not.
  unsigned int sb_multipass_unit_test;
} UnitTestCfg;

typedef struct {
  // Indicates the file path to the VMAF model.
  const char *vmaf_model_path;
  // Indicates the path to the film grain parameters.
  const char *film_grain_table_filename;
  // Indicates the visual tuning metric.
  aom_tune_metric tuning;
  // Indicates if the current content is screen or default type.
  aom_tune_content content;
  // Indicates the film grain parameters.
  int film_grain_test_vector;
  // Indicates the in-block distortion metric to use.
  aom_dist_metric dist_metric;
} TuneCfg;

typedef struct {
  // Indicates the framerate of the input video.
  double init_framerate;
  // Indicates the bit-depth of the input video.
  unsigned int input_bit_depth;
  // Indicates the maximum number of frames to be encoded.
  unsigned int limit;
  // Indicates the chrome subsampling x value.
  unsigned int chroma_subsampling_x;
  // Indicates the chrome subsampling y value.
  unsigned int chroma_subsampling_y;
} InputCfg;

typedef struct {
  // If true, encoder will use fixed QP offsets, that are either:
  // - Given by the user, and stored in 'fixed_qp_offsets' array, OR
  // - Picked automatically from cq_level.
  int use_fixed_qp_offsets;
  // Indicates the minimum flatness of the quantization matrix.
  int qm_minlevel;
  // Indicates the maximum flatness of the quantization matrix.
  int qm_maxlevel;
  // Indicates if adaptive quantize_b should be enabled.
  int quant_b_adapt;
  // Indicates the Adaptive Quantization mode to be used.
  AQ_MODE aq_mode;
  // Indicates the delta q mode to be used.
  DELTAQ_MODE deltaq_mode;
  // Indicates the delta q mode strength.
  DELTAQ_MODE deltaq_strength;
  // Indicates if delta quantization should be enabled in chroma planes.
  bool enable_chroma_deltaq;
  // Indicates if delta quantization should be enabled for hdr video
  bool enable_hdr_deltaq;
  // Indicates if encoding with quantization matrices should be enabled.
  bool using_qm;
} QuantizationCfg;

/*!\endcond */
/*!
 * \brief Algorithm configuration parameters.
 */
typedef struct {
  /*!
   * Controls the level at which rate-distortion optimization of transform
   * coefficients favors sharpness in the block. Has no impact on RD when set
   * to zero (default).
   *
   * For values 1-7, eob and skip block optimization are
   * avoided and rdmult is adjusted in favor of block sharpness.
   *
   * In all-intra mode: it also sets the `loop_filter_sharpness` syntax element
   * in the bitstream. Larger values increasingly reduce how much the filtering
   * can change the sample values on block edges to favor perceived sharpness.
   */
  int sharpness;

  /*!
   * Indicates the trellis optimization mode of quantized coefficients.
   * 0: disabled
   * 1: enabled
   * 2: enabled for rd search
   * 3: true for estimate yrd search
   */
  int disable_trellis_quant;

  /*!
   * The maximum number of frames used to create an arf.
   */
  int arnr_max_frames;

  /*!
   * The temporal filter strength for arf used when creating ARFs.
   */
  int arnr_strength;

  /*!
   * Indicates the CDF update mode
   * 0: no update
   * 1: update on every frame(default)
   * 2: selectively update
   */
  uint8_t cdf_update_mode;

  /*!
   * Indicates if RDO based on frame temporal dependency should be enabled.
   */
  bool enable_tpl_model;

  /*!
   * Indicates if coding of overlay frames for filtered ALTREF frames is
   * enabled.
   */
  bool enable_overlay;

  /*!
   * Controls loop filtering
   * 0: Loop filter is disabled for all frames
   * 1: Loop filter is enabled for all frames
   * 2: Loop filter is disabled for non-reference frames
   * 3: Loop filter is disables for the frames with low motion
   */
  LOOPFILTER_CONTROL loopfilter_control;

  /*!
   * Indicates if the application of post-processing filters should be skipped
   * on reconstructed frame.
   */
  bool skip_postproc_filtering;
} AlgoCfg;
/*!\cond */

typedef struct {
  // Indicates the codec bit-depth.
  aom_bit_depth_t bit_depth;
  // Indicates the superblock size that should be used by the encoder.
  aom_superblock_size_t superblock_size;
  // Indicates if loopfilter modulation should be enabled.
  bool enable_deltalf_mode;
  // Indicates how CDEF should be applied.
  CDEF_CONTROL cdef_control;
  // Indicates if loop restoration filter should be enabled.
  bool enable_restoration;
  // When enabled, video mode should be used even for single frame input.
  bool force_video_mode;
  // Indicates if the error resiliency features should be enabled.
  bool error_resilient_mode;
  // Indicates if frame parallel decoding feature should be enabled.
  bool frame_parallel_decoding_mode;
  // Indicates if the input should be encoded as monochrome.
  bool enable_monochrome;
  // When enabled, the encoder will use a full header even for still pictures.
  // When disabled, a reduced header is used for still pictures.
  bool full_still_picture_hdr;
  // Indicates if dual interpolation filters should be enabled.
  bool enable_dual_filter;
  // Indicates if frame order hint should be enabled or not.
  bool enable_order_hint;
  // Indicates if ref_frame_mvs should be enabled at the sequence level.
  bool ref_frame_mvs_present;
  // Indicates if ref_frame_mvs should be enabled at the frame level.
  bool enable_ref_frame_mvs;
  // Indicates if interintra compound mode is enabled.
  bool enable_interintra_comp;
  // Indicates if global motion should be enabled.
  bool enable_global_motion;
  // Indicates if palette should be enabled.
  bool enable_palette;
} ToolCfg;

/*!\endcond */
/*!
 * \brief Main encoder configuration data structure.
 */
typedef struct AV1EncoderConfig {
  /*!\cond */
  // Configuration related to the input video.
  InputCfg input_cfg;

  // Configuration related to frame-dimensions.
  FrameDimensionCfg frm_dim_cfg;

  /*!\endcond */
  /*!
   * Encoder algorithm configuration.
   */
  AlgoCfg algo_cfg;

  /*!
   * Configuration related to key-frames.
   */
  KeyFrameCfg kf_cfg;

  /*!
   * Rate control configuration
   */
  RateControlCfg rc_cfg;
  /*!\cond */

  // Configuration related to Quantization.
  QuantizationCfg q_cfg;

  // Internal frame size scaling.
  ResizeCfg resize_cfg;

  // Frame Super-Resolution size scaling.
  SuperResCfg superres_cfg;

  /*!\endcond */
  /*!
   * stats_in buffer contains all of the stats packets produced in the first
   * pass, concatenated.
   */
  aom_fixed_buf_t twopass_stats_in;
  /*!\cond */

  // Configuration related to encoder toolsets.
  ToolCfg tool_cfg;

  // Configuration related to Group of frames.
  GFConfig gf_cfg;

  // Tile related configuration parameters.
  TileConfig tile_cfg;

  // Configuration related to Tune.
  TuneCfg tune_cfg;

  // Configuration related to color.
  ColorCfg color_cfg;

  // Configuration related to decoder model.
  DecoderModelCfg dec_model_cfg;

  // Configuration related to reference frames.
  RefFrameCfg ref_frm_cfg;

  // Configuration related to unit tests.
  UnitTestCfg unit_test_cfg;

  // Flags related to motion mode.
  MotionModeCfg motion_mode_cfg;

  // Flags related to intra mode search.
  IntraModeCfg intra_mode_cfg;

  // Flags related to transform size/type.
  TxfmSizeTypeCfg txfm_cfg;

  // Flags related to compound type.
  CompoundTypeCfg comp_type_cfg;

  // Partition related information.
  PartitionCfg part_cfg;

  // Configuration related to frequency of cost update.
  CostUpdateFreq cost_upd_freq;

#if CONFIG_DENOISE
  // Indicates the noise level.
  float noise_level;
  // Indicates the the denoisers block size.
  int noise_block_size;
  // Indicates whether to apply denoising to the frame to be encoded
  int enable_dnl_denoising;
#endif

#if CONFIG_AV1_TEMPORAL_DENOISING
  // Noise sensitivity.
  int noise_sensitivity;
#endif
  // Bit mask to specify which tier each of the 32 possible operating points
  // conforms to.
  unsigned int tier_mask;

  // Indicates the number of pixels off the edge of a reference frame we're
  // allowed to go when forming an inter prediction.
  int border_in_pixels;

  // Indicates the maximum number of threads that may be used by the encoder.
  int max_threads;

  // Indicates the speed preset to be used.
  int speed;

  // Enable the low complexity decode mode.
  unsigned int enable_low_complexity_decode;

  // Indicates the target sequence level index for each operating point(OP).
  AV1_LEVEL target_seq_level_idx[MAX_NUM_OPERATING_POINTS];

  // Indicates the bitstream profile to be used.
  BITSTREAM_PROFILE profile;

  /*!\endcond */
  /*!
   * Indicates the current encoder pass :
   * AOM_RC_ONE_PASS = One pass encode,
   * AOM_RC_FIRST_PASS = First pass of multiple-pass
   * AOM_RC_SECOND_PASS = Second pass of multiple-pass
   * AOM_RC_THIRD_PASS = Third pass of multiple-pass
   */
  enum aom_enc_pass pass;
  /*!\cond */

  // Total number of encoding passes.
  int passes;

  // the name of the second pass output file when passes > 2
  const char *two_pass_output;

  // the name of the second pass log file when passes > 2
  const char *second_pass_log;

  // Indicates if the encoding is GOOD or REALTIME.
  MODE mode;

  // Indicates if row-based multi-threading should be enabled or not.
  bool row_mt;

  // Indicates if frame parallel multi-threading should be enabled or not.
  bool fp_mt;

  // Indicates if 16bit frame buffers are to be used i.e., the content is >
  // 8-bit.
  bool use_highbitdepth;

  // Indicates the bitstream syntax mode. 0 indicates bitstream is saved as
  // Section 5 bitstream, while 1 indicates the bitstream is saved in Annex - B
  // format.
  bool save_as_annexb;

  // The path for partition stats reading and writing, used in the experiment
  // CONFIG_PARTITION_SEARCH_ORDER.
  const char *partition_info_path;

  // The flag that indicates whether we use an external rate distribution to
  // guide adaptive quantization. It requires --deltaq-mode=3. The rate
  // distribution map file name is stored in |rate_distribution_info|.
  unsigned int enable_rate_guide_deltaq;

  // The input file of rate distribution information used in all intra mode
  // to determine delta quantization.
  const char *rate_distribution_info;

  // Exit the encoder when it fails to encode to a given level.
  int strict_level_conformance;

  // Max depth for the GOP after a key frame
  int kf_max_pyr_height;

  // A flag to control if we enable the superblock qp sweep for a given lambda
  int sb_qp_sweep;
  /*!\endcond */
} AV1EncoderConfig;

/*!\cond */
static inline int is_lossless_requested(const RateControlCfg *const rc_cfg) {
  return rc_cfg->best_allowed_q == 0 && rc_cfg->worst_allowed_q == 0;
}
/*!\endcond */

/*!
 * \brief Encoder-side probabilities for pruning of various AV1 tools
 */
typedef struct {
  /*!
   * obmc_probs[i][j] is the probability of OBMC being the best motion mode for
   * jth block size and ith frame update type, averaged over past frames. If
   * obmc_probs[i][j] < thresh, then OBMC search is pruned.
   */
  int obmc_probs[FRAME_UPDATE_TYPES][BLOCK_SIZES_ALL];

  /*!
   * warped_probs[i] is the probability of warped motion being the best motion
   * mode for ith frame update type, averaged over past frames. If
   * warped_probs[i] < thresh, then warped motion search is pruned.
   */
  int warped_probs[FRAME_UPDATE_TYPES];

  /*!
   * tx_type_probs[i][j][k] is the probability of kth tx_type being the best
   * for jth transform size and ith frame update type, averaged over past
   * frames. If tx_type_probs[i][j][k] < thresh, then transform search for that
   * type is pruned.
   */
  int tx_type_probs[FRAME_UPDATE_TYPES][TX_SIZES_ALL][TX_TYPES];

  /*!
   * switchable_interp_probs[i][j][k] is the probability of kth interpolation
   * filter being the best for jth filter context and ith frame update type,
   * averaged over past frames. If switchable_interp_probs[i][j][k] < thresh,
   * then interpolation filter search is pruned for that case.
   */
  int switchable_interp_probs[FRAME_UPDATE_TYPES][SWITCHABLE_FILTER_CONTEXTS]
                             [SWITCHABLE_FILTERS];
} FrameProbInfo;

/*!\cond */

typedef struct FRAME_COUNTS {
// Note: This structure should only contain 'unsigned int' fields, or
// aggregates built solely from 'unsigned int' fields/elements
#if CONFIG_ENTROPY_STATS
  unsigned int kf_y_mode[KF_MODE_CONTEXTS][KF_MODE_CONTEXTS][INTRA_MODES];
  unsigned int angle_delta[DIRECTIONAL_MODES][2 * MAX_ANGLE_DELTA + 1];
  unsigned int y_mode[BLOCK_SIZE_GROUPS][INTRA_MODES];
  unsigned int uv_mode[CFL_ALLOWED_TYPES][INTRA_MODES][UV_INTRA_MODES];
  unsigned int cfl_sign[CFL_JOINT_SIGNS];
  unsigned int cfl_alpha[CFL_ALPHA_CONTEXTS][CFL_ALPHABET_SIZE];
  unsigned int palette_y_mode[PALATTE_BSIZE_CTXS][PALETTE_Y_MODE_CONTEXTS][2];
  unsigned int palette_uv_mode[PALETTE_UV_MODE_CONTEXTS][2];
  unsigned int palette_y_size[PALATTE_BSIZE_CTXS][PALETTE_SIZES];
  unsigned int palette_uv_size[PALATTE_BSIZE_CTXS][PALETTE_SIZES];
  unsigned int palette_y_color_index[PALETTE_SIZES]
                                    [PALETTE_COLOR_INDEX_CONTEXTS]
                                    [PALETTE_COLORS];
  unsigned int palette_uv_color_index[PALETTE_SIZES]
                                     [PALETTE_COLOR_INDEX_CONTEXTS]
                                     [PALETTE_COLORS];
  unsigned int partition[PARTITION_CONTEXTS][EXT_PARTITION_TYPES];
  unsigned int txb_skip[TOKEN_CDF_Q_CTXS][TX_SIZES][TXB_SKIP_CONTEXTS][2];
  unsigned int eob_extra[TOKEN_CDF_Q_CTXS][TX_SIZES][PLANE_TYPES]
                        [EOB_COEF_CONTEXTS][2];
  unsigned int dc_sign[PLANE_TYPES][DC_SIGN_CONTEXTS][2];
  unsigned int coeff_lps[TX_SIZES][PLANE_TYPES][BR_CDF_SIZE - 1][LEVEL_CONTEXTS]
                        [2];
  unsigned int eob_flag[TX_SIZES][PLANE_TYPES][EOB_COEF_CONTEXTS][2];
  unsigned int eob_multi16[TOKEN_CDF_Q_CTXS][PLANE_TYPES][2][5];
  unsigned int eob_multi32[TOKEN_CDF_Q_CTXS][PLANE_TYPES][2][6];
  unsigned int eob_multi64[TOKEN_CDF_Q_CTXS][PLANE_TYPES][2][7];
  unsigned int eob_multi128[TOKEN_CDF_Q_CTXS][PLANE_TYPES][2][8];
  unsigned int eob_multi256[TOKEN_CDF_Q_CTXS][PLANE_TYPES][2][9];
  unsigned int eob_multi512[TOKEN_CDF_Q_CTXS][PLANE_TYPES][2][10];
  unsigned int eob_multi1024[TOKEN_CDF_Q_CTXS][PLANE_TYPES][2][11];
  unsigned int coeff_lps_multi[TOKEN_CDF_Q_CTXS][TX_SIZES][PLANE_TYPES]
                              [LEVEL_CONTEXTS][BR_CDF_SIZE];
  unsigned int coeff_base_multi[TOKEN_CDF_Q_CTXS][TX_SIZES][PLANE_TYPES]
                               [SIG_COEF_CONTEXTS][NUM_BASE_LEVELS + 2];
  unsigned int coeff_base_eob_multi[TOKEN_CDF_Q_CTXS][TX_SIZES][PLANE_TYPES]
                                   [SIG_COEF_CONTEXTS_EOB][NUM_BASE_LEVELS + 1];
  unsigned int newmv_mode[NEWMV_MODE_CONTEXTS][2];
  unsigned int zeromv_mode[GLOBALMV_MODE_CONTEXTS][2];
  unsigned int refmv_mode[REFMV_MODE_CONTEXTS][2];
  unsigned int drl_mode[DRL_MODE_CONTEXTS][2];
  unsigned int inter_compound_mode[INTER_MODE_CONTEXTS][INTER_COMPOUND_MODES];
  unsigned int wedge_idx[BLOCK_SIZES_ALL][16];
  unsigned int interintra[BLOCK_SIZE_GROUPS][2];
  unsigned int interintra_mode[BLOCK_SIZE_GROUPS][INTERINTRA_MODES];
  unsigned int wedge_interintra[BLOCK_SIZES_ALL][2];
  unsigned int compound_type[BLOCK_SIZES_ALL][MASKED_COMPOUND_TYPES];
  unsigned int motion_mode[BLOCK_SIZES_ALL][MOTION_MODES];
  unsigned int obmc[BLOCK_SIZES_ALL][2];
  unsigned int intra_inter[INTRA_INTER_CONTEXTS][2];
  unsigned int comp_inter[COMP_INTER_CONTEXTS][2];
  unsigned int comp_ref_type[COMP_REF_TYPE_CONTEXTS][2];
  unsigned int uni_comp_ref[UNI_COMP_REF_CONTEXTS][UNIDIR_COMP_REFS - 1][2];
  unsigned int single_ref[REF_CONTEXTS][SINGLE_REFS - 1][2];
  unsigned int comp_ref[REF_CONTEXTS][FWD_REFS - 1][2];
  unsigned int comp_bwdref[REF_CONTEXTS][BWD_REFS - 1][2];
  unsigned int intrabc[2];

  unsigned int txfm_partition[TXFM_PARTITION_CONTEXTS][2];
  unsigned int intra_tx_size[MAX_TX_CATS][TX_SIZE_CONTEXTS][MAX_TX_DEPTH + 1];
  unsigned int skip_mode[SKIP_MODE_CONTEXTS][2];
  unsigned int skip_txfm[SKIP_CONTEXTS][2];
  unsigned int compound_index[COMP_INDEX_CONTEXTS][2];
  unsigned int comp_group_idx[COMP_GROUP_IDX_CONTEXTS][2];
  unsigned int delta_q[DELTA_Q_PROBS][2];
  unsigned int delta_lf_multi[FRAME_LF_COUNT][DELTA_LF_PROBS][2];
  unsigned int delta_lf[DELTA_LF_PROBS][2];

  unsigned int inter_ext_tx[EXT_TX_SETS_INTER][EXT_TX_SIZES][TX_TYPES];
  unsigned int intra_ext_tx[EXT_TX_SETS_INTRA][EXT_TX_SIZES][INTRA_MODES]
                           [TX_TYPES];
  unsigned int filter_intra_mode[FILTER_INTRA_MODES];
  unsigned int filter_intra[BLOCK_SIZES_ALL][2];
  unsigned int switchable_restore[RESTORE_SWITCHABLE_TYPES];
  unsigned int wiener_restore[2];
  unsigned int sgrproj_restore[2];
#endif  // CONFIG_ENTROPY_STATS

  unsigned int switchable_interp[SWITCHABLE_FILTER_CONTEXTS]
                                [SWITCHABLE_FILTERS];
} FRAME_COUNTS;

#define INTER_MODE_RD_DATA_OVERALL_SIZE 6400

typedef struct {
  int ready;
  double a;
  double b;
  double dist_mean;
  double ld_mean;
  double sse_mean;
  double sse_sse_mean;
  double sse_ld_mean;
  int num;
  double dist_sum;
  double ld_sum;
  double sse_sum;
  double sse_sse_sum;
  double sse_ld_sum;
} InterModeRdModel;

typedef struct {
  int idx;
  int64_t rd;
} RdIdxPair;
// TODO(angiebird): This is an estimated size. We still need to figure what is
// the maximum number of modes.
#define MAX_INTER_MODES 1024
// TODO(any): rename this struct to something else. There is already another
// struct called inter_mode_info, which makes this terribly confusing.
/*!\endcond */
/*!
 * \brief Struct used to hold inter mode data for fast tx search.
 *
 * This struct is used to perform a full transform search only on winning
 * candidates searched with an estimate for transform coding RD.
 */
typedef struct inter_modes_info {
  /*!
   * The number of inter modes for which data was stored in each of the
   * following arrays.
   */
  int num;
  /*!
   * Mode info struct for each of the candidate modes.
   */
  MB_MODE_INFO mbmi_arr[MAX_INTER_MODES];
  /*!
   * The rate for each of the candidate modes.
   */
  int mode_rate_arr[MAX_INTER_MODES];
  /*!
   * The sse of the predictor for each of the candidate modes.
   */
  int64_t sse_arr[MAX_INTER_MODES];
  /*!
   * The estimated rd of the predictor for each of the candidate modes.
   */
  int64_t est_rd_arr[MAX_INTER_MODES];
  /*!
   * The rate and mode index for each of the candidate modes.
   */
  RdIdxPair rd_idx_pair_arr[MAX_INTER_MODES];
  /*!
   * The full rd stats for each of the candidate modes.
   */
  RD_STATS rd_cost_arr[MAX_INTER_MODES];
  /*!
   * The full rd stats of luma only for each of the candidate modes.
   */
  RD_STATS rd_cost_y_arr[MAX_INTER_MODES];
  /*!
   * The full rd stats of chroma only for each of the candidate modes.
   */
  RD_STATS rd_cost_uv_arr[MAX_INTER_MODES];
} InterModesInfo;

/*!\cond */
typedef struct {
  // TODO(kyslov): consider changing to 64bit

  // This struct is used for computing variance in choose_partitioning(), where
  // the max number of samples within a superblock is 32x32 (with 4x4 avg).
  // With 8bit bitdepth, uint32_t is enough for sum_square_error (2^8 * 2^8 * 32
  // * 32 = 2^26). For high bitdepth we need to consider changing this to 64 bit
  uint32_t sum_square_error;
  int32_t sum_error;
  int log2_count;
  int variance;
} VPartVar;

typedef struct {
  VPartVar none;
  VPartVar horz[2];
  VPartVar vert[2];
} VPVariance;

typedef struct {
  VPVariance part_variances;
  VPartVar split[4];
} VP4x4;

typedef struct {
  VPVariance part_variances;
  VP4x4 split[4];
} VP8x8;

typedef struct {
  VPVariance part_variances;
  VP8x8 split[4];
} VP16x16;

typedef struct {
  VPVariance part_variances;
  VP16x16 split[4];
} VP32x32;

typedef struct {
  VPVariance part_variances;
  VP32x32 split[4];
} VP64x64;

typedef struct {
  VPVariance part_variances;
  VP64x64 *split;
} VP128x128;

/*!\endcond */

/*!
 * \brief Thresholds for variance based partitioning.
 */
typedef struct {
  /*!
   * If block variance > threshold, then that block is forced to split.
   * thresholds[0] - threshold for 128x128;
   * thresholds[1] - threshold for 64x64;
   * thresholds[2] - threshold for 32x32;
   * thresholds[3] - threshold for 16x16;
   * thresholds[4] - threshold for 8x8;
   */
  int64_t thresholds[5];

  /*!
   * MinMax variance threshold for 8x8 sub blocks of a 16x16 block. If actual
   * minmax > threshold_minmax, the 16x16 is forced to split.
   */
  int64_t threshold_minmax;
} VarBasedPartitionInfo;

/*!
 * \brief Encoder parameters for synchronization of row based multi-threading
 */
typedef struct {
#if CONFIG_MULTITHREAD
  /**
   * \name Synchronization objects for top-right dependency.
   */
  /**@{*/
  pthread_mutex_t *mutex_; /*!< Mutex lock object */
  pthread_cond_t *cond_;   /*!< Condition variable */
  /**@}*/
#endif  // CONFIG_MULTITHREAD
  /*!
   * Buffer to store the superblock whose encoding is complete.
   * num_finished_cols[i] stores the number of superblocks which finished
   * encoding in the ith superblock row.
   */
  int *num_finished_cols;
  /*!
   * Denotes the superblock interval at which conditional signalling should
   * happen. Also denotes the minimum number of extra superblocks of the top row
   * to be complete to start encoding the current superblock. A value of 1
   * indicates top-right dependency.
   */
  int sync_range;
  /*!
   * Denotes the additional number of superblocks in the previous row to be
   * complete to start encoding the current superblock when intraBC tool is
   * enabled. This additional top-right delay is required to satisfy the
   * hardware constraints for intraBC tool when row multithreading is enabled.
   */
  int intrabc_extra_top_right_sb_delay;
  /*!
   * Number of superblock rows.
   */
  int rows;
  /*!
   * The superblock row (in units of MI blocks) to be processed next.
   */
  int next_mi_row;
  /*!
   * Number of threads processing the current tile.
   */
  int num_threads_working;
} AV1EncRowMultiThreadSync;

/*!\cond */

// TODO(jingning) All spatially adaptive variables should go to TileDataEnc.
typedef struct TileDataEnc {
  TileInfo tile_info;
  DECLARE_ALIGNED(16, FRAME_CONTEXT, tctx);
  FRAME_CONTEXT *row_ctx;
  uint64_t abs_sum_level;
  uint8_t allow_update_cdf;
  InterModeRdModel inter_mode_rd_models[BLOCK_SIZES_ALL];
  AV1EncRowMultiThreadSync row_mt_sync;
  MV firstpass_top_mv;
} TileDataEnc;

typedef struct RD_COUNTS {
  int compound_ref_used_flag;
  int skip_mode_used_flag;
  int tx_type_used[TX_SIZES_ALL][TX_TYPES];
  int obmc_used[BLOCK_SIZES_ALL][2];
  int warped_used[2];
  int newmv_or_intra_blocks;
  uint64_t seg_tmp_pred_cost[2];
} RD_COUNTS;

typedef struct ThreadData {
  MACROBLOCK mb;
  MvCosts *mv_costs_alloc;
  IntraBCMVCosts *dv_costs_alloc;
  RD_COUNTS rd_counts;
  FRAME_COUNTS *counts;
  PC_TREE_SHARED_BUFFERS shared_coeff_buf;
  SIMPLE_MOTION_DATA_TREE *sms_tree;
  SIMPLE_MOTION_DATA_TREE *sms_root;
  uint32_t *hash_value_buffer[2][2];
  OBMCBuffer obmc_buffer;
  PALETTE_BUFFER *palette_buffer;
  CompoundTypeRdBuffers comp_rd_buffer;
  CONV_BUF_TYPE *tmp_conv_dst;
  uint64_t abs_sum_level;
  uint8_t *tmp_pred_bufs[2];
  uint8_t *wiener_tmp_pred_buf;
  int intrabc_used;
  int deltaq_used;
  int coefficient_size;
  int max_mv_magnitude;
  int interp_filter_selected[SWITCHABLE];
  FRAME_CONTEXT *tctx;
  VP64x64 *vt64x64;
  int32_t num_64x64_blocks;
  PICK_MODE_CONTEXT *firstpass_ctx;
  TemporalFilterData tf_data;
  TplBuffers tpl_tmp_buffers;
  TplTxfmStats tpl_txfm_stats;
  GlobalMotionData gm_data;
  // Pointer to the array of structures to store gradient information of each
  // pixel in a superblock. The buffer constitutes of MAX_SB_SQUARE pixel level
  // structures for each of the plane types (PLANE_TYPE_Y and PLANE_TYPE_UV).
  PixelLevelGradientInfo *pixel_gradient_info;
  // Pointer to the array of structures to store source variance information of
  // each 4x4 sub-block in a superblock. Block4x4VarInfo structure is used to
  // store source variance and log of source variance of each 4x4 sub-block
  // for subsequent retrieval.
  Block4x4VarInfo *src_var_info_of_4x4_sub_blocks;
  // Pointer to pc tree root.
  PC_TREE *pc_root;
} ThreadData;

struct EncWorkerData;

/*!\endcond */

/*!
 * \brief Encoder data related to row-based multi-threading
 */
typedef struct {
  /*!
   * Number of tile rows for which row synchronization memory is allocated.
   */
  int allocated_tile_rows;
  /*!
   * Number of tile cols for which row synchronization memory is allocated.
   */
  int allocated_tile_cols;
  /*!
   * Number of rows for which row synchronization memory is allocated
   * per tile. During first-pass/look-ahead stage this equals the
   * maximum number of macroblock rows in a tile. During encode stage,
   * this equals the maximum number of superblock rows in a tile.
   */
  int allocated_rows;
  /*!
   * Number of columns for which entropy context memory is allocated
   * per tile. During encode stage, this equals the maximum number of
   * superblock columns in a tile minus 1. The entropy context memory
   * is not allocated during first-pass/look-ahead stage.
   */
  int allocated_cols;

  /*!
   * thread_id_to_tile_id[i] indicates the tile id assigned to the ith thread.
   */
  int thread_id_to_tile_id[MAX_NUM_THREADS];

  /*!
   * num_tile_cols_done[i] indicates the number of tile columns whose encoding
   * is complete in the ith superblock row.
   */
  int *num_tile_cols_done;

  /*!
   * Number of superblock rows in a frame for which 'num_tile_cols_done' is
   * allocated.
   */
  int allocated_sb_rows;

  /*!
   * Initialized to false, set to true by the worker thread that encounters an
   * error in order to abort the processing of other worker threads.
   */
  bool row_mt_exit;

  /*!
   * Initialized to false, set to true during first pass encoding by the worker
   * thread that encounters an error in order to abort the processing of other
   * worker threads.
   */
  bool firstpass_mt_exit;

  /*!
   * Initialized to false, set to true in cal_mb_wiener_var_hook() by the worker
   * thread that encounters an error in order to abort the processing of other
   * worker threads.
   */
  bool mb_wiener_mt_exit;

#if CONFIG_MULTITHREAD
  /*!
   * Mutex lock used while dispatching jobs.
   */
  pthread_mutex_t *mutex_;
  /*!
   *  Condition variable used to dispatch loopfilter jobs.
   */
  pthread_cond_t *cond_;
#endif

  /**
   * \name Row synchronization related function pointers.
   */
  /**@{*/
  /*!
   * Reader.
   */
  void (*sync_read_ptr)(AV1EncRowMultiThreadSync *const, int, int);
  /*!
   * Writer.
   */
  void (*sync_write_ptr)(AV1EncRowMultiThreadSync *const, int, int, int);
  /**@}*/
} AV1EncRowMultiThreadInfo;

/*!
 * \brief Encoder data related to multi-threading for allintra deltaq-mode=3
 */
typedef struct {
#if CONFIG_MULTITHREAD
  /*!
   * Mutex lock used while dispatching jobs.
   */
  pthread_mutex_t *mutex_;
  /*!
   *  Condition variable used to dispatch loopfilter jobs.
   */
  pthread_cond_t *cond_;
#endif

  /**
   * \name Row synchronization related function pointers for all intra mode
   */
  /**@{*/
  /*!
   * Reader.
   */
  void (*intra_sync_read_ptr)(AV1EncRowMultiThreadSync *const, int, int);
  /*!
   * Writer.
   */
  void (*intra_sync_write_ptr)(AV1EncRowMultiThreadSync *const, int, int, int);
  /**@}*/
} AV1EncAllIntraMultiThreadInfo;

/*!
 * \brief Max number of recodes used to track the frame probabilities.
 */
#define NUM_RECODES_PER_FRAME 10

/*!
 * \brief Max number of frames that can be encoded in a parallel encode set.
 */
#define MAX_PARALLEL_FRAMES 4

/*!
 * \brief Buffers to be backed up during parallel encode set to be restored
 * later.
 */
typedef struct RestoreStateBuffers {
  /*!
   * Backup of original CDEF srcbuf.
   */
  uint16_t *cdef_srcbuf;

  /*!
   * Backup of original CDEF colbuf.
   */
  uint16_t *cdef_colbuf[MAX_MB_PLANE];

  /*!
   * Backup of original LR rst_tmpbuf.
   */
  int32_t *rst_tmpbuf;

  /*!
   * Backup of original LR rlbs.
   */
  RestorationLineBuffers *rlbs;
} RestoreStateBuffers;

/*!
 * \brief Parameters related to restoration types.
 */
typedef struct {
  /*!
   * Stores the best coefficients for Wiener restoration.
   */
  WienerInfo wiener;

  /*!
   * Stores the best coefficients for Sgrproj restoration.
   */
  SgrprojInfo sgrproj;

  /*!
   * The rtype to use for this unit given a frame rtype as index. Indices:
   * WIENER, SGRPROJ, SWITCHABLE.
   */
  RestorationType best_rtype[RESTORE_TYPES - 1];
} RestUnitSearchInfo;

/*!
 * \brief Structure to hold search parameter per restoration unit and
 * intermediate buffer of Wiener filter used in pick filter stage of Loop
 * restoration.
 */
typedef struct {
  /*!
   * Array of pointers to 'RestUnitSearchInfo' which holds data related to
   * restoration types.
   */
  RestUnitSearchInfo *rusi[MAX_MB_PLANE];

  /*!
   * Buffer used to hold dgd-avg data during SIMD call of Wiener filter.
   */
  int16_t *dgd_avg;
} AV1LrPickStruct;

/*!
 * \brief Primary Encoder parameters related to multi-threading.
 */
typedef struct PrimaryMultiThreadInfo {
  /*!
   * Number of workers created for multi-threading.
   */
  int num_workers;

  /*!
   * Number of workers used for different MT modules.
   */
  int num_mod_workers[NUM_MT_MODULES];

  /*!
   * Synchronization object used to launch job in the worker thread.
   */
  AVxWorker *workers;

  /*!
   * Data specific to each worker in encoder multi-threading.
   * tile_thr_data[i] stores the worker data of the ith thread.
   */
  struct EncWorkerData *tile_thr_data;

  /*!
   * CDEF row multi-threading data.
   */
  AV1CdefWorkerData *cdef_worker;

  /*!
   * Primary(Level 1) Synchronization object used to launch job in the worker
   * thread.
   */
  AVxWorker *p_workers[MAX_PARALLEL_FRAMES];

  /*!
   * Number of primary workers created for multi-threading.
   */
  int p_num_workers;

  /*!
   * Tracks the number of workers in encode stage multi-threading.
   */
  int prev_num_enc_workers;
} PrimaryMultiThreadInfo;

/*!
 * \brief Encoder parameters related to multi-threading.
 */
typedef struct MultiThreadInfo {
  /*!
   * Number of workers created for multi-threading.
   */
  int num_workers;

  /*!
   * Number of workers used for different MT modules.
   */
  int num_mod_workers[NUM_MT_MODULES];

  /*!
   * Synchronization object used to launch job in the worker thread.
   */
  AVxWorker *workers;

  /*!
   * Data specific to each worker in encoder multi-threading.
   * tile_thr_data[i] stores the worker data of the ith thread.
   */
  struct EncWorkerData *tile_thr_data;

  /*!
   * When set, indicates that row based multi-threading of the encoder is
   * enabled.
   */
  bool row_mt_enabled;

  /*!
   * When set, indicates that multi-threading for bitstream packing is enabled.
   */
  bool pack_bs_mt_enabled;

  /*!
   * Encoder row multi-threading data.
   */
  AV1EncRowMultiThreadInfo enc_row_mt;

  /*!
   * Encoder multi-threading data for allintra mode in the preprocessing stage
   * when --deltaq-mode=3.
   */
  AV1EncAllIntraMultiThreadInfo intra_mt;

  /*!
   * Tpl row multi-threading data.
   */
  AV1TplRowMultiThreadInfo tpl_row_mt;

  /*!
   * Loop Filter multi-threading object.
   */
  AV1LfSync lf_row_sync;

  /*!
   * Loop Restoration multi-threading object.
   */
  AV1LrSync lr_row_sync;

  /*!
   * Pack bitstream multi-threading object.
   */
  AV1EncPackBSSync pack_bs_sync;

  /*!
   * Global Motion multi-threading object.
   */
  AV1GlobalMotionSync gm_sync;

  /*!
   * Temporal Filter multi-threading object.
   */
  AV1TemporalFilterSync tf_sync;

  /*!
   * CDEF search multi-threading object.
   */
  AV1CdefSync cdef_sync;

  /*!
   * Pointer to CDEF row multi-threading data for the frame.
   */
  AV1CdefWorkerData *cdef_worker;

  /*!
   * Buffers to be stored/restored before/after parallel encode.
   */
  RestoreStateBuffers restore_state_buf;

  /*!
   * In multi-threaded realtime encoding with row-mt enabled, pipeline
   * loop-filtering after encoding.
   */
  int pipeline_lpf_mt_with_enc;
} MultiThreadInfo;

/*!\cond */

typedef struct ActiveMap {
  int enabled;
  int update;
  unsigned char *map;
} ActiveMap;

/*!\endcond */

/*!
 * \brief Encoder info used for decision on forcing integer motion vectors.
 */
typedef struct {
  /*!
   * cs_rate_array[i] is the fraction of blocks in a frame which either match
   * with the collocated block or are smooth, where i is the rate_index.
   */
  double cs_rate_array[32];
  /*!
   * rate_index is used to index cs_rate_array.
   */
  int rate_index;
  /*!
   * rate_size is the total number of entries populated in cs_rate_array.
   */
  int rate_size;
} ForceIntegerMVInfo;

/*!\cond */

#if CONFIG_INTERNAL_STATS
// types of stats
enum {
  STAT_Y,
  STAT_U,
  STAT_V,
  STAT_ALL,
  NUM_STAT_TYPES  // This should always be the last member of the enum
} UENUM1BYTE(StatType);

typedef struct IMAGE_STAT {
  double stat[NUM_STAT_TYPES];
  double worst;
} ImageStat;
#endif  // CONFIG_INTERNAL_STATS

typedef struct {
  int ref_count;
  YV12_BUFFER_CONFIG buf;
} EncRefCntBuffer;

/*!\endcond */

/*!
 * \brief Buffer to store mode information at mi_alloc_bsize (4x4 or 8x8) level
 *
 * This is used for bitstream preparation.
 */
typedef struct {
  /*!
   * frame_base[mi_row * stride + mi_col] stores the mode information of
   * block (mi_row,mi_col).
   */
  MB_MODE_INFO_EXT_FRAME *frame_base;
  /*!
   * Size of frame_base buffer.
   */
  int alloc_size;
  /*!
   * Stride of frame_base buffer.
   */
  int stride;
} MBMIExtFrameBufferInfo;

/*!\cond */

#if CONFIG_COLLECT_PARTITION_STATS
typedef struct FramePartitionTimingStats {
  int partition_decisions[6][EXT_PARTITION_TYPES];
  int partition_attempts[6][EXT_PARTITION_TYPES];
  int64_t partition_times[6][EXT_PARTITION_TYPES];

  int partition_redo;
} FramePartitionTimingStats;
#endif  // CONFIG_COLLECT_PARTITION_STATS

#if CONFIG_COLLECT_COMPONENT_TIMING
#include "aom_ports/aom_timer.h"
// Adjust the following to add new components.
enum {
  av1_encode_strategy_time,
  av1_get_one_pass_rt_params_time,
  av1_get_second_pass_params_time,
  denoise_and_encode_time,
  apply_filtering_time,
  av1_tpl_setup_stats_time,
  encode_frame_to_data_rate_time,
  encode_with_or_without_recode_time,
  loop_filter_time,
  cdef_time,
  loop_restoration_time,
  av1_pack_bitstream_final_time,
  av1_encode_frame_time,
  av1_compute_global_motion_time,
  av1_setup_motion_field_time,
  encode_sb_row_time,

  rd_pick_partition_time,
  rd_use_partition_time,
  choose_var_based_partitioning_time,
  av1_prune_partitions_time,
  none_partition_search_time,
  split_partition_search_time,
  rectangular_partition_search_time,
  ab_partitions_search_time,
  rd_pick_4partition_time,
  encode_sb_time,

  rd_pick_sb_modes_time,
  av1_rd_pick_intra_mode_sb_time,
  av1_rd_pick_inter_mode_sb_time,
  set_params_rd_pick_inter_mode_time,
  skip_inter_mode_time,
  handle_inter_mode_time,
  evaluate_motion_mode_for_winner_candidates_time,
  do_tx_search_time,
  handle_intra_mode_time,
  refine_winner_mode_tx_time,
  av1_search_palette_mode_time,
  handle_newmv_time,
  compound_type_rd_time,
  interpolation_filter_search_time,
  motion_mode_rd_time,

  nonrd_use_partition_time,
  pick_sb_modes_nonrd_time,
  hybrid_intra_mode_search_time,
  nonrd_pick_inter_mode_sb_time,
  encode_b_nonrd_time,

  kTimingComponents,
} UENUM1BYTE(TIMING_COMPONENT);

static inline char const *get_component_name(int index) {
  switch (index) {
    case av1_encode_strategy_time: return "av1_encode_strategy_time";
    case av1_get_one_pass_rt_params_time:
      return "av1_get_one_pass_rt_params_time";
    case av1_get_second_pass_params_time:
      return "av1_get_second_pass_params_time";
    case denoise_and_encode_time: return "denoise_and_encode_time";
    case apply_filtering_time: return "apply_filtering_time";
    case av1_tpl_setup_stats_time: return "av1_tpl_setup_stats_time";
    case encode_frame_to_data_rate_time:
      return "encode_frame_to_data_rate_time";
    case encode_with_or_without_recode_time:
      return "encode_with_or_without_recode_time";
    case loop_filter_time: return "loop_filter_time";
    case cdef_time: return "cdef_time";
    case loop_restoration_time: return "loop_restoration_time";
    case av1_pack_bitstream_final_time: return "av1_pack_bitstream_final_time";
    case av1_encode_frame_time: return "av1_encode_frame_time";
    case av1_compute_global_motion_time:
      return "av1_compute_global_motion_time";
    case av1_setup_motion_field_time: return "av1_setup_motion_field_time";
    case encode_sb_row_time: return "encode_sb_row_time";

    case rd_pick_partition_time: return "rd_pick_partition_time";
    case rd_use_partition_time: return "rd_use_partition_time";
    case choose_var_based_partitioning_time:
      return "choose_var_based_partitioning_time";
    case av1_prune_partitions_time: return "av1_prune_partitions_time";
    case none_partition_search_time: return "none_partition_search_time";
    case split_partition_search_time: return "split_partition_search_time";
    case rectangular_partition_search_time:
      return "rectangular_partition_search_time";
    case ab_partitions_search_time: return "ab_partitions_search_time";
    case rd_pick_4partition_time: return "rd_pick_4partition_time";
    case encode_sb_time: return "encode_sb_time";

    case rd_pick_sb_modes_time: return "rd_pick_sb_modes_time";
    case av1_rd_pick_intra_mode_sb_time:
      return "av1_rd_pick_intra_mode_sb_time";
    case av1_rd_pick_inter_mode_sb_time:
      return "av1_rd_pick_inter_mode_sb_time";
    case set_params_rd_pick_inter_mode_time:
      return "set_params_rd_pick_inter_mode_time";
    case skip_inter_mode_time: return "skip_inter_mode_time";
    case handle_inter_mode_time: return "handle_inter_mode_time";
    case evaluate_motion_mode_for_winner_candidates_time:
      return "evaluate_motion_mode_for_winner_candidates_time";
    case do_tx_search_time: return "do_tx_search_time";
    case handle_intra_mode_time: return "handle_intra_mode_time";
    case refine_winner_mode_tx_time: return "refine_winner_mode_tx_time";
    case av1_search_palette_mode_time: return "av1_search_palette_mode_time";
    case handle_newmv_time: return "handle_newmv_time";
    case compound_type_rd_time: return "compound_type_rd_time";
    case interpolation_filter_search_time:
      return "interpolation_filter_search_time";
    case motion_mode_rd_time: return "motion_mode_rd_time";

    case nonrd_use_partition_time: return "nonrd_use_partition_time";
    case pick_sb_modes_nonrd_time: return "pick_sb_modes_nonrd_time";
    case hybrid_intra_mode_search_time: return "hybrid_intra_mode_search_time";
    case nonrd_pick_inter_mode_sb_time: return "nonrd_pick_inter_mode_sb_time";
    case encode_b_nonrd_time: return "encode_b_nonrd_time";

    default: assert(0);
  }
  return "error";
}
#endif

// The maximum number of internal ARFs except ALTREF_FRAME
#define MAX_INTERNAL_ARFS (REF_FRAMES - BWDREF_FRAME - 1)

/*!\endcond */

/*!
 * \brief Parameters related to global motion search
 */
typedef struct {
  /*!
   * Flag to indicate if global motion search needs to be rerun.
   */
  bool search_done;

  /*!
   * Array of pointers to the frame buffers holding the reference frames.
   * ref_buf[i] stores the pointer to the reference frame of the ith
   * reference frame type.
   */
  YV12_BUFFER_CONFIG *ref_buf[REF_FRAMES];

  /*!
   * Holds the number of valid reference frames in past and future directions
   * w.r.t. the current frame. num_ref_frames[i] stores the total number of
   * valid reference frames in 'i' direction.
   */
  int num_ref_frames[MAX_DIRECTIONS];

  /*!
   * Array of structure which stores the valid reference frames in past and
   * future directions and their corresponding distance from the source frame.
   * reference_frames[i][j] holds the jth valid reference frame type in the
   * direction 'i' and its temporal distance from the source frame .
   */
  FrameDistPair reference_frames[MAX_DIRECTIONS][REF_FRAMES - 1];

  /**
   * \name Dimensions for which segment map is allocated.
   */
  /**@{*/
  int segment_map_w; /*!< segment map width */
  int segment_map_h; /*!< segment map height */
  /**@}*/
} GlobalMotionInfo;

/*!
 * \brief Flags related to interpolation filter search
 */
typedef struct {
  /*!
   * Stores the default value of skip flag depending on chroma format
   * Set as 1 for monochrome and 3 for other color formats
   */
  int default_interp_skip_flags;
  /*!
   * Filter mask to allow certain interp_filter type.
   */
  uint16_t interp_filter_search_mask;
} InterpSearchFlags;

/*!
 * \brief Parameters for motion vector search process
 */
typedef struct {
  /*!
   * Largest MV component used in a frame.
   * The value from the previous frame is used to set the full pixel search
   * range for the current frame.
   */
  int max_mv_magnitude;
  /*!
   * Parameter indicating initial search window to be used in full-pixel search.
   * Range [0, MAX_MVSEARCH_STEPS-2]. Lower value indicates larger window.
   */
  int mv_step_param;
  /*!
   * Pointer to sub-pixel search function.
   * In encoder: av1_find_best_sub_pixel_tree
   *             av1_find_best_sub_pixel_tree_pruned
   *             av1_find_best_sub_pixel_tree_pruned_more
   * In MV unit test: av1_return_max_sub_pixel_mv
   *                  av1_return_min_sub_pixel_mv
   */
  fractional_mv_step_fp *find_fractional_mv_step;
  /*!
   * Search site configuration for full-pel MV search.
   * search_site_cfg[SS_CFG_SRC]: Used in tpl, rd/non-rd inter mode loop, simple
   * motion search. search_site_cfg[SS_CFG_LOOKAHEAD]: Used in intraBC, temporal
   * filter search_site_cfg[SS_CFG_FPF]: Used during first pass and lookahead
   */
  search_site_config search_site_cfg[SS_CFG_TOTAL][NUM_DISTINCT_SEARCH_METHODS];
} MotionVectorSearchParams;

/*!
 * \brief Refresh frame flags for different type of frames.
 *
 * If the refresh flag is true for a particular reference frame, after the
 * current frame is encoded, the reference frame gets refreshed (updated) to
 * be the current frame. Note: Usually at most one flag will be set to true at
 * a time. But, for key-frames, all flags are set to true at once.
 */
typedef struct {
  bool golden_frame;  /*!< Refresh flag for golden frame */
  bool bwd_ref_frame; /*!< Refresh flag for bwd-ref frame */
  bool alt_ref_frame; /*!< Refresh flag for alt-ref frame */
} RefreshFrameInfo;

/*!
 * \brief Desired dimensions for an externally triggered resize.
 *
 * When resize is triggered externally, the desired dimensions are stored in
 * this struct until used in the next frame to be coded. These values are
 * effective only for one frame and are reset after they are used.
 */
typedef struct {
  int width;  /*!< Desired resized width */
  int height; /*!< Desired resized height */
} ResizePendingParams;

/*!
 * \brief Refrence frame distance related variables.
 */
typedef struct {
  /*!
   * True relative distance of reference frames w.r.t. the current frame.
   */
  int ref_relative_dist[INTER_REFS_PER_FRAME];
  /*!
   * The nearest reference w.r.t. current frame in the past.
   */
  int8_t nearest_past_ref;
  /*!
   * The nearest reference w.r.t. current frame in the future.
   */
  int8_t nearest_future_ref;
} RefFrameDistanceInfo;

/*!
 * \brief Parameters used for winner mode processing.
 *
 * This is a basic two pass approach: in the first pass, we reduce the number of
 * transform searches based on some thresholds during the rdopt process to find
 * the  "winner mode". In the second pass, we perform a more through tx search
 * on the winner mode.
 * There are some arrays in the struct, and their indices are used in the
 * following manner:
 * Index 0: Default mode evaluation, Winner mode processing is not applicable
 * (Eg : IntraBc).
 * Index 1: Mode evaluation.
 * Index 2: Winner mode evaluation
 * Index 1 and 2 are only used when the respective speed feature is on.
 */
typedef struct {
  /*!
   * Threshold to determine if trellis optimization is to be enabled
   * based on :
   * 0 : dist threshold
   * 1 : satd threshold
   * Corresponds to enable_winner_mode_for_coeff_opt speed feature.
   */
  unsigned int coeff_opt_thresholds[MODE_EVAL_TYPES][2];

  /*!
   * Determines the tx size search method during rdopt.
   * Corresponds to enable_winner_mode_for_tx_size_srch speed feature.
   */
  TX_SIZE_SEARCH_METHOD tx_size_search_methods[MODE_EVAL_TYPES];

  /*!
   * Controls how often we should approximate prediction error with tx
   * coefficients. If it's 0, then never. If 1, then it's during the tx_type
   * search only. If 2, then always.
   * Corresponds to tx_domain_dist_level speed feature.
   */
  unsigned int use_transform_domain_distortion[MODE_EVAL_TYPES];

  /*!
   * Threshold to approximate pixel domain distortion with transform domain
   * distortion. This is only used if use_transform_domain_distortion is on.
   * Corresponds to enable_winner_mode_for_use_tx_domain_dist speed feature.
   */
  unsigned int tx_domain_dist_threshold[MODE_EVAL_TYPES];

  /*!
   * Controls how often we should try to skip the transform process based on
   * result from dct.
   * Corresponds to use_skip_flag_prediction speed feature.
   */
  unsigned int skip_txfm_level[MODE_EVAL_TYPES];

  /*!
   * Predict DC only txfm blocks for default, mode and winner mode evaluation.
   * Index 0: Default mode evaluation, Winner mode processing is not applicable.
   * Index 1: Mode evaluation, Index 2: Winner mode evaluation
   */
  unsigned int predict_dc_level[MODE_EVAL_TYPES];
} WinnerModeParams;

/*!
 * \brief Frame refresh flags set by the external interface.
 *
 * Flags set by external interface to determine which reference buffers are
 * refreshed by this frame. When set, the encoder will update the particular
 * reference frame buffer with the contents of the current frame.
 */
typedef struct {
  bool last_frame;     /*!< Refresh flag for last frame */
  bool golden_frame;   /*!< Refresh flag for golden frame */
  bool bwd_ref_frame;  /*!< Refresh flag for bwd-ref frame */
  bool alt2_ref_frame; /*!< Refresh flag for alt2-ref frame */
  bool alt_ref_frame;  /*!< Refresh flag for alt-ref frame */
  /*!
   * Flag indicating if the update of refresh frame flags is pending.
   */
  bool update_pending;
} ExtRefreshFrameFlagsInfo;

/*!
 * \brief Flags signalled by the external interface at frame level.
 */
typedef struct {
  /*!
   * Bit mask to disable certain reference frame types.
   */
  int ref_frame_flags;

  /*!
   * Frame refresh flags set by the external interface.
   */
  ExtRefreshFrameFlagsInfo refresh_frame;

  /*!
   * Flag to enable the update of frame contexts at the end of a frame decode.
   */
  bool refresh_frame_context;

  /*!
   * Flag to indicate that update of refresh_frame_context from external
   * interface is pending.
   */
  bool refresh_frame_context_pending;

  /*!
   * Flag to enable temporal MV prediction.
   */
  bool use_ref_frame_mvs;

  /*!
   * Indicates whether the current frame is to be coded as error resilient.
   */
  bool use_error_resilient;

  /*!
   * Indicates whether the current frame is to be coded as s-frame.
   */
  bool use_s_frame;

  /*!
   * Indicates whether the current frame's primary_ref_frame is set to
   * PRIMARY_REF_NONE.
   */
  bool use_primary_ref_none;
} ExternalFlags;

/*!\cond */

typedef struct {
  // Some misc info
  int high_prec;
  int q;
  int order;

  // MV counters
  int inter_count;
  int intra_count;
  int default_mvs;
  int mv_joint_count[4];
  int last_bit_zero;
  int last_bit_nonzero;

  // Keep track of the rates
  int total_mv_rate;
  int hp_total_mv_rate;
  int lp_total_mv_rate;

  // Texture info
  int horz_text;
  int vert_text;
  int diag_text;

  // Whether the current struct contains valid data
  int valid;
} MV_STATS;

typedef struct WeberStats {
  int64_t mb_wiener_variance;
  int64_t src_variance;
  int64_t rec_variance;
  int16_t src_pix_max;
  int16_t rec_pix_max;
  int64_t distortion;
  int64_t satd;
  double max_scale;
} WeberStats;

typedef struct {
  struct loopfilter lf;
  CdefInfo cdef_info;
  YV12_BUFFER_CONFIG copy_buffer;
  RATE_CONTROL rc;
  MV_STATS mv_stats;
} CODING_CONTEXT;

typedef struct {
  int frame_width;
  int frame_height;
  int mi_rows;
  int mi_cols;
  int mb_rows;
  int mb_cols;
  int num_mbs;
  aom_bit_depth_t bit_depth;
  int subsampling_x;
  int subsampling_y;
} FRAME_INFO;

/*!
 * \brief This structure stores different types of frame indices.
 */
typedef struct {
  int show_frame_count;
} FRAME_INDEX_SET;

/*!\endcond */

/*!
 * \brief Segmentation related information for the current frame.
 */
typedef struct {
  /*!
   * 3-bit number containing the segment affiliation for each 4x4 block in the
   * frame. map[y * stride + x] contains the segment id of the 4x4 block at
   * (x,y) position.
   */
  uint8_t *map;
  /*!
   * Flag to indicate if current frame has lossless segments or not.
   * 1: frame has at least one lossless segment.
   * 0: frame has no lossless segments.
   */
  bool has_lossless_segment;
} EncSegmentationInfo;

/*!
 * \brief Frame time stamps.
 */
typedef struct {
  /*!
   * Start time stamp of the previous frame
   */
  int64_t prev_ts_start;
  /*!
   * End time stamp of the previous frame
   */
  int64_t prev_ts_end;
  /*!
   * Start time stamp of the first frame
   */
  int64_t first_ts_start;
} TimeStamps;

/*!
 * Pointers to the memory allocated for frame level transform coeff related
 * info.
 */
typedef struct {
  /*!
   * Pointer to the transformed coefficients buffer.
   */
  tran_low_t *tcoeff;
  /*!
   * Pointer to the eobs buffer.
   */
  uint16_t *eobs;
  /*!
   * Pointer to the entropy_ctx buffer.
   */
  uint8_t *entropy_ctx;
} CoeffBufferPool;

#if !CONFIG_REALTIME_ONLY
/*!\cond */
// DUCKY_ENCODE_FRAME_MODE is c version of EncodeFrameMode
enum {
  DUCKY_ENCODE_FRAME_MODE_NONE,  // Let native AV1 determine q index and rdmult
  DUCKY_ENCODE_FRAME_MODE_QINDEX,  // DuckyEncode determines q index and AV1
                                   // determines rdmult
  DUCKY_ENCODE_FRAME_MODE_QINDEX_RDMULT,  // DuckyEncode determines q index and
                                          // rdmult
} UENUM1BYTE(DUCKY_ENCODE_FRAME_MODE);

enum {
  DUCKY_ENCODE_GOP_MODE_NONE,  // native AV1 decides GOP
  DUCKY_ENCODE_GOP_MODE_RCL,   // rate control lib decides GOP
} UENUM1BYTE(DUCKY_ENCODE_GOP_MODE);

typedef struct DuckyEncodeFrameInfo {
  DUCKY_ENCODE_FRAME_MODE qp_mode;
  DUCKY_ENCODE_GOP_MODE gop_mode;
  int q_index;
  int rdmult;
  // These two arrays are equivalent to std::vector<SuperblockEncodeParameters>
  int *superblock_encode_qindex;
  int *superblock_encode_rdmult;
  int delta_q_enabled;
} DuckyEncodeFrameInfo;

typedef struct DuckyEncodeFrameResult {
  int global_order_idx;
  int q_index;
  int rdmult;
  int rate;
  int64_t dist;
  double psnr;
} DuckyEncodeFrameResult;

typedef struct DuckyEncodeInfo {
  DuckyEncodeFrameInfo frame_info;
  DuckyEncodeFrameResult frame_result;
} DuckyEncodeInfo;
/*!\endcond */
#endif

/*!\cond */
typedef struct RTC_REF {
  /*!
   * LAST_FRAME (0), LAST2_FRAME(1), LAST3_FRAME(2), GOLDEN_FRAME(3),
   * BWDREF_FRAME(4), ALTREF2_FRAME(5), ALTREF_FRAME(6).
   */
  int reference[INTER_REFS_PER_FRAME];
  int ref_idx[INTER_REFS_PER_FRAME];
  int refresh[REF_FRAMES];
  int set_ref_frame_config;
  int non_reference_frame;
  int ref_frame_comp[3];
  int gld_idx_1layer;
  /*!
   * Frame number of the last frame that refreshed the buffer slot.
   */
  unsigned int buffer_time_index[REF_FRAMES];
  /*!
   * Spatial layer id of the last frame that refreshed the buffer slot.
   */
  unsigned char buffer_spatial_layer[REF_FRAMES];
  /*!
   * Flag to indicate whether closest reference was the previous frame.
   */
  bool reference_was_previous_frame;
  /*!
   * Flag to indicate this frame is based on longer term reference only,
   * for recovery from past loss, and it should be biased for improved coding.
   */
  bool bias_recovery_frame;
} RTC_REF;
/*!\endcond */

/*!
 * \brief Structure to hold data corresponding to an encoded frame.
 */
typedef struct AV1_COMP_DATA {
  /*!
   * Buffer to store packed bitstream data of a frame.
   */
  unsigned char *cx_data;

  /*!
   * Allocated size of the cx_data buffer.
   */
  size_t cx_data_sz;

  /*!
   * Size of data written in the cx_data buffer.
   */
  size_t frame_size;

  /*!
   * Flags for the frame.
   */
  unsigned int lib_flags;

  /*!
   * Time stamp for start of frame.
   */
  int64_t ts_frame_start;

  /*!
   * Time stamp for end of frame.
   */
  int64_t ts_frame_end;

  /*!
   * Flag to indicate flush call.
   */
  int flush;

  /*!
   * Time base for sequence.
   */
  const aom_rational64_t *timestamp_ratio;

  /*!
   * Decide to pop the source for this frame from input buffer queue.
   */
  int pop_lookahead;
} AV1_COMP_DATA;

/*!
 * \brief Top level primary encoder structure
 */
typedef struct AV1_PRIMARY {
  /*!
   * Array of frame level encoder stage top level structures
   */
  struct AV1_COMP *parallel_cpi[MAX_PARALLEL_FRAMES];

  /*!
   * Array of structures to hold data of frames encoded in a given parallel
   * encode set.
   */
  struct AV1_COMP_DATA parallel_frames_data[MAX_PARALLEL_FRAMES - 1];
#if CONFIG_FPMT_TEST
  /*!
   * Flag which enables/disables simulation path for fpmt unit test.
   * 0 - FPMT integration
   * 1 - FPMT simulation
   */
  FPMT_TEST_ENC_CFG fpmt_unit_test_cfg;

  /*!
   * Temporary variable simulating the delayed frame_probability update.
   */
  FrameProbInfo temp_frame_probs;

  /*!
   * Temporary variable holding the updated frame probability across
   * frames. Copy its value to temp_frame_probs for frame_parallel_level 0
   * frames or last frame in parallel encode set.
   */
  FrameProbInfo temp_frame_probs_simulation;

  /*!
   * Temporary variable simulating the delayed update of valid global motion
   * model across frames.
   */
  int temp_valid_gm_model_found[FRAME_UPDATE_TYPES];
#endif  // CONFIG_FPMT_TEST
  /*!
   * Copy of cm->ref_frame_map maintained to facilitate sequential update of
   * ref_frame_map by lower layer depth frames encoded ahead of time in a
   * parallel encode set.
   */
  RefCntBuffer *ref_frame_map_copy[REF_FRAMES];

  /*!
   * Start time stamp of the last encoded show frame
   */
  int64_t ts_start_last_show_frame;

  /*!
   * End time stamp of the last encoded show frame
   */
  int64_t ts_end_last_show_frame;

  /*!
   * Number of frame level contexts(cpis)
   */
  int num_fp_contexts;

  /*!
   * Loopfilter levels of the previous encoded frame.
   */
  int filter_level[2];

  /*!
   * Chrominance component loopfilter level of the previous encoded frame.
   */
  int filter_level_u;

  /*!
   * Chrominance component loopfilter level of the previous encoded frame.
   */
  int filter_level_v;

  /*!
   * Encode stage top level structure
   * During frame parallel encode, this is the same as parallel_cpi[0]
   */
  struct AV1_COMP *cpi;

  /*!
   * Lookahead processing stage top level structure
   */
  struct AV1_COMP *cpi_lap;

  /*!
   * Look-ahead context.
   */
  struct lookahead_ctx *lookahead;

  /*!
   * Sequence parameters have been transmitted already and locked
   * or not. Once locked av1_change_config cannot change the seq
   * parameters. Note that for SVC encoding the sequence parameters
   * (operating_points_cnt_minus_1, operating_point_idc[],
   * has_nonzero_operating_point_idc) should be updated whenever the
   * number of layers is changed. This is done in the
   * ctrl_set_svc_params().
   */
  int seq_params_locked;

  /*!
   * Pointer to internal utility functions that manipulate aom_codec_* data
   * structures.
   */
  struct aom_codec_pkt_list *output_pkt_list;

  /*!
   * When set, indicates that internal ARFs are enabled.
   */
  int internal_altref_allowed;

  /*!
   * Tell if OVERLAY frame shows existing alt_ref frame.
   */
  int show_existing_alt_ref;

  /*!
   * Information related to a gf group.
   */
  GF_GROUP gf_group;

  /*!
   * Track prior gf group state.
   */
  GF_STATE gf_state;

  /*!
   * Flag indicating whether look ahead processing (LAP) is enabled.
   */
  int lap_enabled;

  /*!
   * Parameters for AV1 bitstream levels.
   */
  AV1LevelParams level_params;

  /*!
   * Calculates PSNR on each frame when set to 1.
   */
  int b_calculate_psnr;

  /*!
   * Number of frames left to be encoded, is 0 if limit is not set.
   */
  int frames_left;

  /*!
   * Information related to two pass encoding.
   */
  TWO_PASS twopass;

  /*!
   * Rate control related parameters.
   */
  PRIMARY_RATE_CONTROL p_rc;

  /*!
   * Info and resources used by temporal filtering.
   */
  TEMPORAL_FILTER_INFO tf_info;
  /*!
   * Elements part of the sequence header, that are applicable for all the
   * frames in the video.
   */
  SequenceHeader seq_params;

  /*!
   * Indicates whether to use SVC.
   */
  int use_svc;

  /*!
   * If true, buffer removal times are present.
   */
  bool buffer_removal_time_present;

  /*!
   * Number of temporal layers: may be > 1 for SVC (scalable vector coding).
   */
  unsigned int number_temporal_layers;

  /*!
   * Number of spatial layers: may be > 1 for SVC (scalable vector coding).
   */
  unsigned int number_spatial_layers;

  /*!
   * Code and details about current error status.
   */
  struct aom_internal_error_info error;

  /*!
   * Function pointers to variants of sse/sad/variance computation functions.
   * fn_ptr[i] indicates the list of function pointers corresponding to block
   * size i.
   */
  aom_variance_fn_ptr_t fn_ptr[BLOCK_SIZES_ALL];

  /*!
   * tpl_sb_rdmult_scaling_factors[i] stores the RD multiplier scaling factor of
   * the ith 16 x 16 block in raster scan order.
   */
  double *tpl_sb_rdmult_scaling_factors;

  /*!
   * Parameters related to tpl.
   */
  TplParams tpl_data;

  /*!
   * Motion vector stats of the previous encoded frame.
   */
  MV_STATS mv_stats;

#if CONFIG_INTERNAL_STATS
  /*!\cond */
  uint64_t total_time_receive_data;
  uint64_t total_time_compress_data;

  unsigned int total_mode_chosen_counts[MAX_MODES];

  int count[2];
  uint64_t total_sq_error[2];
  uint64_t total_samples[2];
  ImageStat psnr[2];

  double total_blockiness;
  double worst_blockiness;

  uint64_t total_bytes;
  double summed_quality;
  double summed_weights;
  double summed_quality_hbd;
  double summed_weights_hbd;
  unsigned int total_recode_hits;
  double worst_ssim;
  double worst_ssim_hbd;

  ImageStat fastssim;
  ImageStat psnrhvs;

  int b_calculate_blockiness;
  int b_calculate_consistency;

  double total_inconsistency;
  double worst_consistency;
  Ssimv *ssim_vars;
  Metrics metrics;
  /*!\endcond */
#endif

#if CONFIG_ENTROPY_STATS
  /*!
   * Aggregates frame counts for the sequence.
   */
  FRAME_COUNTS aggregate_fc;
#endif  // CONFIG_ENTROPY_STATS

  /*!
   * For each type of reference frame, this contains the index of a reference
   * frame buffer for a reference frame of the same type.  We use this to
   * choose our primary reference frame (which is the most recent reference
   * frame of the same type as the current frame).
   */
  int fb_of_context_type[REF_FRAMES];

  /*!
   * Primary Multi-threading parameters.
   */
  PrimaryMultiThreadInfo p_mt_info;

  /*!
   * Probabilities for pruning of various AV1 tools.
   */
  FrameProbInfo frame_probs;

  /*!
   * Indicates if a valid global motion model has been found in the different
   * frame update types of a GF group.
   * valid_gm_model_found[i] indicates if valid global motion model has been
   * found in the frame update type with enum value equal to i
   */
  int valid_gm_model_found[FRAME_UPDATE_TYPES];

  /*!
   * Struct for the reference structure for RTC.
   */
  RTC_REF rtc_ref;

  /*!
   * Struct for all intra mode row multi threading in the preprocess stage
   * when --deltaq-mode=3.
   */
  AV1EncRowMultiThreadSync intra_row_mt_sync;
} AV1_PRIMARY;

/*!
 * \brief Top level encoder structure.
 */
typedef struct AV1_COMP {
  /*!
   * Pointer to top level primary encoder structure
   */
  AV1_PRIMARY *ppi;

  /*!
   * Quantization and dequantization parameters for internal quantizer setup
   * in the encoder.
   */
  EncQuantDequantParams enc_quant_dequant_params;

  /*!
   * Structure holding thread specific variables.
   */
  ThreadData td;

  /*!
   * Statistics collected at frame level.
   */
  FRAME_COUNTS counts;

  /*!
   * Holds buffer storing mode information at 4x4/8x8 level.
   */
  MBMIExtFrameBufferInfo mbmi_ext_info;

  /*!
   * Buffer holding the transform block related information.
   * coeff_buffer_base[i] stores the transform block related information of the
   * ith superblock in raster scan order.
   */
  CB_COEFF_BUFFER *coeff_buffer_base;

  /*!
   * Structure holding pointers to frame level memory allocated for transform
   * block related information.
   */
  CoeffBufferPool coeff_buffer_pool;

  /*!
   * Structure holding variables common to encoder and decoder.
   */
  AV1_COMMON common;

  /*!
   * Encoder configuration related parameters.
   */
  AV1EncoderConfig oxcf;

  /*!
   * Stores the trellis optimization type at segment level.
   * optimize_seg_arr[i] stores the trellis opt type for ith segment.
   */
  TRELLIS_OPT_TYPE optimize_seg_arr[MAX_SEGMENTS];

  /*!
   * Pointer to the frame buffer holding the source frame to be used during the
   * current stage of encoding. It can be the raw input, temporally filtered
   * input or scaled input.
   */
  YV12_BUFFER_CONFIG *source;

  /*!
   * Pointer to the frame buffer holding the last raw source frame.
   * last_source is NULL for the following cases:
   * 1) First frame
   * 2) Alt-ref frames
   * 3) All frames for all-intra frame encoding.
   */
  YV12_BUFFER_CONFIG *last_source;

  /*!
   * Pointer to the frame buffer holding the unscaled source frame.
   * It can be either the raw input or temporally filtered input.
   */
  YV12_BUFFER_CONFIG *unscaled_source;

  /*!
   * Frame buffer holding the resized source frame (cropping / superres).
   */
  YV12_BUFFER_CONFIG scaled_source;

  /*!
   * Pointer to the frame buffer holding the unscaled last source frame.
   */
  YV12_BUFFER_CONFIG *unscaled_last_source;

  /*!
   * Frame buffer holding the resized last source frame.
   */
  YV12_BUFFER_CONFIG scaled_last_source;

  /*!
   * Pointer to the original source frame. This is used to determine if the
   * content is screen.
   */
  YV12_BUFFER_CONFIG *unfiltered_source;

  /*!
   * Frame buffer holding the orig source frame for PSNR calculation in rtc tf
   * case.
   */
  YV12_BUFFER_CONFIG orig_source;

  /*!
   * Skip tpl setup when tpl data from gop length decision can be reused.
   */
  int skip_tpl_setup_stats;

  /*!
   * Scaling factors used in the RD multiplier modulation.
   * TODO(sdeng): consider merge the following arrays.
   * tpl_rdmult_scaling_factors is a temporary buffer used to store the
   * intermediate scaling factors which are used in the calculation of
   * tpl_sb_rdmult_scaling_factors. tpl_rdmult_scaling_factors[i] stores the
   * intermediate scaling factor of the ith 16 x 16 block in raster scan order.
   */
  double *tpl_rdmult_scaling_factors;

  /*!
   * Temporal filter context.
   */
  TemporalFilterCtx tf_ctx;

  /*!
   * Pointer to CDEF search context.
   */
  CdefSearchCtx *cdef_search_ctx;

  /*!
   * Variables related to forcing integer mv decisions for the current frame.
   */
  ForceIntegerMVInfo force_intpel_info;

  /*!
   * Pointer to the buffer holding the scaled reference frames.
   * scaled_ref_buf[i] holds the scaled reference frame of type i.
   */
  RefCntBuffer *scaled_ref_buf[INTER_REFS_PER_FRAME];

  /*!
   * Pointer to the buffer holding the last show frame.
   */
  RefCntBuffer *last_show_frame_buf;

  /*!
   * Refresh frame flags for golden, bwd-ref and alt-ref frames.
   */
  RefreshFrameInfo refresh_frame;

  /*!
   * Flag to reduce the number of reference frame buffers used in rt.
   */
  int rt_reduce_num_ref_buffers;

  /*!
   * Flags signalled by the external interface at frame level.
   */
  ExternalFlags ext_flags;

  /*!
   * Temporary frame buffer used to store the non-loop filtered reconstructed
   * frame during the search of loop filter level.
   */
  YV12_BUFFER_CONFIG last_frame_uf;

  /*!
   * Temporary frame buffer used to store the loop restored frame during loop
   * restoration search.
   */
  YV12_BUFFER_CONFIG trial_frame_rst;

  /*!
   * Ambient reconstruction err target for force key frames.
   */
  int64_t ambient_err;

  /*!
   * Parameters related to rate distortion optimization.
   */
  RD_OPT rd;

  /*!
   * Temporary coding context used to save and restore when encoding with and
   * without super-resolution.
   */
  CODING_CONTEXT coding_context;

  /*!
   * Parameters related to global motion search.
   */
  GlobalMotionInfo gm_info;

  /*!
   * Parameters related to winner mode processing.
   */
  WinnerModeParams winner_mode_params;

  /*!
   * Frame time stamps.
   */
  TimeStamps time_stamps;

  /*!
   * Rate control related parameters.
   */
  RATE_CONTROL rc;

  /*!
   * Frame rate of the video.
   */
  double framerate;

  /*!
   * Bitmask indicating which reference buffers may be referenced by this frame.
   */
  int ref_frame_flags;

  /*!
   * speed is passed as a per-frame parameter into the encoder.
   */
  int speed;

  /*!
   * sf contains fine-grained config set internally based on speed.
   */
  SPEED_FEATURES sf;

  /*!
   * Parameters for motion vector search process.
   */
  MotionVectorSearchParams mv_search_params;

  /*!
   * When set, indicates that all reference frames are forward references,
   * i.e., all the reference frames are output before the current frame.
   */
  int all_one_sided_refs;

  /*!
   * Segmentation related information for current frame.
   */
  EncSegmentationInfo enc_seg;

  /*!
   * Parameters related to cyclic refresh aq-mode.
   */
  CYCLIC_REFRESH *cyclic_refresh;
  /*!
   * Parameters related to active map. Active maps indicate
   * if there is any activity on a 4x4 block basis.
   */
  ActiveMap active_map;

  /*!
   * The frame processing order within a GOP.
   */
  unsigned char gf_frame_index;

#if CONFIG_INTERNAL_STATS
  /*!\cond */
  uint64_t time_compress_data;

  unsigned int mode_chosen_counts[MAX_MODES];
  int bytes;
  unsigned int frame_recode_hits;
  /*!\endcond */
#endif

#if CONFIG_SPEED_STATS
  /*!
   * For debugging: number of transform searches we have performed.
   */
  unsigned int tx_search_count;
#endif  // CONFIG_SPEED_STATS

  /*!
   * When set, indicates that the frame is droppable, i.e., this frame
   * does not update any reference buffers.
   */
  int droppable;

  /*!
   * Stores the frame parameters during encoder initialization.
   */
  FRAME_INFO frame_info;

  /*!
   * Stores different types of frame indices.
   */
  FRAME_INDEX_SET frame_index_set;

  /*!
   * Stores the cm->width in the last call of alloc_compressor_data(). Helps
   * determine whether compressor data should be reallocated when cm->width
   * changes.
   */
  int data_alloc_width;

  /*!
   * Stores the cm->height in the last call of alloc_compressor_data(). Helps
   * determine whether compressor data should be reallocated when cm->height
   * changes.
   */
  int data_alloc_height;

  /*!
   * Number of MBs in the full-size frame; to be used to
   * normalize the firstpass stats. This will differ from the
   * number of MBs in the current frame when the frame is
   * scaled.
   */
  int initial_mbs;

  /*!
   * Flag to indicate whether the frame size inforamation has been
   * setup and propagated to associated allocations.
   */
  bool frame_size_related_setup_done;

  /*!
   * The width of the frame that is lastly encoded.
   * It is updated in the function "encoder_encode()".
   */
  int last_coded_width;

  /*!
   * The height of the frame that is lastly encoded.
   * It is updated in the function "encoder_encode()".
   */
  int last_coded_height;

  /*!
   * Resize related parameters.
   */
  ResizePendingParams resize_pending_params;

  /*!
   * Pointer to struct holding adaptive data/contexts/models for the tile during
   * encoding.
   */
  TileDataEnc *tile_data;
  /*!
   * Number of tiles for which memory has been allocated for tile_data.
   */
  int allocated_tiles;

  /*!
   * Structure to store the palette token related information.
   */
  TokenInfo token_info;

  /*!
   * VARIANCE_AQ segment map refresh.
   */
  int vaq_refresh;

  /*!
   * Thresholds for variance based partitioning.
   */
  VarBasedPartitionInfo vbp_info;

  /*!
   * Number of recodes in the frame.
   */
  int num_frame_recode;

  /*!
   * Current frame probability of parallel frames, across recodes.
   */
  FrameProbInfo frame_new_probs[NUM_RECODES_PER_FRAME];

  /*!
   * Retain condition for transform type frame_probability calculation
   */
  int do_update_frame_probs_txtype[NUM_RECODES_PER_FRAME];

  /*!
   * Retain condition for obmc frame_probability calculation
   */
  int do_update_frame_probs_obmc[NUM_RECODES_PER_FRAME];

  /*!
   * Retain condition for warped motion frame_probability calculation
   */
  int do_update_frame_probs_warp[NUM_RECODES_PER_FRAME];

  /*!
   * Retain condition for interpolation filter frame_probability calculation
   */
  int do_update_frame_probs_interpfilter[NUM_RECODES_PER_FRAME];

#if CONFIG_FPMT_TEST
  /*!
   * Temporary variable for simulation.
   * Previous frame's framerate.
   */
  double temp_framerate;
#endif
  /*!
   * Updated framerate for the current parallel frame.
   * cpi->framerate is updated with new_framerate during
   * post encode updates for parallel frames.
   */
  double new_framerate;

  /*!
   * Retain condition for fast_extra_bits calculation.
   */
  int do_update_vbr_bits_off_target_fast;

  /*!
   * Multi-threading parameters.
   */
  MultiThreadInfo mt_info;

  /*!
   * Specifies the frame to be output. It is valid only if show_existing_frame
   * is 1. When show_existing_frame is 0, existing_fb_idx_to_show is set to
   * INVALID_IDX.
   */
  int existing_fb_idx_to_show;

  /*!
   * A flag to indicate if intrabc is ever used in current frame.
   */
  int intrabc_used;

  /*!
   * Mark which ref frames can be skipped for encoding current frame during RDO.
   */
  int prune_ref_frame_mask;

  /*!
   * Loop Restoration context.
   */
  AV1LrStruct lr_ctxt;

  /*!
   * Loop Restoration context used during pick stage.
   */
  AV1LrPickStruct pick_lr_ctxt;

  /*!
   * Pointer to list of tables with film grain parameters.
   */
  aom_film_grain_table_t *film_grain_table;

#if CONFIG_DENOISE
  /*!
   * Pointer to structure holding the denoised image buffers and the helper
   * noise models.
   */
  struct aom_denoise_and_model_t *denoise_and_model;
#endif

  /*!
   * Flags related to interpolation filter search.
   */
  InterpSearchFlags interp_search_flags;

  /*!
   * Turn on screen content tools flag.
   * Note that some videos are not screen content videos, but
   * screen content tools could also improve coding efficiency.
   * For example, videos with large flat regions, gaming videos that look
   * like natural videos.
   */
  int use_screen_content_tools;

  /*!
   * A flag to indicate "real" screen content videos.
   * For example, screen shares, screen editing.
   * This type is true indicates |use_screen_content_tools| must be true.
   * In addition, rate control strategy is adjusted when this flag is true.
   */
  int is_screen_content_type;

#if CONFIG_COLLECT_PARTITION_STATS
  /*!
   * Accumulates the partition timing stat over the whole frame.
   */
  FramePartitionTimingStats partition_stats;
#endif  // CONFIG_COLLECT_PARTITION_STATS

#if CONFIG_COLLECT_COMPONENT_TIMING
  /*!
   * component_time[] are initialized to zero while encoder starts.
   */
  uint64_t component_time[kTimingComponents];
  /*!
   * Stores timing for individual components between calls of start_timing()
   * and end_timing().
   */
  struct aom_usec_timer component_timer[kTimingComponents];
  /*!
   * frame_component_time[] are initialized to zero at beginning of each frame.
   */
  uint64_t frame_component_time[kTimingComponents];
#endif

  /*!
   * Count the number of OBU_FRAME and OBU_FRAME_HEADER for level calculation.
   */
  int frame_header_count;

  /*!
   * Whether any no-zero delta_q was actually used.
   */
  int deltaq_used;

  /*!
   * Refrence frame distance related variables.
   */
  RefFrameDistanceInfo ref_frame_dist_info;

  /*!
   * ssim_rdmult_scaling_factors[i] stores the RD multiplier scaling factor of
   * the ith 16 x 16 block in raster scan order. This scaling factor is used for
   * RD multiplier modulation when SSIM tuning is enabled.
   */
  double *ssim_rdmult_scaling_factors;

#if CONFIG_TUNE_VMAF
  /*!
   * Parameters for VMAF tuning.
   */
  TuneVMAFInfo vmaf_info;
#endif

#if CONFIG_TUNE_BUTTERAUGLI
  /*!
   * Parameters for Butteraugli tuning.
   */
  TuneButteraugliInfo butteraugli_info;
#endif

  /*!
   * Parameters for scalable video coding.
   */
  SVC svc;

  /*!
   * Indicates whether current processing stage is encode stage or LAP stage.
   */
  COMPRESSOR_STAGE compressor_stage;

  /*!
   * Frame type of the last frame. May be used in some heuristics for speeding
   * up the encoding.
   */
  FRAME_TYPE last_frame_type;

  /*!
   * Number of tile-groups.
   */
  int num_tg;

  /*!
   * Super-resolution mode currently being used by the encoder.
   * This may / may not be same as user-supplied mode in oxcf->superres_mode
   * (when we are recoding to try multiple options for example).
   */
  aom_superres_mode superres_mode;

  /*!
   * First pass related data.
   */
  FirstPassData firstpass_data;

  /*!
   * Temporal Noise Estimate
   */
  NOISE_ESTIMATE noise_estimate;

#if CONFIG_AV1_TEMPORAL_DENOISING
  /*!
   * Temporal Denoiser
   */
  AV1_DENOISER denoiser;
#endif

  /*!
   * Count on how many consecutive times a block uses small/zeromv for encoding
   * in a scale of 8x8 block.
   */
  uint8_t *consec_zero_mv;

  /*!
   * Allocated memory size for |consec_zero_mv|.
   */
  int consec_zero_mv_alloc_size;

  /*!
   * Block size of first pass encoding
   */
  BLOCK_SIZE fp_block_size;

  /*!
   * The counter of encoded super block, used to differentiate block names.
   * This number starts from 0 and increases whenever a super block is encoded.
   */
  int sb_counter;

  /*!
   * Available bitstream buffer size in bytes
   */
  size_t available_bs_size;

  /*!
   * The controller of the external partition model.
   * It is used to do partition type selection based on external models.
   */
  ExtPartController ext_part_controller;

  /*!
   * Motion vector stats of the current encoded frame, used to update the
   * ppi->mv_stats during postencode.
   */
  MV_STATS mv_stats;
  /*!
   * Stores the reference refresh index for the current frame.
   */
  int ref_refresh_index;

  /*!
   * A flag to indicate if the reference refresh index is available for the
   * current frame.
   */
  bool refresh_idx_available;

  /*!
   * Reference frame index corresponding to the frame to be excluded from being
   * used as a reference by frame_parallel_level 2 frame in a parallel
   * encode set of lower layer frames.
   */
  int ref_idx_to_skip;
#if CONFIG_FPMT_TEST
  /*!
   * Stores the wanted frame buffer index for choosing primary ref frame by a
   * frame_parallel_level 2 frame in a parallel encode set of lower layer
   * frames.
   */

  int wanted_fb;
#endif  // CONFIG_FPMT_TEST

  /*!
   * A flag to indicate frames that will update their data to the primary
   * context at the end of the encode. It is set for non-parallel frames and the
   * last frame in encode order in a given parallel encode set.
   */
  bool do_frame_data_update;

#if CONFIG_RD_COMMAND
  /*!
   *  A structure for assigning external q_index / rdmult for experiments
   */
  RD_COMMAND rd_command;
#endif  // CONFIG_RD_COMMAND

  /*!
   * Buffer to store MB variance after Wiener filter.
   */
  WeberStats *mb_weber_stats;

  /*!
   * Buffer to store rate cost estimates for each macro block (8x8) in the
   * preprocessing stage used in allintra mode.
   */
  int *prep_rate_estimates;

  /*!
   * Buffer to store rate cost estimates for each 16x16 block read
   * from an external file, used in allintra mode.
   */
  double *ext_rate_distribution;

  /*!
   * The scale that equals sum_rate_uniform_quantizer / sum_ext_rate.
   */
  double ext_rate_scale;

  /*!
   * Buffer to store MB variance after Wiener filter.
   */
  BLOCK_SIZE weber_bsize;

  /*!
   * Frame level Wiener filter normalization.
   */
  int64_t norm_wiener_variance;

  /*!
   * Buffer to store delta-q values for delta-q mode 4.
   */
  int *mb_delta_q;

  /*!
   * Flag to indicate that current frame is dropped.
   */
  bool is_dropped_frame;

#if CONFIG_BITRATE_ACCURACY
  /*!
   * Structure stores information needed for bitrate accuracy experiment.
   */
  VBR_RATECTRL_INFO vbr_rc_info;
#endif

#if CONFIG_RATECTRL_LOG
  /*!
   * Structure stores information of rate control decisions.
   */
  RATECTRL_LOG rc_log;
#endif  // CONFIG_RATECTRL_LOG

  /*!
   * Frame level twopass status and control data
   */
  TWO_PASS_FRAME twopass_frame;

#if CONFIG_THREE_PASS
  /*!
   * Context needed for third pass encoding.
   */
  THIRD_PASS_DEC_CTX *third_pass_ctx;
#endif

  /*!
   * File pointer to second pass log
   */
  FILE *second_pass_log_stream;

  /*!
   * Buffer to store 64x64 SAD
   */
  uint64_t *src_sad_blk_64x64;

  /*!
   * SSE between the current frame and the reconstructed last frame
   * It is only used for CBR mode.
   * It is not used if the reference frame has a different frame size.
   */
  uint64_t rec_sse;

  /*!
   * A flag to indicate whether the encoder is controlled by DuckyEncode or not.
   * 1:yes 0:no
   */
  int use_ducky_encode;

#if !CONFIG_REALTIME_ONLY
  /*! A structure that facilitates the communication between DuckyEncode and AV1
   * encoder.
   */
  DuckyEncodeInfo ducky_encode_info;
#endif  // CONFIG_REALTIME_ONLY
        //
  /*!
   * Frames since last frame with cdf update.
   */
  int frames_since_last_update;

  /*!
   * Block level thresholds to force zeromv-skip at partition level.
   */
  unsigned int zeromv_skip_thresh_exit_part[BLOCK_SIZES_ALL];

  /*!
   *  Should we allocate a downsampling pyramid for each frame buffer?
   *  This is currently only used for global motion
   */
  bool alloc_pyramid;

#if CONFIG_SALIENCY_MAP
  /*!
   * Pixel level saliency map for each frame.
   */
  uint8_t *saliency_map;

  /*!
   * Superblock level rdmult scaling factor driven by saliency map.
   */
  double *sm_scaling_factor;
#endif

  /*!
   * Number of pixels that choose palette mode for luma in the
   * fast encoding pass in av1_determine_sc_tools_with_encoding().
   */
  int palette_pixel_num;

  /*!
   * Flag to indicate scaled_last_source is available,
   * so scaling is not needed for last_source.
   */
  int scaled_last_source_available;
} AV1_COMP;

/*!
 * \brief Input frames and last input frame
 */
typedef struct EncodeFrameInput {
  /*!\cond */
  YV12_BUFFER_CONFIG *source;
  YV12_BUFFER_CONFIG *last_source;
  int64_t ts_duration;
  /*!\endcond */
} EncodeFrameInput;

/*!
 * \brief contains per-frame encoding parameters decided upon by
 * av1_encode_strategy() and passed down to av1_encode().
 */
typedef struct EncodeFrameParams {
  /*!
   * Is error resilient mode enabled
   */
  int error_resilient_mode;
  /*!
   * Frame type (eg KF vs inter frame etc)
   */
  FRAME_TYPE frame_type;

  /*!\cond */
  int primary_ref_frame;
  int order_offset;

  /*!\endcond */
  /*!
   * Should the current frame be displayed after being decoded
   */
  int show_frame;

  /*!\cond */
  int refresh_frame_flags;

  int show_existing_frame;
  int existing_fb_idx_to_show;

  /*!\endcond */
  /*!
   *  Bitmask of which reference buffers may be referenced by this frame.
   */
  int ref_frame_flags;

  /*!
   *  Reference buffer assignment for this frame.
   */
  int remapped_ref_idx[REF_FRAMES];

  /*!
   *  Flags which determine which reference buffers are refreshed by this
   *  frame.
   */
  RefreshFrameInfo refresh_frame;

  /*!
   *  Speed level to use for this frame: Bigger number means faster.
   */
  int speed;
} EncodeFrameParams;

/*!\cond */

void av1_initialize_enc(unsigned int usage, enum aom_rc_mode end_usage);

struct AV1_COMP *av1_create_compressor(AV1_PRIMARY *ppi,
                                       const AV1EncoderConfig *oxcf,
                                       BufferPool *const pool,
                                       COMPRESSOR_STAGE stage,
                                       int lap_lag_in_frames);

struct AV1_PRIMARY *av1_create_primary_compressor(
    struct aom_codec_pkt_list *pkt_list_head, int num_lap_buffers,
    const AV1EncoderConfig *oxcf);

void av1_remove_compressor(AV1_COMP *cpi);

void av1_remove_primary_compressor(AV1_PRIMARY *ppi);

#if CONFIG_ENTROPY_STATS
void print_entropy_stats(AV1_PRIMARY *const ppi);
#endif
#if CONFIG_INTERNAL_STATS
void print_internal_stats(AV1_PRIMARY *ppi);
#endif

void av1_change_config_seq(AV1_PRIMARY *ppi, const AV1EncoderConfig *oxcf,
                           bool *sb_size_changed);

void av1_change_config(AV1_COMP *cpi, const AV1EncoderConfig *oxcf,
                       bool sb_size_changed);

aom_codec_err_t av1_check_initial_width(AV1_COMP *cpi, int use_highbitdepth,
                                        int subsampling_x, int subsampling_y);

void av1_post_encode_updates(AV1_COMP *const cpi,
                             const AV1_COMP_DATA *const cpi_data);

void av1_release_scaled_references_fpmt(AV1_COMP *cpi);

void av1_decrement_ref_counts_fpmt(BufferPool *buffer_pool,
                                   int ref_buffers_used_map);

void av1_init_sc_decisions(AV1_PRIMARY *const ppi);

AV1_COMP *av1_get_parallel_frame_enc_data(AV1_PRIMARY *const ppi,
                                          AV1_COMP_DATA *const first_cpi_data);

int av1_init_parallel_frame_context(const AV1_COMP_DATA *const first_cpi_data,
                                    AV1_PRIMARY *const ppi,
                                    int *ref_buffers_used_map);

/*!\endcond */

/*!\brief Obtain the raw frame data
 *
 * \ingroup high_level_algo
 * This function receives the raw frame data from input.
 *
 * \param[in]     cpi            Top-level encoder structure
 * \param[in]     frame_flags    Flags to decide how to encoding the frame
 * \param[in,out] sd             Contain raw frame data
 * \param[in]     time_stamp     Time stamp of the frame
 * \param[in]     end_time_stamp End time stamp
 *
 * \return Returns a value to indicate if the frame data is received
 * successfully.
 * \note The caller can assume that a copy of this frame is made and not just a
 * copy of the pointer.
 */
int av1_receive_raw_frame(AV1_COMP *cpi, aom_enc_frame_flags_t frame_flags,
                          const YV12_BUFFER_CONFIG *sd, int64_t time_stamp,
                          int64_t end_time_stamp);

/*!\brief Encode a frame
 *
 * \ingroup high_level_algo
 * \callgraph
 * \callergraph
 * This function encodes the raw frame data, and outputs the frame bit stream
 * to the designated buffer. The caller should use the output parameters
 * cpi_data->ts_frame_start and cpi_data->ts_frame_end only when this function
 * returns AOM_CODEC_OK.
 *
 * \param[in]     cpi         Top-level encoder structure
 * \param[in,out] cpi_data    Data corresponding to a frame encode
 *
 * \return Returns a value to indicate if the encoding is done successfully.
 * \retval #AOM_CODEC_OK
 * \retval -1
 *     No frame encoded; more input is required.
 * \retval "A nonzero (positive) aom_codec_err_t code"
 *     The encoding failed with the error. Sets the error code and error message
 * in \c cpi->common.error.
 */
int av1_get_compressed_data(AV1_COMP *cpi, AV1_COMP_DATA *const cpi_data);

/*!\brief Run 1-pass/2-pass encoding
 *
 * \ingroup high_level_algo
 * \callgraph
 * \callergraph
 */
int av1_encode(AV1_COMP *const cpi, uint8_t *const dest, size_t dest_size,
               const EncodeFrameInput *const frame_input,
               const EncodeFrameParams *const frame_params,
               size_t *const frame_size);

/*!\cond */
int av1_get_preview_raw_frame(AV1_COMP *cpi, YV12_BUFFER_CONFIG *dest);

int av1_get_last_show_frame(AV1_COMP *cpi, YV12_BUFFER_CONFIG *frame);

aom_codec_err_t av1_copy_new_frame_enc(AV1_COMMON *cm,
                                       YV12_BUFFER_CONFIG *new_frame,
                                       YV12_BUFFER_CONFIG *sd);

int av1_use_as_reference(int *ext_ref_frame_flags, int ref_frame_flags);

int av1_copy_reference_enc(AV1_COMP *cpi, int idx, YV12_BUFFER_CONFIG *sd);

int av1_set_reference_enc(AV1_COMP *cpi, int idx, YV12_BUFFER_CONFIG *sd);

void av1_set_frame_size(AV1_COMP *cpi, int width, int height);

void av1_set_mv_search_params(AV1_COMP *cpi);

int av1_set_active_map(AV1_COMP *cpi, unsigned char *map, int rows, int cols);

int av1_get_active_map(AV1_COMP *cpi, unsigned char *map, int rows, int cols);

int av1_set_internal_size(AV1EncoderConfig *const oxcf,
                          ResizePendingParams *resize_pending_params,
                          AOM_SCALING_MODE horiz_mode,
                          AOM_SCALING_MODE vert_mode);

int av1_get_quantizer(struct AV1_COMP *cpi);

// This function assumes that the input buffer contains valid OBUs. It should
// not be called on untrusted input.
int av1_convert_sect5obus_to_annexb(uint8_t *buffer, size_t buffer_size,
                                    size_t *input_size);

void av1_alloc_mb_wiener_var_pred_buf(AV1_COMMON *cm, ThreadData *td);

void av1_dealloc_mb_wiener_var_pred_buf(ThreadData *td);

// Set screen content options.
// This function estimates whether to use screen content tools, by counting
// the portion of blocks that have few luma colors.
// Modifies:
//   cpi->commom.features.allow_screen_content_tools
//   cpi->common.features.allow_intrabc
//   cpi->use_screen_content_tools
//   cpi->is_screen_content_type
// However, the estimation is not accurate and may misclassify videos.
// A slower but more accurate approach that determines whether to use screen
// content tools is employed later. See av1_determine_sc_tools_with_encoding().
void av1_set_screen_content_options(struct AV1_COMP *cpi,
                                    FeatureFlags *features);

void av1_update_frame_size(AV1_COMP *cpi);

void av1_set_svc_seq_params(AV1_PRIMARY *const ppi);

typedef struct {
  int pyr_level;
  int disp_order;
} RefFrameMapPair;

static inline void init_ref_map_pair(
    AV1_COMP *cpi, RefFrameMapPair ref_frame_map_pairs[REF_FRAMES]) {
  if (cpi->ppi->gf_group.update_type[cpi->gf_frame_index] == KF_UPDATE) {
    memset(ref_frame_map_pairs, -1, sizeof(*ref_frame_map_pairs) * REF_FRAMES);
    return;
  }
  memset(ref_frame_map_pairs, 0, sizeof(*ref_frame_map_pairs) * REF_FRAMES);
  for (int map_idx = 0; map_idx < REF_FRAMES; map_idx++) {
    // Get reference frame buffer.
    const RefCntBuffer *const buf = cpi->common.ref_frame_map[map_idx];
    if (ref_frame_map_pairs[map_idx].disp_order == -1) continue;
    if (buf == NULL) {
      ref_frame_map_pairs[map_idx].disp_order = -1;
      ref_frame_map_pairs[map_idx].pyr_level = -1;
      continue;
    } else if (buf->ref_count > 1) {
      // Once the keyframe is coded, the slots in ref_frame_map will all
      // point to the same frame. In that case, all subsequent pointers
      // matching the current are considered "free" slots. This will find
      // the next occurrence of the current pointer if ref_count indicates
      // there are multiple instances of it and mark it as free.
      for (int idx2 = map_idx + 1; idx2 < REF_FRAMES; ++idx2) {
        const RefCntBuffer *const buf2 = cpi->common.ref_frame_map[idx2];
        if (buf2 == buf) {
          ref_frame_map_pairs[idx2].disp_order = -1;
          ref_frame_map_pairs[idx2].pyr_level = -1;
        }
      }
    }
    ref_frame_map_pairs[map_idx].disp_order = (int)buf->display_order_hint;
    ref_frame_map_pairs[map_idx].pyr_level = buf->pyramid_level;
  }
}

#if CONFIG_FPMT_TEST
static inline void calc_frame_data_update_flag(
    GF_GROUP *const gf_group, int gf_frame_index,
    bool *const do_frame_data_update) {
  *do_frame_data_update = true;
  // Set the flag to false for all frames in a given parallel encode set except
  // the last frame in the set with frame_parallel_level = 2.
  if (gf_group->frame_parallel_level[gf_frame_index] == 1) {
    *do_frame_data_update = false;
  } else if (gf_group->frame_parallel_level[gf_frame_index] == 2) {
    // Check if this is the last frame in the set with frame_parallel_level = 2.
    for (int i = gf_frame_index + 1; i < gf_group->size; i++) {
      if ((gf_group->frame_parallel_level[i] == 0 &&
           (gf_group->update_type[i] == ARF_UPDATE ||
            gf_group->update_type[i] == INTNL_ARF_UPDATE)) ||
          gf_group->frame_parallel_level[i] == 1) {
        break;
      } else if (gf_group->frame_parallel_level[i] == 2) {
        *do_frame_data_update = false;
        break;
      }
    }
  }
}
#endif

// av1 uses 10,000,000 ticks/second as time stamp
#define TICKS_PER_SEC 10000000LL

static inline int64_t timebase_units_to_ticks(
    const aom_rational64_t *timestamp_ratio, int64_t n) {
  return n * timestamp_ratio->num / timestamp_ratio->den;
}

static inline int64_t ticks_to_timebase_units(
    const aom_rational64_t *timestamp_ratio, int64_t n) {
  int64_t round = timestamp_ratio->num / 2;
  if (round > 0) --round;
  return (n * timestamp_ratio->den + round) / timestamp_ratio->num;
}

static inline int frame_is_kf_gf_arf(const AV1_COMP *cpi) {
  const GF_GROUP *const gf_group = &cpi->ppi->gf_group;
  const FRAME_UPDATE_TYPE update_type =
      gf_group->update_type[cpi->gf_frame_index];

  return frame_is_intra_only(&cpi->common) || update_type == ARF_UPDATE ||
         update_type == GF_UPDATE;
}

// TODO(huisu@google.com, youzhou@microsoft.com): enable hash-me for HBD.
static inline int av1_use_hash_me(const AV1_COMP *const cpi) {
  return (cpi->common.features.allow_screen_content_tools &&
          cpi->common.features.allow_intrabc &&
          frame_is_intra_only(&cpi->common));
}

static inline const YV12_BUFFER_CONFIG *get_ref_frame_yv12_buf(
    const AV1_COMMON *const cm, MV_REFERENCE_FRAME ref_frame) {
  const RefCntBuffer *const buf = get_ref_frame_buf(cm, ref_frame);
  return buf != NULL ? &buf->buf : NULL;
}

static inline void alloc_frame_mvs(AV1_COMMON *const cm, RefCntBuffer *buf) {
  assert(buf != NULL);
  ensure_mv_buffer(buf, cm);
  buf->width = cm->width;
  buf->height = cm->height;
}

// Get the allocated token size for a tile. It does the same calculation as in
// the frame token allocation.
static inline unsigned int allocated_tokens(const TileInfo *tile,
                                            int sb_size_log2, int num_planes) {
  int tile_mb_rows =
      ROUND_POWER_OF_TWO(tile->mi_row_end - tile->mi_row_start, 2);
  int tile_mb_cols =
      ROUND_POWER_OF_TWO(tile->mi_col_end - tile->mi_col_start, 2);

  return get_token_alloc(tile_mb_rows, tile_mb_cols, sb_size_log2, num_planes);
}

static inline void get_start_tok(AV1_COMP *cpi, int tile_row, int tile_col,
                                 int mi_row, TokenExtra **tok, int sb_size_log2,
                                 int num_planes) {
  AV1_COMMON *const cm = &cpi->common;
  const int tile_cols = cm->tiles.cols;
  TileDataEnc *this_tile = &cpi->tile_data[tile_row * tile_cols + tile_col];
  const TileInfo *const tile_info = &this_tile->tile_info;

  const int tile_mb_cols =
      (tile_info->mi_col_end - tile_info->mi_col_start + 2) >> 2;
  const int tile_mb_row = (mi_row - tile_info->mi_row_start + 2) >> 2;

  *tok = cpi->token_info.tile_tok[tile_row][tile_col] +
         get_token_alloc(tile_mb_row, tile_mb_cols, sb_size_log2, num_planes);
}

void av1_apply_encoding_flags(AV1_COMP *cpi, aom_enc_frame_flags_t flags);

#define ALT_MIN_LAG 3
static inline int is_altref_enabled(int lag_in_frames, bool enable_auto_arf) {
  return lag_in_frames >= ALT_MIN_LAG && enable_auto_arf;
}

static inline int can_disable_altref(const GFConfig *gf_cfg) {
  return is_altref_enabled(gf_cfg->lag_in_frames, gf_cfg->enable_auto_arf) &&
         (gf_cfg->gf_min_pyr_height == 0);
}

// Helper function to compute number of blocks on either side of the frame.
static inline int get_num_blocks(const int frame_length, const int mb_length) {
  return (frame_length + mb_length - 1) / mb_length;
}

// Check if statistics generation stage
static inline int is_stat_generation_stage(const AV1_COMP *const cpi) {
  assert(IMPLIES(cpi->compressor_stage == LAP_STAGE,
                 cpi->oxcf.pass == AOM_RC_ONE_PASS && cpi->ppi->lap_enabled));
  return (cpi->oxcf.pass == AOM_RC_FIRST_PASS ||
          (cpi->compressor_stage == LAP_STAGE));
}
// Check if statistics consumption stage
static inline int is_stat_consumption_stage_twopass(const AV1_COMP *const cpi) {
  return (cpi->oxcf.pass >= AOM_RC_SECOND_PASS);
}

// Check if statistics consumption stage
static inline int is_stat_consumption_stage(const AV1_COMP *const cpi) {
  return (is_stat_consumption_stage_twopass(cpi) ||
          (cpi->oxcf.pass == AOM_RC_ONE_PASS &&
           (cpi->compressor_stage == ENCODE_STAGE) && cpi->ppi->lap_enabled));
}

// Decide whether 'dv_costs' need to be allocated/stored during the encoding.
static inline bool av1_need_dv_costs(const AV1_COMP *const cpi) {
  return !cpi->sf.rt_sf.use_nonrd_pick_mode &&
         av1_allow_intrabc(&cpi->common) && !is_stat_generation_stage(cpi);
}

/*!\endcond */
/*!\brief Check if the current stage has statistics
 *
 *\ingroup two_pass_algo
 *
 * \param[in]    cpi     Top - level encoder instance structure
 *
 * \return 0 if no stats for current stage else 1
 */
static inline int has_no_stats_stage(const AV1_COMP *const cpi) {
  assert(
      IMPLIES(!cpi->ppi->lap_enabled, cpi->compressor_stage == ENCODE_STAGE));
  return (cpi->oxcf.pass == AOM_RC_ONE_PASS && !cpi->ppi->lap_enabled);
}

/*!\cond */

static inline int is_one_pass_rt_params(const AV1_COMP *cpi) {
  return has_no_stats_stage(cpi) && cpi->oxcf.mode == REALTIME &&
         cpi->oxcf.gf_cfg.lag_in_frames == 0;
}

// Use default/internal reference structure for single-layer RTC.
static inline int use_rtc_reference_structure_one_layer(const AV1_COMP *cpi) {
  return is_one_pass_rt_params(cpi) && cpi->ppi->number_spatial_layers == 1 &&
         cpi->ppi->number_temporal_layers == 1 &&
         !cpi->ppi->rtc_ref.set_ref_frame_config;
}

// Check if postencode drop is allowed.
static inline int allow_postencode_drop_rtc(const AV1_COMP *cpi) {
  const AV1_COMMON *const cm = &cpi->common;
  return is_one_pass_rt_params(cpi) && cpi->oxcf.rc_cfg.mode == AOM_CBR &&
         cpi->oxcf.rc_cfg.drop_frames_water_mark > 0 &&
         !cpi->rc.rtc_external_ratectrl && !frame_is_intra_only(cm) &&
         cpi->svc.spatial_layer_id == 0;
}

// Function return size of frame stats buffer
static inline int get_stats_buf_size(int num_lap_buffer, int num_lag_buffer) {
  /* if lookahead is enabled return num_lap_buffers else num_lag_buffers */
  return (num_lap_buffer > 0 ? num_lap_buffer + 1 : num_lag_buffer);
}

// TODO(zoeliu): To set up cpi->oxcf.gf_cfg.enable_auto_brf

static inline void set_ref_ptrs(const AV1_COMMON *cm, MACROBLOCKD *xd,
                                MV_REFERENCE_FRAME ref0,
                                MV_REFERENCE_FRAME ref1) {
  xd->block_ref_scale_factors[0] =
      get_ref_scale_factors_const(cm, ref0 >= LAST_FRAME ? ref0 : 1);
  xd->block_ref_scale_factors[1] =
      get_ref_scale_factors_const(cm, ref1 >= LAST_FRAME ? ref1 : 1);
}

static inline int get_chessboard_index(int frame_index) {
  return frame_index & 0x1;
}

static inline const int *cond_cost_list_const(const struct AV1_COMP *cpi,
                                              const int *cost_list) {
  const int use_cost_list = cpi->sf.mv_sf.subpel_search_method != SUBPEL_TREE &&
                            cpi->sf.mv_sf.use_fullpel_costlist;
  return use_cost_list ? cost_list : NULL;
}

static inline int *cond_cost_list(const struct AV1_COMP *cpi, int *cost_list) {
  const int use_cost_list = cpi->sf.mv_sf.subpel_search_method != SUBPEL_TREE &&
                            cpi->sf.mv_sf.use_fullpel_costlist;
  return use_cost_list ? cost_list : NULL;
}

// Compression ratio of current frame.
double av1_get_compression_ratio(const AV1_COMMON *const cm,
                                 size_t encoded_frame_size);

void av1_new_framerate(AV1_COMP *cpi, double framerate);

void av1_setup_frame_size(AV1_COMP *cpi);

#define LAYER_IDS_TO_IDX(sl, tl, num_tl) ((sl) * (num_tl) + (tl))

// Returns 1 if a frame is scaled and 0 otherwise.
static inline int av1_resize_scaled(const AV1_COMMON *cm) {
  return cm->superres_upscaled_width != cm->render_width ||
         cm->superres_upscaled_height != cm->render_height;
}

static inline int av1_frame_scaled(const AV1_COMMON *cm) {
  return av1_superres_scaled(cm) || av1_resize_scaled(cm);
}

// Don't allow a show_existing_frame to coincide with an error resilient
// frame. An exception can be made for a forward keyframe since it has no
// previous dependencies.
static inline int encode_show_existing_frame(const AV1_COMMON *cm) {
  return cm->show_existing_frame && (!cm->features.error_resilient_mode ||
                                     cm->current_frame.frame_type == KEY_FRAME);
}

// Get index into the 'cpi->mbmi_ext_info.frame_base' array for the given
// 'mi_row' and 'mi_col'.
static inline int get_mi_ext_idx(const int mi_row, const int mi_col,
                                 const BLOCK_SIZE mi_alloc_bsize,
                                 const int mbmi_ext_stride) {
  const int mi_ext_size_1d = mi_size_wide[mi_alloc_bsize];
  const int mi_ext_row = mi_row / mi_ext_size_1d;
  const int mi_ext_col = mi_col / mi_ext_size_1d;
  return mi_ext_row * mbmi_ext_stride + mi_ext_col;
}

// Lighter version of set_offsets that only sets the mode info
// pointers.
static inline void set_mode_info_offsets(
    const CommonModeInfoParams *const mi_params,
    const MBMIExtFrameBufferInfo *const mbmi_ext_info, MACROBLOCK *const x,
    MACROBLOCKD *const xd, int mi_row, int mi_col) {
  set_mi_offsets(mi_params, xd, mi_row, mi_col);
  const int ext_idx = get_mi_ext_idx(mi_row, mi_col, mi_params->mi_alloc_bsize,
                                     mbmi_ext_info->stride);
  x->mbmi_ext_frame = mbmi_ext_info->frame_base + ext_idx;
}

// Check to see if the given partition size is allowed for a specified number
// of mi block rows and columns remaining in the image.
// If not then return the largest allowed partition size
static inline BLOCK_SIZE find_partition_size(BLOCK_SIZE bsize, int rows_left,
                                             int cols_left, int *bh, int *bw) {
  int int_size = (int)bsize;
  if (rows_left <= 0 || cols_left <= 0) {
    return AOMMIN(bsize, BLOCK_8X8);
  } else {
    for (; int_size > 0; int_size -= 3) {
      *bh = mi_size_high[int_size];
      *bw = mi_size_wide[int_size];
      if ((*bh <= rows_left) && (*bw <= cols_left)) {
        break;
      }
    }
  }
  return (BLOCK_SIZE)int_size;
}

static const uint8_t av1_ref_frame_flag_list[REF_FRAMES] = { 0,
                                                             AOM_LAST_FLAG,
                                                             AOM_LAST2_FLAG,
                                                             AOM_LAST3_FLAG,
                                                             AOM_GOLD_FLAG,
                                                             AOM_BWD_FLAG,
                                                             AOM_ALT2_FLAG,
                                                             AOM_ALT_FLAG };

// When more than 'max_allowed_refs' are available, we reduce the number of
// reference frames one at a time based on this order.
static const MV_REFERENCE_FRAME disable_order[] = {
  LAST3_FRAME,
  LAST2_FRAME,
  ALTREF2_FRAME,
  BWDREF_FRAME,
};

static const MV_REFERENCE_FRAME
    ref_frame_priority_order[INTER_REFS_PER_FRAME] = {
      LAST_FRAME,    ALTREF_FRAME, BWDREF_FRAME, GOLDEN_FRAME,
      ALTREF2_FRAME, LAST2_FRAME,  LAST3_FRAME,
    };

static inline int get_ref_frame_flags(const SPEED_FEATURES *const sf,
                                      const int use_one_pass_rt_params,
                                      const YV12_BUFFER_CONFIG **ref_frames,
                                      const int ext_ref_frame_flags) {
  // cpi->ext_flags.ref_frame_flags allows certain reference types to be
  // disabled by the external interface.  These are set by
  // av1_apply_encoding_flags(). Start with what the external interface allows,
  // then suppress any reference types which we have found to be duplicates.
  int flags = ext_ref_frame_flags;

  for (int i = 1; i < INTER_REFS_PER_FRAME; ++i) {
    const YV12_BUFFER_CONFIG *const this_ref = ref_frames[i];
    // If this_ref has appeared before, mark the corresponding ref frame as
    // invalid. For one_pass_rt mode, only disable GOLDEN_FRAME if it's the
    // same as LAST_FRAME or ALTREF_FRAME (if ALTREF is being used in nonrd).
    int index =
        (use_one_pass_rt_params && ref_frame_priority_order[i] == GOLDEN_FRAME)
            ? (1 + sf->rt_sf.use_nonrd_altref_frame)
            : i;
    for (int j = 0; j < index; ++j) {
      // If this_ref has appeared before (same as the reference corresponding
      // to lower index j), remove it as a reference only if that reference
      // (for index j) is actually used as a reference.
      if (this_ref == ref_frames[j] &&
          (flags & (1 << (ref_frame_priority_order[j] - 1)))) {
        flags &= ~(1 << (ref_frame_priority_order[i] - 1));
        break;
      }
    }
  }
  return flags;
}

// Returns a Sequence Header OBU stored in an aom_fixed_buf_t, or NULL upon
// failure. When a non-NULL aom_fixed_buf_t pointer is returned by this
// function, the memory must be freed by the caller. Both the buf member of the
// aom_fixed_buf_t, and the aom_fixed_buf_t pointer itself must be freed. Memory
// returned must be freed via call to free().
//
// Note: The OBU returned is in Low Overhead Bitstream Format. Specifically,
// the obu_has_size_field bit is set, and the buffer contains the obu_size
// field.
aom_fixed_buf_t *av1_get_global_headers(AV1_PRIMARY *ppi);

#define MAX_GFUBOOST_FACTOR 10.0
#define MIN_GFUBOOST_FACTOR 4.0

static inline int is_frame_tpl_eligible(const GF_GROUP *const gf_group,
                                        uint8_t index) {
  const FRAME_UPDATE_TYPE update_type = gf_group->update_type[index];
  return update_type == ARF_UPDATE || update_type == GF_UPDATE ||
         update_type == KF_UPDATE;
}

static inline int is_frame_eligible_for_ref_pruning(const GF_GROUP *gf_group,
                                                    int selective_ref_frame,
                                                    int prune_ref_frames,
                                                    int gf_index) {
  return (selective_ref_frame > 0) && (prune_ref_frames > 0) &&
         !is_frame_tpl_eligible(gf_group, gf_index);
}

// Get update type of the current frame.
static inline FRAME_UPDATE_TYPE get_frame_update_type(const GF_GROUP *gf_group,
                                                      int gf_frame_index) {
  return gf_group->update_type[gf_frame_index];
}

static inline int av1_pixels_to_mi(int pixels) {
  return ALIGN_POWER_OF_TWO(pixels, 3) >> MI_SIZE_LOG2;
}

static inline int is_psnr_calc_enabled(const AV1_COMP *cpi) {
  const AV1_COMMON *const cm = &cpi->common;

  return cpi->ppi->b_calculate_psnr && !is_stat_generation_stage(cpi) &&
         cm->show_frame && !cpi->is_dropped_frame;
}

static inline int is_frame_resize_pending(const AV1_COMP *const cpi) {
  const ResizePendingParams *const resize_pending_params =
      &cpi->resize_pending_params;
  return (resize_pending_params->width && resize_pending_params->height &&
          (cpi->common.width != resize_pending_params->width ||
           cpi->common.height != resize_pending_params->height));
}

// Check if loop filter is used.
static inline int is_loopfilter_used(const AV1_COMMON *const cm) {
  return !cm->features.coded_lossless && !cm->tiles.large_scale;
}

// Check if CDEF is used.
static inline int is_cdef_used(const AV1_COMMON *const cm) {
  return cm->seq_params->enable_cdef && !cm->features.coded_lossless &&
         !cm->tiles.large_scale;
}

// Check if loop restoration filter is used.
static inline int is_restoration_used(const AV1_COMMON *const cm) {
  return cm->seq_params->enable_restoration && !cm->features.all_lossless &&
         !cm->tiles.large_scale;
}

// Checks if post-processing filters need to be applied.
// NOTE: This function decides if the application of different post-processing
// filters on the reconstructed frame can be skipped at the encoder side.
// However the computation of different filter parameters that are signaled in
// the bitstream is still required.
static inline unsigned int derive_skip_apply_postproc_filters(
    const AV1_COMP *cpi, int use_loopfilter, int use_cdef, int use_superres,
    int use_restoration) {
  // Though CDEF parameter selection should be dependent on
  // deblocked/loop-filtered pixels for cdef_pick_method <=
  // CDEF_FAST_SEARCH_LVL5, CDEF strength values are calculated based on the
  // pixel values that are not loop-filtered in svc real-time encoding mode.
  // Hence this case is handled separately using the condition below.
  if (cpi->ppi->rtc_ref.non_reference_frame)
    return (SKIP_APPLY_LOOPFILTER | SKIP_APPLY_CDEF);

  if (!cpi->oxcf.algo_cfg.skip_postproc_filtering || cpi->ppi->b_calculate_psnr)
    return 0;
  assert(cpi->oxcf.mode == ALLINTRA);

  // The post-processing filters are applied one after the other in the
  // following order: deblocking->cdef->superres->restoration. In case of
  // ALLINTRA encoding, the reconstructed frame is not used as a reference
  // frame. Hence, the application of these filters can be skipped when
  // 1. filter parameters of the subsequent stages are not dependent on the
  // filtered output of the current stage or
  // 2. subsequent filtering stages are disabled
  if (use_restoration) return SKIP_APPLY_RESTORATION;
  if (use_superres) return SKIP_APPLY_SUPERRES;
  if (use_cdef) {
    // CDEF parameter selection is not dependent on the deblocked frame if
    // cdef_pick_method is CDEF_PICK_FROM_Q. Hence the application of deblocking
    // filters and cdef filters can be skipped in this case.
    return (cpi->sf.lpf_sf.cdef_pick_method == CDEF_PICK_FROM_Q &&
            use_loopfilter)
               ? (SKIP_APPLY_LOOPFILTER | SKIP_APPLY_CDEF)
               : SKIP_APPLY_CDEF;
  }
  if (use_loopfilter) return SKIP_APPLY_LOOPFILTER;

  // If we reach here, all post-processing stages are disabled, so none need to
  // be skipped.
  return 0;
}

static inline void set_postproc_filter_default_params(AV1_COMMON *cm) {
  struct loopfilter *const lf = &cm->lf;
  CdefInfo *const cdef_info = &cm->cdef_info;
  RestorationInfo *const rst_info = cm->rst_info;

  lf->filter_level[0] = 0;
  lf->filter_level[1] = 0;
  lf->backup_filter_level[0] = 0;
  lf->backup_filter_level[1] = 0;
  cdef_info->cdef_bits = 0;
  cdef_info->cdef_strengths[0] = 0;
  cdef_info->nb_cdef_strengths = 1;
  cdef_info->cdef_uv_strengths[0] = 0;
  rst_info[0].frame_restoration_type = RESTORE_NONE;
  rst_info[1].frame_restoration_type = RESTORE_NONE;
  rst_info[2].frame_restoration_type = RESTORE_NONE;
}

static inline int is_inter_tx_size_search_level_one(
    const TX_SPEED_FEATURES *tx_sf) {
  return (tx_sf->inter_tx_size_search_init_depth_rect >= 1 &&
          tx_sf->inter_tx_size_search_init_depth_sqr >= 1);
}

static inline int get_lpf_opt_level(const SPEED_FEATURES *sf) {
  int lpf_opt_level = 0;
  if (is_inter_tx_size_search_level_one(&sf->tx_sf))
    lpf_opt_level = (sf->lpf_sf.lpf_pick == LPF_PICK_FROM_Q) ? 2 : 1;
  return lpf_opt_level;
}

// Enable switchable motion mode only if warp and OBMC tools are allowed
static inline bool is_switchable_motion_mode_allowed(bool allow_warped_motion,
                                                     bool enable_obmc) {
  return (allow_warped_motion || enable_obmc);
}

#if CONFIG_AV1_TEMPORAL_DENOISING
static inline int denoise_svc(const struct AV1_COMP *const cpi) {
  return (!cpi->ppi->use_svc ||
          (cpi->ppi->use_svc &&
           cpi->svc.spatial_layer_id >= cpi->svc.first_layer_denoise));
}
#endif

#if CONFIG_COLLECT_PARTITION_STATS == 2
static inline void av1_print_fr_partition_timing_stats(
    const FramePartitionTimingStats *part_stats, const char *filename) {
  FILE *f = fopen(filename, "w");
  if (!f) {
    return;
  }

  fprintf(f, "bsize,redo,");
  for (int part = 0; part < EXT_PARTITION_TYPES; part++) {
    fprintf(f, "decision_%d,", part);
  }
  for (int part = 0; part < EXT_PARTITION_TYPES; part++) {
    fprintf(f, "attempt_%d,", part);
  }
  for (int part = 0; part < EXT_PARTITION_TYPES; part++) {
    fprintf(f, "time_%d,", part);
  }
  fprintf(f, "\n");

  static const int bsizes[6] = { 128, 64, 32, 16, 8, 4 };

  for (int bsize_idx = 0; bsize_idx < 6; bsize_idx++) {
    fprintf(f, "%d,%d,", bsizes[bsize_idx], part_stats->partition_redo);
    for (int part = 0; part < EXT_PARTITION_TYPES; part++) {
      fprintf(f, "%d,", part_stats->partition_decisions[bsize_idx][part]);
    }
    for (int part = 0; part < EXT_PARTITION_TYPES; part++) {
      fprintf(f, "%d,", part_stats->partition_attempts[bsize_idx][part]);
    }
    for (int part = 0; part < EXT_PARTITION_TYPES; part++) {
      fprintf(f, "%ld,", part_stats->partition_times[bsize_idx][part]);
    }
    fprintf(f, "\n");
  }
  fclose(f);
}
#endif  // CONFIG_COLLECT_PARTITION_STATS == 2

#if CONFIG_COLLECT_PARTITION_STATS
static inline int av1_get_bsize_idx_for_part_stats(BLOCK_SIZE bsize) {
  assert(bsize == BLOCK_128X128 || bsize == BLOCK_64X64 ||
         bsize == BLOCK_32X32 || bsize == BLOCK_16X16 || bsize == BLOCK_8X8 ||
         bsize == BLOCK_4X4);
  switch (bsize) {
    case BLOCK_128X128: return 0;
    case BLOCK_64X64: return 1;
    case BLOCK_32X32: return 2;
    case BLOCK_16X16: return 3;
    case BLOCK_8X8: return 4;
    case BLOCK_4X4: return 5;
    default: assert(0 && "Invalid bsize for partition_stats."); return -1;
  }
}
#endif  // CONFIG_COLLECT_PARTITION_STATS

#if CONFIG_COLLECT_COMPONENT_TIMING
static inline void start_timing(AV1_COMP *cpi, int component) {
  aom_usec_timer_start(&cpi->component_timer[component]);
}
static inline void end_timing(AV1_COMP *cpi, int component) {
  aom_usec_timer_mark(&cpi->component_timer[component]);
  cpi->frame_component_time[component] +=
      aom_usec_timer_elapsed(&cpi->component_timer[component]);
}
static inline char const *get_frame_type_enum(int type) {
  switch (type) {
    case 0: return "KEY_FRAME";
    case 1: return "INTER_FRAME";
    case 2: return "INTRA_ONLY_FRAME";
    case 3: return "S_FRAME";
    default: assert(0);
  }
  return "error";
}
#endif

/*!\endcond */

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_ENCODER_H_
