/*
 * AV1 video decoder
 * *
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

#ifndef AVCODEC_AV1DEC_H
#define AVCODEC_AV1DEC_H

#include <stdint.h>

#include "libavutil/buffer.h"
#include "libavutil/fifo.h"
#include "libavutil/frame.h"
#include "libavutil/pixfmt.h"
#include "avcodec.h"
#include "packet.h"
#include "cbs.h"
#include "cbs_av1.h"
#include "dovi_rpu.h"
#include "progressframe.h"

typedef struct AV1Frame {
    union {
        struct {
            struct AVFrame *f;
        };
        ProgressFrame pf;
    };

    void *hwaccel_picture_private; ///< RefStruct reference

    AV1RawOBU *header_ref; ///< RefStruct reference backing raw_frame_header.
    AV1RawFrameHeader *raw_frame_header;

    int temporal_id;
    int spatial_id;

    uint8_t gm_invalid[AV1_NUM_REF_FRAMES];
    uint8_t gm_type[AV1_NUM_REF_FRAMES];
    int32_t gm_params[AV1_NUM_REF_FRAMES][6];

    uint8_t skip_mode_frame_idx[2];

    AV1RawFilmGrainParams film_grain;

    uint8_t coded_lossless;

    // OrderHint for this frame.
    uint8_t order_hint;
    // RefFrameSignBias[] used when decoding this frame.
    uint8_t ref_frame_sign_bias[AV1_TOTAL_REFS_PER_FRAME];
    // OrderHints[] when this is the current frame, otherwise
    // SavedOrderHints[s][] when is the reference frame in slot s.
    uint8_t order_hints[AV1_TOTAL_REFS_PER_FRAME];

    // force_integer_mv value at the end of the frame header parsing.
    // This is not the same as the syntax element value in
    // raw_frame_header because the specification parsing tables
    // override the value on intra frames.
    uint8_t force_integer_mv;
} AV1Frame;

typedef struct TileGroupInfo {
    uint32_t tile_offset;
    uint32_t tile_size;
    uint16_t tile_row;
    uint16_t tile_column;
} TileGroupInfo;

typedef struct AV1DecContext {
    const AVClass *class;
    AVCodecContext *avctx;

    enum AVPixelFormat pix_fmt;
    CodedBitstreamContext *cbc;
    CodedBitstreamFragment current_obu;
    AVPacket *pkt;

    AVBufferRef *seq_data_ref;
    AV1RawOBU *seq_ref;    ///< RefStruct reference backing raw_seq
    AV1RawSequenceHeader *raw_seq;
    AV1RawOBU *header_ref; ///< RefStruct reference backing raw_frame_header
    AV1RawFrameHeader *raw_frame_header;
    TileGroupInfo *tile_group_info;

    AV1RawOBU *cll_ref;    ///< RefStruct reference backing cll
    AV1RawMetadataHDRCLL *cll;
    AV1RawOBU *mdcv_ref;   ///< RefStruct reference backing mdcv
    AV1RawMetadataHDRMDCV *mdcv;
    DOVIContext dovi;
    AVFifo *itut_t35_fifo;

    uint16_t tile_num;
    uint16_t tg_start;
    uint16_t tg_end;

    int operating_point_idc;

    AV1Frame ref[AV1_NUM_REF_FRAMES];
    AV1Frame cur_frame;

    int nb_unit;           ///< The index of the next OBU to be processed.
    int start_unit;        ///< The index of the first OBU of the current frame.

    // AVOptions
    int operating_point;
} AV1DecContext;

#endif /* AVCODEC_AV1DEC_H */
