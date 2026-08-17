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
#ifndef AOM_AV1_COMMON_CDEF_H_
#define AOM_AV1_COMMON_CDEF_H_

#define CDEF_STRENGTH_BITS 6

#define CDEF_PRI_STRENGTHS 16
#define CDEF_SEC_STRENGTHS 4

#include "config/aom_config.h"

#include "aom/aom_integer.h"
#include "aom_ports/mem.h"
#include "av1/common/av1_common_int.h"
#include "av1/common/cdef_block.h"

enum { TOP, LEFT, BOTTOM, RIGHT, BOUNDARIES } UENUM1BYTE(BOUNDARY);

struct AV1CdefSyncData;

/*!\brief Parameters related to CDEF Block */
typedef struct {
  uint16_t *src;                       /*!< CDEF intermediate buffer */
  uint16_t *top_linebuf[MAX_MB_PLANE]; /*!< CDEF top line buffer */
  uint16_t *bot_linebuf[MAX_MB_PLANE]; /*!< CDEF bottom line buffer */
  uint8_t *dst;                        /*!< CDEF destination buffer */
  cdef_list
      dlist[MI_SIZE_64X64 * MI_SIZE_64X64]; /*!< CDEF 8x8 block positions */

  int xdec;                       /*!< Sub-sampling X */
  int ydec;                       /*!< Sub-sampling X */
  int mi_wide_l2;                 /*!< Pixels per mi unit in width */
  int mi_high_l2;                 /*!< Pixels per mi unit in height */
  int frame_boundary[BOUNDARIES]; /*!< frame boundaries */

  int damping;     /*!< CDEF damping factor */
  int coeff_shift; /*!< Bit-depth based shift for calculating filter strength */
  int level;       /*!< CDEF filtering level */
  int sec_strength; /*!< CDEF secondary strength */
  int cdef_count;   /*!< Number of CDEF sub-blocks in superblock */
  int dir[CDEF_NBLOCKS]
         [CDEF_NBLOCKS]; /*!< CDEF filter direction for all 8x8 sub-blocks*/
  int var[CDEF_NBLOCKS][CDEF_NBLOCKS]; /*!< variance for all 8x8 sub-blocks */

  int dst_stride; /*!< CDEF destination buffer stride */
  int coffset;    /*!< current superblock offset in a row */
  int roffset;    /*!< current row offset */
} CdefBlockInfo;

static inline int sign(int i) { return i < 0 ? -1 : 1; }

static inline int constrain(int diff, int threshold, int damping) {
  if (!threshold) return 0;

  const int shift = AOMMAX(0, damping - get_msb(threshold));
  return sign(diff) *
         AOMMIN(abs(diff), AOMMAX(0, threshold - (abs(diff) >> shift)));
}

#ifdef __cplusplus
extern "C" {
#endif

int av1_cdef_compute_sb_list(const CommonModeInfoParams *const mi_params,
                             int mi_row, int mi_col, cdef_list *dlist,
                             BLOCK_SIZE bsize);

typedef void (*cdef_init_fb_row_t)(
    const AV1_COMMON *const cm, const MACROBLOCKD *const xd,
    CdefBlockInfo *const fb_info, uint16_t **const linebuf, uint16_t *const src,
    struct AV1CdefSyncData *const cdef_sync, int fbr);

/*!\brief Function for applying CDEF to a frame
 *
 * \ingroup in_loop_cdef
 * This function applies CDEF to a frame.
 *
 * \param[in, out]  frame     Compressed frame buffer
 * \param[in, out]  cm        Pointer to top level common structure
 * \param[in]       xd        Pointer to common current coding block structure
 * \param[in]       cdef_init_fb_row_fn   Function Pointer
 *
 * \remark Nothing is returned. Instead, the filtered frame is output in
 * \c frame.
 */
void av1_cdef_frame(YV12_BUFFER_CONFIG *frame, AV1_COMMON *const cm,
                    MACROBLOCKD *xd, cdef_init_fb_row_t cdef_init_fb_row_fn);
void av1_cdef_fb_row(const AV1_COMMON *const cm, MACROBLOCKD *xd,
                     uint16_t **const linebuf, uint16_t **const colbuf,
                     uint16_t *const src, int fbr,
                     cdef_init_fb_row_t cdef_init_fb_row_fn,
                     struct AV1CdefSyncData *const cdef_sync,
                     struct aom_internal_error_info *error_info);
void av1_cdef_init_fb_row(const AV1_COMMON *const cm,
                          const MACROBLOCKD *const xd,
                          CdefBlockInfo *const fb_info,
                          uint16_t **const linebuf, uint16_t *const src,
                          struct AV1CdefSyncData *const cdef_sync, int fbr);

#ifdef __cplusplus
}  // extern "C"
#endif
#endif  // AOM_AV1_COMMON_CDEF_H_
