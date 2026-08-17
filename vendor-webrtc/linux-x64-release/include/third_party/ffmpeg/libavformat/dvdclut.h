/*
 * DVD-Video subpicture CLUT (Color Lookup Table) utilities
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

#ifndef AVFORMAT_DVDCLUT_H
#define AVFORMAT_DVDCLUT_H

#include "libavcodec/codec_par.h"

/* ("palette: ") + ("rrggbb, "*15) + ("rrggbb") + \n + \0 */
#define FF_DVDCLUT_EXTRADATA_SIZE        (9 + (8 * 15) + 6 + 1 + 1)
#define FF_DVDCLUT_CLUT_LEN              16
#define FF_DVDCLUT_CLUT_SIZE             FF_DVDCLUT_CLUT_LEN * sizeof(uint32_t)

int ff_dvdclut_palette_extradata_cat(const uint32_t *clut,
                                     const size_t clut_size,
                                     AVCodecParameters *par);

int ff_dvdclut_yuv_to_rgb(uint32_t *clut, const size_t clut_size);

#endif /* AVFORMAT_DVDCLUT_H */
