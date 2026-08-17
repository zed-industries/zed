// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use super::*;
use crate::predict::PredictionMode;
use crate::predict::PredictionMode::*;
use crate::transform::TxType::*;
use std::mem::MaybeUninit;

pub const MAX_TX_SIZE: usize = 64;

pub const MAX_CODED_TX_SIZE: usize = 32;
pub const MAX_CODED_TX_SQUARE: usize = MAX_CODED_TX_SIZE * MAX_CODED_TX_SIZE;

pub const TX_SIZE_SQR_CONTEXTS: usize = 4; // Coded tx_size <= 32x32, so is the # of CDF contexts from tx sizes

pub const TX_SETS: usize = 6;
pub const TX_SETS_INTRA: usize = 3;
pub const TX_SETS_INTER: usize = 4;

pub const INTRA_MODES: usize = 13;
pub const UV_INTRA_MODES: usize = 14;

const MAX_VARTX_DEPTH: usize = 2;

pub const TXFM_PARTITION_CONTEXTS: usize =
  (TxSize::TX_SIZES - TxSize::TX_8X8 as usize) * 6 - 3;

// Number of transform types in each set type
pub static num_tx_set: [usize; TX_SETS] = [1, 2, 5, 7, 12, 16];
pub static av1_tx_used: [[usize; TX_TYPES]; TX_SETS] = [
  [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
  [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
  [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
  [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
  [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

// Maps set types above to the indices used for intra
static tx_set_index_intra: [i8; TX_SETS] = [0, -1, 2, 1, -1, -1];
// Maps set types above to the indices used for inter
static tx_set_index_inter: [i8; TX_SETS] = [0, 3, -1, -1, 2, 1];

pub static av1_tx_ind: [[usize; TX_TYPES]; TX_SETS] = [
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [1, 3, 4, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [1, 5, 6, 4, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0],
  [3, 4, 5, 8, 6, 7, 9, 10, 11, 0, 1, 2, 0, 0, 0, 0],
  [7, 8, 9, 12, 10, 11, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6],
];

pub static max_txsize_rect_lookup: [TxSize; BlockSize::BLOCK_SIZES_ALL] = [
  TX_4X4,   // 4x4
  TX_4X8,   // 4x8
  TX_8X4,   // 8x4
  TX_8X8,   // 8x8
  TX_8X16,  // 8x16
  TX_16X8,  // 16x8
  TX_16X16, // 16x16
  TX_16X32, // 16x32
  TX_32X16, // 32x16
  TX_32X32, // 32x32
  TX_32X64, // 32x64
  TX_64X32, // 64x32
  TX_64X64, // 64x64
  TX_64X64, // 64x128
  TX_64X64, // 128x64
  TX_64X64, // 128x128
  TX_4X16,  // 4x16
  TX_16X4,  // 16x4
  TX_8X32,  // 8x32
  TX_32X8,  // 32x8
  TX_16X64, // 16x64
  TX_64X16, // 64x16
];

pub static sub_tx_size_map: [TxSize; TxSize::TX_SIZES_ALL] = [
  TX_4X4,   // TX_4X4
  TX_4X4,   // TX_8X8
  TX_8X8,   // TX_16X16
  TX_16X16, // TX_32X32
  TX_32X32, // TX_64X64
  TX_4X4,   // TX_4X8
  TX_4X4,   // TX_8X4
  TX_8X8,   // TX_8X16
  TX_8X8,   // TX_16X8
  TX_16X16, // TX_16X32
  TX_16X16, // TX_32X16
  TX_32X32, // TX_32X64
  TX_32X32, // TX_64X32
  TX_4X8,   // TX_4X16
  TX_8X4,   // TX_16X4
  TX_8X16,  // TX_8X32
  TX_16X8,  // TX_32X8
  TX_16X32, // TX_16X64
  TX_32X16, // TX_64X16
];

#[inline]
pub fn has_chroma(
  bo: TileBlockOffset, bsize: BlockSize, subsampling_x: usize,
  subsampling_y: usize, chroma_sampling: ChromaSampling,
) -> bool {
  if chroma_sampling == ChromaSampling::Cs400 {
    return false;
  };

  let bw = bsize.width_mi();
  let bh = bsize.height_mi();

  ((bo.0.x & 0x01) == 1 || (bw & 0x01) == 0 || subsampling_x == 0)
    && ((bo.0.y & 0x01) == 1 || (bh & 0x01) == 0 || subsampling_y == 0)
}

pub fn get_tx_set(
  tx_size: TxSize, is_inter: bool, use_reduced_set: bool,
) -> TxSet {
  let tx_size_sqr_up = tx_size.sqr_up();
  let tx_size_sqr = tx_size.sqr();

  if tx_size_sqr_up.block_size() > BlockSize::BLOCK_32X32 {
    return TxSet::TX_SET_DCTONLY;
  }

  if is_inter {
    if use_reduced_set || tx_size_sqr_up == TxSize::TX_32X32 {
      TxSet::TX_SET_INTER_3
    } else if tx_size_sqr == TxSize::TX_16X16 {
      TxSet::TX_SET_INTER_2
    } else {
      TxSet::TX_SET_INTER_1
    }
  } else if tx_size_sqr_up == TxSize::TX_32X32 {
    TxSet::TX_SET_DCTONLY
  } else if use_reduced_set || tx_size_sqr == TxSize::TX_16X16 {
    TxSet::TX_SET_INTRA_2
  } else {
    TxSet::TX_SET_INTRA_1
  }
}

pub fn get_tx_set_index(
  tx_size: TxSize, is_inter: bool, use_reduced_set: bool,
) -> i8 {
  let set_type = get_tx_set(tx_size, is_inter, use_reduced_set);

  if is_inter {
    tx_set_index_inter[set_type as usize]
  } else {
    tx_set_index_intra[set_type as usize]
  }
}

static intra_mode_to_tx_type_context: [TxType; INTRA_MODES] = [
  DCT_DCT,   // DC
  ADST_DCT,  // V
  DCT_ADST,  // H
  DCT_DCT,   // D45
  ADST_ADST, // D135
  ADST_DCT,  // D113
  DCT_ADST,  // D157
  DCT_ADST,  // D203
  ADST_DCT,  // D67
  ADST_ADST, // SMOOTH
  ADST_DCT,  // SMOOTH_V
  DCT_ADST,  // SMOOTH_H
  ADST_ADST, // PAETH
];

static uv2y: [PredictionMode; UV_INTRA_MODES] = [
  DC_PRED,       // UV_DC_PRED
  V_PRED,        // UV_V_PRED
  H_PRED,        // UV_H_PRED
  D45_PRED,      // UV_D45_PRED
  D135_PRED,     // UV_D135_PRED
  D113_PRED,     // UV_D113_PRED
  D157_PRED,     // UV_D157_PRED
  D203_PRED,     // UV_D203_PRED
  D67_PRED,      // UV_D67_PRED
  SMOOTH_PRED,   // UV_SMOOTH_PRED
  SMOOTH_V_PRED, // UV_SMOOTH_V_PRED
  SMOOTH_H_PRED, // UV_SMOOTH_H_PRED
  PAETH_PRED,    // UV_PAETH_PRED
  DC_PRED,       // CFL_PRED
];

pub fn uv_intra_mode_to_tx_type_context(pred: PredictionMode) -> TxType {
  intra_mode_to_tx_type_context[uv2y[pred as usize] as usize]
}

// Level Map
pub const TXB_SKIP_CONTEXTS: usize = 13;

pub const EOB_COEF_CONTEXTS: usize = 9;

const SIG_COEF_CONTEXTS_2D: usize = 26;
const SIG_COEF_CONTEXTS_1D: usize = 16;
pub const SIG_COEF_CONTEXTS_EOB: usize = 4;
pub const SIG_COEF_CONTEXTS: usize =
  SIG_COEF_CONTEXTS_2D + SIG_COEF_CONTEXTS_1D;

const COEFF_BASE_CONTEXTS: usize = SIG_COEF_CONTEXTS;
pub const DC_SIGN_CONTEXTS: usize = 3;

const BR_TMP_OFFSET: usize = 12;
const BR_REF_CAT: usize = 4;
pub const LEVEL_CONTEXTS: usize = 21;

pub const NUM_BASE_LEVELS: usize = 2;

pub const BR_CDF_SIZE: usize = 4;
pub const COEFF_BASE_RANGE: usize = 4 * (BR_CDF_SIZE - 1);

pub const COEFF_CONTEXT_BITS: usize = 6;
pub const COEFF_CONTEXT_MASK: usize = (1 << COEFF_CONTEXT_BITS) - 1;
const MAX_BASE_BR_RANGE: usize = COEFF_BASE_RANGE + NUM_BASE_LEVELS + 1;

const BASE_CONTEXT_POSITION_NUM: usize = 12;

// Pad 4 extra columns to remove horizontal availability check.
pub const TX_PAD_HOR_LOG2: usize = 2;
pub const TX_PAD_HOR: usize = 4;
// Pad 6 extra rows (2 on top and 4 on bottom) to remove vertical availability
// check.
pub const TX_PAD_TOP: usize = 2;
pub const TX_PAD_BOTTOM: usize = 4;
pub const TX_PAD_VER: usize = TX_PAD_TOP + TX_PAD_BOTTOM;
// Pad 16 extra bytes to avoid reading overflow in SIMD optimization.
const TX_PAD_END: usize = 16;
pub const TX_PAD_2D: usize = (MAX_CODED_TX_SIZE + TX_PAD_HOR)
  * (MAX_CODED_TX_SIZE + TX_PAD_VER)
  + TX_PAD_END;

const TX_CLASSES: usize = 3;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TxClass {
  TX_CLASS_2D = 0,
  TX_CLASS_HORIZ = 1,
  TX_CLASS_VERT = 2,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SegLvl {
  SEG_LVL_ALT_Q = 0,      /* Use alternate Quantizer .... */
  SEG_LVL_ALT_LF_Y_V = 1, /* Use alternate loop filter value on y plane vertical */
  SEG_LVL_ALT_LF_Y_H = 2, /* Use alternate loop filter value on y plane horizontal */
  SEG_LVL_ALT_LF_U = 3,   /* Use alternate loop filter value on u plane */
  SEG_LVL_ALT_LF_V = 4,   /* Use alternate loop filter value on v plane */
  SEG_LVL_REF_FRAME = 5,  /* Optional Segment reference frame */
  SEG_LVL_SKIP = 6,       /* Optional Segment (0,0) + skip mode */
  SEG_LVL_GLOBALMV = 7,
  SEG_LVL_MAX = 8,
}

pub const seg_feature_bits: [u32; SegLvl::SEG_LVL_MAX as usize] =
  [8, 6, 6, 6, 6, 3, 0, 0];

pub const seg_feature_is_signed: [bool; SegLvl::SEG_LVL_MAX as usize] =
  [true, true, true, true, true, false, false, false];

use crate::context::TxClass::*;

pub static tx_type_to_class: [TxClass; TX_TYPES] = [
  TX_CLASS_2D,    // DCT_DCT
  TX_CLASS_2D,    // ADST_DCT
  TX_CLASS_2D,    // DCT_ADST
  TX_CLASS_2D,    // ADST_ADST
  TX_CLASS_2D,    // FLIPADST_DCT
  TX_CLASS_2D,    // DCT_FLIPADST
  TX_CLASS_2D,    // FLIPADST_FLIPADST
  TX_CLASS_2D,    // ADST_FLIPADST
  TX_CLASS_2D,    // FLIPADST_ADST
  TX_CLASS_2D,    // IDTX
  TX_CLASS_VERT,  // V_DCT
  TX_CLASS_HORIZ, // H_DCT
  TX_CLASS_VERT,  // V_ADST
  TX_CLASS_HORIZ, // H_ADST
  TX_CLASS_VERT,  // V_FLIPADST
  TX_CLASS_HORIZ, // H_FLIPADST
];

pub static eob_to_pos_small: [u8; 33] = [
  0, 1, 2, // 0-2
  3, 3, // 3-4
  4, 4, 4, 4, // 5-8
  5, 5, 5, 5, 5, 5, 5, 5, // 9-16
  6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, // 17-32
];

pub static eob_to_pos_large: [u8; 17] = [
  6, // place holder
  7, // 33-64
  8, 8, // 65-128
  9, 9, 9, 9, // 129-256
  10, 10, 10, 10, 10, 10, 10, 10, // 257-512
  11, // 513-
];

pub static k_eob_group_start: [u16; 12] =
  [0, 1, 2, 3, 5, 9, 17, 33, 65, 129, 257, 513];
pub static k_eob_offset_bits: [u16; 12] = [0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

// The ctx offset table when TX is TX_CLASS_2D.
// TX col and row indices are clamped to 4

#[rustfmt::skip]
pub static av1_nz_map_ctx_offset: [[[i8; 5]; 5]; TxSize::TX_SIZES_ALL] = [
  // TX_4X4
  [
    [ 0,  1,  6,  6, 0],
    [ 1,  6,  6, 21, 0],
    [ 6,  6, 21, 21, 0],
    [ 6, 21, 21, 21, 0],
    [ 0,  0,  0,  0, 0]
  ],
  // TX_8X8
  [
    [ 0,  1,  6,  6, 21],
    [ 1,  6,  6, 21, 21],
    [ 6,  6, 21, 21, 21],
    [ 6, 21, 21, 21, 21],
    [21, 21, 21, 21, 21]
  ],
  // TX_16X16
  [
    [ 0,  1,  6,  6, 21],
    [ 1,  6,  6, 21, 21],
    [ 6,  6, 21, 21, 21],
    [ 6, 21, 21, 21, 21],
    [21, 21, 21, 21, 21]
  ],
  // TX_32X32
  [
    [ 0,  1,  6,  6, 21],
    [ 1,  6,  6, 21, 21],
    [ 6,  6, 21, 21, 21],
    [ 6, 21, 21, 21, 21],
    [21, 21, 21, 21, 21]
  ],
  // TX_64X64
  [
    [ 0,  1,  6,  6, 21],
    [ 1,  6,  6, 21, 21],
    [ 6,  6, 21, 21, 21],
    [ 6, 21, 21, 21, 21],
    [21, 21, 21, 21, 21]
  ],
  // TX_4X8
  [
    [ 0, 11, 11, 11, 0],
    [11, 11, 11, 11, 0],
    [ 6,  6, 21, 21, 0],
    [ 6, 21, 21, 21, 0],
    [21, 21, 21, 21, 0]
  ],
  // TX_8X4
  [
    [ 0, 16,  6,  6, 21],
    [16, 16,  6, 21, 21],
    [16, 16, 21, 21, 21],
    [16, 16, 21, 21, 21],
    [ 0,  0,  0,  0, 0]
  ],
  // TX_8X16
  [
    [ 0, 11, 11, 11, 11],
    [11, 11, 11, 11, 11],
    [ 6,  6, 21, 21, 21],
    [ 6, 21, 21, 21, 21],
    [21, 21, 21, 21, 21]
  ],
  // TX_16X8
  [
    [ 0, 16,  6,  6, 21],
    [16, 16,  6, 21, 21],
    [16, 16, 21, 21, 21],
    [16, 16, 21, 21, 21],
    [16, 16, 21, 21, 21]
  ],
  // TX_16X32
  [
    [ 0, 11, 11, 11, 11],
    [11, 11, 11, 11, 11],
    [ 6,  6, 21, 21, 21],
    [ 6, 21, 21, 21, 21],
    [21, 21, 21, 21, 21]
  ],
  // TX_32X16
  [
    [ 0, 16,  6,  6, 21],
    [16, 16,  6, 21, 21],
    [16, 16, 21, 21, 21],
    [16, 16, 21, 21, 21],
    [16, 16, 21, 21, 21]
  ],
  // TX_32X64
  [
    [ 0, 11, 11, 11, 11],
    [11, 11, 11, 11, 11],
    [ 6,  6, 21, 21, 21],
    [ 6, 21, 21, 21, 21],
    [21, 21, 21, 21, 21]
  ],
  // TX_64X32
  [
    [ 0, 16,  6,  6, 21],
    [16, 16,  6, 21, 21],
    [16, 16, 21, 21, 21],
    [16, 16, 21, 21, 21],
    [16, 16, 21, 21, 21]
  ],
  // TX_4X16
  [
    [ 0, 11, 11, 11, 0],
    [11, 11, 11, 11, 0],
    [ 6,  6, 21, 21, 0],
    [ 6, 21, 21, 21, 0],
    [21, 21, 21, 21, 0]
  ],
  // TX_16X4
  [
    [ 0, 16,  6,  6, 21],
    [16, 16,  6, 21, 21],
    [16, 16, 21, 21, 21],
    [16, 16, 21, 21, 21],
    [ 0,  0,  0,  0, 0]
  ],
  // TX_8X32
  [
    [ 0, 11, 11, 11, 11],
    [11, 11, 11, 11, 11],
    [ 6,  6, 21, 21, 21],
    [ 6, 21, 21, 21, 21],
    [21, 21, 21, 21, 21]
  ],
  // TX_32X8
  [
    [ 0, 16,  6,  6, 21],
    [16, 16,  6, 21, 21],
    [16, 16, 21, 21, 21],
    [16, 16, 21, 21, 21],
    [16, 16, 21, 21, 21]
  ],
  // TX_16X64
  [
    [ 0, 11, 11, 11, 11],
    [11, 11, 11, 11, 11],
    [ 6,  6, 21, 21, 21],
    [ 6, 21, 21, 21, 21],
    [21, 21, 21, 21, 21]
  ],
  // TX_64X16
  [
    [ 0, 16,  6,  6, 21],
    [16, 16,  6, 21, 21],
    [16, 16, 21, 21, 21],
    [16, 16, 21, 21, 21],
    [16, 16, 21, 21, 21]
  ]
];

const NZ_MAP_CTX_0: usize = SIG_COEF_CONTEXTS_2D;
const NZ_MAP_CTX_5: usize = NZ_MAP_CTX_0 + 5;
const NZ_MAP_CTX_10: usize = NZ_MAP_CTX_0 + 10;

pub static nz_map_ctx_offset_1d: [usize; 32] = [
  NZ_MAP_CTX_0,
  NZ_MAP_CTX_5,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
  NZ_MAP_CTX_10,
];

const CONTEXT_MAG_POSITION_NUM: usize = 3;

static mag_ref_offset_with_txclass: [[[usize; 2]; CONTEXT_MAG_POSITION_NUM];
  3] = [
  [[0, 1], [1, 0], [1, 1]],
  [[0, 1], [1, 0], [0, 2]],
  [[0, 1], [1, 0], [2, 0]],
];

// End of Level Map

pub struct TXB_CTX {
  pub txb_skip_ctx: usize,
  pub dc_sign_ctx: usize,
}

impl ContextWriter<'_> {
  /// # Panics
  ///
  /// - If an invalid combination of `tx_type` and `tx_size` is passed
  pub fn write_tx_type<W: Writer>(
    &mut self, w: &mut W, tx_size: TxSize, tx_type: TxType,
    y_mode: PredictionMode, is_inter: bool, use_reduced_tx_set: bool,
  ) {
    let square_tx_size = tx_size.sqr();
    let tx_set = get_tx_set(tx_size, is_inter, use_reduced_tx_set);
    let num_tx_types = num_tx_set[tx_set as usize];

    if num_tx_types > 1 {
      let tx_set_index =
        get_tx_set_index(tx_size, is_inter, use_reduced_tx_set);
      assert!(tx_set_index > 0);
      assert!(av1_tx_used[tx_set as usize][tx_type as usize] != 0);

      if is_inter {
        let s = av1_tx_ind[tx_set as usize][tx_type as usize] as u32;
        if tx_set_index == 1 {
          let cdf = &self.fc.inter_tx_1_cdf[square_tx_size as usize];
          symbol_with_update!(self, w, s, cdf);
        } else if tx_set_index == 2 {
          let cdf = &self.fc.inter_tx_2_cdf[square_tx_size as usize];
          symbol_with_update!(self, w, s, cdf);
        } else {
          let cdf = &self.fc.inter_tx_3_cdf[square_tx_size as usize];
          symbol_with_update!(self, w, s, cdf);
        }
      } else {
        let intra_dir = y_mode;
        // TODO: Once use_filter_intra is enabled,
        // intra_dir =
        // fimode_to_intradir[mbmi->filter_intra_mode_info.filter_intra_mode];

        let s = av1_tx_ind[tx_set as usize][tx_type as usize] as u32;
        if tx_set_index == 1 {
          let cdf = &self.fc.intra_tx_1_cdf[square_tx_size as usize]
            [intra_dir as usize];
          symbol_with_update!(self, w, s, cdf);
        } else {
          let cdf = &self.fc.intra_tx_2_cdf[square_tx_size as usize]
            [intra_dir as usize];
          symbol_with_update!(self, w, s, cdf);
        }
      }
    }
  }

  fn get_tx_size_context(
    &self, bo: TileBlockOffset, bsize: BlockSize,
  ) -> usize {
    let max_tx_size = max_txsize_rect_lookup[bsize as usize];
    let max_tx_wide = max_tx_size.width() as u8;
    let max_tx_high = max_tx_size.height() as u8;
    let has_above = bo.0.y > 0;
    let has_left = bo.0.x > 0;
    let mut above = self.bc.above_tx_context[bo.0.x] >= max_tx_wide;
    let mut left = self.bc.left_tx_context[bo.y_in_sb()] >= max_tx_high;

    if has_above {
      let above_blk = self.bc.blocks.above_of(bo);
      if above_blk.is_inter() {
        above = (above_blk.n4_w << MI_SIZE_LOG2) >= max_tx_wide;
      };
    }
    if has_left {
      let left_blk = self.bc.blocks.left_of(bo);
      if left_blk.is_inter() {
        left = (left_blk.n4_h << MI_SIZE_LOG2) >= max_tx_high;
      };
    }
    if has_above && has_left {
      return above as usize + left as usize;
    };
    if has_above {
      return above as usize;
    };
    if has_left {
      return left as usize;
    };
    0
  }

  pub fn write_tx_size_intra<W: Writer>(
    &mut self, w: &mut W, bo: TileBlockOffset, bsize: BlockSize,
    tx_size: TxSize,
  ) {
    fn tx_size_to_depth(tx_size: TxSize, bsize: BlockSize) -> usize {
      let mut ctx_size = max_txsize_rect_lookup[bsize as usize];
      let mut depth: usize = 0;
      while tx_size != ctx_size {
        depth += 1;
        ctx_size = sub_tx_size_map[ctx_size as usize];
        debug_assert!(depth <= MAX_TX_DEPTH);
      }
      depth
    }
    fn bsize_to_max_depth(bsize: BlockSize) -> usize {
      let mut tx_size: TxSize = max_txsize_rect_lookup[bsize as usize];
      let mut depth = 0;
      while depth < MAX_TX_DEPTH && tx_size != TX_4X4 {
        depth += 1;
        tx_size = sub_tx_size_map[tx_size as usize];
        debug_assert!(depth <= MAX_TX_DEPTH);
      }
      depth
    }
    fn bsize_to_tx_size_cat(bsize: BlockSize) -> usize {
      let mut tx_size: TxSize = max_txsize_rect_lookup[bsize as usize];
      debug_assert!(tx_size != TX_4X4);
      let mut depth = 0;
      while tx_size != TX_4X4 {
        depth += 1;
        tx_size = sub_tx_size_map[tx_size as usize];
      }
      debug_assert!(depth <= MAX_TX_CATS);

      depth - 1
    }

    debug_assert!(!self.bc.blocks[bo].is_inter());
    debug_assert!(bsize > BlockSize::BLOCK_4X4);

    let tx_size_ctx = self.get_tx_size_context(bo, bsize);
    let depth = tx_size_to_depth(tx_size, bsize);

    let max_depths = bsize_to_max_depth(bsize);
    let tx_size_cat = bsize_to_tx_size_cat(bsize);

    debug_assert!(depth <= max_depths);
    debug_assert!(!tx_size.is_rect() || bsize.is_rect_tx_allowed());

    if tx_size_cat > 0 {
      let cdf = &self.fc.tx_size_cdf[tx_size_cat - 1][tx_size_ctx];
      symbol_with_update!(self, w, depth as u32, cdf);
    } else {
      let cdf = &self.fc.tx_size_8x8_cdf[tx_size_ctx];
      symbol_with_update!(self, w, depth as u32, cdf);
    }
  }

  // Based on https://aomediacodec.github.io/av1-spec/#cdf-selection-process
  // Used to decide the cdf (context) for txfm_split
  fn get_above_tx_width(
    &self, bo: TileBlockOffset, _bsize: BlockSize, _tx_size: TxSize,
    first_tx: bool,
  ) -> usize {
    let has_above = bo.0.y > 0;
    if first_tx {
      if !has_above {
        return 64;
      }
      let above_blk = self.bc.blocks.above_of(bo);
      if above_blk.skip && above_blk.is_inter() {
        return above_blk.bsize.width();
      }
    }
    self.bc.above_tx_context[bo.0.x] as usize
  }

  fn get_left_tx_height(
    &self, bo: TileBlockOffset, _bsize: BlockSize, _tx_size: TxSize,
    first_tx: bool,
  ) -> usize {
    let has_left = bo.0.x > 0;
    if first_tx {
      if !has_left {
        return 64;
      }
      let left_blk = self.bc.blocks.left_of(bo);
      if left_blk.skip && left_blk.is_inter() {
        return left_blk.bsize.height();
      }
    }
    self.bc.left_tx_context[bo.y_in_sb()] as usize
  }

  fn txfm_partition_context(
    &self, bo: TileBlockOffset, bsize: BlockSize, tx_size: TxSize, tbx: usize,
    tby: usize,
  ) -> usize {
    debug_assert!(tx_size > TX_4X4);
    debug_assert!(bsize > BlockSize::BLOCK_4X4);

    // TODO: from 2nd level partition, must know whether the tx block is the topmost(or leftmost) within a partition
    let above = (self.get_above_tx_width(bo, bsize, tx_size, tby == 0)
      < tx_size.width()) as usize;
    let left = (self.get_left_tx_height(bo, bsize, tx_size, tbx == 0)
      < tx_size.height()) as usize;

    let max_tx_size: TxSize = bsize.tx_size().sqr_up();
    let category: usize = (tx_size.sqr_up() != max_tx_size) as usize
      + (TxSize::TX_SIZES - 1 - max_tx_size as usize) * 2;

    debug_assert!(category < TXFM_PARTITION_CONTEXTS);

    category * 3 + above + left
  }

  pub fn write_tx_size_inter<W: Writer>(
    &mut self, w: &mut W, bo: TileBlockOffset, bsize: BlockSize,
    tx_size: TxSize, txfm_split: bool, tbx: usize, tby: usize, depth: usize,
  ) {
    if bo.0.x >= self.bc.blocks.cols() || bo.0.y >= self.bc.blocks.rows() {
      return;
    }
    debug_assert!(self.bc.blocks[bo].is_inter());
    debug_assert!(bsize > BlockSize::BLOCK_4X4);
    debug_assert!(!tx_size.is_rect() || bsize.is_rect_tx_allowed());

    if tx_size != TX_4X4 && depth < MAX_VARTX_DEPTH {
      let ctx = self.txfm_partition_context(bo, bsize, tx_size, tbx, tby);
      let cdf = &self.fc.txfm_partition_cdf[ctx];
      symbol_with_update!(self, w, txfm_split as u32, cdf);
    } else {
      debug_assert!(!txfm_split);
    }

    if !txfm_split {
      self.bc.update_tx_size_context(bo, tx_size.block_size(), tx_size, false);
    } else {
      // if txfm_split == true, split one level only
      let split_tx_size = sub_tx_size_map[tx_size as usize];
      let bw = bsize.width_mi() / split_tx_size.width_mi();
      let bh = bsize.height_mi() / split_tx_size.height_mi();

      for by in 0..bh {
        for bx in 0..bw {
          let tx_bo = TileBlockOffset(BlockOffset {
            x: bo.0.x + bx * split_tx_size.width_mi(),
            y: bo.0.y + by * split_tx_size.height_mi(),
          });
          self.write_tx_size_inter(
            w,
            tx_bo,
            bsize,
            split_tx_size,
            false,
            bx,
            by,
            depth + 1,
          );
        }
      }
    }
  }

  #[inline]
  pub const fn get_txsize_entropy_ctx(tx_size: TxSize) -> usize {
    (tx_size.sqr() as usize + tx_size.sqr_up() as usize + 1) >> 1
  }

  pub fn txb_init_levels<T: Coefficient>(
    &self, coeffs: &[T], height: usize, levels: &mut [u8],
    levels_stride: usize,
  ) {
    // Coefficients and levels are transposed from how they work in the spec
    for (coeffs_col, levels_col) in
      coeffs.chunks_exact(height).zip(levels.chunks_exact_mut(levels_stride))
    {
      for (coeff, level) in coeffs_col.iter().zip(levels_col) {
        *level = coeff.abs().min(T::cast_from(127)).as_();
      }
    }
  }

  // Since the coefficients and levels are transposed in relation to how they
  // work in the spec, use the log of block height in our calculations instead
  // of block width.
  #[inline]
  pub const fn get_txb_bhl(tx_size: TxSize) -> usize {
    av1_get_coded_tx_size(tx_size).height_log2()
  }

  /// Returns `(eob_pt, eob_extra)`
  ///
  /// # Panics
  ///
  /// - If `eob` is prior to the start of the group
  #[inline]
  pub fn get_eob_pos_token(eob: u16) -> (u32, u32) {
    let t = if eob < 33 {
      eob_to_pos_small[usize::from(eob)] as u32
    } else {
      let e = usize::from(cmp::min((eob - 1) >> 5, 16));
      eob_to_pos_large[e] as u32
    };
    assert!(eob as i32 >= k_eob_group_start[t as usize] as i32);
    let extra = eob as u32 - k_eob_group_start[t as usize] as u32;

    (t, extra)
  }

  pub fn get_nz_mag(levels: &[u8], bhl: usize, tx_class: TxClass) -> usize {
    // Levels are transposed from how they work in the spec

    // May version.
    // Note: AOMMIN(level, 3) is useless for decoder since level < 3.
    let mut mag = cmp::min(3, levels[1]); // { 1, 0 }
    mag += cmp::min(3, levels[(1 << bhl) + TX_PAD_HOR]); // { 0, 1 }

    if tx_class == TX_CLASS_2D {
      mag += cmp::min(3, levels[(1 << bhl) + TX_PAD_HOR + 1]); // { 1, 1 }
      mag += cmp::min(3, levels[2]); // { 2, 0 }
      mag += cmp::min(3, levels[(2 << bhl) + (2 << TX_PAD_HOR_LOG2)]); // { 0, 2 }
    } else if tx_class == TX_CLASS_VERT {
      mag += cmp::min(3, levels[2]); // { 2, 0 }
      mag += cmp::min(3, levels[3]); // { 3, 0 }
      mag += cmp::min(3, levels[4]); // { 4, 0 }
    } else {
      mag += cmp::min(3, levels[(2 << bhl) + (2 << TX_PAD_HOR_LOG2)]); // { 0, 2 }
      mag += cmp::min(3, levels[(3 << bhl) + (3 << TX_PAD_HOR_LOG2)]); // { 0, 3 }
      mag += cmp::min(3, levels[(4 << bhl) + (4 << TX_PAD_HOR_LOG2)]); // { 0, 4 }
    }

    mag as usize
  }

  fn get_nz_map_ctx_from_stats(
    stats: usize,
    coeff_idx: usize, // raster order
    bhl: usize,
    tx_size: TxSize,
    tx_class: TxClass,
  ) -> usize {
    if (tx_class as u32 | coeff_idx as u32) == 0 {
      return 0;
    };

    // Coefficients are transposed from how they work in the spec
    let col: usize = coeff_idx >> bhl;
    let row: usize = coeff_idx - (col << bhl);

    let ctx = ((stats + 1) >> 1).min(4);

    ctx
      + match tx_class {
        TX_CLASS_2D => {
          // This is the algorithm to generate table av1_nz_map_ctx_offset[].
          // const int width = tx_size_wide[tx_size];
          // const int height = tx_size_high[tx_size];
          // if (width < height) {
          //   if (row < 2) return 11 + ctx;
          // } else if (width > height) {
          //   if (col < 2) return 16 + ctx;
          // }
          // if (row + col < 2) return ctx + 1;
          // if (row + col < 4) return 5 + ctx + 1;
          // return 21 + ctx;
          av1_nz_map_ctx_offset[tx_size as usize][cmp::min(row, 4)]
            [cmp::min(col, 4)] as usize
        }
        TX_CLASS_HORIZ => nz_map_ctx_offset_1d[col],
        TX_CLASS_VERT => nz_map_ctx_offset_1d[row],
      }
  }

  fn get_nz_map_ctx(
    levels: &[u8], coeff_idx: usize, bhl: usize, area: usize, scan_idx: usize,
    is_eob: bool, tx_size: TxSize, tx_class: TxClass,
  ) -> usize {
    if is_eob {
      if scan_idx == 0 {
        return 0;
      }
      if scan_idx <= area / 8 {
        return 1;
      }
      if scan_idx <= area / 4 {
        return 2;
      }
      return 3;
    }

    // Levels are transposed from how they work in the spec
    let padded_idx = coeff_idx + ((coeff_idx >> bhl) << TX_PAD_HOR_LOG2);
    let stats = Self::get_nz_mag(&levels[padded_idx..], bhl, tx_class);

    Self::get_nz_map_ctx_from_stats(stats, coeff_idx, bhl, tx_size, tx_class)
  }

  /// `coeff_contexts_no_scan` is not in the scan order.
  /// Value for `pos = scan[i]` is at `coeff[i]`, not at `coeff[pos]`.
  pub fn get_nz_map_contexts<'c>(
    &self, levels: &mut [u8], scan: &[u16], eob: u16, tx_size: TxSize,
    tx_class: TxClass, coeff_contexts_no_scan: &'c mut [MaybeUninit<i8>],
  ) -> &'c mut [i8] {
    let bhl = Self::get_txb_bhl(tx_size);
    let area = av1_get_coded_tx_size(tx_size).area();

    let scan = &scan[..usize::from(eob)];
    let coeffs = &mut coeff_contexts_no_scan[..usize::from(eob)];
    for (i, (coeff, pos)) in
      coeffs.iter_mut().zip(scan.iter().copied()).enumerate()
    {
      coeff.write(Self::get_nz_map_ctx(
        levels,
        pos as usize,
        bhl,
        area,
        i,
        i == usize::from(eob) - 1,
        tx_size,
        tx_class,
      ) as i8);
    }
    // SAFETY: every element has been initialized
    unsafe { slice_assume_init_mut(coeffs) }
  }

  pub fn get_br_ctx(
    levels: &[u8],
    coeff_idx: usize, // raster order
    bhl: usize,
    tx_class: TxClass,
  ) -> usize {
    // Coefficients and levels are transposed from how they work in the spec
    let col: usize = coeff_idx >> bhl;
    let row: usize = coeff_idx - (col << bhl);
    let stride: usize = (1 << bhl) + TX_PAD_HOR;
    let pos: usize = col * stride + row;
    let mut mag: usize = (levels[pos + 1] + levels[pos + stride]) as usize;

    match tx_class {
      TX_CLASS_2D => {
        mag += levels[pos + stride + 1] as usize;
        mag = cmp::min((mag + 1) >> 1, 6);
        if coeff_idx == 0 {
          return mag;
        }
        if (row < 2) && (col < 2) {
          return mag + 7;
        }
      }
      TX_CLASS_HORIZ => {
        mag += levels[pos + (stride << 1)] as usize;
        mag = cmp::min((mag + 1) >> 1, 6);
        if coeff_idx == 0 {
          return mag;
        }
        if col == 0 {
          return mag + 7;
        }
      }
      TX_CLASS_VERT => {
        mag += levels[pos + 2] as usize;
        mag = cmp::min((mag + 1) >> 1, 6);
        if coeff_idx == 0 {
          return mag;
        }
        if row == 0 {
          return mag + 7;
        }
      }
    }

    mag + 14
  }
}
