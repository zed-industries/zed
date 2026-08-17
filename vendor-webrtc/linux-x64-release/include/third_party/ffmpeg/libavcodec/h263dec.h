/*
 * H.263 decoder internal header
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
#ifndef AVCODEC_H263DEC_H
#define AVCODEC_H263DEC_H

#include "mpegvideo.h"
#include "vlc.h"

/**
 * Return value for header parsers if frame is not coded.
 * */
#define FRAME_SKIPPED 100

// The defines below define the number of bits that are read at once for
// reading vlc values. Changing these may improve speed and data cache needs
// be aware though that decreasing them may need the number of stages that is
// passed to get_vlc* to be increased.
#define H263_MV_VLC_BITS     9
#define INTRA_MCBPC_VLC_BITS 6
#define INTER_MCBPC_VLC_BITS 7
#define CBPY_VLC_BITS 6
#define TEX_VLC_BITS 9

extern VLCElem ff_h263_intra_MCBPC_vlc[];
extern VLCElem ff_h263_inter_MCBPC_vlc[];
extern VLCElem ff_h263_cbpy_vlc[];
extern VLCElem ff_h263_mv_vlc[];

int ff_h263_decode_motion(MpegEncContext * s, int pred, int f_code);
int ff_h263_decode_init(AVCodecContext *avctx);
int ff_h263_decode_frame(AVCodecContext *avctx, AVFrame *frame,
                         int *got_frame, AVPacket *avpkt);
void ff_h263_decode_init_vlc(void);
int ff_h263_decode_picture_header(MpegEncContext *s);
int ff_h263_decode_gob_header(MpegEncContext *s);
int ff_h263_decode_mba(MpegEncContext *s);

/**
 * Print picture info if FF_DEBUG_PICT_INFO is set.
 */
void ff_h263_show_pict_info(MpegEncContext *s);

int ff_intel_h263_decode_picture_header(MpegEncContext *s);
int ff_h263_decode_mb(MpegEncContext *s,
                      int16_t block[6][64]);

int ff_h263_resync(MpegEncContext *s);

#endif
