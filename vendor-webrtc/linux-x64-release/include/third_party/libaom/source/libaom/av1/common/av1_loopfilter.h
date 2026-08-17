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

#ifndef AOM_AV1_COMMON_AV1_LOOPFILTER_H_
#define AOM_AV1_COMMON_AV1_LOOPFILTER_H_

#include "config/aom_config.h"

#include "aom/internal/aom_codec_internal.h"

#include "aom_ports/mem.h"
#include "av1/common/blockd.h"
#include "av1/common/seg_common.h"

#ifdef __cplusplus
extern "C" {
#endif

#define MAX_LOOP_FILTER 63
#define MAX_SHARPNESS 7

#define SIMD_WIDTH 16

enum lf_path {
  LF_PATH_420,
  LF_PATH_444,
  LF_PATH_SLOW,
};

/*!\cond */
enum { VERT_EDGE = 0, HORZ_EDGE = 1, NUM_EDGE_DIRS } UENUM1BYTE(EDGE_DIR);
typedef struct {
  uint64_t bits[4];
} FilterMask;

struct loopfilter {
  int filter_level[2];
  int filter_level_u;
  int filter_level_v;

  int backup_filter_level[2];
  int backup_filter_level_u;
  int backup_filter_level_v;

  int sharpness_level;

  uint8_t mode_ref_delta_enabled;
  uint8_t mode_ref_delta_update;

  // 0 = Intra, Last, Last2+Last3,
  // GF, BRF, ARF2, ARF
  int8_t ref_deltas[REF_FRAMES];

  // 0 = ZERO_MV, MV
  int8_t mode_deltas[MAX_MODE_LF_DELTAS];
};

// Need to align this structure so when it is declared and
// passed it can be loaded into vector registers.
typedef struct {
  DECLARE_ALIGNED(SIMD_WIDTH, uint8_t, mblim[SIMD_WIDTH]);
  DECLARE_ALIGNED(SIMD_WIDTH, uint8_t, lim[SIMD_WIDTH]);
  DECLARE_ALIGNED(SIMD_WIDTH, uint8_t, hev_thr[SIMD_WIDTH]);
} loop_filter_thresh;

typedef struct {
  loop_filter_thresh lfthr[MAX_LOOP_FILTER + 1];
  uint8_t lvl[MAX_MB_PLANE][MAX_SEGMENTS][2][REF_FRAMES][MAX_MODE_LF_DELTAS];
} loop_filter_info_n;

typedef struct AV1_DEBLOCKING_PARAMETERS {
  // length of the filter applied to the outer edge
  uint8_t filter_length;
  // deblocking limits
  const loop_filter_thresh *lfthr;
} AV1_DEBLOCKING_PARAMETERS;

typedef struct LoopFilterWorkerData {
  YV12_BUFFER_CONFIG *frame_buffer;
  struct AV1Common *cm;
  struct macroblockd_plane planes[MAX_MB_PLANE];
  // TODO(Ranjit): When the filter functions are modified to use xd->lossless
  // add lossless as a member here.
  MACROBLOCKD *xd;

  AV1_DEBLOCKING_PARAMETERS params_buf[MAX_MIB_SIZE];
  TX_SIZE tx_buf[MAX_MIB_SIZE];
  struct aom_internal_error_info error_info;
} LFWorkerData;
/*!\endcond */

/* assorted loopfilter functions which get used elsewhere */
struct AV1Common;
struct macroblockd;
struct AV1LfSyncData;

void av1_loop_filter_init(struct AV1Common *cm);

void av1_loop_filter_frame_init(struct AV1Common *cm, int plane_start,
                                int plane_end);

void av1_filter_block_plane_vert(const struct AV1Common *const cm,
                                 const MACROBLOCKD *const xd, const int plane,
                                 const MACROBLOCKD_PLANE *const plane_ptr,
                                 const uint32_t mi_row, const uint32_t mi_col);

void av1_filter_block_plane_horz(const struct AV1Common *const cm,
                                 const MACROBLOCKD *const xd, const int plane,
                                 const MACROBLOCKD_PLANE *const plane_ptr,
                                 const uint32_t mi_row, const uint32_t mi_col);

void av1_filter_block_plane_vert_opt(
    const struct AV1Common *const cm, const MACROBLOCKD *const xd,
    const MACROBLOCKD_PLANE *const plane_ptr, const uint32_t mi_row,
    const uint32_t mi_col, AV1_DEBLOCKING_PARAMETERS *params_buf,
    TX_SIZE *tx_buf, int num_mis_in_lpf_unit_height_log2);

void av1_filter_block_plane_vert_opt_chroma(
    const struct AV1Common *const cm, const MACROBLOCKD *const xd,
    const MACROBLOCKD_PLANE *const plane_ptr, const uint32_t mi_row,
    const uint32_t mi_col, AV1_DEBLOCKING_PARAMETERS *params_buf,
    TX_SIZE *tx_buf, int plane, bool joint_filter_chroma,
    int num_mis_in_lpf_unit_height_log2);

void av1_filter_block_plane_horz_opt(
    const struct AV1Common *const cm, const MACROBLOCKD *const xd,
    const MACROBLOCKD_PLANE *const plane_ptr, const uint32_t mi_row,
    const uint32_t mi_col, AV1_DEBLOCKING_PARAMETERS *params_buf,
    TX_SIZE *tx_buf, int num_mis_in_lpf_unit_height_log2);

void av1_filter_block_plane_horz_opt_chroma(
    const struct AV1Common *const cm, const MACROBLOCKD *const xd,
    const MACROBLOCKD_PLANE *const plane_ptr, const uint32_t mi_row,
    const uint32_t mi_col, AV1_DEBLOCKING_PARAMETERS *params_buf,
    TX_SIZE *tx_buf, int plane, bool joint_filter_chroma,
    int num_mis_in_lpf_unit_height_log2);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_COMMON_AV1_LOOPFILTER_H_
