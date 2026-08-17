/*
 * Bluetooth low-complexity, subband codec (SBC)
 *
 * Copyright (C) 2017  Aurelien Jacobs <aurel@gnuage.org>
 * Copyright (C) 2012-2014  Intel Corporation
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
 * SBC common definitions for the encoder and decoder
 */

#ifndef AVCODEC_SBC_H
#define AVCODEC_SBC_H

#include <stddef.h>
#include <stdint.h>
#include "libavutil/crc.h"
#include "libavutil/mem_internal.h"

#define MSBC_BLOCKS 15

/* sampling frequency */
#define SBC_FREQ_16000  0x00
#define SBC_FREQ_32000  0x01
#define SBC_FREQ_44100  0x02
#define SBC_FREQ_48000  0x03

/* blocks */
#define SBC_BLK_4       0x00
#define SBC_BLK_8       0x01
#define SBC_BLK_12      0x02
#define SBC_BLK_16      0x03

/* channel mode */
#define SBC_MODE_MONO         0x00
#define SBC_MODE_DUAL_CHANNEL 0x01
#define SBC_MODE_STEREO       0x02
#define SBC_MODE_JOINT_STEREO 0x03

/* allocation method */
#define SBC_AM_LOUDNESS 0x00
#define SBC_AM_SNR      0x01

/* subbands */
#define SBC_SB_4        0x00
#define SBC_SB_8        0x01

/* synchronisation words */
#define SBC_SYNCWORD   0x9C
#define MSBC_SYNCWORD  0xAD

/* extra bits of precision for the synthesis filter input data */
#define SBCDEC_FIXED_EXTRA_BITS 2

/*
 * Enforce 16 byte alignment for the data, which is supposed to be used
 * with SIMD optimized code.
 */
#define SBC_ALIGN 16

/* This structure contains an unpacked SBC frame.
   Yes, there is probably quite some unused space herein */
struct sbc_frame {
    uint8_t frequency;
    uint8_t blocks;
    enum {
        MONO         = SBC_MODE_MONO,
        DUAL_CHANNEL = SBC_MODE_DUAL_CHANNEL,
        STEREO       = SBC_MODE_STEREO,
        JOINT_STEREO = SBC_MODE_JOINT_STEREO
    } mode;
    uint8_t channels;
    enum {
        LOUDNESS = SBC_AM_LOUDNESS,
        SNR      = SBC_AM_SNR
    } allocation;
    uint8_t subbands;
    uint8_t bitpool;
    uint16_t codesize;

    /* bit number x set means joint stereo has been used in subband x */
    uint8_t joint;

    /* only the lower 4 bits of every element are to be used */
    DECLARE_ALIGNED(SBC_ALIGN, uint32_t, scale_factor)[2][8];

    /* raw integer subband samples in the frame */
    DECLARE_ALIGNED(SBC_ALIGN, int32_t, sb_sample_f)[16][2][8];

    /* modified subband samples */
    DECLARE_ALIGNED(SBC_ALIGN, int32_t, sb_sample)[16][2][8];

    const AVCRC *crc_ctx;
};

uint8_t ff_sbc_crc8(const AVCRC *crc_ctx, const uint8_t *data, size_t len);
void ff_sbc_calculate_bits(const struct sbc_frame *frame, int (*bits)[8]);

#endif /* AVCODEC_SBC_H */
