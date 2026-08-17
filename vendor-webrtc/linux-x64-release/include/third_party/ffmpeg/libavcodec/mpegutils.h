/*
 * Mpeg video formats-related defines and utility functions
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_MPEGUTILS_H
#define AVCODEC_MPEGUTILS_H

#include <stdint.h>

#include "libavutil/frame.h"

#include "avcodec.h"

/* picture type */
#define PICT_TOP_FIELD     1
#define PICT_BOTTOM_FIELD  2
#define PICT_FRAME         3

#define MAX_MB_BYTES    (30 * 16 * 16 * 3 / 8 + 120)

/* MB types */
#define MB_TYPE_INTRA4x4   (1 <<  0)
#define MB_TYPE_INTRA16x16 (1 <<  1) // FIXME H.264-specific
#define MB_TYPE_INTRA_PCM  (1 <<  2) // FIXME H.264-specific
#define MB_TYPE_16x16      (1 <<  3)
#define MB_TYPE_16x8       (1 <<  4)
#define MB_TYPE_8x16       (1 <<  5)
#define MB_TYPE_8x8        (1 <<  6)
#define MB_TYPE_INTERLACED (1 <<  7)
#define MB_TYPE_DIRECT2    (1 <<  8) // FIXME
#define MB_TYPE_CBP        (1 << 10)
#define MB_TYPE_QUANT      (1 << 11)
#define MB_TYPE_FORWARD_MV (1 << 12)
#define MB_TYPE_BACKWARD_MV (1 << 13)
#define MB_TYPE_BIDIR_MV   (MB_TYPE_FORWARD_MV | MB_TYPE_BACKWARD_MV)
// MB_TYPE_P[01]L[01], MB_TYPE_L[01] and MB_TYPE_L0L1 are H.264 only.
#define MB_TYPE_P0L0       (1 << 12)
#define MB_TYPE_P1L0       (1 << 13)
#define MB_TYPE_P0L1       (1 << 14)
#define MB_TYPE_P1L1       (1 << 15)
#define MB_TYPE_L0         (MB_TYPE_P0L0 | MB_TYPE_P1L0)
#define MB_TYPE_L1         (MB_TYPE_P0L1 | MB_TYPE_P1L1)
#define MB_TYPE_L0L1       (MB_TYPE_L0   | MB_TYPE_L1)
#define MB_TYPE_GMC        (1 << 16)
#define MB_TYPE_SKIP       (1 << 17)
#define MB_TYPE_ACPRED     (1 << 18)

#define MB_TYPE_INTRA    MB_TYPE_INTRA4x4 // default mb_type if there is just one type

// The following MB-type can be used by each codec as it sees fit.
#define MB_TYPE_CODEC_SPECIFIC  (1 << 9)

#define IS_INTRA4x4(a)   ((a) & MB_TYPE_INTRA4x4)
#define IS_INTRA16x16(a) ((a) & MB_TYPE_INTRA16x16)
#define IS_PCM(a)        ((a) & MB_TYPE_INTRA_PCM)
#define IS_INTRA(a)      ((a) & 7)
#define IS_INTER(a)      ((a) & (MB_TYPE_16x16 | MB_TYPE_16x8 | \
                                 MB_TYPE_8x16  | MB_TYPE_8x8))
#define IS_SKIP(a)       ((a) & MB_TYPE_SKIP)
#define IS_INTRA_PCM(a)  ((a) & MB_TYPE_INTRA_PCM)
#define IS_INTERLACED(a) ((a) & MB_TYPE_INTERLACED)
#define IS_DIRECT(a)     ((a) & MB_TYPE_DIRECT2)
#define IS_GMC(a)        ((a) & MB_TYPE_GMC)
#define IS_16X16(a)      ((a) & MB_TYPE_16x16)
#define IS_16X8(a)       ((a) & MB_TYPE_16x8)
#define IS_8X16(a)       ((a) & MB_TYPE_8x16)
#define IS_8X8(a)        ((a) & MB_TYPE_8x8)
#define IS_ACPRED(a)     ((a) & MB_TYPE_ACPRED)
#define IS_QUANT(a)      ((a) & MB_TYPE_QUANT)

#define HAS_CBP(a)       ((a) & MB_TYPE_CBP)
#define HAS_FORWARD_MV(a)  ((a) & MB_TYPE_FORWARD_MV)
#define HAS_BACKWARD_MV(a) ((a) & MB_TYPE_BACKWARD_MV)
// dir == 0 means forward, dir == 1 is backward
#define HAS_MV(a, dir)     ((a) & (MB_TYPE_FORWARD_MV << (dir)))

#define MB_TYPE_MV_2_MV_DIR(a) (((a) >> 12) & (MV_DIR_FORWARD | MV_DIR_BACKWARD))

/**
 * Draw a horizontal band if supported.
 *
 * @param h is the normal height, this will be reduced automatically if needed
 */
void ff_draw_horiz_band(AVCodecContext *avctx, const AVFrame *cur, const AVFrame *last,
                        int y, int h, int picture_structure, int first_field,
                        int low_delay);

/**
 * Print debugging info for the given picture.
 */
void ff_print_debug_info2(AVCodecContext *avctx, AVFrame *pict,
                          const uint32_t *mbtype_table,
                          const int8_t *qscale_table, int16_t (*const motion_val[2])[2],
                          int mb_width, int mb_height, int mb_stride, int quarter_sample);

#endif /* AVCODEC_MPEGUTILS_H */
