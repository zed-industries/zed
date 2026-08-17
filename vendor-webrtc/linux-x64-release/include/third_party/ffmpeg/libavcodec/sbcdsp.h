/*
 * Bluetooth low-complexity, subband codec (SBC)
 *
 * Copyright (C) 2017  Aurelien Jacobs <aurel@gnuage.org>
 * Copyright (C) 2008-2010  Nokia Corporation
 * Copyright (C) 2004-2010  Marcel Holtmann <marcel@holtmann.org>
 * Copyright (C) 2004-2005  Henryk Ploetz <henryk@ploetzli.ch>
 * Copyright (C) 2005-2006  Brad Midgley <bmidgley@xmission.com>
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
 * SBC basic "building bricks"
 */

#ifndef AVCODEC_SBCDSP_H
#define AVCODEC_SBCDSP_H

#include "libavutil/mem_internal.h"

#include "sbc.h"
#include "sbcdsp_data.h"

#define SCALE_OUT_BITS 15
#define SBC_X_BUFFER_SIZE 328

typedef struct sbc_dsp_context SBCDSPContext;

struct sbc_dsp_context {
    int position;
    /* Number of consecutive blocks handled by the encoder */
    uint8_t increment;
    DECLARE_ALIGNED(SBC_ALIGN, int16_t, X)[2][SBC_X_BUFFER_SIZE];
    void (*sbc_analyze_4)(const int16_t *in, int32_t *out, const int16_t *consts);
    void (*sbc_analyze_8)(const int16_t *in, int32_t *out, const int16_t *consts);
    /* Polyphase analysis filter for 4 subbands configuration,
     * it handles "increment" blocks at once */
    void (*sbc_analyze_4s)(SBCDSPContext *s,
                           int16_t *x, int32_t *out, int out_stride);
    /* Polyphase analysis filter for 8 subbands configuration,
     * it handles "increment" blocks at once */
    void (*sbc_analyze_8s)(SBCDSPContext *s,
                           int16_t *x, int32_t *out, int out_stride);
    /* Process input data (deinterleave, endian conversion, reordering),
     * depending on the number of subbands and input data byte order */
    int (*sbc_enc_process_input_4s)(int position, const uint8_t *pcm,
                                    int16_t X[2][SBC_X_BUFFER_SIZE],
                                    int nsamples, int nchannels);
    int (*sbc_enc_process_input_8s)(int position, const uint8_t *pcm,
                                    int16_t X[2][SBC_X_BUFFER_SIZE],
                                    int nsamples, int nchannels);
    /* Scale factors calculation */
    void (*sbc_calc_scalefactors)(int32_t sb_sample_f[16][2][8],
                                  uint32_t scale_factor[2][8],
                                  int blocks, int channels, int subbands);
    /* Scale factors calculation with joint stereo support */
    int (*sbc_calc_scalefactors_j)(int32_t sb_sample_f[16][2][8],
                                   uint32_t scale_factor[2][8],
                                   int blocks, int subbands);
};

/*
 * Initialize pointers to the functions which are the basic "building bricks"
 * of SBC codec. Best implementation is selected based on target CPU
 * capabilities.
 */
void ff_sbcdsp_init(SBCDSPContext *s);

void ff_sbcdsp_init_arm(SBCDSPContext *s);
void ff_sbcdsp_init_x86(SBCDSPContext *s);

#endif /* AVCODEC_SBCDSP_H */
