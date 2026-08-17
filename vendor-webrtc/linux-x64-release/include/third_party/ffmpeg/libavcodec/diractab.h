/*
 * Copyright (C) 2016 Open Broadcast Systems Ltd.
 * Author    (C) 2016 Rostislav Pehlivanov <atomnuker@gmail.com>
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

#ifndef AVCODEC_DIRACTAB_H
#define AVCODEC_DIRACTAB_H

#include <stdint.h>

/* Tables here are shared between the Dirac/VC-2 decoder and the VC-2 encoder */

/* Default quantization tables for each wavelet transform */
extern const uint8_t ff_dirac_default_qmat[7][4][4];

/* Scaling factors needed for quantization/dequantization */
extern const int32_t ff_dirac_qscale_tab[116];

/* Scaling offsets needed for quantization/dequantization, for intra frames */
extern const int32_t ff_dirac_qoffset_intra_tab[120];

/* Scaling offsets needed for quantization/dequantization, for inter frames */
extern const int ff_dirac_qoffset_inter_tab[122];

#define DIRAC_MAX_QUANT_INDEX (FF_ARRAY_ELEMS(ff_dirac_qscale_tab))

#endif /* AVCODEC_DIRACTAB_H */
