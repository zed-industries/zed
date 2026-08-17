/*
 * APNG common header
 * Copyright (c) 2014 Benoit Fouet
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
 * APNG common header
 */

#ifndef AVCODEC_APNG_H
#define AVCODEC_APNG_H

enum {
   APNG_DISPOSE_OP_NONE       = 0,
   APNG_DISPOSE_OP_BACKGROUND = 1,
   APNG_DISPOSE_OP_PREVIOUS   = 2,
};

enum {
    APNG_BLEND_OP_SOURCE = 0,
    APNG_BLEND_OP_OVER   = 1,
};

/* Only the payload data, not including length, fourcc and CRC-32. */
#define APNG_FCTL_CHUNK_SIZE    26

#endif /* AVCODEC_APNG_H */
