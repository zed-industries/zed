/*
 * Block Gilbert-Moore decoder
 * Copyright (c) 2010 Thilo Borgmann <thilo.borgmann _at_ mail.de>
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
 * Block Gilbert-Moore decoder header
 * @author Thilo Borgmann <thilo.borgmann _at_ mail.de>
 */


#ifndef AVCODEC_BGMC_H
#define AVCODEC_BGMC_H


#include "get_bits.h"


int ff_bgmc_init(void *logctx, uint8_t **cf_lut, int **cf_lut_status);


void ff_bgmc_end(uint8_t **cf_lut, int **cf_lut_status);


int ff_bgmc_decode_init(GetBitContext *gb,
                      unsigned int *h, unsigned int *l, unsigned int *v);


void ff_bgmc_decode_end(GetBitContext *gb);


void ff_bgmc_decode(GetBitContext *gb, unsigned int num, int32_t *dst,
                 int delta, unsigned int sx,
                 unsigned int *h, unsigned int *l, unsigned int *v,
                 uint8_t *cf_lut, int *cf_lut_status);


#endif /* AVCODEC_BGMC_H */
