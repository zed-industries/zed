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

#ifndef AOM_AV1_COMMON_AV1_COMMON_INT_H_
#define AOM_AV1_COMMON_AV1_COMMON_INT_H_

#include <stdbool.h>

#include "config/aom_config.h"
#include "config/av1_rtcd.h"

#include "aom/internal/aom_codec_internal.h"
#include "aom_dsp/flow_estimation/corner_detect.h"
#include "aom_util/aom_pthread.h"
#include "av1/common/alloccommon.h"
#include "av1/common/av1_loopfilter.h"
#include "av1/common/entropy.h"
#include "av1/common/entropymode.h"
#include "av1/common/entropymv.h"
#include "av1/common/enums.h"
#include "av1/common/frame_buffers.h"
#include "av1/common/mv.h"
#include "av1/common/quant_common.h"
#include "av1/common/restoration.h"
#include "av1/common/tile_common.h"
#include "av1/common/timing.h"
#include "aom_dsp/grain_params.h"
#include "aom_dsp/grain_table.h"
#include "aom_dsp/odintrin.h"
#ifdef __cplusplus
extern "C" {
#endif

#if defined(__clang__) && defined(__has_warning)
#if __has_feature(cxx_attributes) && __has_warning("-Wimplicit-fallthrough")
#define AOM_FALLTHROUGH_INTENDED [[clang::fallthrough]]  // NOLINT
#endif
#elif defined(__GNUC__) && __GNUC__ >= 7
#define AOM_FALLTHROUGH_INTENDED __attribute__((fallthrough))  // NOLINT
#endif

#ifndef AOM_FALLTHROUGH_INTENDED
#define AOM_FALLTHROUGH_INTENDED \
  do {                           \
  } while (0)
#endif

#define CDEF_MAX_STRENGTHS 16

/* Constant values while waiting for the sequence header */
#define FRAME_ID_LENGTH 15
#define DELTA_FRAME_ID_LENGTH 14

#define FRAME_CONTEXTS (FRAME_BUFFERS + 1)
// Extra frame context which is always kept at default values
#define FRAME_CONTEXT_DEFAULTS (FRAME_CONTEXTS - 1)
#define PRIMARY_REF_BITS 3
#define PRIMARY_REF_NONE 7

#define NUM_PING_PONG_BUFFERS 2

#define MAX_NUM_TEMPORAL_LAYERS 8
#define MAX_NUM_SPATIAL_LAYERS 4
/* clang-format off */
// clang-format seems to think this is a pointer dereference and not a
// multiplication.
#define MAX_NUM_OPERATING_POINTS \
  (MAX_NUM_TEMPORAL_LAYERS * MAX_NUM_SPATIAL_LAYERS)
/* clang-format on */

// TODO(jingning): Turning this on to set up transform coefficient
// processing timer.
#define TXCOEFF_TIMER 0
#define TXCOEFF_COST_TIMER 0

/*!\cond */

enum {
  SINGLE_REFERENCE = 0,
  COMPOUND_REFERENCE = 1,
  REFERENCE_MODE_SELECT = 2,
  REFERENCE_MODES = 3,
} UENUM1BYTE(REFERENCE_MODE);

enum {
  /**
   * Frame context updates are disabled
   */
  REFRESH_FRAME_CONTEXT_DISABLED,
  /**
   * Update frame context to values resulting from backward probability
   * updates based on entropy/counts in the decoded frame
   */
  REFRESH_FRAME_CONTEXT_BACKWARD,
} UENUM1BYTE(REFRESH_FRAME_CONTEXT_MODE);

#define MFMV_STACK_SIZE 3
typedef struct {
  int_mv mfmv0;
  uint8_t ref_frame_offset;
} TPL_MV_REF;

typedef struct {
  int_mv mv;
  MV_REFERENCE_FRAME ref_frame;
} MV_REF;

typedef struct RefCntBuffer {
  // For a RefCntBuffer, the following are reference-holding variables:
  // - cm->ref_frame_map[]
  // - cm->cur_frame
  // - cm->scaled_ref_buf[] (encoder only)
  // - pbi->output_frame_index[] (decoder only)
  // With that definition, 'ref_count' is the number of reference-holding
  // variables that are currently referencing this buffer.
  // For example:
  // - suppose this buffer is at index 'k' in the buffer pool, and
  // - Total 'n' of the variables / array elements above have value 'k' (that
  // is, they are pointing to buffer at index 'k').
  // Then, pool->frame_bufs[k].ref_count = n.
  int ref_count;

  unsigned int order_hint;
  unsigned int ref_order_hints[INTER_REFS_PER_FRAME];

  // These variables are used only in encoder and compare the absolute
  // display order hint to compute the relative distance and overcome
  // the limitation of get_relative_dist() which returns incorrect
  // distance when a very old frame is used as a reference.
  unsigned int display_order_hint;
  unsigned int ref_display_order_hint[INTER_REFS_PER_FRAME];
  // Frame's level within the hierarchical structure.
  unsigned int pyramid_level;
  MV_REF *mvs;
  uint8_t *seg_map;
  struct segmentation seg;
  int mi_rows;
  int mi_cols;
  // Width and height give the size of the buffer (before any upscaling, unlike
  // the sizes that can be derived from the buf structure)
  int width;
  int height;
  WarpedMotionParams global_motion[REF_FRAMES];
  int showable_frame;  // frame can be used as show existing frame in future
  uint8_t film_grain_params_present;
  aom_film_grain_t film_grain_params;
  aom_codec_frame_buffer_t raw_frame_buffer;
  YV12_BUFFER_CONFIG buf;
  int temporal_id;  // Temporal layer ID of the frame
  int spatial_id;   // Spatial layer ID of the frame
  FRAME_TYPE frame_type;

  // This is only used in the encoder but needs to be indexed per ref frame
  // so it's extremely convenient to keep it here.
  int interp_filter_selected[SWITCHABLE];

  // Inter frame reference frame delta for loop filter
  int8_t ref_deltas[REF_FRAMES];

  // 0 = ZERO_MV, MV
  int8_t mode_deltas[MAX_MODE_LF_DELTAS];

  FRAME_CONTEXT frame_context;

  int filter_level[2];
} RefCntBuffer;

typedef struct BufferPool {
// Protect BufferPool from being accessed by several FrameWorkers at
// the same time during frame parallel decode.
// TODO(hkuang): Try to use atomic variable instead of locking the whole pool.
// TODO(wtc): Remove this. See
// https://chromium-review.googlesource.com/c/webm/libvpx/+/560630.
#if CONFIG_MULTITHREAD
  pthread_mutex_t pool_mutex;
#endif

  // Private data associated with the frame buffer callbacks.
  void *cb_priv;

  aom_get_frame_buffer_cb_fn_t get_fb_cb;
  aom_release_frame_buffer_cb_fn_t release_fb_cb;

  RefCntBuffer *frame_bufs;
  uint8_t num_frame_bufs;

  // Frame buffers allocated internally by the codec.
  InternalFrameBufferList int_frame_buffers;
} BufferPool;

/*!\endcond */

/*!\brief Parameters related to CDEF */
typedef struct {
  //! CDEF column line buffer
  uint16_t *colbuf[MAX_MB_PLANE];
  //! CDEF top & bottom line buffer
  uint16_t *linebuf[MAX_MB_PLANE];
  //! CDEF intermediate buffer
  uint16_t *srcbuf;
  //! CDEF column line buffer sizes
  size_t allocated_colbuf_size[MAX_MB_PLANE];
  //! CDEF top and bottom line buffer sizes
  size_t allocated_linebuf_size[MAX_MB_PLANE];
  //! CDEF intermediate buffer size
  size_t allocated_srcbuf_size;
  //! CDEF damping factor
  int cdef_damping;
  //! Number of CDEF strength values
  int nb_cdef_strengths;
  //! CDEF strength values for luma
  int cdef_strengths[CDEF_MAX_STRENGTHS];
  //! CDEF strength values for chroma
  int cdef_uv_strengths[CDEF_MAX_STRENGTHS];
  //! Number of CDEF strength values in bits
  int cdef_bits;
  //! Number of rows in the frame in 4 pixel
  int allocated_mi_rows;
  //! Number of CDEF workers
  int allocated_num_workers;
} CdefInfo;

/*!\cond */

typedef struct {
  int delta_q_present_flag;
  // Resolution of delta quant
  int delta_q_res;
  int delta_lf_present_flag;
  // Resolution of delta lf level
  int delta_lf_res;
  // This is a flag for number of deltas of loop filter level
  // 0: use 1 delta, for y_vertical, y_horizontal, u, and v
  // 1: use separate deltas for each filter level
  int delta_lf_multi;
} DeltaQInfo;

typedef struct {
  int enable_order_hint;        // 0 - disable order hint, and related tools
  int order_hint_bits_minus_1;  // dist_wtd_comp, ref_frame_mvs,
                                // frame_sign_bias
                                // if 0, enable_dist_wtd_comp and
                                // enable_ref_frame_mvs must be set as 0.
  int enable_dist_wtd_comp;     // 0 - disable dist-wtd compound modes
                                // 1 - enable it
  int enable_ref_frame_mvs;     // 0 - disable ref frame mvs
                                // 1 - enable it
} OrderHintInfo;

// Sequence header structure.
// Note: All syntax elements of sequence_header_obu that need to be
// bit-identical across multiple sequence headers must be part of this struct,
// so that consistency is checked by are_seq_headers_consistent() function.
// One exception is the last member 'op_params' that is ignored by
// are_seq_headers_consistent() function.
typedef struct SequenceHeader {
  int num_bits_width;
  int num_bits_height;
  int max_frame_width;
  int max_frame_height;
  // Whether current and reference frame IDs are signaled in the bitstream.
  // Frame id numbers are additional information that do not affect the
  // decoding process, but provide decoders with a way of detecting missing
  // reference frames so that appropriate action can be taken.
  uint8_t frame_id_numbers_present_flag;
  int frame_id_length;
  int delta_frame_id_length;
  BLOCK_SIZE sb_size;  // Size of the superblock used for this frame
  int mib_size;        // Size of the superblock in units of MI blocks
  int mib_size_log2;   // Log 2 of above.

  OrderHintInfo order_hint_info;

  uint8_t force_screen_content_tools;  // 0 - force off
                                       // 1 - force on
                                       // 2 - adaptive
  uint8_t still_picture;               // Video is a single frame still picture
  uint8_t reduced_still_picture_hdr;   // Use reduced header for still picture
  uint8_t force_integer_mv;            // 0 - Don't force. MV can use subpel
                                       // 1 - force to integer
                                       // 2 - adaptive
  uint8_t enable_filter_intra;         // enables/disables filterintra
  uint8_t enable_intra_edge_filter;    // enables/disables edge upsampling
  uint8_t enable_interintra_compound;  // enables/disables interintra_compound
  uint8_t enable_masked_compound;      // enables/disables masked compound
  uint8_t enable_dual_filter;          // 0 - disable dual interpolation filter
                                       // 1 - enable vert/horz filter selection
  uint8_t enable_warped_motion;        // 0 - disable warp for the sequence
                                       // 1 - enable warp for the sequence
  uint8_t enable_superres;             // 0 - Disable superres for the sequence
                                       //     and no frame level superres flag
                                       // 1 - Enable superres for the sequence
                                       //     enable per-frame superres flag
  uint8_t enable_cdef;                 // To turn on/off CDEF
  uint8_t enable_restoration;          // To turn on/off loop restoration
  BITSTREAM_PROFILE profile;

  // Color config.
  aom_bit_depth_t bit_depth;  // AOM_BITS_8 in profile 0 or 1,
                              // AOM_BITS_10 or AOM_BITS_12 in profile 2 or 3.
  uint8_t use_highbitdepth;   // If true, we need to use 16bit frame buffers.
  uint8_t monochrome;         // Monochrome video
  aom_color_primaries_t color_primaries;
  aom_transfer_characteristics_t transfer_characteristics;
  aom_matrix_coefficients_t matrix_coefficients;
  int color_range;
  int subsampling_x;  // Chroma subsampling for x
  int subsampling_y;  // Chroma subsampling for y
  aom_chroma_sample_position_t chroma_sample_position;
  uint8_t separate_uv_delta_q;
  uint8_t film_grain_params_present;

  // Operating point info.
  int operating_points_cnt_minus_1;
  int operating_point_idc[MAX_NUM_OPERATING_POINTS];
  // True if operating_point_idc[op] is not equal to 0 for any value of op from
  // 0 to operating_points_cnt_minus_1.
  bool has_nonzero_operating_point_idc;
  int timing_info_present;
  aom_timing_info_t timing_info;
  uint8_t decoder_model_info_present_flag;
  aom_dec_model_info_t decoder_model_info;
  uint8_t display_model_info_present_flag;
  AV1_LEVEL seq_level_idx[MAX_NUM_OPERATING_POINTS];
  uint8_t tier[MAX_NUM_OPERATING_POINTS];  // seq_tier in spec. One bit: 0 or 1.

  // IMPORTANT: the op_params member must be at the end of the struct so that
  // are_seq_headers_consistent() can be implemented with a memcmp() call.
  // TODO(urvang): We probably don't need the +1 here.
  aom_dec_model_op_parameters_t op_params[MAX_NUM_OPERATING_POINTS + 1];
} SequenceHeader;

typedef struct {
  int skip_mode_allowed;
  int skip_mode_flag;
  int ref_frame_idx_0;
  int ref_frame_idx_1;
} SkipModeInfo;

typedef struct {
  FRAME_TYPE frame_type;
  REFERENCE_MODE reference_mode;

  unsigned int order_hint;
  unsigned int display_order_hint;
  // Frame's level within the hierarchical structure.
  unsigned int pyramid_level;
  unsigned int frame_number;
  SkipModeInfo skip_mode_info;
  int refresh_frame_flags;  // Which ref frames are overwritten by this frame
  int frame_refs_short_signaling;
} CurrentFrame;

/*!\endcond */

/*!
 * \brief Frame level features.
 */
typedef struct {
  /*!
   * If true, CDF update in the symbol encoding/decoding process is disabled.
   */
  bool disable_cdf_update;
  /*!
   * If true, motion vectors are specified to eighth pel precision; and
   * if false, motion vectors are specified to quarter pel precision.
   */
  bool allow_high_precision_mv;
  /*!
   * If true, force integer motion vectors; if false, use the default.
   */
  bool cur_frame_force_integer_mv;
  /*!
   * If true, palette tool and/or intra block copy tools may be used.
   */
  bool allow_screen_content_tools;
  bool allow_intrabc;       /*!< If true, intra block copy tool may be used. */
  bool allow_warped_motion; /*!< If true, frame may use warped motion mode. */
  /*!
   * If true, using previous frames' motion vectors for prediction is allowed.
   */
  bool allow_ref_frame_mvs;
  /*!
   * If true, frame is fully lossless at coded resolution.
   * */
  bool coded_lossless;
  /*!
   * If true, frame is fully lossless at upscaled resolution.
   */
  bool all_lossless;
  /*!
   * If true, the frame is restricted to a reduced subset of the full set of
   * transform types.
   */
  bool reduced_tx_set_used;
  /*!
   * If true, error resilient mode is enabled.
   * Note: Error resilient mode allows the syntax of a frame to be parsed
   * independently of previously decoded frames.
   */
  bool error_resilient_mode;
  /*!
   * If false, only MOTION_MODE that may be used is SIMPLE_TRANSLATION;
   * if true, all MOTION_MODES may be used.
   */
  bool switchable_motion_mode;
  TX_MODE tx_mode;            /*!< Transform mode at frame level. */
  InterpFilter interp_filter; /*!< Interpolation filter at frame level. */
  /*!
   * The reference frame that contains the CDF values and other state that
   * should be loaded at the start of the frame.
   */
  int primary_ref_frame;
  /*!
   * Byte alignment of the planes in the reference buffers.
   */
  int byte_alignment;
  /*!
   * Flag signaling how frame contexts should be updated at the end of
   * a frame decode.
   */
  REFRESH_FRAME_CONTEXT_MODE refresh_frame_context;
} FeatureFlags;

/*!
 * \brief Params related to tiles.
 */
typedef struct CommonTileParams {
  int cols;          /*!< number of tile columns that frame is divided into */
  int rows;          /*!< number of tile rows that frame is divided into */
  int max_width_sb;  /*!< maximum tile width in superblock units. */
  int max_height_sb; /*!< maximum tile height in superblock units. */

  /*!
   * Min width of non-rightmost tile in MI units. Only valid if cols > 1.
   */
  int min_inner_width;

  /*!
   * If true, tiles are uniformly spaced with power-of-two number of rows and
   * columns.
   * If false, tiles have explicitly configured widths and heights.
   */
  int uniform_spacing;

  /**
   * \name Members only valid when uniform_spacing == 1
   */
  /**@{*/
  int log2_cols; /*!< log2 of 'cols'. */
  int log2_rows; /*!< log2 of 'rows'. */
  int width;     /*!< tile width in MI units */
  int height;    /*!< tile height in MI units */
  /**@}*/

  /*!
   * Min num of tile columns possible based on 'max_width_sb' and frame width.
   */
  int min_log2_cols;
  /*!
   * Min num of tile rows possible based on 'max_height_sb' and frame height.
   */
  int min_log2_rows;
  /*!
   * Max num of tile columns possible based on frame width.
   */
  int max_log2_cols;
  /*!
   * Max num of tile rows possible based on frame height.
   */
  int max_log2_rows;
  /*!
   * log2 of min number of tiles (same as min_log2_cols + min_log2_rows).
   */
  int min_log2;
  /*!
   * col_start_sb[i] is the start position of tile column i in superblock units.
   * valid for 0 <= i <= cols
   */
  int col_start_sb[MAX_TILE_COLS + 1];
  /*!
   * row_start_sb[i] is the start position of tile row i in superblock units.
   * valid for 0 <= i <= rows
   */
  int row_start_sb[MAX_TILE_ROWS + 1];
  /*!
   * If true, we are using large scale tile mode.
   */
  unsigned int large_scale;
  /*!
   * Only relevant when large_scale == 1.
   * If true, the independent decoding of a single tile or a section of a frame
   * is allowed.
   */
  unsigned int single_tile_decoding;
} CommonTileParams;

typedef struct CommonModeInfoParams CommonModeInfoParams;
/*!
 * \brief Params related to MB_MODE_INFO arrays and related info.
 */
struct CommonModeInfoParams {
  /*!
   * Number of rows in the frame in 16 pixel units.
   * This is computed from frame height aligned to a multiple of 8.
   */
  int mb_rows;
  /*!
   * Number of cols in the frame in 16 pixel units.
   * This is computed from frame width aligned to a multiple of 8.
   */
  int mb_cols;

  /*!
   * Total MBs = mb_rows * mb_cols.
   */
  int MBs;

  /*!
   * Number of rows in the frame in 4 pixel (MB_MODE_INFO) units.
   * This is computed from frame height aligned to a multiple of 8.
   */
  int mi_rows;
  /*!
   * Number of cols in the frame in 4 pixel (MB_MODE_INFO) units.
   * This is computed from frame width aligned to a multiple of 8.
   */
  int mi_cols;

  /*!
   * An array of MB_MODE_INFO structs for every 'mi_alloc_bsize' sized block
   * in the frame.
   * Note: This array should be treated like a scratch memory, and should NOT be
   * accessed directly, in most cases. Please use 'mi_grid_base' array instead.
   */
  MB_MODE_INFO *mi_alloc;
  /*!
   * Number of allocated elements in 'mi_alloc'.
   */
  int mi_alloc_size;
  /*!
   * Stride for 'mi_alloc' array.
   */
  int mi_alloc_stride;
  /*!
   * The minimum block size that each element in 'mi_alloc' can correspond to.
   * For decoder, this is always BLOCK_4X4.
   * For encoder, this is BLOCK_8X8 for resolution >= 4k case or REALTIME mode
   * case. Otherwise, this is BLOCK_4X4.
   */
  BLOCK_SIZE mi_alloc_bsize;

  /*!
   * Grid of pointers to 4x4 MB_MODE_INFO structs allocated in 'mi_alloc'.
   * It's possible that:
   * - Multiple pointers in the grid point to the same element in 'mi_alloc'
   * (for example, for all 4x4 blocks that belong to the same partition block).
   * - Some pointers can be NULL (for example, for blocks outside visible area).
   */
  MB_MODE_INFO **mi_grid_base;
  /*!
   * Number of allocated elements in 'mi_grid_base' (and 'tx_type_map' also).
   */
  int mi_grid_size;
  /*!
   * Stride for 'mi_grid_base' (and 'tx_type_map' also).
   */
  int mi_stride;

  /*!
   * An array of tx types for each 4x4 block in the frame.
   * Number of allocated elements is same as 'mi_grid_size', and stride is
   * same as 'mi_grid_size'. So, indexing into 'tx_type_map' is same as that of
   * 'mi_grid_base'.
   */
  TX_TYPE *tx_type_map;

  /**
   * \name Function pointers to allow separate logic for encoder and decoder.
   */
  /**@{*/
  /*!
   * Free the memory allocated to arrays in 'mi_params'.
   * \param[in,out]   mi_params   object containing common mode info parameters
   */
  void (*free_mi)(struct CommonModeInfoParams *mi_params);
  /*!
   * Initialize / reset appropriate arrays in 'mi_params'.
   * \param[in,out]   mi_params   object containing common mode info parameters
   */
  void (*setup_mi)(struct CommonModeInfoParams *mi_params);
  /*!
   * Allocate required memory for arrays in 'mi_params'.
   * \param[in,out]   mi_params           object containing common mode info
   *                                      parameters
   * \param           width               frame width
   * \param           height              frame height
   * \param           min_partition_size  minimum partition size allowed while
   *                                      encoding
   */
  void (*set_mb_mi)(struct CommonModeInfoParams *mi_params, int width,
                    int height, BLOCK_SIZE min_partition_size);
  /**@}*/
};

typedef struct CommonQuantParams CommonQuantParams;
/*!
 * \brief Parameters related to quantization at the frame level.
 */
struct CommonQuantParams {
  /*!
   * Base qindex of the frame in the range 0 to 255.
   */
  int base_qindex;

  /*!
   * Sharpness adjustment in the quantization process.
   */
  int sharpness;

  /*!
   * Delta of qindex (from base_qindex) for Y plane DC coefficient.
   * Note: y_ac_delta_q is implicitly 0.
   */
  int y_dc_delta_q;

  /*!
   * Delta of qindex (from base_qindex) for U plane DC coefficients.
   */
  int u_dc_delta_q;
  /*!
   * Delta of qindex (from base_qindex) for U plane AC coefficients.
   */
  int v_dc_delta_q;

  /*!
   * Delta of qindex (from base_qindex) for V plane DC coefficients.
   * Same as those for U plane if cm->seq_params->separate_uv_delta_q == 0.
   */
  int u_ac_delta_q;
  /*!
   * Delta of qindex (from base_qindex) for V plane AC coefficients.
   * Same as those for U plane if cm->seq_params->separate_uv_delta_q == 0.
   */
  int v_ac_delta_q;

  /*
   * Note: The qindex per superblock may have a delta from the qindex obtained
   * at frame level from parameters above, based on 'cm->delta_q_info'.
   */

  /**
   * \name True dequantizers.
   * The dequantizers below are true dequantizers used only in the
   * dequantization process.  They have the same coefficient
   * shift/scale as TX.
   */
  /**@{*/
  int16_t y_dequant_QTX[MAX_SEGMENTS][2]; /*!< Dequant for Y plane */
  int16_t u_dequant_QTX[MAX_SEGMENTS][2]; /*!< Dequant for U plane */
  int16_t v_dequant_QTX[MAX_SEGMENTS][2]; /*!< Dequant for V plane */
  /**@}*/

  /**
   * \name Global quantization matrix tables.
   */
  /**@{*/
  /*!
   * Global dequantization matrix table.
   */
  const qm_val_t *giqmatrix[NUM_QM_LEVELS][3][TX_SIZES_ALL];
  /*!
   * Global quantization matrix table.
   */
  const qm_val_t *gqmatrix[NUM_QM_LEVELS][3][TX_SIZES_ALL];
  /**@}*/

  /**
   * \name Local dequantization matrix tables for each frame.
   */
  /**@{*/
  /*!
   * Local dequant matrix for Y plane.
   */
  const qm_val_t *y_iqmatrix[MAX_SEGMENTS][TX_SIZES_ALL];
  /*!
   * Local dequant matrix for U plane.
   */
  const qm_val_t *u_iqmatrix[MAX_SEGMENTS][TX_SIZES_ALL];
  /*!
   * Local dequant matrix for V plane.
   */
  const qm_val_t *v_iqmatrix[MAX_SEGMENTS][TX_SIZES_ALL];
  /**@}*/

  /*!
   * Flag indicating whether quantization matrices are being used:
   *  - If true, qm_level_y, qm_level_u and qm_level_v indicate the level
   *    indices to be used to access appropriate global quant matrix tables.
   *  - If false, we implicitly use level index 'NUM_QM_LEVELS - 1'.
   */
  bool using_qmatrix;
  /**
   * \name Valid only when using_qmatrix == true
   * Indicate the level indices to be used to access appropriate global quant
   * matrix tables.
   */
  /**@{*/
  int qmatrix_level_y; /*!< Level index for Y plane */
  int qmatrix_level_u; /*!< Level index for U plane */
  int qmatrix_level_v; /*!< Level index for V plane */
  /**@}*/
};

typedef struct CommonContexts CommonContexts;
/*!
 * \brief Contexts used for transmitting various symbols in the bitstream.
 */
struct CommonContexts {
  /*!
   * Context used by 'FRAME_CONTEXT.partition_cdf' to transmit partition type.
   * partition[i][j] is the context for ith tile row, jth mi_col.
   */
  PARTITION_CONTEXT **partition;

  /*!
   * Context used to derive context for multiple symbols:
   * - 'TXB_CTX.txb_skip_ctx' used by 'FRAME_CONTEXT.txb_skip_cdf' to transmit
   * to transmit skip_txfm flag.
   * - 'TXB_CTX.dc_sign_ctx' used by 'FRAME_CONTEXT.dc_sign_cdf' to transmit
   * sign.
   * entropy[i][j][k] is the context for ith plane, jth tile row, kth mi_col.
   */
  ENTROPY_CONTEXT **entropy[MAX_MB_PLANE];

  /*!
   * Context used to derive context for 'FRAME_CONTEXT.txfm_partition_cdf' to
   * transmit 'is_split' flag to indicate if this transform block should be
   * split into smaller sub-blocks.
   * txfm[i][j] is the context for ith tile row, jth mi_col.
   */
  TXFM_CONTEXT **txfm;

  /*!
   * Dimensions that were used to allocate the arrays above.
   * If these dimensions change, the arrays may have to be re-allocated.
   */
  int num_planes;    /*!< Corresponds to av1_num_planes(cm) */
  int num_tile_rows; /*!< Corresponds to cm->tiles.row */
  int num_mi_cols;   /*!< Corresponds to cm->mi_params.mi_cols */
};

/*!
 * \brief Top level common structure used by both encoder and decoder.
 */
typedef struct AV1Common {
  /*!
   * Information about the current frame that is being coded.
   */
  CurrentFrame current_frame;
  /*!
   * Code and details about current error status.
   */
  struct aom_internal_error_info *error;

  /*!
   * AV1 allows two types of frame scaling operations:
   * 1. Frame super-resolution: that allows coding a frame at lower resolution
   * and after decoding the frame, normatively scales and restores the frame --
   * inside the coding loop.
   * 2. Frame resize: that allows coding frame at lower/higher resolution, and
   * then non-normatively upscale the frame at the time of rendering -- outside
   * the coding loop.
   * Hence, the need for 3 types of dimensions.
   */

  /**
   * \name Coded frame dimensions.
   */
  /**@{*/
  int width;  /*!< Coded frame width */
  int height; /*!< Coded frame height */
  /**@}*/

  /**
   * \name Rendered frame dimensions.
   * Dimensions after applying both super-resolution and resize to the coded
   * frame. Different from coded dimensions if super-resolution and/or resize
   * are being used for this frame.
   */
  /**@{*/
  int render_width;  /*!< Rendered frame width */
  int render_height; /*!< Rendered frame height */
  /**@}*/

  /**
   * \name Super-resolved frame dimensions.
   * Frame dimensions after applying super-resolution to the coded frame (if
   * present), but before applying resize.
   * Larger than the coded dimensions if super-resolution is being used for
   * this frame.
   * Different from rendered dimensions if resize is being used for this frame.
   */
  /**@{*/
  int superres_upscaled_width;  /*!< Super-resolved frame width */
  int superres_upscaled_height; /*!< Super-resolved frame height */
  /**@}*/

  /*!
   * The denominator of the superres scale used by this frame.
   * Note: The numerator is fixed to be SCALE_NUMERATOR.
   */
  uint8_t superres_scale_denominator;

  /*!
   * buffer_removal_times[op_num] specifies the frame removal time in units of
   * DecCT clock ticks counted from the removal time of the last random access
   * point for operating point op_num.
   * TODO(urvang): We probably don't need the +1 here.
   */
  uint32_t buffer_removal_times[MAX_NUM_OPERATING_POINTS + 1];
  /*!
   * Presentation time of the frame in clock ticks DispCT counted from the
   * removal time of the last random access point for the operating point that
   * is being decoded.
   */
  uint32_t frame_presentation_time;

  /*!
   * Buffer where previous frame is stored.
   */
  RefCntBuffer *prev_frame;

  /*!
   * Buffer into which the current frame will be stored and other related info.
   * TODO(hkuang): Combine this with cur_buf in macroblockd.
   */
  RefCntBuffer *cur_frame;

  /*!
   * For encoder, we have a two-level mapping from reference frame type to the
   * corresponding buffer in the buffer pool:
   * * 'remapped_ref_idx[i - 1]' maps reference type 'i' (range: LAST_FRAME ...
   * EXTREF_FRAME) to a remapped index 'j' (in range: 0 ... REF_FRAMES - 1)
   * * Later, 'cm->ref_frame_map[j]' maps the remapped index 'j' to a pointer to
   * the reference counted buffer structure RefCntBuffer, taken from the buffer
   * pool cm->buffer_pool->frame_bufs.
   *
   * LAST_FRAME,                        ...,      EXTREF_FRAME
   *      |                                           |
   *      v                                           v
   * remapped_ref_idx[LAST_FRAME - 1],  ...,  remapped_ref_idx[EXTREF_FRAME - 1]
   *      |                                           |
   *      v                                           v
   * ref_frame_map[],                   ...,     ref_frame_map[]
   *
   * Note: INTRA_FRAME always refers to the current frame, so there's no need to
   * have a remapped index for the same.
   */
  int remapped_ref_idx[REF_FRAMES];

  /*!
   * Scale of the current frame with respect to itself.
   * This is currently used for intra block copy, which behaves like an inter
   * prediction mode, where the reference frame is the current frame itself.
   */
  struct scale_factors sf_identity;

  /*!
   * Scale factors of the reference frame with respect to the current frame.
   * This is required for generating inter prediction and will be non-identity
   * for a reference frame, if it has different dimensions than the coded
   * dimensions of the current frame.
   */
  struct scale_factors ref_scale_factors[REF_FRAMES];

  /*!
   * For decoder, ref_frame_map[i] maps reference type 'i' to a pointer to
   * the buffer in the buffer pool 'cm->buffer_pool.frame_bufs'.
   * For encoder, ref_frame_map[j] (where j = remapped_ref_idx[i]) maps
   * remapped reference index 'j' (that is, original reference type 'i') to
   * a pointer to the buffer in the buffer pool 'cm->buffer_pool.frame_bufs'.
   */
  RefCntBuffer *ref_frame_map[REF_FRAMES];

  /*!
   * If true, this frame is actually shown after decoding.
   * If false, this frame is coded in the bitstream, but not shown. It is only
   * used as a reference for other frames coded later.
   */
  int show_frame;

  /*!
   * If true, this frame can be used as a show-existing frame for other frames
   * coded later.
   * When 'show_frame' is true, this is always true for all non-keyframes.
   * When 'show_frame' is false, this value is transmitted in the bitstream.
   */
  int showable_frame;

  /*!
   * If true, show an existing frame coded before, instead of actually coding a
   * frame. The existing frame comes from one of the existing reference buffers,
   * as signaled in the bitstream.
   */
  int show_existing_frame;

  /*!
   * Whether some features are allowed or not.
   */
  FeatureFlags features;

  /*!
   * Params related to MB_MODE_INFO arrays and related info.
   */
  CommonModeInfoParams mi_params;

#if CONFIG_ENTROPY_STATS
  /*!
   * Context type used by token CDFs, in the range 0 .. (TOKEN_CDF_Q_CTXS - 1).
   */
  int coef_cdf_category;
#endif  // CONFIG_ENTROPY_STATS

  /*!
   * Quantization params.
   */
  CommonQuantParams quant_params;

  /*!
   * Segmentation info for current frame.
   */
  struct segmentation seg;

  /*!
   * Segmentation map for previous frame.
   */
  uint8_t *last_frame_seg_map;

  /**
   * \name Deblocking filter parameters.
   */
  /**@{*/
  loop_filter_info_n lf_info; /*!< Loop filter info */
  struct loopfilter lf;       /*!< Loop filter parameters */
  /**@}*/

  /**
   * \name Loop Restoration filter parameters.
   */
  /**@{*/
  RestorationInfo rst_info[MAX_MB_PLANE]; /*!< Loop Restoration filter info */
  int32_t *rst_tmpbuf; /*!< Scratch buffer for self-guided restoration */
  RestorationLineBuffers *rlbs; /*!< Line buffers needed by loop restoration */
  YV12_BUFFER_CONFIG rst_frame; /*!< Stores the output of loop restoration */
  /**@}*/

  /*!
   * CDEF (Constrained Directional Enhancement Filter) parameters.
   */
  CdefInfo cdef_info;

  /*!
   * Parameters for film grain synthesis.
   */
  aom_film_grain_t film_grain_params;

  /*!
   * Parameters for delta quantization and delta loop filter level.
   */
  DeltaQInfo delta_q_info;

  /*!
   * Global motion parameters for each reference frame.
   */
  WarpedMotionParams global_motion[REF_FRAMES];

  /*!
   * Elements part of the sequence header, that are applicable for all the
   * frames in the video.
   */
  SequenceHeader *seq_params;

  /*!
   * Current CDFs of all the symbols for the current frame.
   */
  FRAME_CONTEXT *fc;
  /*!
   * Default CDFs used when features.primary_ref_frame = PRIMARY_REF_NONE
   * (e.g. for a keyframe). These default CDFs are defined by the bitstream and
   * copied from default CDF tables for each symbol.
   */
  FRAME_CONTEXT *default_frame_context;

  /*!
   * Parameters related to tiling.
   */
  CommonTileParams tiles;

  /*!
   * External BufferPool passed from outside.
   */
  BufferPool *buffer_pool;

  /*!
   * Above context buffers and their sizes.
   * Note: above contexts are allocated in this struct, as their size is
   * dependent on frame width, while left contexts are declared and allocated in
   * MACROBLOCKD struct, as they have a fixed size.
   */
  CommonContexts above_contexts;

  /**
   * \name Signaled when cm->seq_params->frame_id_numbers_present_flag == 1
   */
  /**@{*/
  int current_frame_id;         /*!< frame ID for the current frame. */
  int ref_frame_id[REF_FRAMES]; /*!< frame IDs for the reference frames. */
  /**@}*/

  /*!
   * Motion vectors provided by motion field estimation.
   * tpl_mvs[row * stride + col] stores MV for block at [mi_row, mi_col] where:
   * mi_row = 2 * row,
   * mi_col = 2 * col, and
   * stride = cm->mi_params.mi_stride / 2
   */
  TPL_MV_REF *tpl_mvs;
  /*!
   * Allocated size of 'tpl_mvs' array. Refer to 'ensure_mv_buffer()' function.
   */
  int tpl_mvs_mem_size;
  /*!
   * ref_frame_sign_bias[k] is 1 if relative distance between reference 'k' and
   * current frame is positive; and 0 otherwise.
   */
  int ref_frame_sign_bias[REF_FRAMES];
  /*!
   * ref_frame_side[k] is 1 if relative distance between reference 'k' and
   * current frame is positive, -1 if relative distance is 0; and 0 otherwise.
   * TODO(jingning): This can be combined with sign_bias later.
   */
  int8_t ref_frame_side[REF_FRAMES];

  /*!
   * Temporal layer ID of this frame
   * (in the range 0 ... (number_temporal_layers - 1)).
   */
  int temporal_layer_id;

  /*!
   * Spatial layer ID of this frame
   * (in the range 0 ... (number_spatial_layers - 1)).
   */
  int spatial_layer_id;

#if TXCOEFF_TIMER
  int64_t cum_txcoeff_timer;
  int64_t txcoeff_timer;
  int txb_count;
#endif  // TXCOEFF_TIMER

#if TXCOEFF_COST_TIMER
  int64_t cum_txcoeff_cost_timer;
  int64_t txcoeff_cost_timer;
  int64_t txcoeff_cost_count;
#endif  // TXCOEFF_COST_TIMER
} AV1_COMMON;

/*!\cond */

// TODO(hkuang): Don't need to lock the whole pool after implementing atomic
// frame reference count.
static void lock_buffer_pool(BufferPool *const pool) {
#if CONFIG_MULTITHREAD
  pthread_mutex_lock(&pool->pool_mutex);
#else
  (void)pool;
#endif
}

static void unlock_buffer_pool(BufferPool *const pool) {
#if CONFIG_MULTITHREAD
  pthread_mutex_unlock(&pool->pool_mutex);
#else
  (void)pool;
#endif
}

static inline YV12_BUFFER_CONFIG *get_ref_frame(AV1_COMMON *cm, int index) {
  if (index < 0 || index >= REF_FRAMES) return NULL;
  if (cm->ref_frame_map[index] == NULL) return NULL;
  return &cm->ref_frame_map[index]->buf;
}

static inline int get_free_fb(AV1_COMMON *cm) {
  RefCntBuffer *const frame_bufs = cm->buffer_pool->frame_bufs;
  int i;

  lock_buffer_pool(cm->buffer_pool);
  const int num_frame_bufs = cm->buffer_pool->num_frame_bufs;
  for (i = 0; i < num_frame_bufs; ++i)
    if (frame_bufs[i].ref_count == 0) break;

  if (i != num_frame_bufs) {
    if (frame_bufs[i].buf.use_external_reference_buffers) {
      // If this frame buffer's y_buffer, u_buffer, and v_buffer point to the
      // external reference buffers. Restore the buffer pointers to point to the
      // internally allocated memory.
      YV12_BUFFER_CONFIG *ybf = &frame_bufs[i].buf;
      ybf->y_buffer = ybf->store_buf_adr[0];
      ybf->u_buffer = ybf->store_buf_adr[1];
      ybf->v_buffer = ybf->store_buf_adr[2];
      ybf->use_external_reference_buffers = 0;
    }

    frame_bufs[i].ref_count = 1;
  } else {
    // We should never run out of free buffers. If this assertion fails, there
    // is a reference leak.
    assert(0 && "Ran out of free frame buffers. Likely a reference leak.");
    // Reset i to be INVALID_IDX to indicate no free buffer found.
    i = INVALID_IDX;
  }

  unlock_buffer_pool(cm->buffer_pool);
  return i;
}

static inline RefCntBuffer *assign_cur_frame_new_fb(AV1_COMMON *const cm) {
  // Release the previously-used frame-buffer
  if (cm->cur_frame != NULL) {
    --cm->cur_frame->ref_count;
    cm->cur_frame = NULL;
  }

  // Assign a new framebuffer
  const int new_fb_idx = get_free_fb(cm);
  if (new_fb_idx == INVALID_IDX) return NULL;

  cm->cur_frame = &cm->buffer_pool->frame_bufs[new_fb_idx];
#if CONFIG_AV1_ENCODER && !CONFIG_REALTIME_ONLY
  aom_invalidate_pyramid(cm->cur_frame->buf.y_pyramid);
  av1_invalidate_corner_list(cm->cur_frame->buf.corners);
#endif  // CONFIG_AV1_ENCODER && !CONFIG_REALTIME_ONLY
  av1_zero(cm->cur_frame->interp_filter_selected);
  return cm->cur_frame;
}

// Modify 'lhs_ptr' to reference the buffer at 'rhs_ptr', and update the ref
// counts accordingly.
static inline void assign_frame_buffer_p(RefCntBuffer **lhs_ptr,
                                         RefCntBuffer *rhs_ptr) {
  RefCntBuffer *const old_ptr = *lhs_ptr;
  if (old_ptr != NULL) {
    assert(old_ptr->ref_count > 0);
    // One less reference to the buffer at 'old_ptr', so decrease ref count.
    --old_ptr->ref_count;
  }

  *lhs_ptr = rhs_ptr;
  // One more reference to the buffer at 'rhs_ptr', so increase ref count.
  ++rhs_ptr->ref_count;
}

static inline int frame_is_intra_only(const AV1_COMMON *const cm) {
  return cm->current_frame.frame_type == KEY_FRAME ||
         cm->current_frame.frame_type == INTRA_ONLY_FRAME;
}

static inline int frame_is_sframe(const AV1_COMMON *cm) {
  return cm->current_frame.frame_type == S_FRAME;
}

// These functions take a reference frame label between LAST_FRAME and
// EXTREF_FRAME inclusive.  Note that this is different to the indexing
// previously used by the frame_refs[] array.
static inline int get_ref_frame_map_idx(const AV1_COMMON *const cm,
                                        const MV_REFERENCE_FRAME ref_frame) {
  return (ref_frame >= LAST_FRAME && ref_frame <= EXTREF_FRAME)
             ? cm->remapped_ref_idx[ref_frame - LAST_FRAME]
             : INVALID_IDX;
}

static inline RefCntBuffer *get_ref_frame_buf(
    const AV1_COMMON *const cm, const MV_REFERENCE_FRAME ref_frame) {
  const int map_idx = get_ref_frame_map_idx(cm, ref_frame);
  return (map_idx != INVALID_IDX) ? cm->ref_frame_map[map_idx] : NULL;
}

// Both const and non-const versions of this function are provided so that it
// can be used with a const AV1_COMMON if needed.
static inline const struct scale_factors *get_ref_scale_factors_const(
    const AV1_COMMON *const cm, const MV_REFERENCE_FRAME ref_frame) {
  const int map_idx = get_ref_frame_map_idx(cm, ref_frame);
  return (map_idx != INVALID_IDX) ? &cm->ref_scale_factors[map_idx] : NULL;
}

static inline struct scale_factors *get_ref_scale_factors(
    AV1_COMMON *const cm, const MV_REFERENCE_FRAME ref_frame) {
  const int map_idx = get_ref_frame_map_idx(cm, ref_frame);
  return (map_idx != INVALID_IDX) ? &cm->ref_scale_factors[map_idx] : NULL;
}

static inline RefCntBuffer *get_primary_ref_frame_buf(
    const AV1_COMMON *const cm) {
  const int primary_ref_frame = cm->features.primary_ref_frame;
  if (primary_ref_frame == PRIMARY_REF_NONE) return NULL;
  const int map_idx = get_ref_frame_map_idx(cm, primary_ref_frame + 1);
  return (map_idx != INVALID_IDX) ? cm->ref_frame_map[map_idx] : NULL;
}

// Returns 1 if this frame might allow mvs from some reference frame.
static inline int frame_might_allow_ref_frame_mvs(const AV1_COMMON *cm) {
  return !cm->features.error_resilient_mode &&
         cm->seq_params->order_hint_info.enable_ref_frame_mvs &&
         cm->seq_params->order_hint_info.enable_order_hint &&
         !frame_is_intra_only(cm);
}

// Returns 1 if this frame might use warped_motion
static inline int frame_might_allow_warped_motion(const AV1_COMMON *cm) {
  return !cm->features.error_resilient_mode && !frame_is_intra_only(cm) &&
         cm->seq_params->enable_warped_motion;
}

static inline void ensure_mv_buffer(RefCntBuffer *buf, AV1_COMMON *cm) {
  const int buf_rows = buf->mi_rows;
  const int buf_cols = buf->mi_cols;
  const CommonModeInfoParams *const mi_params = &cm->mi_params;

  if (buf->mvs == NULL || buf_rows != mi_params->mi_rows ||
      buf_cols != mi_params->mi_cols) {
    aom_free(buf->mvs);
    buf->mi_rows = mi_params->mi_rows;
    buf->mi_cols = mi_params->mi_cols;
    CHECK_MEM_ERROR(cm, buf->mvs,
                    (MV_REF *)aom_calloc(((mi_params->mi_rows + 1) >> 1) *
                                             ((mi_params->mi_cols + 1) >> 1),
                                         sizeof(*buf->mvs)));
    aom_free(buf->seg_map);
    CHECK_MEM_ERROR(
        cm, buf->seg_map,
        (uint8_t *)aom_calloc(mi_params->mi_rows * mi_params->mi_cols,
                              sizeof(*buf->seg_map)));
  }

  const int mem_size =
      ((mi_params->mi_rows + MAX_MIB_SIZE) >> 1) * (mi_params->mi_stride >> 1);

  if (cm->tpl_mvs == NULL || cm->tpl_mvs_mem_size < mem_size) {
    aom_free(cm->tpl_mvs);
    CHECK_MEM_ERROR(cm, cm->tpl_mvs,
                    (TPL_MV_REF *)aom_calloc(mem_size, sizeof(*cm->tpl_mvs)));
    cm->tpl_mvs_mem_size = mem_size;
  }
}

#if !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
void cfl_init(CFL_CTX *cfl, const SequenceHeader *seq_params);
#endif

static inline int av1_num_planes(const AV1_COMMON *cm) {
  return cm->seq_params->monochrome ? 1 : MAX_MB_PLANE;
}

static inline void av1_init_above_context(CommonContexts *above_contexts,
                                          int num_planes, int tile_row,
                                          MACROBLOCKD *xd) {
  for (int i = 0; i < num_planes; ++i) {
    xd->above_entropy_context[i] = above_contexts->entropy[i][tile_row];
  }
  xd->above_partition_context = above_contexts->partition[tile_row];
  xd->above_txfm_context = above_contexts->txfm[tile_row];
}

static inline void av1_init_macroblockd(AV1_COMMON *cm, MACROBLOCKD *xd) {
  const int num_planes = av1_num_planes(cm);
  const CommonQuantParams *const quant_params = &cm->quant_params;

  for (int i = 0; i < num_planes; ++i) {
    if (xd->plane[i].plane_type == PLANE_TYPE_Y) {
      memcpy(xd->plane[i].seg_dequant_QTX, quant_params->y_dequant_QTX,
             sizeof(quant_params->y_dequant_QTX));
      memcpy(xd->plane[i].seg_iqmatrix, quant_params->y_iqmatrix,
             sizeof(quant_params->y_iqmatrix));

    } else {
      if (i == AOM_PLANE_U) {
        memcpy(xd->plane[i].seg_dequant_QTX, quant_params->u_dequant_QTX,
               sizeof(quant_params->u_dequant_QTX));
        memcpy(xd->plane[i].seg_iqmatrix, quant_params->u_iqmatrix,
               sizeof(quant_params->u_iqmatrix));
      } else {
        memcpy(xd->plane[i].seg_dequant_QTX, quant_params->v_dequant_QTX,
               sizeof(quant_params->v_dequant_QTX));
        memcpy(xd->plane[i].seg_iqmatrix, quant_params->v_iqmatrix,
               sizeof(quant_params->v_iqmatrix));
      }
    }
  }
  xd->mi_stride = cm->mi_params.mi_stride;
  xd->error_info = cm->error;
#if !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
  cfl_init(&xd->cfl, cm->seq_params);
#endif
}

static inline void set_entropy_context(MACROBLOCKD *xd, int mi_row, int mi_col,
                                       const int num_planes) {
  int i;
  int row_offset = mi_row;
  int col_offset = mi_col;
  for (i = 0; i < num_planes; ++i) {
    struct macroblockd_plane *const pd = &xd->plane[i];
    // Offset the buffer pointer
    const BLOCK_SIZE bsize = xd->mi[0]->bsize;
    if (pd->subsampling_y && (mi_row & 0x01) && (mi_size_high[bsize] == 1))
      row_offset = mi_row - 1;
    if (pd->subsampling_x && (mi_col & 0x01) && (mi_size_wide[bsize] == 1))
      col_offset = mi_col - 1;
    int above_idx = col_offset;
    int left_idx = row_offset & MAX_MIB_MASK;
    pd->above_entropy_context =
        &xd->above_entropy_context[i][above_idx >> pd->subsampling_x];
    pd->left_entropy_context =
        &xd->left_entropy_context[i][left_idx >> pd->subsampling_y];
  }
}

static inline int calc_mi_size(int len) {
  // len is in mi units. Align to a multiple of SBs.
  return ALIGN_POWER_OF_TWO(len, MAX_MIB_SIZE_LOG2);
}

static inline void set_plane_n4(MACROBLOCKD *const xd, int bw, int bh,
                                const int num_planes) {
  int i;
  for (i = 0; i < num_planes; i++) {
    xd->plane[i].width = (bw * MI_SIZE) >> xd->plane[i].subsampling_x;
    xd->plane[i].height = (bh * MI_SIZE) >> xd->plane[i].subsampling_y;

    xd->plane[i].width = AOMMAX(xd->plane[i].width, 4);
    xd->plane[i].height = AOMMAX(xd->plane[i].height, 4);
  }
}

static inline void set_mi_row_col(MACROBLOCKD *xd, const TileInfo *const tile,
                                  int mi_row, int bh, int mi_col, int bw,
                                  int mi_rows, int mi_cols) {
  xd->mb_to_top_edge = -GET_MV_SUBPEL(mi_row * MI_SIZE);
  xd->mb_to_bottom_edge = GET_MV_SUBPEL((mi_rows - bh - mi_row) * MI_SIZE);
  xd->mb_to_left_edge = -GET_MV_SUBPEL((mi_col * MI_SIZE));
  xd->mb_to_right_edge = GET_MV_SUBPEL((mi_cols - bw - mi_col) * MI_SIZE);

  xd->mi_row = mi_row;
  xd->mi_col = mi_col;

  // Are edges available for intra prediction?
  xd->up_available = (mi_row > tile->mi_row_start);

  const int ss_x = xd->plane[1].subsampling_x;
  const int ss_y = xd->plane[1].subsampling_y;

  xd->left_available = (mi_col > tile->mi_col_start);
  xd->chroma_up_available = xd->up_available;
  xd->chroma_left_available = xd->left_available;
  if (ss_x && bw < mi_size_wide[BLOCK_8X8])
    xd->chroma_left_available = (mi_col - 1) > tile->mi_col_start;
  if (ss_y && bh < mi_size_high[BLOCK_8X8])
    xd->chroma_up_available = (mi_row - 1) > tile->mi_row_start;
  if (xd->up_available) {
    xd->above_mbmi = xd->mi[-xd->mi_stride];
  } else {
    xd->above_mbmi = NULL;
  }

  if (xd->left_available) {
    xd->left_mbmi = xd->mi[-1];
  } else {
    xd->left_mbmi = NULL;
  }

  const int chroma_ref = ((mi_row & 0x01) || !(bh & 0x01) || !ss_y) &&
                         ((mi_col & 0x01) || !(bw & 0x01) || !ss_x);
  xd->is_chroma_ref = chroma_ref;
  if (chroma_ref) {
    // To help calculate the "above" and "left" chroma blocks, note that the
    // current block may cover multiple luma blocks (e.g., if partitioned into
    // 4x4 luma blocks).
    // First, find the top-left-most luma block covered by this chroma block
    MB_MODE_INFO **base_mi =
        &xd->mi[-(mi_row & ss_y) * xd->mi_stride - (mi_col & ss_x)];

    // Then, we consider the luma region covered by the left or above 4x4 chroma
    // prediction. We want to point to the chroma reference block in that
    // region, which is the bottom-right-most mi unit.
    // This leads to the following offsets:
    MB_MODE_INFO *chroma_above_mi =
        xd->chroma_up_available ? base_mi[-xd->mi_stride + ss_x] : NULL;
    xd->chroma_above_mbmi = chroma_above_mi;

    MB_MODE_INFO *chroma_left_mi =
        xd->chroma_left_available ? base_mi[ss_y * xd->mi_stride - 1] : NULL;
    xd->chroma_left_mbmi = chroma_left_mi;
  }

  xd->height = bh;
  xd->width = bw;

  xd->is_last_vertical_rect = 0;
  if (xd->width < xd->height) {
    if (!((mi_col + xd->width) & (xd->height - 1))) {
      xd->is_last_vertical_rect = 1;
    }
  }

  xd->is_first_horizontal_rect = 0;
  if (xd->width > xd->height)
    if (!(mi_row & (xd->width - 1))) xd->is_first_horizontal_rect = 1;
}

static inline aom_cdf_prob *get_y_mode_cdf(FRAME_CONTEXT *tile_ctx,
                                           const MB_MODE_INFO *above_mi,
                                           const MB_MODE_INFO *left_mi) {
  const PREDICTION_MODE above = av1_above_block_mode(above_mi);
  const PREDICTION_MODE left = av1_left_block_mode(left_mi);
  const int above_ctx = intra_mode_context[above];
  const int left_ctx = intra_mode_context[left];
  return tile_ctx->kf_y_cdf[above_ctx][left_ctx];
}

static inline void update_partition_context(MACROBLOCKD *xd, int mi_row,
                                            int mi_col, BLOCK_SIZE subsize,
                                            BLOCK_SIZE bsize) {
  PARTITION_CONTEXT *const above_ctx = xd->above_partition_context + mi_col;
  PARTITION_CONTEXT *const left_ctx =
      xd->left_partition_context + (mi_row & MAX_MIB_MASK);

  const int bw = mi_size_wide[bsize];
  const int bh = mi_size_high[bsize];
  memset(above_ctx, partition_context_lookup[subsize].above, bw);
  memset(left_ctx, partition_context_lookup[subsize].left, bh);
}

static inline int is_chroma_reference(int mi_row, int mi_col, BLOCK_SIZE bsize,
                                      int subsampling_x, int subsampling_y) {
  assert(bsize < BLOCK_SIZES_ALL);
  const int bw = mi_size_wide[bsize];
  const int bh = mi_size_high[bsize];
  int ref_pos = ((mi_row & 0x01) || !(bh & 0x01) || !subsampling_y) &&
                ((mi_col & 0x01) || !(bw & 0x01) || !subsampling_x);
  return ref_pos;
}

static inline aom_cdf_prob cdf_element_prob(const aom_cdf_prob *cdf,
                                            size_t element) {
  assert(cdf != NULL);
  return (element > 0 ? cdf[element - 1] : CDF_PROB_TOP) - cdf[element];
}

static inline void partition_gather_horz_alike(aom_cdf_prob *out,
                                               const aom_cdf_prob *const in,
                                               BLOCK_SIZE bsize) {
  (void)bsize;
  out[0] = CDF_PROB_TOP;
  out[0] -= cdf_element_prob(in, PARTITION_HORZ);
  out[0] -= cdf_element_prob(in, PARTITION_SPLIT);
  out[0] -= cdf_element_prob(in, PARTITION_HORZ_A);
  out[0] -= cdf_element_prob(in, PARTITION_HORZ_B);
  out[0] -= cdf_element_prob(in, PARTITION_VERT_A);
  if (bsize != BLOCK_128X128) out[0] -= cdf_element_prob(in, PARTITION_HORZ_4);
  out[0] = AOM_ICDF(out[0]);
  out[1] = AOM_ICDF(CDF_PROB_TOP);
}

static inline void partition_gather_vert_alike(aom_cdf_prob *out,
                                               const aom_cdf_prob *const in,
                                               BLOCK_SIZE bsize) {
  (void)bsize;
  out[0] = CDF_PROB_TOP;
  out[0] -= cdf_element_prob(in, PARTITION_VERT);
  out[0] -= cdf_element_prob(in, PARTITION_SPLIT);
  out[0] -= cdf_element_prob(in, PARTITION_HORZ_A);
  out[0] -= cdf_element_prob(in, PARTITION_VERT_A);
  out[0] -= cdf_element_prob(in, PARTITION_VERT_B);
  if (bsize != BLOCK_128X128) out[0] -= cdf_element_prob(in, PARTITION_VERT_4);
  out[0] = AOM_ICDF(out[0]);
  out[1] = AOM_ICDF(CDF_PROB_TOP);
}

static inline void update_ext_partition_context(MACROBLOCKD *xd, int mi_row,
                                                int mi_col, BLOCK_SIZE subsize,
                                                BLOCK_SIZE bsize,
                                                PARTITION_TYPE partition) {
  if (bsize >= BLOCK_8X8) {
    const int hbs = mi_size_wide[bsize] / 2;
    BLOCK_SIZE bsize2 = get_partition_subsize(bsize, PARTITION_SPLIT);
    switch (partition) {
      case PARTITION_SPLIT:
        if (bsize != BLOCK_8X8) break;
        AOM_FALLTHROUGH_INTENDED;
      case PARTITION_NONE:
      case PARTITION_HORZ:
      case PARTITION_VERT:
      case PARTITION_HORZ_4:
      case PARTITION_VERT_4:
        update_partition_context(xd, mi_row, mi_col, subsize, bsize);
        break;
      case PARTITION_HORZ_A:
        update_partition_context(xd, mi_row, mi_col, bsize2, subsize);
        update_partition_context(xd, mi_row + hbs, mi_col, subsize, subsize);
        break;
      case PARTITION_HORZ_B:
        update_partition_context(xd, mi_row, mi_col, subsize, subsize);
        update_partition_context(xd, mi_row + hbs, mi_col, bsize2, subsize);
        break;
      case PARTITION_VERT_A:
        update_partition_context(xd, mi_row, mi_col, bsize2, subsize);
        update_partition_context(xd, mi_row, mi_col + hbs, subsize, subsize);
        break;
      case PARTITION_VERT_B:
        update_partition_context(xd, mi_row, mi_col, subsize, subsize);
        update_partition_context(xd, mi_row, mi_col + hbs, bsize2, subsize);
        break;
      default: assert(0 && "Invalid partition type");
    }
  }
}

static inline int partition_plane_context(const MACROBLOCKD *xd, int mi_row,
                                          int mi_col, BLOCK_SIZE bsize) {
  const PARTITION_CONTEXT *above_ctx = xd->above_partition_context + mi_col;
  const PARTITION_CONTEXT *left_ctx =
      xd->left_partition_context + (mi_row & MAX_MIB_MASK);
  // Minimum partition point is 8x8. Offset the bsl accordingly.
  const int bsl = mi_size_wide_log2[bsize] - mi_size_wide_log2[BLOCK_8X8];
  int above = (*above_ctx >> bsl) & 1, left = (*left_ctx >> bsl) & 1;

  assert(mi_size_wide_log2[bsize] == mi_size_high_log2[bsize]);
  assert(bsl >= 0);

  return (left * 2 + above) + bsl * PARTITION_PLOFFSET;
}

// Return the number of elements in the partition CDF when
// partitioning the (square) block with luma block size of bsize.
static inline int partition_cdf_length(BLOCK_SIZE bsize) {
  if (bsize <= BLOCK_8X8)
    return PARTITION_TYPES;
  else if (bsize == BLOCK_128X128)
    return EXT_PARTITION_TYPES - 2;
  else
    return EXT_PARTITION_TYPES;
}

static inline int max_block_wide(const MACROBLOCKD *xd, BLOCK_SIZE bsize,
                                 int plane) {
  assert(bsize < BLOCK_SIZES_ALL);
  int max_blocks_wide = block_size_wide[bsize];

  if (xd->mb_to_right_edge < 0) {
    const struct macroblockd_plane *const pd = &xd->plane[plane];
    max_blocks_wide += xd->mb_to_right_edge >> (3 + pd->subsampling_x);
  }

  // Scale the width in the transform block unit.
  return max_blocks_wide >> MI_SIZE_LOG2;
}

static inline int max_block_high(const MACROBLOCKD *xd, BLOCK_SIZE bsize,
                                 int plane) {
  int max_blocks_high = block_size_high[bsize];

  if (xd->mb_to_bottom_edge < 0) {
    const struct macroblockd_plane *const pd = &xd->plane[plane];
    max_blocks_high += xd->mb_to_bottom_edge >> (3 + pd->subsampling_y);
  }

  // Scale the height in the transform block unit.
  return max_blocks_high >> MI_SIZE_LOG2;
}

static inline void av1_zero_above_context(AV1_COMMON *const cm,
                                          const MACROBLOCKD *xd,
                                          int mi_col_start, int mi_col_end,
                                          const int tile_row) {
  const SequenceHeader *const seq_params = cm->seq_params;
  const int num_planes = av1_num_planes(cm);
  const int width = mi_col_end - mi_col_start;
  const int aligned_width =
      ALIGN_POWER_OF_TWO(width, seq_params->mib_size_log2);
  const int offset_y = mi_col_start;
  const int width_y = aligned_width;
  const int offset_uv = offset_y >> seq_params->subsampling_x;
  const int width_uv = width_y >> seq_params->subsampling_x;
  CommonContexts *const above_contexts = &cm->above_contexts;

  av1_zero_array(above_contexts->entropy[0][tile_row] + offset_y, width_y);
  if (num_planes > 1) {
    if (above_contexts->entropy[1][tile_row] &&
        above_contexts->entropy[2][tile_row]) {
      av1_zero_array(above_contexts->entropy[1][tile_row] + offset_uv,
                     width_uv);
      av1_zero_array(above_contexts->entropy[2][tile_row] + offset_uv,
                     width_uv);
    } else {
      aom_internal_error(xd->error_info, AOM_CODEC_CORRUPT_FRAME,
                         "Invalid value of planes");
    }
  }

  av1_zero_array(above_contexts->partition[tile_row] + mi_col_start,
                 aligned_width);

  memset(above_contexts->txfm[tile_row] + mi_col_start,
         tx_size_wide[TX_SIZES_LARGEST], aligned_width * sizeof(TXFM_CONTEXT));
}

static inline void av1_zero_left_context(MACROBLOCKD *const xd) {
  av1_zero(xd->left_entropy_context);
  av1_zero(xd->left_partition_context);

  memset(xd->left_txfm_context_buffer, tx_size_high[TX_SIZES_LARGEST],
         sizeof(xd->left_txfm_context_buffer));
}

static inline void set_txfm_ctx(TXFM_CONTEXT *txfm_ctx, uint8_t txs, int len) {
  int i;
  for (i = 0; i < len; ++i) txfm_ctx[i] = txs;
}

static inline void set_txfm_ctxs(TX_SIZE tx_size, int n4_w, int n4_h, int skip,
                                 const MACROBLOCKD *xd) {
  uint8_t bw = tx_size_wide[tx_size];
  uint8_t bh = tx_size_high[tx_size];

  if (skip) {
    bw = n4_w * MI_SIZE;
    bh = n4_h * MI_SIZE;
  }

  set_txfm_ctx(xd->above_txfm_context, bw, n4_w);
  set_txfm_ctx(xd->left_txfm_context, bh, n4_h);
}

static inline int get_mi_grid_idx(const CommonModeInfoParams *const mi_params,
                                  int mi_row, int mi_col) {
  return mi_row * mi_params->mi_stride + mi_col;
}

static inline int get_alloc_mi_idx(const CommonModeInfoParams *const mi_params,
                                   int mi_row, int mi_col) {
  const int mi_alloc_size_1d = mi_size_wide[mi_params->mi_alloc_bsize];
  const int mi_alloc_row = mi_row / mi_alloc_size_1d;
  const int mi_alloc_col = mi_col / mi_alloc_size_1d;

  return mi_alloc_row * mi_params->mi_alloc_stride + mi_alloc_col;
}

// For this partition block, set pointers in mi_params->mi_grid_base and xd->mi.
static inline void set_mi_offsets(const CommonModeInfoParams *const mi_params,
                                  MACROBLOCKD *const xd, int mi_row,
                                  int mi_col) {
  // 'mi_grid_base' should point to appropriate memory in 'mi'.
  const int mi_grid_idx = get_mi_grid_idx(mi_params, mi_row, mi_col);
  const int mi_alloc_idx = get_alloc_mi_idx(mi_params, mi_row, mi_col);
  mi_params->mi_grid_base[mi_grid_idx] = &mi_params->mi_alloc[mi_alloc_idx];
  // 'xd->mi' should point to an offset in 'mi_grid_base';
  xd->mi = mi_params->mi_grid_base + mi_grid_idx;
  // 'xd->tx_type_map' should point to an offset in 'mi_params->tx_type_map'.
  xd->tx_type_map = mi_params->tx_type_map + mi_grid_idx;
  xd->tx_type_map_stride = mi_params->mi_stride;
}

static inline void txfm_partition_update(TXFM_CONTEXT *above_ctx,
                                         TXFM_CONTEXT *left_ctx,
                                         TX_SIZE tx_size, TX_SIZE txb_size) {
  BLOCK_SIZE bsize = txsize_to_bsize[txb_size];
  int bh = mi_size_high[bsize];
  int bw = mi_size_wide[bsize];
  uint8_t txw = tx_size_wide[tx_size];
  uint8_t txh = tx_size_high[tx_size];
  int i;
  for (i = 0; i < bh; ++i) left_ctx[i] = txh;
  for (i = 0; i < bw; ++i) above_ctx[i] = txw;
}

static inline TX_SIZE get_sqr_tx_size(int tx_dim) {
  switch (tx_dim) {
    case 128:
    case 64: return TX_64X64; break;
    case 32: return TX_32X32; break;
    case 16: return TX_16X16; break;
    case 8: return TX_8X8; break;
    default: return TX_4X4;
  }
}

static inline TX_SIZE get_tx_size(int width, int height) {
  if (width == height) {
    return get_sqr_tx_size(width);
  }
  if (width < height) {
    if (width + width == height) {
      switch (width) {
        case 4: return TX_4X8; break;
        case 8: return TX_8X16; break;
        case 16: return TX_16X32; break;
        case 32: return TX_32X64; break;
      }
    } else {
      switch (width) {
        case 4: return TX_4X16; break;
        case 8: return TX_8X32; break;
        case 16: return TX_16X64; break;
      }
    }
  } else {
    if (height + height == width) {
      switch (height) {
        case 4: return TX_8X4; break;
        case 8: return TX_16X8; break;
        case 16: return TX_32X16; break;
        case 32: return TX_64X32; break;
      }
    } else {
      switch (height) {
        case 4: return TX_16X4; break;
        case 8: return TX_32X8; break;
        case 16: return TX_64X16; break;
      }
    }
  }
  assert(0);
  return TX_4X4;
}

static inline int txfm_partition_context(const TXFM_CONTEXT *const above_ctx,
                                         const TXFM_CONTEXT *const left_ctx,
                                         BLOCK_SIZE bsize, TX_SIZE tx_size) {
  const uint8_t txw = tx_size_wide[tx_size];
  const uint8_t txh = tx_size_high[tx_size];
  const int above = *above_ctx < txw;
  const int left = *left_ctx < txh;
  int category = TXFM_PARTITION_CONTEXTS;

  // dummy return, not used by others.
  if (tx_size <= TX_4X4) return 0;

  TX_SIZE max_tx_size =
      get_sqr_tx_size(AOMMAX(block_size_wide[bsize], block_size_high[bsize]));

  if (max_tx_size >= TX_8X8) {
    category =
        (txsize_sqr_up_map[tx_size] != max_tx_size && max_tx_size > TX_8X8) +
        (TX_SIZES - 1 - max_tx_size) * 2;
  }
  assert(category != TXFM_PARTITION_CONTEXTS);
  return category * 3 + above + left;
}

// Compute the next partition in the direction of the sb_type stored in the mi
// array, starting with bsize.
static inline PARTITION_TYPE get_partition(const AV1_COMMON *const cm,
                                           int mi_row, int mi_col,
                                           BLOCK_SIZE bsize) {
  const CommonModeInfoParams *const mi_params = &cm->mi_params;
  if (mi_row >= mi_params->mi_rows || mi_col >= mi_params->mi_cols)
    return PARTITION_INVALID;

  const int offset = mi_row * mi_params->mi_stride + mi_col;
  MB_MODE_INFO **mi = mi_params->mi_grid_base + offset;
  const BLOCK_SIZE subsize = mi[0]->bsize;

  assert(bsize < BLOCK_SIZES_ALL);

  if (subsize == bsize) return PARTITION_NONE;

  const int bhigh = mi_size_high[bsize];
  const int bwide = mi_size_wide[bsize];
  const int sshigh = mi_size_high[subsize];
  const int sswide = mi_size_wide[subsize];

  if (bsize > BLOCK_8X8 && mi_row + bwide / 2 < mi_params->mi_rows &&
      mi_col + bhigh / 2 < mi_params->mi_cols) {
    // In this case, the block might be using an extended partition
    // type.
    const MB_MODE_INFO *const mbmi_right = mi[bwide / 2];
    const MB_MODE_INFO *const mbmi_below = mi[bhigh / 2 * mi_params->mi_stride];

    if (sswide == bwide) {
      // Smaller height but same width. Is PARTITION_HORZ_4, PARTITION_HORZ or
      // PARTITION_HORZ_B. To distinguish the latter two, check if the lower
      // half was split.
      if (sshigh * 4 == bhigh) return PARTITION_HORZ_4;
      assert(sshigh * 2 == bhigh);

      if (mbmi_below->bsize == subsize)
        return PARTITION_HORZ;
      else
        return PARTITION_HORZ_B;
    } else if (sshigh == bhigh) {
      // Smaller width but same height. Is PARTITION_VERT_4, PARTITION_VERT or
      // PARTITION_VERT_B. To distinguish the latter two, check if the right
      // half was split.
      if (sswide * 4 == bwide) return PARTITION_VERT_4;
      assert(sswide * 2 == bwide);

      if (mbmi_right->bsize == subsize)
        return PARTITION_VERT;
      else
        return PARTITION_VERT_B;
    } else {
      // Smaller width and smaller height. Might be PARTITION_SPLIT or could be
      // PARTITION_HORZ_A or PARTITION_VERT_A. If subsize isn't halved in both
      // dimensions, we immediately know this is a split (which will recurse to
      // get to subsize). Otherwise look down and to the right. With
      // PARTITION_VERT_A, the right block will have height bhigh; with
      // PARTITION_HORZ_A, the lower block with have width bwide. Otherwise
      // it's PARTITION_SPLIT.
      if (sswide * 2 != bwide || sshigh * 2 != bhigh) return PARTITION_SPLIT;

      if (mi_size_wide[mbmi_below->bsize] == bwide) return PARTITION_HORZ_A;
      if (mi_size_high[mbmi_right->bsize] == bhigh) return PARTITION_VERT_A;

      return PARTITION_SPLIT;
    }
  }
  const int vert_split = sswide < bwide;
  const int horz_split = sshigh < bhigh;
  const int split_idx = (vert_split << 1) | horz_split;
  assert(split_idx != 0);

  static const PARTITION_TYPE base_partitions[4] = {
    PARTITION_INVALID, PARTITION_HORZ, PARTITION_VERT, PARTITION_SPLIT
  };

  return base_partitions[split_idx];
}

static inline void set_sb_size(SequenceHeader *const seq_params,
                               BLOCK_SIZE sb_size) {
  seq_params->sb_size = sb_size;
  seq_params->mib_size = mi_size_wide[seq_params->sb_size];
  seq_params->mib_size_log2 = mi_size_wide_log2[seq_params->sb_size];
}

// Returns true if the frame is fully lossless at the coded resolution.
// Note: If super-resolution is used, such a frame will still NOT be lossless at
// the upscaled resolution.
static inline int is_coded_lossless(const AV1_COMMON *cm,
                                    const MACROBLOCKD *xd) {
  int coded_lossless = 1;
  if (cm->seg.enabled) {
    for (int i = 0; i < MAX_SEGMENTS; ++i) {
      if (!xd->lossless[i]) {
        coded_lossless = 0;
        break;
      }
    }
  } else {
    coded_lossless = xd->lossless[0];
  }
  return coded_lossless;
}

static inline int is_valid_seq_level_idx(AV1_LEVEL seq_level_idx) {
  return seq_level_idx == SEQ_LEVEL_MAX ||
         (seq_level_idx < SEQ_LEVELS &&
          // The following levels are currently undefined.
          seq_level_idx != SEQ_LEVEL_2_2 && seq_level_idx != SEQ_LEVEL_2_3 &&
          seq_level_idx != SEQ_LEVEL_3_2 && seq_level_idx != SEQ_LEVEL_3_3 &&
          seq_level_idx != SEQ_LEVEL_4_2 && seq_level_idx != SEQ_LEVEL_4_3
#if !CONFIG_CWG_C013
          && seq_level_idx != SEQ_LEVEL_7_0 && seq_level_idx != SEQ_LEVEL_7_1 &&
          seq_level_idx != SEQ_LEVEL_7_2 && seq_level_idx != SEQ_LEVEL_7_3 &&
          seq_level_idx != SEQ_LEVEL_8_0 && seq_level_idx != SEQ_LEVEL_8_1 &&
          seq_level_idx != SEQ_LEVEL_8_2 && seq_level_idx != SEQ_LEVEL_8_3
#endif
         );
}

/*!\endcond */

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_COMMON_AV1_COMMON_INT_H_
