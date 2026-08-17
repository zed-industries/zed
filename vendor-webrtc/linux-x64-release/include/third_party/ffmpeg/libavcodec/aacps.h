/*
 * MPEG-4 Parametric Stereo definitions and declarations
 * Copyright (c) 2010 Alex Converse <alex.converse@gmail.com>
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

#ifndef AVCODEC_AACPS_H
#define AVCODEC_AACPS_H

#include <stdint.h>

#include "libavutil/mem_internal.h"

#include "aacpsdsp.h"
#include "get_bits.h"

#define PS_MAX_NUM_ENV 5
#define PS_MAX_NR_IIDICC 34
#define PS_MAX_NR_IPDOPD 17
#define PS_MAX_SSB 91
#define PS_MAX_AP_BANDS 50
#define PS_QMF_TIME_SLOTS 32
#define PS_MAX_DELAY 14
#define PS_AP_LINKS 3
#define PS_MAX_AP_DELAY 5
#define PS_BASELINE 0  ///< Operate in Baseline PS mode
                       ///< Baseline implies 10 or 20 stereo bands,
                       ///< mixing mode A, and no ipd/opd

#define numQMFSlots 32 //numTimeSlots * RATE

typedef struct PSCommonContext {
    int    start;
    int    enable_iid;
    int    iid_quant;
    int    nr_iid_par;
    int    nr_ipdopd_par;
    int    enable_icc;
    int    icc_mode;
    int    nr_icc_par;
    int    enable_ext;
    int    frame_class;
    int    num_env_old;
    int    num_env;
    int    enable_ipdopd;
    int    border_position[PS_MAX_NUM_ENV+1];
    int8_t iid_par[PS_MAX_NUM_ENV][PS_MAX_NR_IIDICC]; ///< Inter-channel Intensity Difference Parameters
    int8_t icc_par[PS_MAX_NUM_ENV][PS_MAX_NR_IIDICC]; ///< Inter-Channel Coherence Parameters
    /* ipd/opd is iid/icc sized so that the same functions can handle both */
    int8_t ipd_par[PS_MAX_NUM_ENV][PS_MAX_NR_IIDICC]; ///< Inter-channel Phase Difference Parameters
    int8_t opd_par[PS_MAX_NUM_ENV][PS_MAX_NR_IIDICC]; ///< Overall Phase Difference Parameters
    int    is34bands;
    int    is34bands_old;
} PSCommonContext;

typedef struct PSContext {
    PSCommonContext common;

    DECLARE_ALIGNED(16, INTFLOAT, in_buf)[5][44][2];
    DECLARE_ALIGNED(16, INTFLOAT, delay)[PS_MAX_SSB][PS_QMF_TIME_SLOTS + PS_MAX_DELAY][2];
    DECLARE_ALIGNED(16, INTFLOAT, ap_delay)[PS_MAX_AP_BANDS][PS_AP_LINKS][PS_QMF_TIME_SLOTS + PS_MAX_AP_DELAY][2];
    DECLARE_ALIGNED(16, INTFLOAT, peak_decay_nrg)[34];
    DECLARE_ALIGNED(16, INTFLOAT, power_smooth)[34];
    DECLARE_ALIGNED(16, INTFLOAT, peak_decay_diff_smooth)[34];
    DECLARE_ALIGNED(16, INTFLOAT, H11)[2][PS_MAX_NUM_ENV+1][PS_MAX_NR_IIDICC];
    DECLARE_ALIGNED(16, INTFLOAT, H12)[2][PS_MAX_NUM_ENV+1][PS_MAX_NR_IIDICC];
    DECLARE_ALIGNED(16, INTFLOAT, H21)[2][PS_MAX_NUM_ENV+1][PS_MAX_NR_IIDICC];
    DECLARE_ALIGNED(16, INTFLOAT, H22)[2][PS_MAX_NUM_ENV+1][PS_MAX_NR_IIDICC];
    DECLARE_ALIGNED(16, INTFLOAT, Lbuf)[91][32][2];
    DECLARE_ALIGNED(16, INTFLOAT, Rbuf)[91][32][2];
    int8_t opd_hist[PS_MAX_NR_IIDICC];
    int8_t ipd_hist[PS_MAX_NR_IIDICC];
    PSDSPContext dsp;
} PSContext;

extern const int8_t ff_k_to_i_20[];
extern const int8_t ff_k_to_i_34[];

void ff_ps_init_common(void);
void AAC_RENAME(ff_ps_init)(void);

static inline void AAC_RENAME(ff_ps_ctx_init)(PSContext *ps)
{
    AAC_RENAME(ff_psdsp_init)(&ps->dsp);
}

int ff_ps_read_data(void *logctx, GetBitContext *gb,
                     PSCommonContext *ps, int bits_left);
int AAC_RENAME(ff_ps_apply)(PSContext *ps, INTFLOAT L[2][38][64], INTFLOAT R[2][38][64], int top);

#endif /* AVCODEC_AACPS_H */
