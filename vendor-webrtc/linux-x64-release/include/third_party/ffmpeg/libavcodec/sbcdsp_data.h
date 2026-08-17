/*
 * Bluetooth low-complexity, subband codec (SBC)
 *
 * Copyright (C) 2017  Aurelien Jacobs <aurel@gnuage.org>
 * Copyright (C) 2008-2010  Nokia Corporation
 * Copyright (C) 2004-2010  Marcel Holtmann <marcel@holtmann.org>
 * Copyright (C) 2004-2005  Henryk Ploetz <henryk@ploetzli.ch>
 * Copyright (C) 2005-2006  Brad Midgley <bmidgley@xmission.com>
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
 * miscellaneous SBC tables
 */

#ifndef AVCODEC_SBCDSP_DATA_H
#define AVCODEC_SBCDSP_DATA_H

#include "sbc.h"

#define SBC_PROTO_FIXED_SCALE      16
#define SBC_COS_TABLE_FIXED_SCALE  15

/*
 * Constant tables for the use in SIMD optimized analysis filters
 * Each table consists of two parts:
 * 1. reordered "proto" table
 * 2. reordered "cos" table
 *
 * Due to non-symmetrical reordering, separate tables for "even"
 * and "odd" cases are needed
 */

extern const int16_t ff_sbcdsp_analysis_consts_fixed4_simd_even[];
extern const int16_t ff_sbcdsp_analysis_consts_fixed4_simd_odd[];
extern const int16_t ff_sbcdsp_analysis_consts_fixed8_simd_even[];
extern const int16_t ff_sbcdsp_analysis_consts_fixed8_simd_odd[];

#endif /* AVCODEC_SBCDSP_DATA_H */
