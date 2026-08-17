/*
 * Real Audio 1.0 (14.4K)
 * Copyright (c) 2003 The FFmpeg project
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

#ifndef AVCODEC_RA144_H
#define AVCODEC_RA144_H

#include <stdint.h>

#include "libavutil/mem_internal.h"

#include "lpc.h"
#include "audio_frame_queue.h"
#include "audiodsp.h"

#define NBLOCKS         4       ///< number of subblocks within a block
#define BLOCKSIZE       40      ///< subblock size in 16-bit words
#define BUFFERSIZE      146     ///< the size of the adaptive codebook
#define FIXED_CB_SIZE   128     ///< size of fixed codebooks
#define FRAME_SIZE      20      ///< size of encoded frame
#define LPC_ORDER       10      ///< order of LPC filter

typedef struct RA144Context {
    AVCodecContext *avctx;
    AudioDSPContext adsp;
    LPCContext lpc_ctx;
    AudioFrameQueue afq;
    int last_frame;

    unsigned int     old_energy;        ///< previous frame energy

    unsigned int     lpc_tables[2][10];

    /** LPC coefficients: lpc_coef[0] is the coefficients of the current frame
     *  and lpc_coef[1] of the previous one. */
    unsigned int    *lpc_coef[2];

    unsigned int     lpc_refl_rms[2];

    int16_t curr_block[NBLOCKS * BLOCKSIZE];

    /** The current subblock padded by the last 10 values of the previous one. */
    int16_t curr_sblock[50];

    /** Adaptive codebook, its size is two units bigger to avoid a
     *  buffer overflow. */
    int16_t adapt_cb[146+2];

    DECLARE_ALIGNED(16, int16_t, buffer_a)[FFALIGN(BLOCKSIZE,16)];
} RA144Context;

void ff_copy_and_dup(int16_t *target, const int16_t *source, int offset);
int ff_eval_refl(int *refl, const int16_t *coefs, AVCodecContext *avctx);
void ff_eval_coefs(int *coefs, const int *refl);
void ff_int_to_int16(int16_t *out, const int *inp);
int ff_t_sqrt(unsigned int x);
unsigned int ff_rms(const int *data);
int ff_interp(RA144Context *ractx, int16_t *out, int a, int copyold,
              int energy);
unsigned int ff_rescale_rms(unsigned int rms, unsigned int energy);
int ff_irms(AudioDSPContext *adsp, const int16_t *data/*align 16*/);
void ff_subblock_synthesis(RA144Context *ractx, const int16_t *lpc_coefs,
                           int cba_idx, int cb1_idx, int cb2_idx,
                           int gval, int gain);

extern const int16_t ff_gain_val_tab[256][3];
extern const uint8_t ff_gain_exp_tab[256];
extern const int8_t ff_cb1_vects[128][40];
extern const int8_t ff_cb2_vects[128][40];
extern const uint16_t ff_cb1_base[128];
extern const uint16_t ff_cb2_base[128];
extern const int16_t ff_energy_tab[32];
extern const int16_t * const ff_lpc_refl_cb[10];

#endif /* AVCODEC_RA144_H */
