/*
 * H.263 encoder header
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
#ifndef AVCODEC_H263ENC_H
#define AVCODEC_H263ENC_H

#include <stdint.h>
#include "h263data.h"
#include "mpegvideoenc.h"

const uint8_t (*ff_h263_get_mv_penalty(void))[MAX_DMV*2+1];

void ff_h263_encode_init(MpegEncContext *s);
void ff_h263_encode_picture_header(MpegEncContext *s);
void ff_h263_encode_gob_header(MpegEncContext * s, int mb_line);
void ff_h263_encode_mb(MpegEncContext *s,
                       int16_t block[6][64],
                       int motion_x, int motion_y);
void ff_h263_encode_mba(MpegEncContext *s);

void ff_clean_h263_qscales(MpegEncContext *s);

void ff_h263_encode_motion(PutBitContext *pb, int val, int f_code);
void ff_h263_update_mb(MpegEncContext *s);

static inline void ff_h263_encode_motion_vector(MpegEncContext * s,
                                                int x, int y, int f_code)
{
    ff_h263_encode_motion(&s->pb, x, f_code);
    ff_h263_encode_motion(&s->pb, y, f_code);
}

static inline int get_p_cbp(MpegEncContext * s,
                      int16_t block[6][64],
                      int motion_x, int motion_y){
    int cbp;

    if (s->mpv_flags & FF_MPV_FLAG_CBP_RD) {
        int best_cbpy_score = INT_MAX;
        int best_cbpc_score = INT_MAX;
        int cbpc = (-1), cbpy = (-1);
        const int offset = (s->mv_type == MV_TYPE_16X16 ? 0 : 16) + (s->dquant ? 8 : 0);
        const int lambda = s->lambda2 >> (FF_LAMBDA_SHIFT - 6);

        for (int i = 0; i < 4; i++) {
            int score = ff_h263_inter_MCBPC_bits[i + offset] * lambda;
            if (i & 1) score += s->coded_score[5];
            if (i & 2) score += s->coded_score[4];

            if (score < best_cbpc_score) {
                best_cbpc_score = score;
                cbpc = i;
            }
        }

        for (int i = 0; i < 16; i++) {
            int score= ff_h263_cbpy_tab[i ^ 0xF][1] * lambda;
            if (i & 1) score += s->coded_score[3];
            if (i & 2) score += s->coded_score[2];
            if (i & 4) score += s->coded_score[1];
            if (i & 8) score += s->coded_score[0];

            if (score < best_cbpy_score) {
                best_cbpy_score = score;
                cbpy = i;
            }
        }
        cbp = cbpc + 4 * cbpy;
        if (!(motion_x | motion_y | s->dquant) && s->mv_type == MV_TYPE_16X16) {
            if (best_cbpy_score + best_cbpc_score + 2 * lambda >= 0)
                cbp= 0;
        }

        for (int i = 0; i < 6; i++) {
            if (s->block_last_index[i] >= 0 && !((cbp >> (5 - i)) & 1)) {
                s->block_last_index[i] = -1;
                s->bdsp.clear_block(s->block[i]);
            }
        }
    } else {
        cbp = 0;
        for (int i = 0; i < 6; i++) {
            if (s->block_last_index[i] >= 0)
                cbp |= 1 << (5 - i);
        }
    }
    return cbp;
}

#endif
