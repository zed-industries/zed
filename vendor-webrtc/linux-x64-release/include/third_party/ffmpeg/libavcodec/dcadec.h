/*
 * Copyright (C) 2016 foo86
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

#ifndef AVCODEC_DCADEC_H
#define AVCODEC_DCADEC_H

#include <stdint.h>

#include "libavutil/crc.h"
#include "libavutil/float_dsp.h"
#include "libavutil/log.h"

#include "avcodec.h"
#include "get_bits.h"
#include "dca.h"
#include "dcadsp.h"
#include "dca_core.h"
#include "dca_exss.h"
#include "dca_xll.h"
#include "dca_lbr.h"

#define DCA_PACKET_CORE         0x01
#define DCA_PACKET_EXSS         0x02
#define DCA_PACKET_XLL          0x04
#define DCA_PACKET_LBR          0x08
#define DCA_PACKET_MASK         0x0f

#define DCA_PACKET_RECOVERY     0x10    ///< Sync error recovery flag
#define DCA_PACKET_RESIDUAL     0x20    ///< Core valid for residual decoding

enum DCAOutputChannelOrder {
    CHANNEL_ORDER_DEFAULT,
    CHANNEL_ORDER_CODED,
};

typedef struct DCAContext {
    const AVClass   *class;       ///< class for AVOptions
    AVCodecContext  *avctx;

    DCACoreDecoder core;  ///< Core decoder context
    DCAExssParser  exss;  ///< EXSS parser context
    DCAXllDecoder  xll;   ///< XLL decoder context
    DCALbrDecoder  lbr;   ///< LBR decoder context

    DCADSPContext   dcadsp;

    const AVCRC     *crctab;

    uint8_t         *buffer;    ///< Packet buffer
    unsigned int    buffer_size;

    int     packet; ///< Packet flags

    int     request_channel_layout; ///< Converted from avctx.request_channel_layout
    int     core_only;              ///< Core only decoding flag
    int     output_channel_order;
    AVChannelLayout downmix_layout;
} DCAContext;

int ff_dca_set_channel_layout(AVCodecContext *avctx, int *ch_remap, int dca_mask);

void ff_dca_downmix_to_stereo_fixed(DCADSPContext *dcadsp, int32_t **samples,
                                    int *coeff_l, int nsamples, int ch_mask);
void ff_dca_downmix_to_stereo_float(AVFloatDSPContext *fdsp, float **samples,
                                    int *coeff_l, int nsamples, int ch_mask);

static inline int ff_dca_check_crc(AVCodecContext *avctx, GetBitContext *s,
                                   int p1, int p2)
{
    DCAContext *dca = avctx->priv_data;

    if (!(avctx->err_recognition & (AV_EF_CRCCHECK | AV_EF_CAREFUL)))
        return 0;
    if (((p1 | p2) & 7) || p1 < 0 || p2 > s->size_in_bits || p2 - p1 < 16)
        return -1;
    if (av_crc(dca->crctab, 0xffff, s->buffer + p1 / 8, (p2 - p1) / 8))
        return -1;
    return 0;
}

static inline int ff_dca_seek_bits(GetBitContext *s, int p)
{
    if (p < get_bits_count(s) || p > s->size_in_bits)
        return -1;
    skip_bits_long(s, p - get_bits_count(s));
    return 0;
}

#endif
