/*
 * copyright (c) 2008 Michael Niedermayer <michaelni@gmx.at>
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

#ifndef AVCODEC_SYNTH_FILTER_H
#define AVCODEC_SYNTH_FILTER_H

#include "libavutil/tx.h"
#include "dcadct.h"

typedef struct SynthFilterContext {
    void (*synth_filter_float)(AVTXContext *imdct,
                               float *synth_buf_ptr, int *synth_buf_offset,
                               float synth_buf2[32], const float window[512],
                               float out[32], float in[32],
                               float scale, av_tx_fn imdct_fn);
    void (*synth_filter_float_64)(AVTXContext *imdct,
                                  float *synth_buf_ptr, int *synth_buf_offset,
                                  float synth_buf2[64], const float window[1024],
                                  float out[64], float in[64], float scale,
                                  av_tx_fn imdct_fn);
    void (*synth_filter_fixed)(DCADCTContext *imdct,
                               int32_t *synth_buf_ptr, int *synth_buf_offset,
                               int32_t synth_buf2[32], const int32_t window[512],
                               int32_t out[32], const int32_t in[32]);
    void (*synth_filter_fixed_64)(DCADCTContext *imdct,
                                  int32_t *synth_buf_ptr, int *synth_buf_offset,
                                  int32_t synth_buf2[64], const int32_t window[1024],
                                  int32_t out[64], const int32_t in[64]);
} SynthFilterContext;

void ff_synth_filter_init(SynthFilterContext *c);
void ff_synth_filter_init_aarch64(SynthFilterContext *c);
void ff_synth_filter_init_arm(SynthFilterContext *c);
void ff_synth_filter_init_x86(SynthFilterContext *c);

#endif /* AVCODEC_SYNTH_FILTER_H */
