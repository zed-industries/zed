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

#ifndef AVCODEC_H2645_VUI_H
#define AVCODEC_H2645_VUI_H

#include "libavutil/pixfmt.h"
#include "libavutil/rational.h"

#include "get_bits.h"

typedef struct H2645VUI {
    AVRational sar;
    int aspect_ratio_idc;
    int aspect_ratio_info_present_flag;

    int overscan_info_present_flag;
    int overscan_appropriate_flag;

    int video_signal_type_present_flag;
    int video_format;
    int video_full_range_flag;
    int colour_description_present_flag;
    enum AVColorPrimaries colour_primaries;
    enum AVColorTransferCharacteristic transfer_characteristics;
    enum AVColorSpace matrix_coeffs;

    int chroma_loc_info_present_flag;
    int chroma_sample_loc_type_top_field;
    int chroma_sample_loc_type_bottom_field;
    enum AVChromaLocation chroma_location;
} H2645VUI;

void ff_h2645_decode_common_vui_params(GetBitContext *gb, H2645VUI *vui, void *logctx);

#endif /* AVCODEC_H2645_VUI_H */
