/*
 * VVC 1D transform
 *
 * Copyright (C) 2023 Nuo Mi
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

#ifndef AVCODEC_VVC_ITX_1D_H
#define AVCODEC_VVC_ITX_1D_H

#include <stdint.h>
#include <stddef.h>

#define vvc_itx_1d_fn(name) \
    void (name)(int *coeffs, ptrdiff_t stride, size_t nz)
typedef vvc_itx_1d_fn(*vvc_itx_1d_fn);

vvc_itx_1d_fn(ff_vvc_inv_dct2_2);
vvc_itx_1d_fn(ff_vvc_inv_dct2_4);
vvc_itx_1d_fn(ff_vvc_inv_dct2_8);
vvc_itx_1d_fn(ff_vvc_inv_dct2_16);
vvc_itx_1d_fn(ff_vvc_inv_dct2_32);
vvc_itx_1d_fn(ff_vvc_inv_dct2_64);
vvc_itx_1d_fn(ff_vvc_inv_dst7_4);
vvc_itx_1d_fn(ff_vvc_inv_dst7_8);
vvc_itx_1d_fn(ff_vvc_inv_dst7_16);
vvc_itx_1d_fn(ff_vvc_inv_dst7_32);
vvc_itx_1d_fn(ff_vvc_inv_dct8_4);
vvc_itx_1d_fn(ff_vvc_inv_dct8_8);
vvc_itx_1d_fn(ff_vvc_inv_dct8_16);
vvc_itx_1d_fn(ff_vvc_inv_dct8_32);


void ff_vvc_inv_lfnst_1d(int *v, const int *u, int no_zero_size, int n_tr_s,
    int pred_mode_intra, int lfnst_idx, int log2_transform_range);

#endif // AVCODEC_VVC_ITX_1D_H
