/*
 * MLP codec common header file
 * Copyright (c) 2007-2008 Ian Caulfield
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

#ifndef AVCODEC_MLP_H
#define AVCODEC_MLP_H

#include <stdint.h>

#include "libavutil/channel_layout.h"

#define SYNC_MLP        0xbb
#define SYNC_TRUEHD     0xba

/** Last possible matrix channel for each codec */
#define MAX_MATRIX_CHANNEL_MLP      5
#define MAX_MATRIX_CHANNEL_TRUEHD   7
/** Maximum number of channels in a valid stream.
 *  MLP   : 5.1 + 2 noise channels -> 8 channels
 *  TrueHD: 7.1                    -> 8 channels
 */
#define MAX_CHANNELS                8

/** Maximum number of matrices used in decoding; most streams have one matrix
 *  per output channel, but some rematrix a channel (usually 0) more than once.
 */
#define MAX_MATRICES_MLP            6
#define MAX_MATRICES_TRUEHD         8
#define MAX_MATRICES                8

/** Maximum number of substreams that can be decoded.
 *  MLP's limit is 2. TrueHD supports at least up to 3.
 */
#define MAX_SUBSTREAMS      4

/** which multiple of 48000 the maximum sample rate is */
#define MAX_RATEFACTOR      4
/** maximum sample frequency seen in files */
#define MAX_SAMPLERATE      (MAX_RATEFACTOR * 48000)

/** maximum number of audio samples within one access unit */
#define MAX_BLOCKSIZE       (40 * MAX_RATEFACTOR)
/** next power of two greater than MAX_BLOCKSIZE */
#define MAX_BLOCKSIZE_POW2  (64 * MAX_RATEFACTOR)

/** number of allowed filters */
#define NUM_FILTERS         2

/** The maximum number of taps in IIR and FIR filters. */
#define MAX_FIR_ORDER       8
#define MAX_IIR_ORDER       4

/** Code that signals end of a stream. */
#define END_OF_STREAM       0xd234d234

#define PARAM_BLOCKSIZE     (1 << 7)
#define PARAM_MATRIX        (1 << 6)
#define PARAM_OUTSHIFT      (1 << 5)
#define PARAM_QUANTSTEP     (1 << 4)
#define PARAM_FIR           (1 << 3)
#define PARAM_IIR           (1 << 2)
#define PARAM_HUFFOFFSET    (1 << 1)
#define PARAM_PRESENCE      (1 << 0)

#define FIR 0
#define IIR 1

/** filter data */
typedef struct FilterParams {
    uint8_t     order; ///< number of taps in filter
    uint8_t     shift; ///< Right shift to apply to output of filter.

    int32_t     state[MAX_FIR_ORDER];

    int coeff_bits;
    int coeff_shift;
} FilterParams;

/** sample data coding information */
typedef struct ChannelParams {
    FilterParams filter_params[NUM_FILTERS];
    int32_t     coeff[NUM_FILTERS][MAX_FIR_ORDER];

    int16_t     huff_offset;      ///< Offset to apply to residual values.
    int32_t     sign_huff_offset; ///< sign/rounding-corrected version of huff_offset
    uint8_t     codebook;         ///< Which VLC codebook to use to read residuals.
    uint8_t     huff_lsbs;        ///< Size of residual suffix not encoded using VLC.
} ChannelParams;

/** Tables defining the Huffman codes.
 *  There are three entropy coding methods used in MLP (four if you count
 *  "none" as a method). These use the same sequences for codes starting with
 *  00 or 01, but have different codes starting with 1.
 */
extern const uint8_t ff_mlp_huffman_tables[3][18][2];

typedef struct {
    uint8_t channel_occupancy;
    uint8_t group1_channels;
    uint8_t group2_channels;
    uint8_t summary_info;
} ChannelInformation;

/** Tables defining channel information.
 *
 *  Possible channel arrangements are:
 *
 *  (Group 1)   C
 *  (Group 1)   L,  R
 *  (Group 1)   Lf, Rf          /  (Group 2)   S
 *  (Group 1)   Lf, Rf          /  (Group 2)   Ls, Rs
 *  (Group 1)   Lf, Rf          /  (Group 2)   LFE
 *  (Group 1)   Lf, Rf          /  (Group 2)   LFE, S
 *  (Group 1)   Lf, Rf          /  (Group 2)   LFE, Ls, Rs
 *  (Group 1)   Lf, Rf          /  (Group 2)   C
 *  (Group 1)   Lf, Rf          /  (Group 2)   C, S
 *  (Group 1)   Lf, Rf          /  (Group 2)   C, Ls, Rs
 *  (Group 1)   Lf, Rf          /  (Group 2)   C, LFE
 *  (Group 1)   Lf, Rf          /  (Group 2)   C, LFE, S
 *  (Group 1)   Lf, Rf          /  (Group 2)   C, LFE, Ls,  Rs
 *  (Group 1)   Lf, Rf  C       /  (Group 2)   S
 *  (Group 1)   Lf, Rf  C       /  (Group 2)   Ls, Rs
 *  (Group 1)   Lf, Rf  C       /  (Group 2)   LFE
 *  (Group 1)   Lf, Rf  C       /  (Group 2)   LFE, S
 *  (Group 1)   Lf, Rf  C       /  (Group 2)   LFE, Ls, Rs
 *  (Group 1)   Lf, Rf  Ls  Rs  /  (Group 2)   LFE
 *  (Group 1)   Lf, Rf  Ls  Rs  /  (Group 2)   C
 *  (Group 1)   Lf, Rf, Ls, Rs  /  (Group 2)   C, LFE
 */
extern const ChannelInformation ff_mlp_ch_info[21];

extern const AVChannelLayout ff_mlp_ch_layouts[12];

/** MLP uses checksums that seem to be based on the standard CRC algorithm, but
 *  are not (in implementation terms, the table lookup and XOR are reversed).
 *  We can implement this behavior using a standard av_crc on all but the
 *  last element, then XOR that with the last element.
 */
uint8_t  ff_mlp_checksum8 (const uint8_t *buf, unsigned int buf_size);
uint16_t ff_mlp_checksum16(const uint8_t *buf, unsigned int buf_size);

/** Calculate an 8-bit checksum over a restart header -- a non-multiple-of-8
 *  number of bits, starting two bits into the first byte of buf.
 */
uint8_t ff_mlp_restart_checksum(const uint8_t *buf, unsigned int bit_size);

/** XOR together all the bytes of a buffer.
 *  Does this belong in dspcontext?
 */
uint8_t ff_mlp_calculate_parity(const uint8_t *buf, unsigned int buf_size);

void ff_mlp_init_crc(void);

/** XOR four bytes into one. */
static inline uint8_t xor_32_to_8(uint32_t value)
{
    value ^= value >> 16;
    value ^= value >>  8;
    return value;
}

typedef enum THDChannelModifier {
    THD_CH_MODIFIER_NOTINDICATED  = 0x0,
    THD_CH_MODIFIER_STEREO        = 0x0, // Stereo (not Dolby Surround)
    THD_CH_MODIFIER_LTRT          = 0x1, // Dolby Surround
    THD_CH_MODIFIER_LBINRBIN      = 0x2, // Dolby Headphone
    THD_CH_MODIFIER_MONO          = 0x3, // Mono or Dual Mono
    THD_CH_MODIFIER_NOTSURROUNDEX = 0x1, // Not Dolby Digital EX
    THD_CH_MODIFIER_SURROUNDEX    = 0x2, // Dolby Digital EX
} THDChannelModifier;

#endif /* AVCODEC_MLP_H */
