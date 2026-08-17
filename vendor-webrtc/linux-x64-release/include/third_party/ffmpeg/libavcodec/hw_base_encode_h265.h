/*
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

#ifndef AVCODEC_HW_BASE_ENCODE_H265_H
#define AVCODEC_HW_BASE_ENCODE_H265_H

#include "hw_base_encode.h"
#include "cbs_h265.h"

typedef struct FFHWBaseEncodeH265 {
    H265RawVPS raw_vps;
    H265RawSPS raw_sps;
    H265RawPPS raw_pps;

    int dpb_frames;
} FFHWBaseEncodeH265;

typedef struct FFHWBaseEncodeH265Opts {
    int tier;
    int fixed_qp_idr;
    int cu_qp_delta_enabled_flag;

    int tile_rows;
    int tile_cols;

    int nb_slices;
    int slice_block_rows;
    int slice_block_cols;

    // Tile width of the i-th column.
    int col_width[22];
    // Tile height of i-th row.
    int row_height[22];
} FFHWBaseEncodeH265Opts;

int ff_hw_base_encode_init_params_h265(FFHWBaseEncodeContext *base_ctx,
                                       AVCodecContext *avctx,
                                       FFHWBaseEncodeH265 *common,
                                       FFHWBaseEncodeH265Opts *opts);

#endif /* AVCODEC_HW_BASE_ENCODE_H265_H */
