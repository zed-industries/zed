/*
 * LCL (LossLess Codec Library) Codec
 * Copyright (c) 2002-2004 Roberto Togni
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

#ifndef AVCODEC_LCL_H
#define AVCODEC_LCL_H

#define BMPTYPE_YUV 1
#define BMPTYPE_RGB 2

#define IMGTYPE_YUV111 0
#define IMGTYPE_YUV422 1
#define IMGTYPE_RGB24 2
#define IMGTYPE_YUV411 3
#define IMGTYPE_YUV211 4
#define IMGTYPE_YUV420 5

#define COMP_MSZH 0
#define COMP_MSZH_NOCOMP 1
#define COMP_ZLIB_HISPEED 1
#define COMP_ZLIB_HICOMP 9
#define COMP_ZLIB_NORMAL -1

#define FLAG_MULTITHREAD 1
#define FLAG_NULLFRAME 2
#define FLAG_PNGFILTER 4
#define FLAGMASK_UNUSED 0xf8

#define CODEC_MSZH 1
#define CODEC_ZLIB 3

#endif /* AVCODEC_LCL_H */
