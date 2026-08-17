/*
 * BitJazz SheerVideo decoder
 * Copyright (c) 2016 Paul B Mahol
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

#ifndef AVCODEC_SHEERVIDEODATA_H
#define AVCODEC_SHEERVIDEODATA_H

#include "libavutil/common.h"

typedef struct SheerTable {
    uint8_t  lens[2 * 15];
    uint16_t nb_16s;
} SheerTable;

static const SheerTable rgb[2] = {
    {
        {  0,  0,  2,  2,  3,  3,  5,  5,  8,  8, 10,  9, 14, 15, 18,
          17, 16, 13, 10, 10,  8,  7,  6,  5,  3,  2,  3,  0,  0,  0 }, 54
    },
    {
        {  0,  2,  0,  2,  0,  1,  1,  0,  2,  1,  3,  3,  4,  7, 13,
          11,  8,  4,  3,  3,  1,  2,  1,  0,  1,  0,  1,  2,  0,  0 }, 180
    }
};

static const SheerTable rgbi[2] = {
    {
        {  0,  0,  1,  3,  3,  3,  6,  8,  8, 11, 12, 15, 18, 21, 38,
           0, 22, 19, 15, 12, 11,  7,  8,  6,  4,  2,  3,  0,  0,  0 }, 0
    },
    {
        {  1,  0,  1,  1,  1,  1,  2,  1,  2,  4,  3,  5,  5,  6, 12,
          14,  6,  6,  5,  3,  3,  3,  2,  1,  1,  2,  0,  1,  0,  0 }, 164
    }
};

static const SheerTable ybr[2] = {
    {
        {  0,  0,  2,  2,  2,  3,  5,  5,  7,  7,  8,  9, 13, 13, 19,
          16, 14, 12,  9,  9,  7,  6,  6,  4,  4,  1,  2,  1,  0,  0 }, 70
    },
    {
        {  1,  1,  0,  1,  0,  1,  0,  0,  1,  1,  2,  2,  3,  5,  5,
           5,  5,  3,  2,  2,  1,  0,  1,  0,  0,  1,  0,  1,  0,  0 }, 212
    }
};

static const SheerTable ybyr[2] = {
    {
        {  0,  0,  2,  2,  3,  3,  5,  5,  8,  8, 10, 10, 13, 15, 19,
          18, 15, 12, 10, 10,  8,  7,  6,  5,  3,  2,  3,  0,  0,  0 }, 54
    },
    {
        {  1,  1,  0,  1,  0,  1,  0,  1,  1,  2,  2,  3,  2,  5,  5,
           5,  4,  3,  2,  2,  2,  1,  1,  1,  1,  0,  0,  1,  0,  0 }, 208
    }
};

static const SheerTable byry[2] = {
    {
        {  0,  0,  2,  2,  2,  3,  5,  5,  7,  7,  8, 11, 10, 14, 19,
          14, 16, 12, 10,  8,  7,  6,  6,  4,  4,  1,  2,  1,  0,  0 }, 70
    },
    {
        {  1,  1,  0,  1,  0,  1,  0,  1,  2,  1,  2,  2,  3,  4,  6,
           6,  4,  2,  3,  2,  1,  1,  1,  1,  1,  0,  0,  1,  0,  0 }, 208
    }
};

static const SheerTable ybr10i[2] = {
    {
        {  0,  0,  1,  0,  3,  8,  9, 12, 19, 27, 27, 39, 50, 63, 93,
          89, 64, 50, 38, 26, 26, 20, 12,  9,  8,  3,  0,  0,  0,  0 }, 328
    },
    {
        {  0,  1,  1,  2,  2,  1,  2,  2,  4,  4,  6,  7,  9, 13, 28,
          28, 12, 11,  6,  7,  5,  3,  3,  1,  1,  2,  2,  1,  0,  0 }, 860
    }
};

static const SheerTable ybr10[2] = {
    {
        {  0,  0,  0,  1,  6,  6,  8, 12, 18, 21, 27, 29, 36, 47, 71,
          72, 46, 36, 29, 27, 21, 17, 13,  7,  7,  5,  0,  0,  0,  0 }, 462
    },
    {
        {  0,  1,  2,  1,  2,  1,  1,  1,  2,  3,  2,  5,  6, 10, 20,
          20, 10,  6,  4,  3,  2,  2,  2,  1,  1,  1,  2,  1,  0,  0 }, 912
    }
};

static const SheerTable rgbx[2] = {
    {
        {  0,  0,  0,  1,  3,  9, 10, 13, 19, 26, 28, 35, 40, 53, 77,
          77, 50, 42, 34, 28, 25, 19, 13, 10,  8,  4,  0,  0,  0,  0 }, 400
    },
    {
        {  0,  0,  1,  2,  6,  4,  3,  2,  3,  4,  6,  8, 10, 18, 39,
          39, 18, 11,  8,  6,  4,  4,  1,  3,  5,  4,  3,  0,  0,  0 }, 812
    }
};

static const SheerTable yry10[2] = {
    {
        {  0,  0,  0,  1,  6,  6,  8, 12, 18, 21, 27, 29, 36, 47, 71,
          72, 46, 36, 29, 27, 21, 17, 13,  7,  7,  5,  0,  0,  0,  0 }, 462
    },
    {
        {  0,  1,  2,  1,  1,  1,  2,  3,  2,  4,  5,  5,  8, 14, 16,
          18, 11,  7,  7,  4,  4,  3,  2,  2,  1,  1,  2,  1,  0,  0 }, 896
    }
};

static const SheerTable yry10i[2] = {
    {
        {  0,  0,  1,  0,  3,  8,  9, 12, 19, 27, 27, 40, 48, 64, 93,
          89, 65, 49, 38, 26, 26, 20, 12,  9,  8,  3,  0,  0,  0,  0 }, 328
    },
    {
        {  0,  1,  0,  3,  1,  3,  3,  3,  6,  7,  7, 12, 11, 19, 23,
          20, 18, 12, 12,  8,  6,  5,  4,  3,  2,  2,  2,  1,  0,  0 }, 830
    }
};

static const SheerTable ybri[2] = {
    {
        {  0,  0,  2,  2,  2,  3,  5,  5,  7, 10, 11, 13, 15, 13, 26,
          20, 16, 17, 12, 11,  9,  7,  5,  5,  3,  3,  1,  1,  0,  0 }, 32
    },
    {
        {  1,  0,  1,  0,  1,  1,  0,  2,  1,  2,  2,  2,  3,  6,  6,
           5,  6,  3,  2,  2,  2,  1,  2,  0,  1,  1,  0,  0,  1,  0 }, 202
    }
};

static const SheerTable byryi[2] = {
    {
        {  0,  0,  2,  2,  2,  2,  6,  5,  8,  8, 12, 12, 16, 14, 24,
          20, 16, 18, 12, 12,  8,  7,  5,  6,  3,  1,  2,  1,  0,  0 }, 32
    },
    {
        {  1,  0,  1,  1,  0,  2,  1,  2,  2,  3,  3,  4,  5,  4,  6,
           7,  5,  4,  4,  3,  3,  2,  2,  2,  0,  1,  1,  1,  0,  0 }, 186
    }
};

static const SheerTable rgbxi[2] = {
    {
        {  0,  0,  1,  3,  2,  3,  4,  6, 16, 23, 27, 29, 24, 29, 76,
          78, 29, 21, 29, 27, 23, 15,  7,  4,  3,  2,  3,  0,  0,  0 }, 540
    },
    {
        {  0,  1,  1,  2,  0,  2,  6,  4,  3,  9,  7, 12, 13, 16, 29,
          32, 17, 14, 12,  7,  8,  4,  4,  6,  2,  0,  2,  1,  0,  0 }, 810
    }
};

#endif /* AVCODEC_SHEERVIDEODATA_H */
