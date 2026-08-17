/*
 * MPEG-4 decoder internal header.
 * Copyright (c) 2000,2001 Fabrice Bellard
 * Copyright (c) 2002-2010 Michael Niedermayer <michaelni@gmx.at>
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

#ifndef AVCODEC_MPEG4VIDEODEC_H
#define AVCODEC_MPEG4VIDEODEC_H

#include <stdint.h>

#include "get_bits.h"
#include "mpegvideo.h"
#include "mpeg4videodsp.h"

#include "libavutil/mem_internal.h"

typedef struct Mpeg4DecContext {
    MpegEncContext m;

    /// number of bits to represent the fractional part of time
    int time_increment_bits;
    int shape;
    int vol_sprite_usage;
    int sprite_brightness_change;
    int sprite_warping_accuracy;
    int num_sprite_warping_points;
    int real_sprite_warping_points;
    int sprite_offset[2][2];         ///< sprite offset[isChroma][isMVY]
    int sprite_delta[2][2];          ///< sprite_delta [isY][isMVY]
    /// sprite trajectory points
    uint16_t sprite_traj[4][2];
    /// sprite shift [isChroma]
    int sprite_shift[2];

    // reversible vlc
    int rvlc;
    /// could this stream contain resync markers
    int resync_marker;
    /// time distance of first I -> B, used for interlaced B-frames
    int t_frame;

    int new_pred;
    int enhancement_type;
    int scalability;

    int quant_precision;

    /// QP above which the ac VLC should be used for intra dc
    int intra_dc_threshold;

    /* bug workarounds */
    int divx_version;
    int divx_build;
    int xvid_build;
    int lavc_build;
    /// Divx 5.01 puts several frames in a single one, this is used to reorder them
    AVBufferRef *bitstream_buffer;

    int vo_type;

    /// flag for having shown the warning about invalid Divx B-frames
    int showed_packed_warning;
    /** does the stream contain the low_delay flag,
     *  used to work around buggy encoders. */
    int vol_control_parameters;
    int cplx_estimation_trash_i;
    int cplx_estimation_trash_p;
    int cplx_estimation_trash_b;

    int rgb;

    Mpeg4VideoDSPContext mdsp;

    DECLARE_ALIGNED(8, int32_t, block32)[12][64];
    // 0 = DCT, 1 = DPCM top to bottom scan, -1 = DPCM bottom to top scan
    int dpcm_direction;
    int16_t dpcm_macroblock[3][256];
} Mpeg4DecContext;

int ff_mpeg4_decode_picture_header(MpegEncContext *s);
int ff_mpeg4_parse_picture_header(Mpeg4DecContext *ctx, GetBitContext *gb,
                                  int header, int parse_only);
void ff_mpeg4_decode_studio(MpegEncContext *s, uint8_t *dest_y, uint8_t *dest_cb,
                            uint8_t *dest_cr, int block_size, int uvlinesize,
                            int dct_linesize, int dct_offset);
void ff_mpeg4_mcsel_motion(MpegEncContext *s,
                           uint8_t *dest_y, uint8_t *dest_cb, uint8_t *dest_cr,
                           uint8_t *const *ref_picture);
int ff_mpeg4_decode_partitions(Mpeg4DecContext *ctx);
int ff_mpeg4_decode_video_packet_header(Mpeg4DecContext *ctx);
int ff_mpeg4_decode_studio_slice_header(Mpeg4DecContext *ctx);
int ff_mpeg4_workaround_bugs(AVCodecContext *avctx);
void ff_mpeg4_pred_ac(MpegEncContext *s, int16_t *block, int n,
                      int dir);
int ff_mpeg4_frame_end(AVCodecContext *avctx, const AVPacket *pkt);


#endif
