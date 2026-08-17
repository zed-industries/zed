/*
 * MMX/SSE constants used across x86 dsp optimizations.
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

#ifndef AVCODEC_X86_CONSTANTS_H
#define AVCODEC_X86_CONSTANTS_H

#include <stdint.h>

#include "libavutil/x86/asm.h"

extern const ymm_reg  ff_pw_1;
extern const ymm_reg  ff_pw_2;
extern const xmm_reg  ff_pw_3;
extern const ymm_reg  ff_pw_4;
extern const xmm_reg  ff_pw_5;
extern const xmm_reg  ff_pw_8;
extern const xmm_reg  ff_pw_9;
extern const uint64_t ff_pw_15;
extern const xmm_reg  ff_pw_16;
extern const xmm_reg  ff_pw_18;
extern const xmm_reg  ff_pw_20;
extern const xmm_reg  ff_pw_32;
extern const uint64_t ff_pw_53;
extern const xmm_reg  ff_pw_64;
extern const uint64_t ff_pw_128;
extern const ymm_reg  ff_pw_255;
extern const ymm_reg  ff_pw_256;
extern const ymm_reg  ff_pw_512;
extern const ymm_reg  ff_pw_1023;
extern const ymm_reg  ff_pw_1024;
extern const ymm_reg  ff_pw_2048;
extern const ymm_reg  ff_pw_4095;
extern const ymm_reg  ff_pw_4096;
extern const ymm_reg  ff_pw_8192;
extern const ymm_reg  ff_pw_m1;

extern const ymm_reg  ff_pb_0;
extern const ymm_reg  ff_pb_1;
extern const ymm_reg  ff_pb_2;
extern const ymm_reg  ff_pb_3;
extern const ymm_reg  ff_pb_80;
extern const ymm_reg  ff_pb_FE;
extern const uint64_t ff_pb_FC;

extern const xmm_reg  ff_ps_neg;

extern const ymm_reg  ff_pd_1;
extern const ymm_reg  ff_pd_16;
extern const ymm_reg  ff_pd_32;
extern const ymm_reg  ff_pd_8192;
extern const ymm_reg  ff_pd_65535;

#endif /* AVCODEC_X86_CONSTANTS_H */
