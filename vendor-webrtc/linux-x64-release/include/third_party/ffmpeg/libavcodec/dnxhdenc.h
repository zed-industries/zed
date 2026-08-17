/*
 * VC3/DNxHD encoder structure definitions and prototypes
 * Copyright (c) 2007 Baptiste Coudurier <baptiste dot coudurier at smartjog dot com>
 *
 * VC-3 encoder funded by the British Broadcasting Corporation
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

#ifndef AVCODEC_DNXHDENC_H
#define AVCODEC_DNXHDENC_H

#include <stdint.h>

#include "libavutil/mem_internal.h"

#include "mpegvideo.h"
#include "dnxhddata.h"

typedef struct RCCMPEntry {
    uint32_t mb;
    int value;
} RCCMPEntry;

typedef struct RCEntry {
    int ssd;
    int bits;
} RCEntry;

typedef struct DNXHDEncContext {
    AVClass *class;
    MpegEncContext m; ///< Used for quantization dsp functions

    int cid;
    int profile;
    int bit_depth;
    int is_444;
    const CIDEntry *cid_table;
    uint8_t *msip; ///< Macroblock Scan Indexes Payload
    uint32_t *slice_size;
    uint32_t *slice_offs;

    struct DNXHDEncContext *thread[MAX_THREADS];

    // Because our samples are either 8 or 16 bits for 8-bit and 10-bit
    // encoding respectively, these refer either to bytes or to two-byte words.
    unsigned dct_y_offset;
    unsigned dct_uv_offset;
    unsigned block_width_l2;

    int frame_size;
    int coding_unit_size;
    int data_offset;

    int interlaced;
    int cur_field;

    int nitris_compat;
    unsigned min_padding;
    int intra_quant_bias;

    DECLARE_ALIGNED(32, int16_t, blocks)[12][64];
    DECLARE_ALIGNED(16, uint8_t, edge_buf_y)[512]; // has to hold 16x16 uint16 when depth=10
    DECLARE_ALIGNED(16, uint8_t, edge_buf_uv)[2][512]; // has to hold 16x16 uint16_t when depth=10

    int      (*qmatrix_c)     [64];
    int      (*qmatrix_l)     [64];
    uint16_t (*qmatrix_l16)[2][64];
    uint16_t (*qmatrix_c16)[2][64];

    unsigned frame_bits;
    const uint8_t *src[3];

    uint32_t *orig_vlc_codes;
    uint8_t  *orig_vlc_bits;
    uint32_t *vlc_codes;
    uint8_t  *vlc_bits;
    uint16_t *run_codes;
    uint8_t  *run_bits;

    /** Rate control */
    unsigned slice_bits;
    unsigned qscale;
    unsigned lambda;

    uint32_t *mb_bits;
    uint8_t  *mb_qscale;

    RCCMPEntry *mb_cmp;
    RCCMPEntry *mb_cmp_tmp;
    RCEntry    *mb_rc;

    void (*get_pixels_8x4_sym)(int16_t *restrict /* align 16 */ block,
                               const uint8_t *pixels, ptrdiff_t line_size);
} DNXHDEncContext;

void ff_dnxhdenc_init(DNXHDEncContext *ctx);
void ff_dnxhdenc_init_x86(DNXHDEncContext *ctx);

#endif /* AVCODEC_DNXHDENC_H */
