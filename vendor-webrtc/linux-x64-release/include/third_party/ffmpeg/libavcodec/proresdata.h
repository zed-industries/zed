/*
 * Apple ProRes compatible decoder
 *
 * Copyright (c) 2010-2011 Maxim Poliakovski
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

#ifndef AVCODEC_PRORESDATA_H
#define AVCODEC_PRORESDATA_H

#include <stdint.h>

#define FRAME_ID MKBETAG('i', 'c', 'p', 'f')

extern const uint8_t ff_prores_progressive_scan[64];
extern const uint8_t ff_prores_interlaced_scan[64];
extern const uint8_t ff_prores_dc_codebook[7];
extern const uint8_t ff_prores_run_to_cb[16];
extern const uint8_t ff_prores_level_to_cb[10];

#define FIRST_DC_CB 0xB8 // rice_order = 5, exp_golomb_order = 6, switch_bits = 0

#endif /* AVCODEC_PRORESDATA_H */
