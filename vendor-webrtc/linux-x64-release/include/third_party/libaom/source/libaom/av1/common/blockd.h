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

#ifndef AOM_AV1_COMMON_BLOCKD_H_
#define AOM_AV1_COMMON_BLOCKD_H_

#include "config/aom_config.h"

#include "aom_dsp/aom_dsp_common.h"
#include "aom_ports/mem.h"
#include "aom_scale/yv12config.h"

#include "av1/common/common_data.h"
#include "av1/common/quant_common.h"
#include "av1/common/entropy.h"
#include "av1/common/entropymode.h"
#include "av1/common/mv.h"
#include "av1/common/scale.h"
#include "av1/common/seg_common.h"
#include "av1/common/tile_common.h"

#ifdef __cplusplus
extern "C" {
#endif

#define USE_B_QUANT_NO_TRELLIS 1

#define MAX_MB_PLANE 3

#define MAX_DIFFWTD_MASK_BITS 1

#define INTERINTRA_WEDGE_SIGN 0

#define DEFAULT_INTER_TX_TYPE DCT_DCT

#define MAX_PALETTE_BLOCK_WIDTH 64

#define MAX_PALETTE_BLOCK_HEIGHT 64

/*!\cond */

// DIFFWTD_MASK_TYPES should not surpass 1 << MAX_DIFFWTD_MASK_BITS
enum {
  DIFFWTD_38 = 0,
  DIFFWTD_38_INV,
  DIFFWTD_MASK_TYPES,
} UENUM1BYTE(DIFFWTD_MASK_TYPE);

enum {
  KEY_FRAME = 0,
  INTER_FRAME = 1,
  INTRA_ONLY_FRAME = 2,  // replaces intra-only
  S_FRAME = 3,
  FRAME_TYPES,
} UENUM1BYTE(FRAME_TYPE);

static inline int is_comp_ref_allowed(BLOCK_SIZE bsize) {
  return AOMMIN(block_size_wide[bsize], block_size_high[bsize]) >= 8;
}

static inline int is_inter_mode(PREDICTION_MODE mode) {
  return mode >= INTER_MODE_START && mode < INTER_MODE_END;
}

typedef struct {
  uint8_t *plane[MAX_MB_PLANE];
  int stride[MAX_MB_PLANE];
} BUFFER_SET;

static inline int is_inter_singleref_mode(PREDICTION_MODE mode) {
  return mode >= SINGLE_INTER_MODE_START && mode < SINGLE_INTER_MODE_END;
}
static inline int is_inter_compound_mode(PREDICTION_MODE mode) {
  return mode >= COMP_INTER_MODE_START && mode < COMP_INTER_MODE_END;
}

static inline PREDICTION_MODE compound_ref0_mode(PREDICTION_MODE mode) {
  static const PREDICTION_MODE lut[] = {
    DC_PRED,        // DC_PRED
    V_PRED,         // V_PRED
    H_PRED,         // H_PRED
    D45_PRED,       // D45_PRED
    D135_PRED,      // D135_PRED
    D113_PRED,      // D113_PRED
    D157_PRED,      // D157_PRED
    D203_PRED,      // D203_PRED
    D67_PRED,       // D67_PRED
    SMOOTH_PRED,    // SMOOTH_PRED
    SMOOTH_V_PRED,  // SMOOTH_V_PRED
    SMOOTH_H_PRED,  // SMOOTH_H_PRED
    PAETH_PRED,     // PAETH_PRED
    NEARESTMV,      // NEARESTMV
    NEARMV,         // NEARMV
    GLOBALMV,       // GLOBALMV
    NEWMV,          // NEWMV
    NEARESTMV,      // NEAREST_NEARESTMV
    NEARMV,         // NEAR_NEARMV
    NEARESTMV,      // NEAREST_NEWMV
    NEWMV,          // NEW_NEARESTMV
    NEARMV,         // NEAR_NEWMV
    NEWMV,          // NEW_NEARMV
    GLOBALMV,       // GLOBAL_GLOBALMV
    NEWMV,          // NEW_NEWMV
  };
  assert(NELEMENTS(lut) == MB_MODE_COUNT);
  assert(is_inter_compound_mode(mode) || is_inter_singleref_mode(mode));
  return lut[mode];
}

static inline PREDICTION_MODE compound_ref1_mode(PREDICTION_MODE mode) {
  static const PREDICTION_MODE lut[] = {
    MB_MODE_COUNT,  // DC_PRED
    MB_MODE_COUNT,  // V_PRED
    MB_MODE_COUNT,  // H_PRED
    MB_MODE_COUNT,  // D45_PRED
    MB_MODE_COUNT,  // D135_PRED
    MB_MODE_COUNT,  // D113_PRED
    MB_MODE_COUNT,  // D157_PRED
    MB_MODE_COUNT,  // D203_PRED
    MB_MODE_COUNT,  // D67_PRED
    MB_MODE_COUNT,  // SMOOTH_PRED
    MB_MODE_COUNT,  // SMOOTH_V_PRED
    MB_MODE_COUNT,  // SMOOTH_H_PRED
    MB_MODE_COUNT,  // PAETH_PRED
    MB_MODE_COUNT,  // NEARESTMV
    MB_MODE_COUNT,  // NEARMV
    MB_MODE_COUNT,  // GLOBALMV
    MB_MODE_COUNT,  // NEWMV
    NEARESTMV,      // NEAREST_NEARESTMV
    NEARMV,         // NEAR_NEARMV
    NEWMV,          // NEAREST_NEWMV
    NEARESTMV,      // NEW_NEARESTMV
    NEWMV,          // NEAR_NEWMV
    NEARMV,         // NEW_NEARMV
    GLOBALMV,       // GLOBAL_GLOBALMV
    NEWMV,          // NEW_NEWMV
  };
  assert(NELEMENTS(lut) == MB_MODE_COUNT);
  assert(is_inter_compound_mode(mode));
  return lut[mode];
}

static inline int have_nearmv_in_inter_mode(PREDICTION_MODE mode) {
  return (mode == NEARMV || mode == NEAR_NEARMV || mode == NEAR_NEWMV ||
          mode == NEW_NEARMV);
}

static inline int have_newmv_in_inter_mode(PREDICTION_MODE mode) {
  return (mode == NEWMV || mode == NEW_NEWMV || mode == NEAREST_NEWMV ||
          mode == NEW_NEARESTMV || mode == NEAR_NEWMV || mode == NEW_NEARMV);
}

static inline int is_masked_compound_type(COMPOUND_TYPE type) {
  return (type == COMPOUND_WEDGE || type == COMPOUND_DIFFWTD);
}

/* For keyframes, intra block modes are predicted by the (already decoded)
   modes for the Y blocks to the left and above us; for interframes, there
   is a single probability table. */

typedef struct {
  // Value of base colors for Y, U, and V
  uint16_t palette_colors[3 * PALETTE_MAX_SIZE];
  // Number of base colors for Y (0) and UV (1)
  uint8_t palette_size[2];
} PALETTE_MODE_INFO;

typedef struct {
  FILTER_INTRA_MODE filter_intra_mode;
  uint8_t use_filter_intra;
} FILTER_INTRA_MODE_INFO;

static const PREDICTION_MODE fimode_to_intradir[FILTER_INTRA_MODES] = {
  DC_PRED, V_PRED, H_PRED, D157_PRED, DC_PRED
};

#if CONFIG_RD_DEBUG
#define TXB_COEFF_COST_MAP_SIZE (MAX_MIB_SIZE)
#endif

typedef struct RD_STATS {
  int rate;
  int zero_rate;
  int64_t dist;
  // Please be careful of using rdcost, it's not guaranteed to be set all the
  // time.
  // TODO(angiebird): Create a set of functions to manipulate the RD_STATS. In
  // these functions, make sure rdcost is always up-to-date according to
  // rate/dist.
  int64_t rdcost;
  int64_t sse;
  uint8_t skip_txfm;  // sse should equal to dist when skip_txfm == 1
#if CONFIG_RD_DEBUG
  int txb_coeff_cost[MAX_MB_PLANE];
#endif  // CONFIG_RD_DEBUG
} RD_STATS;

// This struct is used to group function args that are commonly
// sent together in functions related to interinter compound modes
typedef struct {
  uint8_t *seg_mask;
  int8_t wedge_index;
  int8_t wedge_sign;
  DIFFWTD_MASK_TYPE mask_type;
  COMPOUND_TYPE type;
} INTERINTER_COMPOUND_DATA;

#define INTER_TX_SIZE_BUF_LEN 16
#define TXK_TYPE_BUF_LEN 64
/*!\endcond */

/*! \brief Stores the prediction/txfm mode of the current coding block
 */
typedef struct MB_MODE_INFO {
  /*****************************************************************************
   * \name General Info of the Coding Block
   ****************************************************************************/
  /**@{*/
  /*! \brief The block size of the current coding block */
  BLOCK_SIZE bsize;
  /*! \brief The partition type of the current coding block. */
  PARTITION_TYPE partition;
  /*! \brief The prediction mode used */
  PREDICTION_MODE mode;
  /*! \brief The UV mode when intra is used */
  UV_PREDICTION_MODE uv_mode;
  /*! \brief The q index for the current coding block. */
  int current_qindex;
  /**@}*/

  /*****************************************************************************
   * \name Inter Mode Info
   ****************************************************************************/
  /**@{*/
  /*! \brief The motion vectors used by the current inter mode */
  int_mv mv[2];
  /*! \brief The reference frames for the MV */
  MV_REFERENCE_FRAME ref_frame[2];
  /*! \brief Filter used in subpel interpolation. */
  int_interpfilters interp_filters;
  /*! \brief The motion mode used by the inter prediction. */
  MOTION_MODE motion_mode;
  /*! \brief Number of samples used by warp causal */
  uint8_t num_proj_ref;
  /*! \brief The number of overlapped neighbors above/left for obmc/warp motion
   * mode. */
  uint8_t overlappable_neighbors;
  /*! \brief The parameters used in warp motion mode. */
  WarpedMotionParams wm_params;
  /*! \brief The type of intra mode used by inter-intra */
  INTERINTRA_MODE interintra_mode;
  /*! \brief The type of wedge used in interintra mode. */
  int8_t interintra_wedge_index;
  /*! \brief Struct that stores the data used in interinter compound mode. */
  INTERINTER_COMPOUND_DATA interinter_comp;
  /**@}*/

  /*****************************************************************************
   * \name Intra Mode Info
   ****************************************************************************/
  /**@{*/
  /*! \brief Directional mode delta: the angle is base angle + (angle_delta *
   * step). */
  int8_t angle_delta[PLANE_TYPES];
  /*! \brief The type of filter intra mode used (if applicable). */
  FILTER_INTRA_MODE_INFO filter_intra_mode_info;
  /*! \brief Chroma from Luma: Joint sign of alpha Cb and alpha Cr */
  int8_t cfl_alpha_signs;
  /*! \brief Chroma from Luma: Index of the alpha Cb and alpha Cr combination */
  uint8_t cfl_alpha_idx;
  /*! \brief Stores the size and colors of palette mode */
  PALETTE_MODE_INFO palette_mode_info;
  /**@}*/

  /*****************************************************************************
   * \name Transform Info
   ****************************************************************************/
  /**@{*/
  /*! \brief Whether to skip transforming and sending. */
  uint8_t skip_txfm;
  /*! \brief Transform size when fixed size txfm is used (e.g. intra modes). */
  TX_SIZE tx_size;
  /*! \brief Transform size when recursive txfm tree is on. */
  TX_SIZE inter_tx_size[INTER_TX_SIZE_BUF_LEN];
  /**@}*/

  /*****************************************************************************
   * \name Loop Filter Info
   ****************************************************************************/
  /**@{*/
  /*! \copydoc MACROBLOCKD::delta_lf_from_base */
  int8_t delta_lf_from_base;
  /*! \copydoc MACROBLOCKD::delta_lf */
  int8_t delta_lf[FRAME_LF_COUNT];
  /**@}*/

  /*****************************************************************************
   * \name Bitfield for Memory Reduction
   ****************************************************************************/
  /**@{*/
  /*! \brief The segment id */
  uint8_t segment_id : 3;
  /*! \brief Only valid when temporal update if off. */
  uint8_t seg_id_predicted : 1;
  /*! \brief Which ref_mv to use */
  uint8_t ref_mv_idx : 2;
  /*! \brief Inter skip mode */
  uint8_t skip_mode : 1;
  /*! \brief Whether intrabc is used. */
  uint8_t use_intrabc : 1;
  /*! \brief Indicates if masked compound is used(1) or not (0). */
  uint8_t comp_group_idx : 1;
  /*! \brief Indicates whether dist_wtd_comp(0) is used or not (0). */
  uint8_t compound_idx : 1;
  /*! \brief Whether to use interintra wedge */
  uint8_t use_wedge_interintra : 1;
  /*! \brief CDEF strength per BLOCK_64X64 */
  int8_t cdef_strength : 4;
  /**@}*/

#if CONFIG_RD_DEBUG
  /*! \brief RD info used for debugging */
  RD_STATS rd_stats;
  /*! \brief The current row in unit of 4x4 blocks for debugging */
  int mi_row;
  /*! \brief The current col in unit of 4x4 blocks for debugging */
  int mi_col;
#endif
#if CONFIG_INSPECTION
  /*! \brief Whether we are skipping the current rows or columns. */
  int16_t tx_skip[TXK_TYPE_BUF_LEN];
#endif
} MB_MODE_INFO;

/*!\cond */

static inline int is_intrabc_block(const MB_MODE_INFO *mbmi) {
  return mbmi->use_intrabc;
}

static inline PREDICTION_MODE get_uv_mode(UV_PREDICTION_MODE mode) {
  assert(mode < UV_INTRA_MODES);
  static const PREDICTION_MODE uv2y[] = {
    DC_PRED,        // UV_DC_PRED
    V_PRED,         // UV_V_PRED
    H_PRED,         // UV_H_PRED
    D45_PRED,       // UV_D45_PRED
    D135_PRED,      // UV_D135_PRED
    D113_PRED,      // UV_D113_PRED
    D157_PRED,      // UV_D157_PRED
    D203_PRED,      // UV_D203_PRED
    D67_PRED,       // UV_D67_PRED
    SMOOTH_PRED,    // UV_SMOOTH_PRED
    SMOOTH_V_PRED,  // UV_SMOOTH_V_PRED
    SMOOTH_H_PRED,  // UV_SMOOTH_H_PRED
    PAETH_PRED,     // UV_PAETH_PRED
    DC_PRED,        // UV_CFL_PRED
    INTRA_INVALID,  // UV_INTRA_MODES
    INTRA_INVALID,  // UV_MODE_INVALID
  };
  return uv2y[mode];
}

static inline int is_inter_block(const MB_MODE_INFO *mbmi) {
  return is_intrabc_block(mbmi) || mbmi->ref_frame[0] > INTRA_FRAME;
}

static inline int has_second_ref(const MB_MODE_INFO *mbmi) {
  return mbmi->ref_frame[1] > INTRA_FRAME;
}

static inline int has_uni_comp_refs(const MB_MODE_INFO *mbmi) {
  return has_second_ref(mbmi) && (!((mbmi->ref_frame[0] >= BWDREF_FRAME) ^
                                    (mbmi->ref_frame[1] >= BWDREF_FRAME)));
}

static inline MV_REFERENCE_FRAME comp_ref0(int ref_idx) {
  static const MV_REFERENCE_FRAME lut[] = {
    LAST_FRAME,     // LAST_LAST2_FRAMES,
    LAST_FRAME,     // LAST_LAST3_FRAMES,
    LAST_FRAME,     // LAST_GOLDEN_FRAMES,
    BWDREF_FRAME,   // BWDREF_ALTREF_FRAMES,
    LAST2_FRAME,    // LAST2_LAST3_FRAMES
    LAST2_FRAME,    // LAST2_GOLDEN_FRAMES,
    LAST3_FRAME,    // LAST3_GOLDEN_FRAMES,
    BWDREF_FRAME,   // BWDREF_ALTREF2_FRAMES,
    ALTREF2_FRAME,  // ALTREF2_ALTREF_FRAMES,
  };
  assert(NELEMENTS(lut) == TOTAL_UNIDIR_COMP_REFS);
  return lut[ref_idx];
}

static inline MV_REFERENCE_FRAME comp_ref1(int ref_idx) {
  static const MV_REFERENCE_FRAME lut[] = {
    LAST2_FRAME,    // LAST_LAST2_FRAMES,
    LAST3_FRAME,    // LAST_LAST3_FRAMES,
    GOLDEN_FRAME,   // LAST_GOLDEN_FRAMES,
    ALTREF_FRAME,   // BWDREF_ALTREF_FRAMES,
    LAST3_FRAME,    // LAST2_LAST3_FRAMES
    GOLDEN_FRAME,   // LAST2_GOLDEN_FRAMES,
    GOLDEN_FRAME,   // LAST3_GOLDEN_FRAMES,
    ALTREF2_FRAME,  // BWDREF_ALTREF2_FRAMES,
    ALTREF_FRAME,   // ALTREF2_ALTREF_FRAMES,
  };
  assert(NELEMENTS(lut) == TOTAL_UNIDIR_COMP_REFS);
  return lut[ref_idx];
}

PREDICTION_MODE av1_left_block_mode(const MB_MODE_INFO *left_mi);

PREDICTION_MODE av1_above_block_mode(const MB_MODE_INFO *above_mi);

static inline int is_global_mv_block(const MB_MODE_INFO *const mbmi,
                                     TransformationType type) {
  const PREDICTION_MODE mode = mbmi->mode;
  const BLOCK_SIZE bsize = mbmi->bsize;
  const int block_size_allowed =
      AOMMIN(block_size_wide[bsize], block_size_high[bsize]) >= 8;
  return (mode == GLOBALMV || mode == GLOBAL_GLOBALMV) && type > TRANSLATION &&
         block_size_allowed;
}

#if CONFIG_MISMATCH_DEBUG
static inline void mi_to_pixel_loc(int *pixel_c, int *pixel_r, int mi_col,
                                   int mi_row, int tx_blk_col, int tx_blk_row,
                                   int subsampling_x, int subsampling_y) {
  *pixel_c = ((mi_col >> subsampling_x) << MI_SIZE_LOG2) +
             (tx_blk_col << MI_SIZE_LOG2);
  *pixel_r = ((mi_row >> subsampling_y) << MI_SIZE_LOG2) +
             (tx_blk_row << MI_SIZE_LOG2);
}
#endif

enum { MV_PRECISION_Q3, MV_PRECISION_Q4 } UENUM1BYTE(mv_precision);

struct buf_2d {
  uint8_t *buf;
  uint8_t *buf0;
  int width;
  int height;
  int stride;
};

typedef struct eob_info {
  uint16_t eob;
  uint16_t max_scan_line;
} eob_info;

typedef struct {
  DECLARE_ALIGNED(32, tran_low_t, dqcoeff[MAX_MB_PLANE][MAX_SB_SQUARE]);
  eob_info eob_data[MAX_MB_PLANE]
                   [MAX_SB_SQUARE / (TX_SIZE_W_MIN * TX_SIZE_H_MIN)];
  DECLARE_ALIGNED(16, uint8_t, color_index_map[2][MAX_SB_SQUARE]);
} CB_BUFFER;

typedef struct macroblockd_plane {
  PLANE_TYPE plane_type;
  int subsampling_x;
  int subsampling_y;
  struct buf_2d dst;
  struct buf_2d pre[2];
  ENTROPY_CONTEXT *above_entropy_context;
  ENTROPY_CONTEXT *left_entropy_context;

  // The dequantizers below are true dequantizers used only in the
  // dequantization process.  They have the same coefficient
  // shift/scale as TX.
  int16_t seg_dequant_QTX[MAX_SEGMENTS][2];
  // Pointer to color index map of:
  // - Current coding block, on encoder side.
  // - Current superblock, on decoder side.
  uint8_t *color_index_map;

  // block size in pixels
  uint8_t width, height;

  qm_val_t *seg_iqmatrix[MAX_SEGMENTS][TX_SIZES_ALL];
  qm_val_t *seg_qmatrix[MAX_SEGMENTS][TX_SIZES_ALL];
} MACROBLOCKD_PLANE;

#define BLOCK_OFFSET(i) ((i) << 4)

/*!\endcond */

/*!\brief Parameters related to Wiener Filter */
typedef struct {
  /*!
   * Vertical filter kernel.
   */
  DECLARE_ALIGNED(16, InterpKernel, vfilter);

  /*!
   * Horizontal filter kernel.
   */
  DECLARE_ALIGNED(16, InterpKernel, hfilter);
} WienerInfo;

/*!\brief Parameters related to Sgrproj Filter */
typedef struct {
  /*!
   * Parameter index.
   */
  int ep;

  /*!
   * Weights for linear combination of filtered versions
   */
  int xqd[2];
} SgrprojInfo;

/*!\cond */

#define CFL_MAX_BLOCK_SIZE (BLOCK_32X32)
#define CFL_BUF_LINE (32)
#define CFL_BUF_LINE_I128 (CFL_BUF_LINE >> 3)
#define CFL_BUF_LINE_I256 (CFL_BUF_LINE >> 4)
#define CFL_BUF_SQUARE (CFL_BUF_LINE * CFL_BUF_LINE)
typedef struct cfl_ctx {
  // Q3 reconstructed luma pixels (only Q2 is required, but Q3 is used to avoid
  // shifts)
  uint16_t recon_buf_q3[CFL_BUF_SQUARE];
  // Q3 AC contributions (reconstructed luma pixels - tx block avg)
  int16_t ac_buf_q3[CFL_BUF_SQUARE];

  // Cache the DC_PRED when performing RDO, so it does not have to be recomputed
  // for every scaling parameter
  bool dc_pred_is_cached[CFL_PRED_PLANES];
  // Whether the DC_PRED cache is enabled. The DC_PRED cache is disabled when
  // decoding.
  bool use_dc_pred_cache;
  // Only cache the first row of the DC_PRED
  int16_t dc_pred_cache[CFL_PRED_PLANES][CFL_BUF_LINE];

  // Height and width currently used in the CfL prediction buffer.
  int buf_height, buf_width;

  int are_parameters_computed;

  // Chroma subsampling
  int subsampling_x, subsampling_y;

  // Whether the reconstructed luma pixels need to be stored
  int store_y;
} CFL_CTX;

typedef struct dist_wtd_comp_params {
  int use_dist_wtd_comp_avg;
  int fwd_offset;
  int bck_offset;
} DIST_WTD_COMP_PARAMS;

struct scale_factors;

/*!\endcond */

/*! \brief Variables related to current coding block.
 *
 * This is a common set of variables used by both encoder and decoder.
 * Most/all of the pointers are mere pointers to actual arrays are allocated
 * elsewhere. This is mostly for coding convenience.
 */
typedef struct macroblockd {
  /**
   * \name Position of current macroblock in mi units
   */
  /**@{*/
  int mi_row; /*!< Row position in mi units. */
  int mi_col; /*!< Column position in mi units. */
  /**@}*/

  /*!
   * Same as cm->mi_params.mi_stride, copied here for convenience.
   */
  int mi_stride;

  /*!
   * True if current block transmits chroma information.
   * More detail:
   * Smallest supported block size for both luma and chroma plane is 4x4. Hence,
   * in case of subsampled chroma plane (YUV 4:2:0 or YUV 4:2:2), multiple luma
   * blocks smaller than 8x8 maybe combined into one chroma block.
   * For example, for YUV 4:2:0, let's say an 8x8 area is split into four 4x4
   * luma blocks. Then, a single chroma block of size 4x4 will cover the area of
   * these four luma blocks. This is implemented in bitstream as follows:
   * - There are four MB_MODE_INFO structs for the four luma blocks.
   * - First 3 MB_MODE_INFO have is_chroma_ref = false, and so do not transmit
   * any information for chroma planes.
   * - Last block will have is_chroma_ref = true and transmits chroma
   * information for the 4x4 chroma block that covers whole 8x8 area covered by
   * four luma blocks.
   * Similar logic applies for chroma blocks that cover 2 or 3 luma blocks.
   */
  bool is_chroma_ref;

  /*!
   * Info specific to each plane.
   */
  struct macroblockd_plane plane[MAX_MB_PLANE];

  /*!
   * Tile related info.
   */
  TileInfo tile;

  /*!
   * Appropriate offset inside cm->mi_params.mi_grid_base based on current
   * mi_row and mi_col.
   */
  MB_MODE_INFO **mi;

  /*!
   * True if 4x4 block above the current block is available.
   */
  bool up_available;
  /*!
   * True if 4x4 block to the left of the current block is available.
   */
  bool left_available;
  /*!
   * True if the above chrome reference block is available.
   */
  bool chroma_up_available;
  /*!
   * True if the left chrome reference block is available.
   */
  bool chroma_left_available;

  /*!
   * MB_MODE_INFO for 4x4 block to the left of the current block, if
   * left_available == true; otherwise NULL.
   */
  MB_MODE_INFO *left_mbmi;
  /*!
   * MB_MODE_INFO for 4x4 block above the current block, if
   * up_available == true; otherwise NULL.
   */
  MB_MODE_INFO *above_mbmi;
  /*!
   * Above chroma reference block if is_chroma_ref == true for the current block
   * and chroma_up_available == true; otherwise NULL.
   * See also: the special case logic when current chroma block covers more than
   * one luma blocks in set_mi_row_col().
   */
  MB_MODE_INFO *chroma_left_mbmi;
  /*!
   * Left chroma reference block if is_chroma_ref == true for the current block
   * and chroma_left_available == true; otherwise NULL.
   * See also: the special case logic when current chroma block covers more than
   * one luma blocks in set_mi_row_col().
   */
  MB_MODE_INFO *chroma_above_mbmi;

  /*!
   * Appropriate offset based on current 'mi_row' and 'mi_col', inside
   * 'tx_type_map' in one of 'CommonModeInfoParams', 'PICK_MODE_CONTEXT' or
   * 'MACROBLOCK' structs.
   */
  uint8_t *tx_type_map;
  /*!
   * Stride for 'tx_type_map'. Note that this may / may not be same as
   * 'mi_stride', depending on which actual array 'tx_type_map' points to.
   */
  int tx_type_map_stride;

  /**
   * \name Distance of this macroblock from frame edges in 1/8th pixel units.
   */
  /**@{*/
  int mb_to_left_edge;   /*!< Distance from left edge */
  int mb_to_right_edge;  /*!< Distance from right edge */
  int mb_to_top_edge;    /*!< Distance from top edge */
  int mb_to_bottom_edge; /*!< Distance from bottom edge */
  /**@}*/

  /*!
   * Scale factors for reference frames of the current block.
   * These are pointers into 'cm->ref_scale_factors'.
   */
  const struct scale_factors *block_ref_scale_factors[2];

  /*!
   * - On encoder side: points to cpi->source, which is the buffer containing
   * the current *source* frame (maybe filtered).
   * - On decoder side: points to cm->cur_frame->buf, which is the buffer into
   * which current frame is being *decoded*.
   */
  const YV12_BUFFER_CONFIG *cur_buf;

  /*!
   * Entropy contexts for the above blocks.
   * above_entropy_context[i][j] corresponds to above entropy context for ith
   * plane and jth mi column of this *frame*, wrt current 'mi_row'.
   * These are pointers into 'cm->above_contexts.entropy'.
   */
  ENTROPY_CONTEXT *above_entropy_context[MAX_MB_PLANE];
  /*!
   * Entropy contexts for the left blocks.
   * left_entropy_context[i][j] corresponds to left entropy context for ith
   * plane and jth mi row of this *superblock*, wrt current 'mi_col'.
   * Note: These contain actual data, NOT pointers.
   */
  ENTROPY_CONTEXT left_entropy_context[MAX_MB_PLANE][MAX_MIB_SIZE];

  /*!
   * Partition contexts for the above blocks.
   * above_partition_context[i] corresponds to above partition context for ith
   * mi column of this *frame*, wrt current 'mi_row'.
   * This is a pointer into 'cm->above_contexts.partition'.
   */
  PARTITION_CONTEXT *above_partition_context;
  /*!
   * Partition contexts for the left blocks.
   * left_partition_context[i] corresponds to left partition context for ith
   * mi row of this *superblock*, wrt current 'mi_col'.
   * Note: These contain actual data, NOT pointers.
   */
  PARTITION_CONTEXT left_partition_context[MAX_MIB_SIZE];

  /*!
   * Transform contexts for the above blocks.
   * above_txfm_context[i] corresponds to above transform context for ith mi col
   * from the current position (mi row and mi column) for this *frame*.
   * This is a pointer into 'cm->above_contexts.txfm'.
   */
  TXFM_CONTEXT *above_txfm_context;
  /*!
   * Transform contexts for the left blocks.
   * left_txfm_context[i] corresponds to left transform context for ith mi row
   * from the current position (mi_row and mi_col) for this *superblock*.
   * This is a pointer into 'left_txfm_context_buffer'.
   */
  TXFM_CONTEXT *left_txfm_context;
  /*!
   * left_txfm_context_buffer[i] is the left transform context for ith mi_row
   * in this *superblock*.
   * Behaves like an internal actual buffer which 'left_txt_context' points to,
   * and never accessed directly except to fill in initial default values.
   */
  TXFM_CONTEXT left_txfm_context_buffer[MAX_MIB_SIZE];

  /**
   * \name Default values for the two restoration filters for each plane.
   * Default values for the two restoration filters for each plane.
   * These values are used as reference values when writing the bitstream. That
   * is, we transmit the delta between the actual values in
   * cm->rst_info[plane].unit_info[unit_idx] and these reference values.
   */
  /**@{*/
  WienerInfo wiener_info[MAX_MB_PLANE];   /*!< Defaults for Wiener filter*/
  SgrprojInfo sgrproj_info[MAX_MB_PLANE]; /*!< Defaults for SGR filter */
  /**@}*/

  /**
   * \name Block dimensions in MB_MODE_INFO units.
   */
  /**@{*/
  uint8_t width;  /*!< Block width in MB_MODE_INFO units */
  uint8_t height; /*!< Block height in MB_MODE_INFO units */
  /**@}*/

  /*!
   * Contains the motion vector candidates found during motion vector prediction
   * process. ref_mv_stack[i] contains the candidates for ith type of
   * reference frame (single/compound). The actual number of candidates found in
   * ref_mv_stack[i] is stored in either dcb->ref_mv_count[i] (decoder side)
   * or mbmi_ext->ref_mv_count[i] (encoder side).
   */
  CANDIDATE_MV ref_mv_stack[MODE_CTX_REF_FRAMES][MAX_REF_MV_STACK_SIZE];
  /*!
   * weight[i][j] is the weight for ref_mv_stack[i][j] and used to compute the
   * DRL (dynamic reference list) mode contexts.
   */
  uint16_t weight[MODE_CTX_REF_FRAMES][MAX_REF_MV_STACK_SIZE];

  /*!
   * True if this is the last vertical rectangular block in a VERTICAL or
   * VERTICAL_4 partition.
   */
  bool is_last_vertical_rect;
  /*!
   * True if this is the 1st horizontal rectangular block in a HORIZONTAL or
   * HORIZONTAL_4 partition.
   */
  bool is_first_horizontal_rect;

  /*!
   * Counts of each reference frame in the above and left neighboring blocks.
   * NOTE: Take into account both single and comp references.
   */
  uint8_t neighbors_ref_counts[REF_FRAMES];

  /*!
   * Current CDFs of all the symbols for the current tile.
   */
  FRAME_CONTEXT *tile_ctx;

  /*!
   * Bit depth: copied from cm->seq_params->bit_depth for convenience.
   */
  int bd;

  /*!
   * Quantizer index for each segment (base qindex + delta for each segment).
   */
  int qindex[MAX_SEGMENTS];
  /*!
   * lossless[s] is true if segment 's' is coded losslessly.
   */
  int lossless[MAX_SEGMENTS];
  /*!
   * Q index for the coding blocks in this superblock will be stored in
   * mbmi->current_qindex. Now, when cm->delta_q_info.delta_q_present_flag is
   * true, mbmi->current_qindex is computed by taking 'current_base_qindex' as
   * the base, and adding any transmitted delta qindex on top of it.
   * Precisely, this is the latest qindex used by the first coding block of a
   * non-skip superblock in the current tile; OR
   * same as cm->quant_params.base_qindex (if not explicitly set yet).
   * Note: This is 'CurrentQIndex' in the AV1 spec.
   */
  int current_base_qindex;

  /*!
   * Same as cm->features.cur_frame_force_integer_mv.
   */
  int cur_frame_force_integer_mv;

  /*!
   * Pointer to cm->error.
   */
  struct aom_internal_error_info *error_info;

  /*!
   * Same as cm->global_motion.
   */
  const WarpedMotionParams *global_motion;

  /*!
   * Since actual frame level loop filtering level value is not available
   * at the beginning of the tile (only available during actual filtering)
   * at encoder side.we record the delta_lf (against the frame level loop
   * filtering level) and code the delta between previous superblock's delta
   * lf and current delta lf. It is equivalent to the delta between previous
   * superblock's actual lf and current lf.
   */
  int8_t delta_lf_from_base;
  /*!
   * We have four frame filter levels for different plane and direction. So, to
   * support the per superblock update, we need to add a few more params:
   * 0. delta loop filter level for y plane vertical
   * 1. delta loop filter level for y plane horizontal
   * 2. delta loop filter level for u plane
   * 3. delta loop filter level for v plane
   * To make it consistent with the reference to each filter level in segment,
   * we need to -1, since
   * - SEG_LVL_ALT_LF_Y_V = 1;
   * - SEG_LVL_ALT_LF_Y_H = 2;
   * - SEG_LVL_ALT_LF_U   = 3;
   * - SEG_LVL_ALT_LF_V   = 4;
   */
  int8_t delta_lf[FRAME_LF_COUNT];
  /*!
   * cdef_transmitted[i] is true if CDEF strength for ith CDEF unit in the
   * current superblock has already been read from (decoder) / written to
   * (encoder) the bitstream; and false otherwise.
   * More detail:
   * 1. CDEF strength is transmitted only once per CDEF unit, in the 1st
   * non-skip coding block. So, we need this array to keep track of whether CDEF
   * strengths for the given CDEF units have been transmitted yet or not.
   * 2. Superblock size can be either 128x128 or 64x64, but CDEF unit size is
   * fixed to be 64x64. So, there may be 4 CDEF units within a superblock (if
   * superblock size is 128x128). Hence the array size is 4.
   * 3. In the current implementation, CDEF strength for this CDEF unit is
   * stored in the MB_MODE_INFO of the 1st block in this CDEF unit (inside
   * cm->mi_params.mi_grid_base).
   */
  bool cdef_transmitted[4];

  /*!
   * Mask for this block used for compound prediction.
   */
  uint8_t *seg_mask;

  /*!
   * CFL (chroma from luma) related parameters.
   */
  CFL_CTX cfl;

  /*!
   * Offset to plane[p].color_index_map.
   * Currently:
   * - On encoder side, this is always 0 as 'color_index_map' is allocated per
   * *coding block* there.
   * - On decoder side, this may be non-zero, as 'color_index_map' is a (static)
   * memory pointing to the base of a *superblock* there, and we need an offset
   * to it to get the color index map for current coding block.
   */
  uint16_t color_index_map_offset[2];

  /*!
   * Temporary buffer used for convolution in case of compound reference only
   * for (weighted or uniform) averaging operation.
   * There are pointers to actual buffers allocated elsewhere: e.g.
   * - In decoder, 'pbi->td.tmp_conv_dst' or
   * 'pbi->thread_data[t].td->xd.tmp_conv_dst' and
   * - In encoder, 'x->tmp_conv_dst' or
   * 'cpi->tile_thr_data[t].td->mb.tmp_conv_dst'.
   */
  CONV_BUF_TYPE *tmp_conv_dst;
  /*!
   * Temporary buffers used to build OBMC prediction by above (index 0) and left
   * (index 1) predictors respectively.
   * tmp_obmc_bufs[i][p * MAX_SB_SQUARE] is the buffer used for plane 'p'.
   * There are pointers to actual buffers allocated elsewhere: e.g.
   * - In decoder, 'pbi->td.tmp_obmc_bufs' or
   * 'pbi->thread_data[t].td->xd.tmp_conv_dst' and
   * -In encoder, 'x->tmp_pred_bufs' or
   * 'cpi->tile_thr_data[t].td->mb.tmp_pred_bufs'.
   */
  uint8_t *tmp_obmc_bufs[2];
} MACROBLOCKD;

/*!\cond */

static inline int is_cur_buf_hbd(const MACROBLOCKD *xd) {
#if CONFIG_AV1_HIGHBITDEPTH
  return xd->cur_buf->flags & YV12_FLAG_HIGHBITDEPTH ? 1 : 0;
#else
  (void)xd;
  return 0;
#endif
}

static inline uint8_t *get_buf_by_bd(const MACROBLOCKD *xd, uint8_t *buf16) {
#if CONFIG_AV1_HIGHBITDEPTH
  return (xd->cur_buf->flags & YV12_FLAG_HIGHBITDEPTH)
             ? CONVERT_TO_BYTEPTR(buf16)
             : buf16;
#else
  (void)xd;
  return buf16;
#endif
}

typedef struct BitDepthInfo {
  int bit_depth;
  /*! Is the image buffer high bit depth?
   * Low bit depth buffer uses uint8_t.
   * High bit depth buffer uses uint16_t.
   * Equivalent to cm->seq_params->use_highbitdepth
   */
  int use_highbitdepth_buf;
} BitDepthInfo;

static inline BitDepthInfo get_bit_depth_info(const MACROBLOCKD *xd) {
  BitDepthInfo bit_depth_info;
  bit_depth_info.bit_depth = xd->bd;
  bit_depth_info.use_highbitdepth_buf = is_cur_buf_hbd(xd);
  assert(IMPLIES(!bit_depth_info.use_highbitdepth_buf,
                 bit_depth_info.bit_depth == 8));
  return bit_depth_info;
}

static inline int get_sqr_bsize_idx(BLOCK_SIZE bsize) {
  switch (bsize) {
    case BLOCK_4X4: return 0;
    case BLOCK_8X8: return 1;
    case BLOCK_16X16: return 2;
    case BLOCK_32X32: return 3;
    case BLOCK_64X64: return 4;
    case BLOCK_128X128: return 5;
    default: return SQR_BLOCK_SIZES;
  }
}

// For a square block size 'bsize', returns the size of the sub-blocks used by
// the given partition type. If the partition produces sub-blocks of different
// sizes, then the function returns the largest sub-block size.
// Implements the Partition_Subsize lookup table in the spec (Section 9.3.
// Conversion tables).
// Note: the input block size should be square.
// Otherwise it's considered invalid.
static inline BLOCK_SIZE get_partition_subsize(BLOCK_SIZE bsize,
                                               PARTITION_TYPE partition) {
  if (partition == PARTITION_INVALID) {
    return BLOCK_INVALID;
  } else {
    const int sqr_bsize_idx = get_sqr_bsize_idx(bsize);
    return sqr_bsize_idx >= SQR_BLOCK_SIZES
               ? BLOCK_INVALID
               : subsize_lookup[partition][sqr_bsize_idx];
  }
}

static TX_TYPE intra_mode_to_tx_type(const MB_MODE_INFO *mbmi,
                                     PLANE_TYPE plane_type) {
  static const TX_TYPE _intra_mode_to_tx_type[INTRA_MODES] = {
    DCT_DCT,    // DC_PRED
    ADST_DCT,   // V_PRED
    DCT_ADST,   // H_PRED
    DCT_DCT,    // D45_PRED
    ADST_ADST,  // D135_PRED
    ADST_DCT,   // D113_PRED
    DCT_ADST,   // D157_PRED
    DCT_ADST,   // D203_PRED
    ADST_DCT,   // D67_PRED
    ADST_ADST,  // SMOOTH_PRED
    ADST_DCT,   // SMOOTH_V_PRED
    DCT_ADST,   // SMOOTH_H_PRED
    ADST_ADST,  // PAETH_PRED
  };
  const PREDICTION_MODE mode =
      (plane_type == PLANE_TYPE_Y) ? mbmi->mode : get_uv_mode(mbmi->uv_mode);
  assert(mode < INTRA_MODES);
  return _intra_mode_to_tx_type[mode];
}

static inline int is_rect_tx(TX_SIZE tx_size) { return tx_size >= TX_SIZES; }

static inline int block_signals_txsize(BLOCK_SIZE bsize) {
  return bsize > BLOCK_4X4;
}

// Number of transform types in each set type
static const int av1_num_ext_tx_set[EXT_TX_SET_TYPES] = {
  1, 2, 5, 7, 12, 16,
};

static const int av1_ext_tx_used[EXT_TX_SET_TYPES][TX_TYPES] = {
  { 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 },
  { 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0 },
  { 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0 },
  { 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0 },
  { 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0 },
  { 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 },
};

// The bitmask corresponds to the transform types as defined in
// enums.h TX_TYPE enumeration type. Setting the bit 0 means to disable
// the use of the corresponding transform type in that table.
// The av1_derived_intra_tx_used_flag table is used when
// use_reduced_intra_txset is set to 2, where one only searches
// the transform types derived from residual statistics.
static const uint16_t av1_derived_intra_tx_used_flag[INTRA_MODES] = {
  0x0209,  // DC_PRED:       0000 0010 0000 1001
  0x0403,  // V_PRED:        0000 0100 0000 0011
  0x0805,  // H_PRED:        0000 1000 0000 0101
  0x020F,  // D45_PRED:      0000 0010 0000 1111
  0x0009,  // D135_PRED:     0000 0000 0000 1001
  0x0009,  // D113_PRED:     0000 0000 0000 1001
  0x0009,  // D157_PRED:     0000 0000 0000 1001
  0x0805,  // D203_PRED:     0000 1000 0000 0101
  0x0403,  // D67_PRED:      0000 0100 0000 0011
  0x0205,  // SMOOTH_PRED:   0000 0010 0000 1001
  0x0403,  // SMOOTH_V_PRED: 0000 0100 0000 0011
  0x0805,  // SMOOTH_H_PRED: 0000 1000 0000 0101
  0x0209,  // PAETH_PRED:    0000 0010 0000 1001
};

static const uint16_t av1_reduced_intra_tx_used_flag[INTRA_MODES] = {
  0x080F,  // DC_PRED:       0000 1000 0000 1111
  0x040F,  // V_PRED:        0000 0100 0000 1111
  0x080F,  // H_PRED:        0000 1000 0000 1111
  0x020F,  // D45_PRED:      0000 0010 0000 1111
  0x080F,  // D135_PRED:     0000 1000 0000 1111
  0x040F,  // D113_PRED:     0000 0100 0000 1111
  0x080F,  // D157_PRED:     0000 1000 0000 1111
  0x080F,  // D203_PRED:     0000 1000 0000 1111
  0x040F,  // D67_PRED:      0000 0100 0000 1111
  0x080F,  // SMOOTH_PRED:   0000 1000 0000 1111
  0x040F,  // SMOOTH_V_PRED: 0000 0100 0000 1111
  0x080F,  // SMOOTH_H_PRED: 0000 1000 0000 1111
  0x0C0E,  // PAETH_PRED:    0000 1100 0000 1110
};

static const uint16_t av1_ext_tx_used_flag[EXT_TX_SET_TYPES] = {
  0x0001,  // 0000 0000 0000 0001
  0x0201,  // 0000 0010 0000 0001
  0x020F,  // 0000 0010 0000 1111
  0x0E0F,  // 0000 1110 0000 1111
  0x0FFF,  // 0000 1111 1111 1111
  0xFFFF,  // 1111 1111 1111 1111
};

static const TxSetType av1_ext_tx_set_lookup[2][2] = {
  { EXT_TX_SET_DTT4_IDTX_1DDCT, EXT_TX_SET_DTT4_IDTX },
  { EXT_TX_SET_ALL16, EXT_TX_SET_DTT9_IDTX_1DDCT },
};

static inline TxSetType av1_get_ext_tx_set_type(TX_SIZE tx_size, int is_inter,
                                                int use_reduced_set) {
  const TX_SIZE tx_size_sqr_up = txsize_sqr_up_map[tx_size];
  if (tx_size_sqr_up > TX_32X32) return EXT_TX_SET_DCTONLY;
  if (tx_size_sqr_up == TX_32X32)
    return is_inter ? EXT_TX_SET_DCT_IDTX : EXT_TX_SET_DCTONLY;
  if (use_reduced_set)
    return is_inter ? EXT_TX_SET_DCT_IDTX : EXT_TX_SET_DTT4_IDTX;
  const TX_SIZE tx_size_sqr = txsize_sqr_map[tx_size];
  return av1_ext_tx_set_lookup[is_inter][tx_size_sqr == TX_16X16];
}

// Maps tx set types to the indices.
static const int ext_tx_set_index[2][EXT_TX_SET_TYPES] = {
  { // Intra
    0, -1, 2, 1, -1, -1 },
  { // Inter
    0, 3, -1, -1, 2, 1 },
};

static inline int get_ext_tx_set(TX_SIZE tx_size, int is_inter,
                                 int use_reduced_set) {
  const TxSetType set_type =
      av1_get_ext_tx_set_type(tx_size, is_inter, use_reduced_set);
  return ext_tx_set_index[is_inter][set_type];
}

static inline int get_ext_tx_types(TX_SIZE tx_size, int is_inter,
                                   int use_reduced_set) {
  const int set_type =
      av1_get_ext_tx_set_type(tx_size, is_inter, use_reduced_set);
  return av1_num_ext_tx_set[set_type];
}

#define TXSIZEMAX(t1, t2) (tx_size_2d[(t1)] >= tx_size_2d[(t2)] ? (t1) : (t2))
#define TXSIZEMIN(t1, t2) (tx_size_2d[(t1)] <= tx_size_2d[(t2)] ? (t1) : (t2))

static inline TX_SIZE tx_size_from_tx_mode(BLOCK_SIZE bsize, TX_MODE tx_mode) {
  const TX_SIZE largest_tx_size = tx_mode_to_biggest_tx_size[tx_mode];
  const TX_SIZE max_rect_tx_size = max_txsize_rect_lookup[bsize];
  if (bsize == BLOCK_4X4)
    return AOMMIN(max_txsize_lookup[bsize], largest_tx_size);
  if (txsize_sqr_map[max_rect_tx_size] <= largest_tx_size)
    return max_rect_tx_size;
  else
    return largest_tx_size;
}

static const uint8_t mode_to_angle_map[INTRA_MODES] = {
  0, 90, 180, 45, 135, 113, 157, 203, 67, 0, 0, 0, 0,
};

// Converts block_index for given transform size to index of the block in raster
// order.
static inline int av1_block_index_to_raster_order(TX_SIZE tx_size,
                                                  int block_idx) {
  // For transform size 4x8, the possible block_idx values are 0 & 2, because
  // block_idx values are incremented in steps of size 'tx_width_unit x
  // tx_height_unit'. But, for this transform size, block_idx = 2 corresponds to
  // block number 1 in raster order, inside an 8x8 MI block.
  // For any other transform size, the two indices are equivalent.
  return (tx_size == TX_4X8 && block_idx == 2) ? 1 : block_idx;
}

// Inverse of above function.
// Note: only implemented for transform sizes 4x4, 4x8 and 8x4 right now.
static inline int av1_raster_order_to_block_index(TX_SIZE tx_size,
                                                  int raster_order) {
  assert(tx_size == TX_4X4 || tx_size == TX_4X8 || tx_size == TX_8X4);
  // We ensure that block indices are 0 & 2 if tx size is 4x8 or 8x4.
  return (tx_size == TX_4X4) ? raster_order : (raster_order > 0) ? 2 : 0;
}

static inline TX_TYPE get_default_tx_type(PLANE_TYPE plane_type,
                                          const MACROBLOCKD *xd,
                                          TX_SIZE tx_size,
                                          int use_screen_content_tools) {
  const MB_MODE_INFO *const mbmi = xd->mi[0];

  if (is_inter_block(mbmi) || plane_type != PLANE_TYPE_Y ||
      xd->lossless[mbmi->segment_id] || tx_size >= TX_32X32 ||
      use_screen_content_tools)
    return DEFAULT_INTER_TX_TYPE;

  return intra_mode_to_tx_type(mbmi, plane_type);
}

// Implements the get_plane_residual_size() function in the spec (Section
// 5.11.38. Get plane residual size function).
static inline BLOCK_SIZE get_plane_block_size(BLOCK_SIZE bsize,
                                              int subsampling_x,
                                              int subsampling_y) {
  assert(bsize < BLOCK_SIZES_ALL);
  assert(subsampling_x >= 0 && subsampling_x < 2);
  assert(subsampling_y >= 0 && subsampling_y < 2);
  return av1_ss_size_lookup[bsize][subsampling_x][subsampling_y];
}

/*
 * Logic to generate the lookup tables:
 *
 * TX_SIZE txs = max_txsize_rect_lookup[bsize];
 * for (int level = 0; level < MAX_VARTX_DEPTH - 1; ++level)
 *   txs = sub_tx_size_map[txs];
 * const int tx_w_log2 = tx_size_wide_log2[txs] - MI_SIZE_LOG2;
 * const int tx_h_log2 = tx_size_high_log2[txs] - MI_SIZE_LOG2;
 * const int bw_uint_log2 = mi_size_wide_log2[bsize];
 * const int stride_log2 = bw_uint_log2 - tx_w_log2;
 */
static inline int av1_get_txb_size_index(BLOCK_SIZE bsize, int blk_row,
                                         int blk_col) {
  static const uint8_t tw_w_log2_table[BLOCK_SIZES_ALL] = {
    0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 0, 1, 1, 2, 2, 3,
  };
  static const uint8_t tw_h_log2_table[BLOCK_SIZES_ALL] = {
    0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 1, 0, 2, 1, 3, 2,
  };
  static const uint8_t stride_log2_table[BLOCK_SIZES_ALL] = {
    0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 2, 2, 0, 1, 0, 1, 0, 1,
  };
  const int index =
      ((blk_row >> tw_h_log2_table[bsize]) << stride_log2_table[bsize]) +
      (blk_col >> tw_w_log2_table[bsize]);
  assert(index < INTER_TX_SIZE_BUF_LEN);
  return index;
}

#if CONFIG_INSPECTION
/*
 * Here is the logic to generate the lookup tables:
 *
 * TX_SIZE txs = max_txsize_rect_lookup[bsize];
 * for (int level = 0; level < MAX_VARTX_DEPTH; ++level)
 *   txs = sub_tx_size_map[txs];
 * const int tx_w_log2 = tx_size_wide_log2[txs] - MI_SIZE_LOG2;
 * const int tx_h_log2 = tx_size_high_log2[txs] - MI_SIZE_LOG2;
 * const int bw_uint_log2 = mi_size_wide_log2[bsize];
 * const int stride_log2 = bw_uint_log2 - tx_w_log2;
 */
static inline int av1_get_txk_type_index(BLOCK_SIZE bsize, int blk_row,
                                         int blk_col) {
  static const uint8_t tw_w_log2_table[BLOCK_SIZES_ALL] = {
    0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 2, 2, 2, 0, 0, 1, 1, 2, 2,
  };
  static const uint8_t tw_h_log2_table[BLOCK_SIZES_ALL] = {
    0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 2, 2, 2, 0, 0, 1, 1, 2, 2,
  };
  static const uint8_t stride_log2_table[BLOCK_SIZES_ALL] = {
    0, 0, 1, 1, 1, 2, 2, 1, 2, 2, 1, 2, 2, 2, 3, 3, 0, 2, 0, 2, 0, 2,
  };
  const int index =
      ((blk_row >> tw_h_log2_table[bsize]) << stride_log2_table[bsize]) +
      (blk_col >> tw_w_log2_table[bsize]);
  assert(index < TXK_TYPE_BUF_LEN);
  return index;
}
#endif  // CONFIG_INSPECTION

static inline void update_txk_array(MACROBLOCKD *const xd, int blk_row,
                                    int blk_col, TX_SIZE tx_size,
                                    TX_TYPE tx_type) {
  const int stride = xd->tx_type_map_stride;
  xd->tx_type_map[blk_row * stride + blk_col] = tx_type;

  const int txw = tx_size_wide_unit[tx_size];
  const int txh = tx_size_high_unit[tx_size];
  // The 16x16 unit is due to the constraint from tx_64x64 which sets the
  // maximum tx size for chroma as 32x32. Coupled with 4x1 transform block
  // size, the constraint takes effect in 32x16 / 16x32 size too. To solve
  // the intricacy, cover all the 16x16 units inside a 64 level transform.
  if (txw == tx_size_wide_unit[TX_64X64] ||
      txh == tx_size_high_unit[TX_64X64]) {
    const int tx_unit = tx_size_wide_unit[TX_16X16];
    for (int idy = 0; idy < txh; idy += tx_unit) {
      for (int idx = 0; idx < txw; idx += tx_unit) {
        xd->tx_type_map[(blk_row + idy) * stride + blk_col + idx] = tx_type;
      }
    }
  }
}

static inline TX_TYPE av1_get_tx_type(const MACROBLOCKD *xd,
                                      PLANE_TYPE plane_type, int blk_row,
                                      int blk_col, TX_SIZE tx_size,
                                      int reduced_tx_set) {
  const MB_MODE_INFO *const mbmi = xd->mi[0];
  if (xd->lossless[mbmi->segment_id] || txsize_sqr_up_map[tx_size] > TX_32X32) {
    return DCT_DCT;
  }

  TX_TYPE tx_type;
  if (plane_type == PLANE_TYPE_Y) {
    tx_type = xd->tx_type_map[blk_row * xd->tx_type_map_stride + blk_col];
  } else {
    if (is_inter_block(mbmi)) {
      // scale back to y plane's coordinate
      const struct macroblockd_plane *const pd = &xd->plane[plane_type];
      blk_row <<= pd->subsampling_y;
      blk_col <<= pd->subsampling_x;
      tx_type = xd->tx_type_map[blk_row * xd->tx_type_map_stride + blk_col];
    } else {
      // In intra mode, uv planes don't share the same prediction mode as y
      // plane, so the tx_type should not be shared
      tx_type = intra_mode_to_tx_type(mbmi, PLANE_TYPE_UV);
    }
    const TxSetType tx_set_type =
        av1_get_ext_tx_set_type(tx_size, is_inter_block(mbmi), reduced_tx_set);
    if (!av1_ext_tx_used[tx_set_type][tx_type]) tx_type = DCT_DCT;
  }
  assert(tx_type < TX_TYPES);
  assert(av1_ext_tx_used[av1_get_ext_tx_set_type(tx_size, is_inter_block(mbmi),
                                                 reduced_tx_set)][tx_type]);
  return tx_type;
}

void av1_setup_block_planes(MACROBLOCKD *xd, int ss_x, int ss_y,
                            const int num_planes);

/*
 * Logic to generate the lookup table:
 *
 * TX_SIZE tx_size = max_txsize_rect_lookup[bsize];
 * int depth = 0;
 * while (depth < MAX_TX_DEPTH && tx_size != TX_4X4) {
 *   depth++;
 *   tx_size = sub_tx_size_map[tx_size];
 * }
 */
static inline int bsize_to_max_depth(BLOCK_SIZE bsize) {
  static const uint8_t bsize_to_max_depth_table[BLOCK_SIZES_ALL] = {
    0, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  };
  return bsize_to_max_depth_table[bsize];
}

/*
 * Logic to generate the lookup table:
 *
 * TX_SIZE tx_size = max_txsize_rect_lookup[bsize];
 * assert(tx_size != TX_4X4);
 * int depth = 0;
 * while (tx_size != TX_4X4) {
 *   depth++;
 *   tx_size = sub_tx_size_map[tx_size];
 * }
 * assert(depth < 10);
 */
static inline int bsize_to_tx_size_cat(BLOCK_SIZE bsize) {
  assert(bsize < BLOCK_SIZES_ALL);
  static const uint8_t bsize_to_tx_size_depth_table[BLOCK_SIZES_ALL] = {
    0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 4, 4, 2, 2, 3, 3, 4, 4,
  };
  const int depth = bsize_to_tx_size_depth_table[bsize];
  assert(depth <= MAX_TX_CATS);
  return depth - 1;
}

static inline TX_SIZE depth_to_tx_size(int depth, BLOCK_SIZE bsize) {
  TX_SIZE max_tx_size = max_txsize_rect_lookup[bsize];
  TX_SIZE tx_size = max_tx_size;
  for (int d = 0; d < depth; ++d) tx_size = sub_tx_size_map[tx_size];
  return tx_size;
}

static inline TX_SIZE av1_get_adjusted_tx_size(TX_SIZE tx_size) {
  switch (tx_size) {
    case TX_64X64:
    case TX_64X32:
    case TX_32X64: return TX_32X32;
    case TX_64X16: return TX_32X16;
    case TX_16X64: return TX_16X32;
    default: return tx_size;
  }
}

static inline TX_SIZE av1_get_max_uv_txsize(BLOCK_SIZE bsize, int subsampling_x,
                                            int subsampling_y) {
  const BLOCK_SIZE plane_bsize =
      get_plane_block_size(bsize, subsampling_x, subsampling_y);
  assert(plane_bsize < BLOCK_SIZES_ALL);
  const TX_SIZE uv_tx = max_txsize_rect_lookup[plane_bsize];
  return av1_get_adjusted_tx_size(uv_tx);
}

static inline TX_SIZE av1_get_tx_size(int plane, const MACROBLOCKD *xd) {
  const MB_MODE_INFO *mbmi = xd->mi[0];
  if (xd->lossless[mbmi->segment_id]) return TX_4X4;
  if (plane == 0) return mbmi->tx_size;
  const MACROBLOCKD_PLANE *pd = &xd->plane[plane];
  return av1_get_max_uv_txsize(mbmi->bsize, pd->subsampling_x,
                               pd->subsampling_y);
}

void av1_reset_entropy_context(MACROBLOCKD *xd, BLOCK_SIZE bsize,
                               const int num_planes);

void av1_reset_loop_filter_delta(MACROBLOCKD *xd, int num_planes);

void av1_reset_loop_restoration(MACROBLOCKD *xd, const int num_planes);

typedef void (*foreach_transformed_block_visitor)(int plane, int block,
                                                  int blk_row, int blk_col,
                                                  BLOCK_SIZE plane_bsize,
                                                  TX_SIZE tx_size, void *arg);

void av1_set_entropy_contexts(const MACROBLOCKD *xd,
                              struct macroblockd_plane *pd, int plane,
                              BLOCK_SIZE plane_bsize, TX_SIZE tx_size,
                              int has_eob, int aoff, int loff);

#define MAX_INTERINTRA_SB_SQUARE 32 * 32
static inline int is_interintra_mode(const MB_MODE_INFO *mbmi) {
  return (mbmi->ref_frame[0] > INTRA_FRAME &&
          mbmi->ref_frame[1] == INTRA_FRAME);
}

static inline int is_interintra_allowed_bsize(const BLOCK_SIZE bsize) {
  return (bsize >= BLOCK_8X8) && (bsize <= BLOCK_32X32);
}

static inline int is_interintra_allowed_mode(const PREDICTION_MODE mode) {
  return (mode >= SINGLE_INTER_MODE_START) && (mode < SINGLE_INTER_MODE_END);
}

static inline int is_interintra_allowed_ref(const MV_REFERENCE_FRAME rf[2]) {
  return (rf[0] > INTRA_FRAME) && (rf[1] <= INTRA_FRAME);
}

static inline int is_interintra_allowed(const MB_MODE_INFO *mbmi) {
  return is_interintra_allowed_bsize(mbmi->bsize) &&
         is_interintra_allowed_mode(mbmi->mode) &&
         is_interintra_allowed_ref(mbmi->ref_frame);
}

static inline int is_interintra_allowed_bsize_group(int group) {
  int i;
  for (i = 0; i < BLOCK_SIZES_ALL; i++) {
    if (size_group_lookup[i] == group &&
        is_interintra_allowed_bsize((BLOCK_SIZE)i)) {
      return 1;
    }
  }
  return 0;
}

static inline int is_interintra_pred(const MB_MODE_INFO *mbmi) {
  return mbmi->ref_frame[0] > INTRA_FRAME &&
         mbmi->ref_frame[1] == INTRA_FRAME && is_interintra_allowed(mbmi);
}

static inline int get_vartx_max_txsize(const MACROBLOCKD *xd, BLOCK_SIZE bsize,
                                       int plane) {
  if (xd->lossless[xd->mi[0]->segment_id]) return TX_4X4;
  const TX_SIZE max_txsize = max_txsize_rect_lookup[bsize];
  if (plane == 0) return max_txsize;            // luma
  return av1_get_adjusted_tx_size(max_txsize);  // chroma
}

static inline int is_motion_variation_allowed_bsize(BLOCK_SIZE bsize) {
  assert(bsize < BLOCK_SIZES_ALL);
  return AOMMIN(block_size_wide[bsize], block_size_high[bsize]) >= 8;
}

static inline int is_motion_variation_allowed_compound(
    const MB_MODE_INFO *mbmi) {
  return !has_second_ref(mbmi);
}

// input: log2 of length, 0(4), 1(8), ...
static const int max_neighbor_obmc[6] = { 0, 1, 2, 3, 4, 4 };

static inline int check_num_overlappable_neighbors(const MB_MODE_INFO *mbmi) {
  return mbmi->overlappable_neighbors != 0;
}

static inline MOTION_MODE motion_mode_allowed(
    const WarpedMotionParams *gm_params, const MACROBLOCKD *xd,
    const MB_MODE_INFO *mbmi, int allow_warped_motion) {
  if (!check_num_overlappable_neighbors(mbmi)) return SIMPLE_TRANSLATION;
  if (xd->cur_frame_force_integer_mv == 0) {
    const TransformationType gm_type = gm_params[mbmi->ref_frame[0]].wmtype;
    if (is_global_mv_block(mbmi, gm_type)) return SIMPLE_TRANSLATION;
  }
  if (is_motion_variation_allowed_bsize(mbmi->bsize) &&
      is_inter_mode(mbmi->mode) && mbmi->ref_frame[1] != INTRA_FRAME &&
      is_motion_variation_allowed_compound(mbmi)) {
    assert(!has_second_ref(mbmi));
    if (mbmi->num_proj_ref >= 1 && allow_warped_motion &&
        !xd->cur_frame_force_integer_mv &&
        !av1_is_scaled(xd->block_ref_scale_factors[0])) {
      return WARPED_CAUSAL;
    }
    return OBMC_CAUSAL;
  }
  return SIMPLE_TRANSLATION;
}

static inline int is_neighbor_overlappable(const MB_MODE_INFO *mbmi) {
  return (is_inter_block(mbmi));
}

static inline int av1_allow_palette(int allow_screen_content_tools,
                                    BLOCK_SIZE sb_type) {
  assert(sb_type < BLOCK_SIZES_ALL);
  return allow_screen_content_tools &&
         block_size_wide[sb_type] <= MAX_PALETTE_BLOCK_WIDTH &&
         block_size_high[sb_type] <= MAX_PALETTE_BLOCK_HEIGHT &&
         sb_type >= BLOCK_8X8;
}

// Returns sub-sampled dimensions of the given block.
// The output values for 'rows_within_bounds' and 'cols_within_bounds' will
// differ from 'height' and 'width' when part of the block is outside the
// right
// and/or bottom image boundary.
static inline void av1_get_block_dimensions(BLOCK_SIZE bsize, int plane,
                                            const MACROBLOCKD *xd, int *width,
                                            int *height,
                                            int *rows_within_bounds,
                                            int *cols_within_bounds) {
  const int block_height = block_size_high[bsize];
  const int block_width = block_size_wide[bsize];
  const int block_rows = (xd->mb_to_bottom_edge >= 0)
                             ? block_height
                             : (xd->mb_to_bottom_edge >> 3) + block_height;
  const int block_cols = (xd->mb_to_right_edge >= 0)
                             ? block_width
                             : (xd->mb_to_right_edge >> 3) + block_width;
  const struct macroblockd_plane *const pd = &xd->plane[plane];
  assert(IMPLIES(plane == PLANE_TYPE_Y, pd->subsampling_x == 0));
  assert(IMPLIES(plane == PLANE_TYPE_Y, pd->subsampling_y == 0));
  assert(block_width >= block_cols);
  assert(block_height >= block_rows);
  const int plane_block_width = block_width >> pd->subsampling_x;
  const int plane_block_height = block_height >> pd->subsampling_y;
  // Special handling for chroma sub8x8.
  const int is_chroma_sub8_x = plane > 0 && plane_block_width < 4;
  const int is_chroma_sub8_y = plane > 0 && plane_block_height < 4;
  if (width) {
    *width = plane_block_width + 2 * is_chroma_sub8_x;
    assert(*width >= 0);
  }
  if (height) {
    *height = plane_block_height + 2 * is_chroma_sub8_y;
    assert(*height >= 0);
  }
  if (rows_within_bounds) {
    *rows_within_bounds =
        (block_rows >> pd->subsampling_y) + 2 * is_chroma_sub8_y;
    assert(*rows_within_bounds >= 0);
  }
  if (cols_within_bounds) {
    *cols_within_bounds =
        (block_cols >> pd->subsampling_x) + 2 * is_chroma_sub8_x;
    assert(*cols_within_bounds >= 0);
  }
}

/* clang-format off */
// Pointer to a three-dimensional array whose first dimension is PALETTE_SIZES.
typedef aom_cdf_prob (*MapCdf)[PALETTE_COLOR_INDEX_CONTEXTS]
                              [CDF_SIZE(PALETTE_COLORS)];
// Pointer to a const three-dimensional array whose first dimension is
// PALETTE_SIZES.
typedef const int (*ColorCost)[PALETTE_COLOR_INDEX_CONTEXTS][PALETTE_COLORS];
/* clang-format on */

typedef struct {
  int rows;
  int cols;
  int n_colors;
  int plane_width;
  int plane_height;
  uint8_t *color_map;
  MapCdf map_cdf;
  ColorCost color_cost;
} Av1ColorMapParam;

static inline int is_nontrans_global_motion(const MACROBLOCKD *xd,
                                            const MB_MODE_INFO *mbmi) {
  int ref;

  // First check if all modes are GLOBALMV
  if (mbmi->mode != GLOBALMV && mbmi->mode != GLOBAL_GLOBALMV) return 0;

  if (AOMMIN(mi_size_wide[mbmi->bsize], mi_size_high[mbmi->bsize]) < 2)
    return 0;

  // Now check if all global motion is non translational
  for (ref = 0; ref < 1 + has_second_ref(mbmi); ++ref) {
    if (xd->global_motion[mbmi->ref_frame[ref]].wmtype == TRANSLATION) return 0;
  }
  return 1;
}

static inline PLANE_TYPE get_plane_type(int plane) {
  return (plane == 0) ? PLANE_TYPE_Y : PLANE_TYPE_UV;
}

static inline int av1_get_max_eob(TX_SIZE tx_size) {
  if (tx_size == TX_64X64 || tx_size == TX_64X32 || tx_size == TX_32X64) {
    return 1024;
  }
  if (tx_size == TX_16X64 || tx_size == TX_64X16) {
    return 512;
  }
  return tx_size_2d[tx_size];
}

/*!\endcond */

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_COMMON_BLOCKD_H_
