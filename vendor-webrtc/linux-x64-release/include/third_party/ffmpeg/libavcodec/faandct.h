/*
 * Floating point AAN DCT
 * Copyright (c) 2003 Michael Niedermayer <michaelni@gmx.at>
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
 * @brief
 *     Floating point AAN DCT
 * @author Michael Niedermayer <michaelni@gmx.at>
 */

#ifndef AVCODEC_FAANDCT_H
#define AVCODEC_FAANDCT_H

#include <stdint.h>

void ff_faandct(int16_t *data);
void ff_faandct248(int16_t *data);

#endif /* AVCODEC_FAANDCT_H */
