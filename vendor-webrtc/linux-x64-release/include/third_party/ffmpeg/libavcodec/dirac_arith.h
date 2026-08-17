/*
 * Copyright (C) 2007 Marco Gerards <marco@gnu.org>
 * Copyright (C) 2009 David Conrad
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
 * Arithmetic decoder for Dirac
 * @author Marco Gerards <marco@gnu.org>
 */

#ifndef AVCODEC_DIRAC_ARITH_H
#define AVCODEC_DIRAC_ARITH_H

#include "config.h"

#if ARCH_X86
#include "libavutil/x86/asm.h"
#endif
#include "bytestream.h"
#include "get_bits.h"

enum dirac_arith_contexts {
    CTX_ZPZN_F1,
    CTX_ZPNN_F1,
    CTX_NPZN_F1,
    CTX_NPNN_F1,
    CTX_ZP_F2,
    CTX_ZP_F3,
    CTX_ZP_F4,
    CTX_ZP_F5,
    CTX_ZP_F6,
    CTX_NP_F2,
    CTX_NP_F3,
    CTX_NP_F4,
    CTX_NP_F5,
    CTX_NP_F6,
    CTX_COEFF_DATA,
    CTX_SIGN_NEG,
    CTX_SIGN_ZERO,
    CTX_SIGN_POS,
    CTX_ZERO_BLOCK,
    CTX_DELTA_Q_F,
    CTX_DELTA_Q_DATA,
    CTX_DELTA_Q_SIGN,

    DIRAC_CTX_COUNT
};

// Dirac resets the arith decoder between decoding various types of data,
// so many contexts are never used simultaneously. Thus, we can reduce
// the number of contexts needed by reusing them.
#define CTX_SB_F1        CTX_ZP_F5
#define CTX_SB_DATA      0
#define CTX_PMODE_REF1   0
#define CTX_PMODE_REF2   1
#define CTX_GLOBAL_BLOCK 2
#define CTX_MV_F1        CTX_ZP_F2
#define CTX_MV_DATA      0
#define CTX_DC_F1        CTX_ZP_F5
#define CTX_DC_DATA      0

typedef struct {
    unsigned low;
    uint16_t range;
    int16_t  counter;

    const uint8_t *bytestream;
    const uint8_t *bytestream_end;

    uint16_t contexts[DIRAC_CTX_COUNT];
    int error;
    int overread;
} DiracArith;

extern const uint8_t ff_dirac_next_ctx[DIRAC_CTX_COUNT];
extern int16_t ff_dirac_prob_branchless[256][2];

static inline void renorm(DiracArith *c)
{
#if HAVE_FAST_CLZ
    int shift = 14 - av_log2_16bit(c->range-1) + ((c->range-1)>>15);

    c->low    <<= shift;
    c->range  <<= shift;
    c->counter += shift;
#else
    while (c->range <= 0x4000) {
        c->low   <<= 1;
        c->range <<= 1;
        c->counter++;
    }
#endif
}

static inline void refill(DiracArith *c)
{
    int counter = c->counter;

    if (counter >= 0) {
        int new = bytestream_get_be16(&c->bytestream);

        // the spec defines overread bits to be 1, and streams rely on this
        if (c->bytestream > c->bytestream_end) {
            new |= 0xff;
            if (c->bytestream > c->bytestream_end+1)
                new |= 0xff00;

            c->bytestream = c->bytestream_end;
            c->overread ++;
            if (c->overread > 4)
                c->error = AVERROR_INVALIDDATA;
        }

        c->low += new << counter;
        counter -= 16;
    }
    c->counter = counter;
}

static inline int dirac_get_arith_bit(DiracArith *c, int ctx)
{
    int prob_zero = c->contexts[ctx];
    int range_times_prob, bit;
    unsigned low = c->low;
    int    range = c->range;

    range_times_prob = (c->range * prob_zero) >> 16;

#if ARCH_X86 && HAVE_FAST_CMOV && HAVE_INLINE_ASM && HAVE_6REGS
    low   -= range_times_prob << 16;
    range -= range_times_prob;
    bit = 0;
    __asm__(
        "cmpl   %5, %4 \n\t"
        "setae  %b0    \n\t"
        "cmovb  %3, %2 \n\t"
        "cmovb  %5, %1 \n\t"
        : "+q"(bit), "+r"(range), "+r"(low)
        : "r"(c->low), "r"(c->low>>16),
          "r"(range_times_prob)
    );
#else
    bit = (low >> 16) >= range_times_prob;
    if (bit) {
        low   -= range_times_prob << 16;
        range -= range_times_prob;
    } else {
        range  = range_times_prob;
    }
#endif

    c->contexts[ctx] += ff_dirac_prob_branchless[prob_zero>>8][bit];
    c->low   = low;
    c->range = range;

    renorm(c);
    refill(c);
    return bit;
}

static inline int dirac_get_arith_uint(DiracArith *c, int follow_ctx, int data_ctx)
{
    int ret = 1;
    while (!dirac_get_arith_bit(c, follow_ctx)) {
        if (ret >= 0x40000000) {
            av_log(NULL, AV_LOG_ERROR, "dirac_get_arith_uint overflow\n");
            c->error = AVERROR_INVALIDDATA;
            return -1;
        }
        ret <<= 1;
        ret += dirac_get_arith_bit(c, data_ctx);
        follow_ctx = ff_dirac_next_ctx[follow_ctx];
    }
    return ret-1;
}

static inline int dirac_get_arith_int(DiracArith *c, int follow_ctx, int data_ctx)
{
    int ret = dirac_get_arith_uint(c, follow_ctx, data_ctx);
    if (ret && dirac_get_arith_bit(c, data_ctx+1))
        ret = -ret;
    return ret;
}

void ff_dirac_init_arith_tables(void);
void ff_dirac_init_arith_decoder(DiracArith *c, GetBitContext *gb, int length);

#endif /* AVCODEC_DIRAC_ARITH_H */
