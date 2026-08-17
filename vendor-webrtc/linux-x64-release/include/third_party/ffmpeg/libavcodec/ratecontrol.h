/*
 * Ratecontrol
 * Copyright (c) 2000, 2001, 2002 Fabrice Bellard
 * Copyright (c) 2002-2004 Michael Niedermayer
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

#ifndef AVCODEC_RATECONTROL_H
#define AVCODEC_RATECONTROL_H

/**
 * @file
 * ratecontrol header.
 */

#include <stdint.h>

typedef struct Predictor{
    double coeff;
    double count;
    double decay;
} Predictor;

typedef struct RateControlEntry{
    int pict_type;
    float qscale;
    int i_count;
    int f_code;
    int b_code;
    int mv_bits;
    int i_tex_bits;
    int p_tex_bits;
    int misc_bits;
    int header_bits;
    uint64_t expected_bits;
    int new_pict_type;
    float new_qscale;
    int64_t mc_mb_var_sum;
    int64_t mb_var_sum;
}RateControlEntry;

/**
 * rate control context.
 */
typedef struct RateControlContext{
    int num_entries;              ///< number of RateControlEntries
    RateControlEntry *entry;
    double buffer_index;          ///< amount of bits in the video/audio buffer
    Predictor pred[5];
    double short_term_qsum;       ///< sum of recent qscales
    double short_term_qcount;     ///< count of recent qscales
    double pass1_rc_eq_output_sum;///< sum of the output of the rc equation, this is used for normalization
    double pass1_wanted_bits;     ///< bits which should have been output by the pass1 code (including complexity init)
    double last_qscale;
    double last_qscale_for[5];    ///< last qscale for a specific pict type, used for max_diff & ipb factor stuff
    int64_t last_mc_mb_var_sum;
    int64_t last_mb_var_sum;
    uint64_t i_cplx_sum[5];
    uint64_t p_cplx_sum[5];
    uint64_t mv_bits_sum[5];
    uint64_t qscale_sum[5];
    int frame_count[5];
    int last_non_b_pict_type;

    /**
     * ratecontrol qmin qmax limiting method
     * 0-> clipping, 1-> use a nice continuous function to limit qscale within qmin/qmax.
     */
    float qsquish;
    float qmod_amp;
    int   qmod_freq;
    float initial_cplx;
    float buffer_aggressivity;

    char *rc_eq;
    struct AVExpr *rc_eq_eval;

    float *cplx_tab, *bits_tab;
}RateControlContext;

struct MpegEncContext;

/* rate control */
int ff_rate_control_init(struct MpegEncContext *s);
float ff_rate_estimate_qscale(struct MpegEncContext *s, int dry_run);
void ff_write_pass1_stats(struct MpegEncContext *s);
int ff_vbv_update(struct MpegEncContext *s, int frame_size);
void ff_get_2pass_fcode(struct MpegEncContext *s);
void ff_rate_control_uninit(RateControlContext *rcc);

#endif /* AVCODEC_RATECONTROL_H */
