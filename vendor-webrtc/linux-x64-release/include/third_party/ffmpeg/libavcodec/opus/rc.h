/*
 * Copyright (c) 2012 Andrew D'Addesio
 * Copyright (c) 2013-2014 Mozilla Corporation
 * Copyright (c) 2017 Rostislav Pehlivanov <atomnuker@gmail.com>
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

#ifndef AVCODEC_OPUS_RC_H
#define AVCODEC_OPUS_RC_H

#include <stdint.h>

#include "libavcodec/get_bits.h"

#include "opus.h"

#define opus_ilog(i) (av_log2(i) + !!(i))

typedef struct RawBitsContext {
    const uint8_t *position;
    uint32_t bytes;
    uint32_t cachelen;
    uint32_t cacheval;
} RawBitsContext;

typedef struct OpusRangeCoder {
    GetBitContext gb;
    RawBitsContext rb;
    uint32_t range;
    uint32_t value;
    uint32_t total_bits;

    /* Encoder */
    uint8_t buf[OPUS_MAX_FRAME_SIZE + 12]; /* memcpy vs (memmove + overreading) */
    uint8_t *rng_cur;                      /* Current range coded byte */
    int ext;                               /* Awaiting propagation */
    int rem;                               /* Carryout flag */

    /* Encoding stats */
    int waste;
} OpusRangeCoder;

/**
 * CELT: estimate bits of entropy that have thus far been consumed for the
 *       current CELT frame, to integer and fractional (1/8th bit) precision
 */
static av_always_inline uint32_t opus_rc_tell(const OpusRangeCoder *rc)
{
    return rc->total_bits - av_log2(rc->range) - 1;
}

static av_always_inline uint32_t opus_rc_tell_frac(const OpusRangeCoder *rc)
{
    uint32_t i, total_bits, rcbuffer, range;

    total_bits = rc->total_bits << 3;
    rcbuffer   = av_log2(rc->range) + 1;
    range      = rc->range >> (rcbuffer-16);

    for (i = 0; i < 3; i++) {
        int bit;
        range = range * range >> 15;
        bit = range >> 16;
        rcbuffer = rcbuffer << 1 | bit;
        range >>= bit;
    }

    return total_bits - rcbuffer;
}

uint32_t ff_opus_rc_dec_cdf(OpusRangeCoder *rc, const uint16_t *cdf);
void     ff_opus_rc_enc_cdf(OpusRangeCoder *rc, int val, const uint16_t *cdf);

uint32_t ff_opus_rc_dec_log(OpusRangeCoder *rc, uint32_t bits);
void     ff_opus_rc_enc_log(OpusRangeCoder *rc, int val, uint32_t bits);

uint32_t ff_opus_rc_dec_uint_step(OpusRangeCoder *rc, int k0);
void     ff_opus_rc_enc_uint_step(OpusRangeCoder *rc, uint32_t val, int k0);

uint32_t ff_opus_rc_dec_uint_tri(OpusRangeCoder *rc, int qn);
void     ff_opus_rc_enc_uint_tri(OpusRangeCoder *rc, uint32_t k, int qn);

uint32_t ff_opus_rc_dec_uint(OpusRangeCoder *rc, uint32_t size);
void     ff_opus_rc_enc_uint(OpusRangeCoder *rc, uint32_t val, uint32_t size);

uint32_t ff_opus_rc_get_raw(OpusRangeCoder *rc, uint32_t count);
void     ff_opus_rc_put_raw(OpusRangeCoder *rc, uint32_t val, uint32_t count);

int      ff_opus_rc_dec_laplace(OpusRangeCoder *rc, uint32_t symbol, int decay);
void     ff_opus_rc_enc_laplace(OpusRangeCoder *rc, int *value, uint32_t symbol, int decay);

int      ff_opus_rc_dec_init(OpusRangeCoder *rc, const uint8_t *data, int size);
void     ff_opus_rc_dec_raw_init(OpusRangeCoder *rc, const uint8_t *rightend, uint32_t bytes);

void     ff_opus_rc_enc_end(OpusRangeCoder *rc, uint8_t *dst, int size);
void     ff_opus_rc_enc_init(OpusRangeCoder *rc);

#define OPUS_RC_CHECKPOINT_UPDATE(rc) \
    rc_rollback_bits = opus_rc_tell_frac(rc); \
    rc_rollback_ctx  = *rc

#define OPUS_RC_CHECKPOINT_SPAWN(rc) \
    uint32_t rc_rollback_bits = opus_rc_tell_frac(rc); \
    OpusRangeCoder rc_rollback_ctx = *rc \

#define OPUS_RC_CHECKPOINT_BITS(rc) \
    (opus_rc_tell_frac(rc) - rc_rollback_bits)

#define OPUS_RC_CHECKPOINT_ROLLBACK(rc) \
    memcpy(rc, &rc_rollback_ctx, sizeof(OpusRangeCoder)); \

#endif /* AVCODEC_OPUS_RC_H */
