/*
 * MJPEG decoder
 * Copyright (c) 2000, 2001 Fabrice Bellard
 * Copyright (c) 2003 Alex Beregszaszi
 * Copyright (c) 2003-2004 Michael Niedermayer
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

/**
 * @file
 * MJPEG decoder.
 */

#ifndef AVCODEC_MJPEGDEC_H
#define AVCODEC_MJPEGDEC_H

#include "libavutil/log.h"
#include "libavutil/mem_internal.h"
#include "libavutil/pixdesc.h"
#include "libavutil/stereo3d.h"

#include "avcodec.h"
#include "blockdsp.h"
#include "get_bits.h"
#include "hpeldsp.h"
#include "idctdsp.h"

#undef near /* This file uses struct member 'near' which in windows.h is defined as empty. */

#define MAX_COMPONENTS 4

typedef struct ICCEntry {
    uint8_t *data;
    int    length;
} ICCEntry;

struct JLSState;

typedef struct MJpegDecodeContext {
    AVClass *class;
    AVCodecContext *avctx;
    GetBitContext gb;
    int buf_size;

    int start_code; /* current start code */
    int buffer_size;
    uint8_t *buffer;

    uint16_t quant_matrixes[4][64];
    VLC vlcs[3][4];
    int qscale[4];      ///< quantizer scale calculated from quant_matrixes

    int orig_height;  /* size given at codec init */
    int first_picture;    /* true if decoding first picture */
    int interlaced;     /* true if interlaced */
    int bottom_field;   /* true if bottom field */
    int lossless;
    int ls;
    int progressive;
    int bayer;          /* true if it's a bayer-encoded JPEG embedded in a DNG */
    int rgb;
    uint8_t upscale_h[4];
    uint8_t upscale_v[4];
    int rct;            /* standard rct */
    int pegasus_rct;    /* pegasus reversible colorspace transform */
    int bits;           /* bits per component */
    int colr;
    int xfrm;
    int adobe_transform;

    int maxval;
    int near;         ///< near lossless bound (si 0 for lossless)
    int t1,t2,t3;
    int reset;        ///< context halfing interval ?rename

    int width, height;
    int mb_width, mb_height;
    int nb_components;
    int block_stride[MAX_COMPONENTS];
    int component_id[MAX_COMPONENTS];
    int h_count[MAX_COMPONENTS]; /* horizontal and vertical count for each component */
    int v_count[MAX_COMPONENTS];
    int comp_index[MAX_COMPONENTS];
    int dc_index[MAX_COMPONENTS];
    int ac_index[MAX_COMPONENTS];
    int nb_blocks[MAX_COMPONENTS];
    int h_scount[MAX_COMPONENTS];
    int v_scount[MAX_COMPONENTS];
    int quant_sindex[MAX_COMPONENTS];
    int h_max, v_max; /* maximum h and v counts */
    int quant_index[4];   /* quant table index for each component */
    int last_dc[MAX_COMPONENTS]; /* last DEQUANTIZED dc (XXX: am I right to do that ?) */
    AVFrame *picture; /* picture structure */
    AVFrame *picture_ptr; /* pointer to picture structure */
    int got_picture;                                ///< we found a SOF and picture is valid, too.
    int linesize[MAX_COMPONENTS];                   ///< linesize << interlaced
    int8_t *qscale_table;
    DECLARE_ALIGNED(32, int16_t, block)[64];
    int16_t (*blocks[MAX_COMPONENTS])[64]; ///< intermediate sums (progressive mode)
    uint8_t *last_nnz[MAX_COMPONENTS];
    uint64_t coefs_finished[MAX_COMPONENTS]; ///< bitmask of which coefs have been completely decoded (progressive mode)
    int palette_index;
    int force_pal8;
    uint8_t permutated_scantable[64];
    BlockDSPContext bdsp;
    HpelDSPContext hdsp;
    IDCTDSPContext idsp;

    int restart_interval;
    int restart_count;

    int buggy_avid;
    int cs_itu601;
    int interlace_polarity;
    int multiscope;

    int mjpb_skiptosod;

    int cur_scan; /* current scan, used by JPEG-LS */
    int flipped; /* true if picture is flipped */

    uint16_t (*ljpeg_buffer)[4];
    unsigned int ljpeg_buffer_size;

    int extern_huff;
    AVDictionary *exif_metadata;

    AVStereo3D *stereo3d; ///!< stereoscopic information (cached, since it is read before frame allocation)

    const AVPixFmtDescriptor *pix_desc;

    ICCEntry *iccentries;
    int iccnum;
    int iccread;

    AVFrame *smv_frame;
    int smv_frames_per_jpeg;
    int smv_next_frame;

    // Raw stream data for hwaccel use.
    const uint8_t *raw_image_buffer;
    size_t         raw_image_buffer_size;
    const uint8_t *raw_scan_buffer;
    size_t         raw_scan_buffer_size;

    uint8_t raw_huffman_lengths[2][4][16];
    uint8_t raw_huffman_values[2][4][256];

    enum AVPixelFormat hwaccel_sw_pix_fmt;
    enum AVPixelFormat hwaccel_pix_fmt;
    void *hwaccel_picture_private;
    struct JLSState *jls_state;
} MJpegDecodeContext;

int ff_mjpeg_build_vlc(VLC *vlc, const uint8_t *bits_table,
                       const uint8_t *val_table, int is_ac, void *logctx);
int ff_mjpeg_decode_init(AVCodecContext *avctx);
int ff_mjpeg_decode_end(AVCodecContext *avctx);
int ff_mjpeg_decode_frame(AVCodecContext *avctx,
                          AVFrame *frame, int *got_frame,
                          AVPacket *avpkt);
int ff_mjpeg_decode_frame_from_buf(AVCodecContext *avctx,
                                   AVFrame *frame, int *got_frame,
                                   const AVPacket *avpkt, const uint8_t *buf, int buf_size);
int ff_mjpeg_decode_dqt(MJpegDecodeContext *s);
int ff_mjpeg_decode_dht(MJpegDecodeContext *s);
int ff_mjpeg_decode_sof(MJpegDecodeContext *s);
int ff_mjpeg_decode_sos(MJpegDecodeContext *s,
                        const uint8_t *mb_bitmask,int mb_bitmask_size,
                        const AVFrame *reference);
int ff_mjpeg_find_marker(MJpegDecodeContext *s,
                         const uint8_t **buf_ptr, const uint8_t *buf_end,
                         const uint8_t **unescaped_buf_ptr, int *unescaped_buf_size);

#endif /* AVCODEC_MJPEGDEC_H */
