/*
 * RealVideo 4 decoder
 * copyright (c) 2007 Konstantin Shishkov
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
 * miscellaneous RV30/40 tables
 */

#ifndef AVCODEC_RV34DATA_H
#define AVCODEC_RV34DATA_H

#include <stdint.h>

/**
 * number of ones in nibble minus one
 */
static const uint8_t rv34_count_ones[16] = {
    0, 0, 0, 1, 0, 1, 1, 2, 0, 1, 1, 2, 1, 2, 2, 3
};

/**
 * values used to reconstruct coded block pattern
 */
static const uint8_t rv34_cbp_code[16] = {
    0x00, 0x20, 0x10, 0x30, 0x02, 0x22, 0x12, 0x32,
    0x01, 0x21, 0x11, 0x31, 0x03, 0x23, 0x13, 0x33
};

/**
 * precalculated results of division by three and modulo three for values 0-107
 *
 * A lot of four-tuples in RV40 are represented as c0*27+c1*9+c2*3+c3.
 * This table allows conversion from a value back to a vector.
 */
static const uint8_t modulo_three_table[108] = {
    0x00, 0x01, 0x02, 0x04, 0x05, 0x06, 0x08, 0x09, 0x0A,
    0x10, 0x11, 0x12, 0x14, 0x15, 0x16, 0x18, 0x19, 0x1A,
    0x20, 0x21, 0x22, 0x24, 0x25, 0x26, 0x28, 0x29, 0x2A,

    0x40, 0x41, 0x42, 0x44, 0x45, 0x46, 0x48, 0x49, 0x4A,
    0x50, 0x51, 0x52, 0x54, 0x55, 0x56, 0x58, 0x59, 0x5A,
    0x60, 0x61, 0x62, 0x64, 0x65, 0x66, 0x68, 0x69, 0x6A,

    0x80, 0x81, 0x82, 0x84, 0x85, 0x86, 0x88, 0x89, 0x8A,
    0x90, 0x91, 0x92, 0x94, 0x95, 0x96, 0x98, 0x99, 0x9A,
    0xA0, 0xA1, 0xA2, 0xA4, 0xA5, 0xA6, 0xA8, 0xA9, 0xAA,

    0xC0, 0xC1, 0xC2, 0xC4, 0xC5, 0xC6, 0xC8, 0xC9, 0xCA,
    0xD0, 0xD1, 0xD2, 0xD4, 0xD5, 0xD6, 0xD8, 0xD9, 0xDA,
    0xE0, 0xE1, 0xE2, 0xE4, 0xE5, 0xE6, 0xE8, 0xE9, 0xEA,
};

/**
 * quantizer values used for AC and DC coefficients in chroma blocks
 */
static const uint8_t rv34_chroma_quant[2][32] = {
 {  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
   16, 17, 17, 18, 19, 20, 20, 21, 22, 22, 23, 23, 24, 24, 25, 25 },
 {  0,  0,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13,
   14, 15, 15, 16, 17, 18, 18, 19, 20, 20, 21, 21, 22, 22, 23, 23 }
};

/**
 * This table is used for dequantizing.
 */
static const uint16_t rv34_qscale_tab[32] = {
  60,   67,   76,   85,   96,  108,  121,  136,
 152,  171,  192,  216,  242,  272,  305,  341,
 383,  432,  481,  544,  606,  683,  767,  854,
 963, 1074, 1212, 1392, 1566, 1708, 1978, 2211
};

/**
 * tables used to translate a quantizer value into a VLC set for decoding
 * The first table is used for intraframes. The two last entries are invalid.
 */
static const uint8_t rv34_quant_to_vlc_set[2][32] = {
 { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1,
   2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 0, 0 },
 { 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3,
   3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 6 },
};

/**
 * maximum number of macroblocks for each of the possible slice offset sizes
 * @todo This is the same as ff_mba_max, maybe use it instead.
 */
static const uint16_t rv34_mb_max_sizes[6] = { 0x2F, 0x62, 0x18B, 0x62F, 0x18BF, 0x23FF };
/**
 * bits needed to code the slice offset for the given size
 * @todo This is the same as ff_mba_length, maybe use it instead.
 */
static const uint8_t rv34_mb_bits_sizes[6] = { 6, 7, 9, 11, 13, 14 };

#endif /* AVCODEC_RV34DATA_H */
