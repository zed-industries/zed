/*
 * Direct Stream Digital (DSD) decoder
 * based on BSD licensed dsd2pcm by Sebastian Gesemann
 * Copyright (c) 2009, 2011 Sebastian Gesemann. All rights reserved.
 * Copyright (c) 2014 Peter Ross
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

#ifndef AVCODEC_DSD_H
#define AVCODEC_DSD_H

#include <stddef.h>
#include <stdint.h>

#define HTAPS   48               /** number of FIR constants */
#define FIFOSIZE 16              /** must be a power of two */
#define FIFOMASK (FIFOSIZE - 1)  /** bit mask for FIFO offsets */

#if FIFOSIZE * 8 < HTAPS * 2
#error "FIFOSIZE too small"
#endif

/**
 * Per-channel buffer
 */
typedef struct DSDContext {
    uint8_t buf[FIFOSIZE];
    unsigned pos;
} DSDContext;

void ff_init_dsd_data(void);

void ff_dsd2pcm_translate(DSDContext* s, size_t samples, int lsbf,
                          const uint8_t *src, ptrdiff_t src_stride,
                          float *dst, ptrdiff_t dst_stride);
#endif /* AVCODEC_DSD_H */
