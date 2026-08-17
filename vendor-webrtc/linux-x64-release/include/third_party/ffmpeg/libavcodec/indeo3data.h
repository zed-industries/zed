/*
 * Indeo Video v3 compatible decoder
 * Copyright (c) 2009 - 2011 Maxim Poliakovski
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

#ifndef AVCODEC_INDEO3DATA_H
#define AVCODEC_INDEO3DATA_H

#include <stdint.h>

#include "config.h"

/*
 * Define compressed VQ tables.
 */

#define TAB_1_1 \
    PD(   0,   0), E2(   2,   2), E4(  -1,   3), E2(   4,   4), E4(   1,   5),\
    E2(  -4,   4), E4(  -2,   6), E4(   4,   9), E2(   9,   9), E4(   1,  10),\
    E4(  -5,   8), E4(   9,  15), E4(  -3,  12), E4(   4,  16), E2(  16,  16),\
    E4(   0,  18), E2( -12,  12), E4(  -9,  16), E4(  11,  27), E4(  19,  28),\
    E4(  -6,  22), E4(   4,  29), E2(  30,  30), E4(  -2,  33), E4( -18,  23),\
    E4( -15,  30), E4(  22,  46), E4(  13,  47), E4(  35,  49), E4( -11,  41),\
    E4(   4,  51), E2(  54,  54), E2( -34,  34), E4( -29,  42), E4(  -6,  60),\
    E4(  27,  76), E4(  43,  77), E4( -24,  55), E4(  14,  79), E4(  63,  83),\
    E4( -20,  74), E4(   2,  88), E2(  93,  93), E4( -52,  61), E4(  52, 120),\
    E4( -45,  75), E4(  75, 125), E4(  33, 122), E4( -13, 103), E4( -40,  96),\
    E4( -34, 127), E2( -89,  89), E4( -78, 105), E2(  12,  12), E2(  23,  23),\
    E2(  42,  42), E2(  73,  73)

#define TAB_1_2 \
    PD(   0,   0), E2(   3,   3), E4(  -1,   4), E2(   7,   7), E4(   2,   8),\
    E4(  -2,   9), E2(  -6,   6), E4(   6,  13), E2(  13,  13), E4(   1,  14),\
    E4(  -8,  12), E4(  14,  23), E4(  -5,  18), E4(   6,  24), E2(  24,  24),\
    E4(  -1,  27), E2( -17,  17), E4( -13,  23), E4(  16,  40), E4(  28,  41),\
    E4(  -9,  33), E4(   6,  43), E2(  46,  46), E4(  -4,  50), E4( -27,  34),\
    E4( -22,  45), E4(  34,  69), E4(  19,  70), E4(  53,  73), E4( -17,  62),\
    E4(   5,  77), E2(  82,  82), E2( -51,  51), E4( -43,  64), E4( -10,  90),\
    E4(  41, 114), E4(  64, 116), E4( -37,  82), E4(  22, 119), E4(  95, 124),\
    E4( -30, 111), E4( -78,  92), E4( -68, 113), E2(  18,  18), E2(  34,  34),\
    E2(  63,  63), E2( 109, 109)

#define TAB_1_3 \
    PD(   0,   0), E2(   4,   4), E4(  -1,   5), E4(   3,  10), E2(   9,   9),\
    E2(  -7,   7), E4(  -3,  12), E4(   8,  17), E2(  17,  17), E4(   1,  19),\
    E4( -11,  16), E4(  -6,  23), E4(  18,  31), E4(   8,  32), E2(  33,  33),\
    E4(  -1,  36), E2( -23,  23), E4( -17,  31), E4(  21,  54), E4(  37,  55),\
    E4( -12,  44), E4(   8,  57), E2(  61,  61), E4(  -5,  66), E4( -36,  45),\
    E4( -29,  60), E4(  45,  92), E4(  25,  93), E4(  71,  97), E4( -22,  83),\
    E4(   7, 102), E2( 109, 109), E2( -68,  68), E4( -57,  85), E4( -13, 120),\
    E4( -49, 110), E4(-104, 123), E2(  24,  24), E2(  46,  46), E2(  84,  84)

#define TAB_1_4 \
    PD(   0,   0), E2(   5,   5), E4(  -2,   7), E2(  11,  11), E4(   3,  13),\
    E2(  -9,   9), E4(  -4,  15), E4(  11,  22), E2(  21,  21), E4(   2,  24),\
    E4( -14,  20), E4(  23,  38), E4(  -8,  29), E4(  11,  39), E2(  41,  41),\
    E4(  -1,  45), E2( -29,  29), E4( -22,  39), E4(  27,  67), E4(  47,  69),\
    E4( -15,  56), E4(  11,  71), E2(  76,  76), E4(  -6,  83), E4( -45,  57),\
    E4( -36,  75), E4(  56, 115), E4(  31, 117), E4(  88, 122), E4( -28, 104),\
    E2( -85,  85), E4( -72, 106), E2(  30,  30), E2(  58,  58), E2( 105, 105)

#define TAB_1_5 \
    PD(   0,   0), E2(   6,   6), E4(  -2,   8), E2(  13,  13), E4(   4,  15),\
    E2( -11,  11), E4(  -5,  18), E4(  13,  26), E2(  26,  26), E4(   2,  29),\
    E4( -16,  24), E4(  28,  46), E4(  -9,  35), E4(  13,  47), E2(  49,  49),\
    E4(  -1,  54), E2( -35,  35), E4( -26,  47), E4(  32,  81), E4(  56,  83),\
    E4( -18,  67), E4(  13,  86), E2(  91,  91), E4(  -7,  99), E4( -54,  68),\
    E4( -44,  90), E4( -33, 124), E2(-103, 103), E4( -86, 127), E2(  37,  37),\
    E2(  69,  69)

#define TAB_1_6 \
    PD(   0,   0), E2(   7,   7), E4(  -3,  10), E2(  16,  16), E4(   5,  18),\
    E2( -13,  13), E4(  -6,  21), E4(  15,  30), E2(  30,  30), E4(   2,  34),\
    E4( -19,  28), E4(  32,  54), E4( -11,  41), E4(  15,  55), E2(  57,  57),\
    E4(  -1,  63), E2( -40,  40), E4( -30,  55), E4(  37,  94), E4(  65,  96),\
    E4( -21,  78), E4(  15, 100), E2( 106, 106), E4(  -8, 116), E4( -63,  79),\
    E4( -51, 105), E2(-120, 120), E2(  43,  43), E2(  80,  80)

#define TAB_1_7 \
    PD(   0,   0), E2(   8,   8), E4(  -3,  11), E2(  18,  18), E4(   5,  20),\
    E2( -15,  15), E4(  -7,  24), E4(  17,  35), E2(  34,  34), E4(   3,  38),\
    E4( -22,  32), E4(  37,  61), E4( -13,  47), E4(  17,  63), E2(  65,  65),\
    E4(  -1,  72), E2( -46,  46), E4( -35,  63), E4(  43, 107), E4(  75, 110),\
    E4( -24,  89), E4(  17, 114), E2( 121, 121), E4( -72,  91), E4( -58, 120),\
    E2(  49,  49), E2(  92,  92)

#define TAB_1_8 \
    PD(   0,   0), E2(   9,   9), E4(  -3,  12), E2(  20,  20), E4(   6,  23),\
    E2( -17,  17), E4(  -7,  27), E4(  19,  39), E2(  39,  39), E4(   3,  43),\
    E4( -24,  36), E4(  42,  69), E4( -14,  53), E4(  19,  71), E2(  73,  73),\
    E4(  -2,  80), E2( -52,  52), E4( -39,  70), E4(  48, 121), E4(  84, 124),\
    E4( -27, 100), E4( -81, 102), E2(  55,  55), E2( 104, 104)

#define TAB_2_1 \
    PD(   0,   0), E2(   2,   2), E4(   0,   2), E2(   4,   4), E4(   0,   4),\
    E2(  -4,   4), E4(  -2,   6), E4(   4,   8), E2(   8,   8), E4(   0,  10),\
    E4(  -4,   8), E4(   8,  14), E4(  -2,  12), E4(   4,  16), E2(  16,  16),\
    E4(   0,  18), E2( -12,  12), E4(  -8,  16), E4(  10,  26), E4(  18,  28),\
    E4(  -6,  22), E4(   4,  28), E2(  30,  30), E4(  -2,  32), E4( -18,  22),\
    E4( -14,  30), E4(  22,  46), E4(  12,  46), E4(  34,  48), E4( -10,  40),\
    E4(   4,  50), E2(  54,  54), E2( -34,  34), E4( -28,  42), E4(  -6,  60),\
    E4(  26,  76), E4(  42,  76), E4( -24,  54), E4(  14,  78), E4(  62,  82),\
    E4( -20,  74), E4(   2,  88), E2(  92,  92), E4( -52,  60), E4(  52, 118),\
    E4( -44,  74), E4(  74, 118), E4(  32, 118), E4( -12, 102), E4( -40,  96),\
    E4( -34, 118), E2( -88,  88), E4( -78, 104), E2(  12,  12), E2(  22,  22),\
    E2(  42,  42), E2(  72,  72)

#define TAB_2_2 \
    PD(   0,   0), E2(   3,   3), E4(   0,   3), E2(   6,   6), E4(   3,   9),\
    E4(  -3,   9), E2(  -6,   6), E4(   6,  12), E2(  12,  12), E4(   0,  15),\
    E4(  -9,  12), E4(  15,  24), E4(  -6,  18), E4(   6,  24), E2(  24,  24),\
    E4(   0,  27), E2( -18,  18), E4( -12,  24), E4(  15,  39), E4(  27,  42),\
    E4(  -9,  33), E4(   6,  42), E2(  45,  45), E4(  -3,  51), E4( -27,  33),\
    E4( -21,  45), E4(  33,  69), E4(  18,  69), E4(  54,  72), E4( -18,  63),\
    E4(   6,  78), E2(  81,  81), E2( -51,  51), E4( -42,  63), E4(  -9,  90),\
    E4(  42, 114), E4(  63, 117), E4( -36,  81), E4(  21, 120), E4(  96, 123),\
    E4( -30, 111), E4( -78,  93), E4( -69, 114), E2(  18,  18), E2(  33,  33),\
    E2(  63,  63), E2( 108, 108)

#define TAB_2_3 \
    PD(   0,   0), E2(   4,   4), E4(   0,   4), E4(   4,   8), E2(   8,   8),\
    E2(  -8,   8), E4(  -4,  12), E4(   8,  16), E2(  16,  16), E4(   0,  20),\
    E4( -12,  16), E4(  -4,  24), E4(  16,  32), E4(   8,  32), E2(  32,  32),\
    E4(   0,  36), E2( -24,  24), E4( -16,  32), E4(  20,  52), E4(  36,  56),\
    E4( -12,  44), E4(   8,  56), E2(  60,  60), E4(  -4,  64), E4( -36,  44),\
    E4( -28,  60), E4(  44,  92), E4(  24,  92), E4(  72,  96), E4( -20,  84),\
    E4(   8, 100), E2( 108, 108), E2( -68,  68), E4( -56,  84), E4( -12, 120),\
    E4( -48, 108), E4(-104, 124), E2(  24,  24), E2(  44,  44), E2(  84,  84)

#define TAB_2_4 \
    PD(   0,   0), E2(   5,   5), E4(   0,   5), E2(  10,  10), E4(   5,  15),\
    E2( -10,  10), E4(  -5,  15), E4(  10,  20), E2(  20,  20), E4(   0,  25),\
    E4( -15,  20), E4(  25,  40), E4( -10,  30), E4(  10,  40), E2(  40,  40),\
    E4(   0,  45), E2( -30,  30), E4( -20,  40), E4(  25,  65), E4(  45,  70),\
    E4( -15,  55), E4(  10,  70), E2(  75,  75), E4(  -5,  85), E4( -45,  55),\
    E4( -35,  75), E4(  55, 115), E4(  30, 115), E4(  90, 120), E4( -30, 105),\
    E2( -85,  85), E4( -70, 105), E2(  30,  30), E2(  60,  60), E2( 105, 105)

#define TAB_2_5 \
    PD(   0,   0), E2(   6,   6), E4(   0,   6), E2(  12,  12), E4(   6,  12),\
    E2( -12,  12), E4(  -6,  18), E4(  12,  24), E2(  24,  24), E4(   0,  30),\
    E4( -18,  24), E4(  30,  48), E4(  -6,  36), E4(  12,  48), E2(  48,  48),\
    E4(   0,  54), E2( -36,  36), E4( -24,  48), E4(  30,  78), E4(  54,  84),\
    E4( -18,  66), E4(  12,  84), E2(  90,  90), E4(  -6,  96), E4( -54,  66),\
    E4( -42,  90), E4( -30, 126), E2(-102, 102), E4( -84, 126), E2(  36,  36),\
    E2(  66,  66)

#define TAB_2_6 \
    PD(   0,   0), E2(   7,   7), E4(   0,   7), E2(  14,  14), E4(   7,  21),\
    E2( -14,  14), E4(  -7,  21), E4(  14,  28), E2(  28,  28), E4(   0,  35),\
    E4( -21,  28), E4(  35,  56), E4( -14,  42), E4(  14,  56), E2(  56,  56),\
    E4(   0,  63), E2( -42,  42), E4( -28,  56), E4(  35,  91), E4(  63,  98),\
    E4( -21,  77), E4(  14,  98), E2( 105, 105), E4(  -7, 119), E4( -63,  77),\
    E4( -49, 105), E2(-119, 119), E2(  42,  42), E2(  77,  77)

#define TAB_2_7 \
    PD(   0,   0), E2(   8,   8), E4(   0,   8), E2(  16,  16), E4(   8,  16),\
    E2( -16,  16), E4(  -8,  24), E4(  16,  32), E2(  32,  32), E4(   0,  40),\
    E4( -24,  32), E4(  40,  64), E4( -16,  48), E4(  16,  64), E2(  64,  64),\
    E4(   0,  72), E2( -48,  48), E4( -32,  64), E4(  40, 104), E4(  72, 112),\
    E4( -24,  88), E4(  16, 112), E2( 120, 120), E4( -72,  88), E4( -56, 120),\
    E2(  48,  48), E2(  88,  88)

#define TAB_2_8 \
    PD(   0,   0), E2(   9,   9), E4(   0,   9), E2(  18,  18), E4(   9,  27),\
    E2( -18,  18), E4(  -9,  27), E4(  18,  36), E2(  36,  36), E4(   0,  45),\
    E4( -27,  36), E4(  45,  72), E4( -18,  54), E4(  18,  72), E2(  72,  72),\
    E4(   0,  81), E2( -54,  54), E4( -36,  72), E4(  45, 117), E4(  81, 126),\
    E4( -27,  99), E4( -81,  99), E2(  54,  54), E2( 108, 108)

#define TAB_3_1 \
    PD(   0,   0), E2(   2,   2), E4(   0,   3), E2(   6,   6), E4(   0,   7),\
    E2(  -5,   5), E2(   5,  -5), E4(   6,  11), E4(   0,   8), E2(  11,  11),\
    E4(   0,  12), E4(  12,  17), E2(  17,  17), E4(   6,  18), E4(  -8,  11),\
    E4(   0,  15), E4(   0,  20), E4(  18,  25), E4(  11,  25), E2(  25,  25),\
    E2( -14,  14), E2(  14, -14), E4(   0,  26), E4( -11,  18), E4(  -7,  22),\
    E4(  26,  34), E4(  18,  34), E2(  34,  34), E4(  11,  35), E4(   0,  29),\
    E4( -19,  22), E4( -15,  26), E4(   0,  37), E4(  27,  44), E4(  36,  44),\
    E4(  18,  44), E4( -10,  33), E2(  45,  45)

#define TAB_3_2 \
    PD(   0,   0), E4(   0,   2), E2(   2,   2), E2(   6,   6), E4(   0,   6),\
    E2(  -4,   4), E2(  10,  -6), E2(   0, -12), PD(  -6, -12), E2(   6, -12),\
    PD(   6,  12), E2( -14,   0), E2(  12,  12), E2(   0, -18), E2(  14, -12),\
    PD( -18,  -6), E2(  18,  -6), PD(  18,   6), PD( -10, -18), E2(  10, -18),\
    PD(  10,  18), E2( -22,   0), E2(   0, -24), PD( -22, -12), E2(  22, -12),\
    PD(  22,  12), PD(  -8, -24), E2(   8, -24), PD(   8,  24), PD( -26,  -6),\
    E2(  26,  -6), PD(  26,   6), E2( -28,   0), E2(  20,  20), E2( -14, -26),\
    E2( -30, -12), E2( -10, -32), E2( -18, -32), E2( -26, -26), E2( -34, -20),\
    E2( -38, -12), E2( -32, -32), PD(  32,  32), PD( -22, -40), E2( -34, -34)

#define TAB_3_3 \
    PD(   0,   0), E4(   0,   2), E2(   4,   4), E2(  10,  10), E4(   0,  10),\
    E2(  -6,   6), E2(  14,  -8), E2( -18,   0), E2(  10, -16), E2(   0, -24),\
    PD( -24,  -8), E2(  24,  -8), PD(  24,   8), E2(  18,  18), E2(  20, -16),\
    PD( -14, -26), E2(  14, -26), PD(  14,  26), E2( -30,   0), E2(   0, -34),\
    PD( -34,  -8), E2(  34,  -8), PD(  34,   8), PD( -30, -18), E2(  30, -18),\
    PD(  30,  18), PD( -10, -34), E2(  10, -34), PD(  10,  34), E2( -20, -34),\
    E2( -40,   0), E2(  30,  30), E2( -40, -18), E2(   0, -44), E2( -16, -44),\
    PD( -36, -36), E2( -36, -36), E2( -26, -44), E2( -46, -26), E2( -52, -18),\
    PD( -20, -54), E2( -44, -44), PD( -32, -54), PD( -46, -46), E2( -46, -46)

#define TAB_3_4 \
    PD(   0,   0), E4(   0,   4), E2(   4,   4), E2(  12,  12), E4(   0,  12),\
    E2(  -8,   8), E2(   8, -16), E2(   0, -24), PD( -24,  -8), E2(  24,  -8),\
    PD(  24,   8), E2(  20, -16), E2( -28,   0), PD( -16, -24), E2(  16, -24),\
    PD(  16,  24), E2(   0, -32), PD( -28, -16), E2(  28, -16), PD(  28,  16),\
    PD(  -8, -32), PD(   8, -32), PD( -32,  -8), E2(  32,  -8), PD(  32,   8),\
    PD(  -8,  32), PD(   8,  32), E2(  24,  24), E2(  24, -24), E2( -20, -32),\
    E2( -40,   0), E2( -40, -16), PD(   0, -44), PD(   0, -44), E2( -44,   0),\
    PD(   0,  44), PD(   0,  44), E2( -32, -32), E2( -16, -44), PD( -24, -44),\
    E2( -44, -24), PD(  24,  44), E2( -48, -16), PD( -36, -36), E2( -36, -36),\
    PD(  36,  36), PD( -20, -52), E2(  40,  40), PD( -32, -52)

#define TAB_3_5 \
    PD(   0,   0), E2(   2,   2), E2(   6,   6), E2(  12,  12), E2(  20,  20),\
    E2(  32,  32), E2(  46,  46)


/**
 * Pack two delta values (a,b) into one 16-bit word
 * according with endianness of the host machine.
 */
#if HAVE_BIGENDIAN
#define PD(a,b) (((a) * (1 << 8)) + (b))
#else
#define PD(a,b) (((b) * (1 << 8)) + (a))
#endif

/**
 * Expand a pair of delta values (a,b)
 * into two/four delta entries.
 */
#define E2(a, b) PD(a, b), PD(-(a), -(b))
#define E4(a, b) PD(a, b), PD(-(a), -(b)), PD(b, a), PD(-(b), -(a))

/*
 * VQ tables for 4x4 block modes.
 * Let the compiler decompress and build the tables for us.
 */
static const int16_t delta_tab_1_1[195] = { TAB_1_1 };
static const int16_t delta_tab_1_2[159] = { TAB_1_2 };
static const int16_t delta_tab_1_3[133] = { TAB_1_3 };
static const int16_t delta_tab_1_4[115] = { TAB_1_4 };
static const int16_t delta_tab_1_5[101] = { TAB_1_5 };
static const int16_t delta_tab_1_6[93]  = { TAB_1_6 };
static const int16_t delta_tab_1_7[87]  = { TAB_1_7 };
static const int16_t delta_tab_1_8[77]  = { TAB_1_8 };

static const int16_t delta_tab_2_1[195] = { TAB_2_1 };
static const int16_t delta_tab_2_2[159] = { TAB_2_2 };
static const int16_t delta_tab_2_3[133] = { TAB_2_3 };
static const int16_t delta_tab_2_4[115] = { TAB_2_4 };
static const int16_t delta_tab_2_5[101] = { TAB_2_5 };
static const int16_t delta_tab_2_6[93]  = { TAB_2_6 };
static const int16_t delta_tab_2_7[87]  = { TAB_2_7 };
static const int16_t delta_tab_2_8[77]  = { TAB_2_8 };

static const int16_t delta_tab_3_1[128] = { TAB_3_1 };
static const int16_t delta_tab_3_2[79]  = { TAB_3_2 };
static const int16_t delta_tab_3_3[79]  = { TAB_3_3 };
static const int16_t delta_tab_3_4[79]  = { TAB_3_4 };
static const int16_t delta_tab_3_5[79]  = { TAB_3_5 };

#undef PD

/**
 * Pack four delta values (a,a,b,b) into one 32-bit word
 * according with endianness of the host machine.
 */
#if HAVE_BIGENDIAN
#define PD(a,b) (((a) * (1 << 24)) + ((a) * (1 << 16)) + ((b) * (1 << 8)) + (b))
#else
#define PD(a,b) (((b) * (1 << 24)) + ((b) * (1 << 16)) + ((a) * (1 << 8)) + (a))
#endif

/*
 * VQ tables for 8x8 block modes.
 * Those are based on the same delta tables by using
 * each value twice: ABCD --> AABBCCDD.
 */
static const int32_t delta_tab_1_1_m10[195] = { TAB_1_1 };
static const int32_t delta_tab_1_2_m10[159] = { TAB_1_2 };
static const int32_t delta_tab_1_3_m10[133] = { TAB_1_3 };
static const int32_t delta_tab_1_4_m10[115] = { TAB_1_4 };
static const int32_t delta_tab_1_5_m10[101] = { TAB_1_5 };
static const int32_t delta_tab_1_6_m10[93]  = { TAB_1_6 };
static const int32_t delta_tab_1_7_m10[87]  = { TAB_1_7 };
static const int32_t delta_tab_1_8_m10[77]  = { TAB_1_8 };

static const int32_t delta_tab_2_1_m10[195] = { TAB_2_1 };
static const int32_t delta_tab_2_2_m10[159] = { TAB_2_2 };
static const int32_t delta_tab_2_3_m10[133] = { TAB_2_3 };
static const int32_t delta_tab_2_4_m10[115] = { TAB_2_4 };
static const int32_t delta_tab_2_5_m10[101] = { TAB_2_5 };
static const int32_t delta_tab_2_6_m10[93]  = { TAB_2_6 };
static const int32_t delta_tab_2_7_m10[87]  = { TAB_2_7 };
static const int32_t delta_tab_2_8_m10[77]  = { TAB_2_8 };

static const int32_t delta_tab_3_1_m10[128] = { TAB_3_1 };
static const int32_t delta_tab_3_2_m10[79]  = { TAB_3_2 };
static const int32_t delta_tab_3_3_m10[79]  = { TAB_3_3 };
static const int32_t delta_tab_3_4_m10[79]  = { TAB_3_4 };
static const int32_t delta_tab_3_5_m10[79]  = { TAB_3_5 };


typedef struct vqEntry {
    const int16_t  *deltas;     ///< delta tables for 4x4 block modes
    const int32_t  *deltas_m10; ///< delta tables for 8x8 block modes
    uint8_t        num_dyads;   ///< number of two-pixel deltas
    uint8_t        quad_exp;    ///< log2 of four-pixel deltas
} vqEntry;

static const vqEntry vq_tab[24] = {
    /* set 1 */
    { delta_tab_1_1, delta_tab_1_1_m10, 195,  7 },
    { delta_tab_1_2, delta_tab_1_2_m10, 159,  9 },
    { delta_tab_1_3, delta_tab_1_3_m10, 133, 10 },
    { delta_tab_1_4, delta_tab_1_4_m10, 115, 11 },
    { delta_tab_1_5, delta_tab_1_5_m10, 101, 12 },
    { delta_tab_1_6, delta_tab_1_6_m10,  93, 12 },
    { delta_tab_1_7, delta_tab_1_7_m10,  87, 12 },
    { delta_tab_1_8, delta_tab_1_8_m10,  77, 13 },

    /* set 2 */
    { delta_tab_2_1, delta_tab_2_1_m10, 195,  7 },
    { delta_tab_2_2, delta_tab_2_2_m10, 159,  9 },
    { delta_tab_2_3, delta_tab_2_3_m10, 133, 10 },
    { delta_tab_2_4, delta_tab_2_4_m10, 115, 11 },
    { delta_tab_2_5, delta_tab_2_5_m10, 101, 12 },
    { delta_tab_2_6, delta_tab_2_6_m10,  93, 12 },
    { delta_tab_2_7, delta_tab_2_7_m10,  87, 12 },
    { delta_tab_2_8, delta_tab_2_8_m10,  77, 13 },

    /* set 3 */
    { delta_tab_3_1, delta_tab_3_1_m10, 128, 11 },
    { delta_tab_3_2, delta_tab_3_2_m10,  79, 13 },
    { delta_tab_3_3, delta_tab_3_3_m10,  79, 13 },
    { delta_tab_3_4, delta_tab_3_4_m10,  79, 13 },
    { delta_tab_3_5, delta_tab_3_5_m10,  79, 13 },
    { delta_tab_3_5, delta_tab_3_5_m10,  79, 13 },
    { delta_tab_3_5, delta_tab_3_5_m10,  79, 13 },
    { delta_tab_3_5, delta_tab_3_5_m10,  79, 13 }
};

#endif /* AVCODEC_INDEO3DATA_H */
