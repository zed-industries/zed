/*
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
 * AAN (Arai, Agui and Nakajima) (I)DCT tables
 */

#ifndef AVCODEC_AANDCTTAB_H
#define AVCODEC_AANDCTTAB_H

#include <stdint.h>

extern const uint16_t ff_aanscales[64];
extern const uint16_t ff_inv_aanscales[64];

#endif /* AVCODEC_AANDCTTAB_H */
