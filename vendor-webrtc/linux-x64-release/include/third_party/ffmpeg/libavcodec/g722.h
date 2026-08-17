/*
 * Copyright (c) CMU 1993 Computer Science, Speech Group
 *                        Chengxiang Lu and Alex Hauptmann
 * Copyright (c) 2005 Steve Underwood <steveu at coppice.org>
 * Copyright (c) 2009 Kenan Gillet
 * Copyright (c) 2010 Martin Storsjo
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

#ifndef AVCODEC_G722_H
#define AVCODEC_G722_H

#include <stdint.h>
#include "libavutil/log.h"
#include "g722dsp.h"

#define PREV_SAMPLES_BUF_SIZE 1024

typedef struct G722Context {
    const AVClass *class;
    int     bits_per_codeword;
    int16_t prev_samples[PREV_SAMPLES_BUF_SIZE]; ///< memory of past decoded samples
    int     prev_samples_pos;        ///< the number of values in prev_samples

    /**
     * The band[0] and band[1] correspond respectively to the lower band and higher band.
     */
    struct G722Band {
        int16_t s_predictor;         ///< predictor output value
        int32_t s_zero;              ///< previous output signal from zero predictor
        int8_t  part_reconst_mem[2]; ///< signs of previous partially reconstructed signals
        int16_t prev_qtzd_reconst;   ///< previous quantized reconstructed signal (internal value, using low_inv_quant4)
        int16_t pole_mem[2];         ///< second-order pole section coefficient buffer
        int32_t diff_mem[6];         ///< quantizer difference signal memory
        int16_t zero_mem[6];         ///< Seventh-order zero section coefficient buffer
        int16_t log_factor;          ///< delayed 2-logarithmic quantizer factor
        int16_t scale_factor;        ///< delayed quantizer scale factor
    } band[2];

    struct TrellisNode {
        struct G722Band state;
        uint32_t ssd;
        int path;
    } *node_buf[2], **nodep_buf[2];

    struct TrellisPath {
        int value;
        int prev;
    } *paths[2];

    G722DSPContext dsp;
} G722Context;

extern const int16_t ff_g722_high_inv_quant[4];
extern const int16_t ff_g722_low_inv_quant4[16];
extern const int16_t ff_g722_low_inv_quant6[64];

void ff_g722_update_low_predictor(struct G722Band *band, const int ilow);

void ff_g722_update_high_predictor(struct G722Band *band, const int dhigh,
                                   const int ihigh);

#endif /* AVCODEC_G722_H */
