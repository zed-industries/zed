/*
 * Common Ut Video header
 * Copyright (c) 2011 Konstantin Shishkov
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

#ifndef AVCODEC_UTVIDEO_H
#define AVCODEC_UTVIDEO_H

/**
 * @file
 * Common Ut Video header
 */

#include "libavutil/macros.h"

enum {
    PRED_NONE = 0,
    PRED_LEFT,
    PRED_GRADIENT,
    PRED_MEDIAN,
};

enum {
    COMP_NONE = 0,
    COMP_HUFF,
};

/*
 * "Original format" markers.
 * Based on values gotten from the official VFW encoder.
 * They are not used during decoding, but they do have
 * an informative role on seeing what was input
 * to the encoder.
 */
enum {
    UTVIDEO_RGB  = MKTAG(0x00, 0x00, 0x01, 0x18),
    UTVIDEO_RGBA = MKTAG(0x00, 0x00, 0x02, 0x18),
    UTVIDEO_420  = MKTAG('Y', 'V', '1', '2'),
    UTVIDEO_422  = MKTAG('Y', 'U', 'Y', '2'),
    UTVIDEO_444  = MKTAG('Y', 'V', '2', '4'),
};

#endif /* AVCODEC_UTVIDEO_H */
