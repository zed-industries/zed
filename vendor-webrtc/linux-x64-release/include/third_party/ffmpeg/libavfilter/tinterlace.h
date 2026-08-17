/*
 * Copyright (c) 2011 Stefano Sabatini
 * Copyright (c) 2010 Baptiste Coudurier
 * Copyright (c) 2003 Michael Zucchi <notzed@ximian.com>
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with FFmpeg; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

/**
 * @file
 * temporal field interlace filter, ported from MPlayer/libmpcodecs
 */
#ifndef AVFILTER_TINTERLACE_H
#define AVFILTER_TINTERLACE_H

#include "libavutil/bswap.h"
#include "libavutil/opt.h"
#include "libavutil/pixdesc.h"
#include "drawutils.h"
#include "avfilter.h"
#include "ccfifo.h"

#define TINTERLACE_FLAG_VLPF 01
#define TINTERLACE_FLAG_CVLPF 2
#define TINTERLACE_FLAG_EXACT_TB 4
#define TINTERLACE_FLAG_BYPASS_IL 8

enum VLPFilter {
    VLPF_OFF = 0,
    VLPF_LIN = 1,
    VLPF_CMP = 2,
};

enum TInterlaceMode {
    MODE_MERGE = 0,
    MODE_DROP_EVEN,
    MODE_DROP_ODD,
    MODE_PAD,
    MODE_INTERLEAVE_TOP,
    MODE_INTERLEAVE_BOTTOM,
    MODE_INTERLACEX2,
    MODE_MERGEX2,
    MODE_NB,
};

enum InterlaceScanMode {
    MODE_TFF = 0,
    MODE_BFF,
};

typedef struct TInterlaceContext {
    const AVClass *class;
    int mode;                   ///< TInterlaceMode, interlace mode selected
    AVRational preout_time_base;
    int flags;                  ///< flags affecting interlacing algorithm
    int lowpass;                ///< legacy interlace filter lowpass mode
    int vsub;                   ///< chroma vertical subsampling
    AVFrame *cur;
    AVFrame *next;
    uint8_t *black_data[2][4];  ///< buffer used to fill padded lines (limited/full)
    int black_linesize[4];
    FFDrawContext draw;
    FFDrawColor color;
    const AVPixFmtDescriptor *csp;
    void (*lowpass_line)(uint8_t *dstp, ptrdiff_t width, const uint8_t *srcp,
                         ptrdiff_t mref, ptrdiff_t pref, int clip_max);
    CCFifo cc_fifo;
} TInterlaceContext;

void ff_tinterlace_init_x86(TInterlaceContext *interlace);

#endif /* AVFILTER_TINTERLACE_H */
