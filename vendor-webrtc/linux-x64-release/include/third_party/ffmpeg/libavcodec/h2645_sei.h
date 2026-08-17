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

#ifndef AVCODEC_H2645_SEI_H
#define AVCODEC_H2645_SEI_H

#include <stdint.h>

#include "libavutil/buffer.h"
#include "libavutil/frame.h"
#include "libavutil/film_grain_params.h"

#include "aom_film_grain.h"
#include "avcodec.h"
#include "bytestream.h"
#include "codec_id.h"
#include "get_bits.h"
#include "h2645_vui.h"
#include "sei.h"

typedef struct H2645SEIA53Caption {
    AVBufferRef *buf_ref;
} H2645SEIA53Caption;

typedef struct H2645SEIAFD {
    int present;
    uint8_t active_format_description;
} H2645SEIAFD;

typedef struct HEVCSEIDynamicHDRPlus {
    AVBufferRef *info;
} HEVCSEIDynamicHDRPlus;

typedef struct HEVCSEIDynamicHDRVivid {
    AVBufferRef *info;
} HEVCSEIDynamicHDRVivid;

typedef struct HEVCSEILCEVC {
    AVBufferRef *info;
} HEVCSEILCEVC;

typedef struct H2645SEIUnregistered {
    AVBufferRef **buf_ref;
    unsigned nb_buf_ref;
    int x264_build;           //< H.264 only
} H2645SEIUnregistered;

typedef struct H2645SEIFramePacking {
    int present;
    int arrangement_id;
    int arrangement_cancel_flag;  ///< is previous arrangement canceled, -1 if never received (currently H.264 only)
    SEIFpaType arrangement_type;
    int arrangement_repetition_period;
    int content_interpretation_type;
    int quincunx_sampling_flag;
    int current_frame_is_frame0_flag;
} H2645SEIFramePacking;

typedef struct H2645SEIDisplayOrientation {
    int present;
    int anticlockwise_rotation;
    int hflip, vflip;
} H2645SEIDisplayOrientation;

typedef struct H2645SEIAlternativeTransfer {
    int present;
    int preferred_transfer_characteristics;
} H2645SEIAlternativeTransfer;

typedef struct H2645SEIAmbientViewingEnvironment {
    int present;
    uint32_t ambient_illuminance;
    uint16_t ambient_light_x;
    uint16_t ambient_light_y;
} H2645SEIAmbientViewingEnvironment;

typedef struct H2645SEIFilmGrainCharacteristics {
    int present;
    int model_id;
    int separate_colour_description_present_flag;
    int bit_depth_luma;
    int bit_depth_chroma;
    int full_range;
    int color_primaries;
    int transfer_characteristics;
    int matrix_coeffs;
    int blending_mode_id;
    int log2_scale_factor;
    int comp_model_present_flag[3];
    uint16_t num_intensity_intervals[3];
    uint8_t num_model_values[3];
    uint8_t intensity_interval_lower_bound[3][256];
    uint8_t intensity_interval_upper_bound[3][256];
    int16_t comp_model_value[3][256][6];
    int repetition_period;       //< H.264 only
    int persistence_flag;        //< HEVC  only
} H2645SEIFilmGrainCharacteristics;

typedef struct H2645SEIMasteringDisplay {
    int present;
    uint16_t display_primaries[3][2];
    uint16_t white_point[2];
    uint32_t max_luminance;
    uint32_t min_luminance;
} H2645SEIMasteringDisplay;

typedef struct H2645SEIContentLight {
    int present;
    uint16_t max_content_light_level;
    uint16_t max_pic_average_light_level;
} H2645SEIContentLight;

typedef struct H2645SEI {
    H2645SEIA53Caption a53_caption;
    H2645SEIAFD afd;
    HEVCSEIDynamicHDRPlus  dynamic_hdr_plus;     //< HEVC only
    HEVCSEIDynamicHDRVivid dynamic_hdr_vivid;    //< HEVC only
    HEVCSEILCEVC lcevc;
    H2645SEIUnregistered unregistered;
    H2645SEIFramePacking frame_packing;
    H2645SEIDisplayOrientation display_orientation;
    H2645SEIAlternativeTransfer alternative_transfer;
    H2645SEIAmbientViewingEnvironment ambient_viewing_environment;
    H2645SEIMasteringDisplay mastering_display;
    H2645SEIContentLight content_light;
    AVFilmGrainAFGS1Params aom_film_grain;

    // Dynamic allocations due to large size.
    H2645SEIFilmGrainCharacteristics *film_grain_characteristics;
} H2645SEI;

enum {
    FF_H2645_SEI_MESSAGE_HANDLED = 0,
    FF_H2645_SEI_MESSAGE_UNHANDLED,
};

/**
 * Decode a single SEI message.
 *
 * This function may either use gb or gbyte to decode the SEI message.
 *
 * @param[in, out] gb    GetBitContext that needs to be at the start
 *                       of the payload (i.e. after the payload_size bytes);
 *                       it needs to be initially byte-aligned
 * @param[in, out] gbyte a GetByteContext for the same data as gb
 * @return < 0 on error, FF_H2645_SEI_MESSAGE_HANDLED if the SEI message
 *         has been handled or FF_H2645_SEI_MESSAGE_UNHANDLED if not.
 */
int ff_h2645_sei_message_decode(H2645SEI *h, enum SEIType type,
                                enum AVCodecID codec_id, GetBitContext *gb,
                                GetByteContext *gbyte, void *logctx);

int ff_h2645_sei_ctx_replace(H2645SEI *dst, const H2645SEI *src);

void ff_h2645_sei_reset(H2645SEI *s);

int ff_h2645_sei_to_frame(AVFrame *frame, H2645SEI *sei,
                          enum AVCodecID codec_id,
                          AVCodecContext *avctx, const H2645VUI *vui,
                          unsigned bit_depth_luma, unsigned bit_depth_chroma,
                          int seed);

int ff_h2645_sei_to_context(AVCodecContext *avctx, H2645SEI *sei);

#endif /* AVCODEC_H2645_SEI_H */
