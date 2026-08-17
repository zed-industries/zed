/*
 * Copyright (c) 2017 Google Inc.
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

#ifndef AVCODEC_AARCH64_VP9DSP_INIT_H
#define AVCODEC_AARCH64_VP9DSP_INIT_H

#include "libavcodec/vp9dsp.h"

void ff_vp9dsp_init_10bpp_aarch64(VP9DSPContext *dsp);
void ff_vp9dsp_init_12bpp_aarch64(VP9DSPContext *dsp);

#endif /* AVCODEC_AARCH64_VP9DSP_INIT_H */
