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

#ifndef AVCODEC_LCEVCDEC_H
#define AVCODEC_LCEVCDEC_H

#include "config.h"

#include <stdint.h>
#if CONFIG_LIBLCEVC_DEC
#include <LCEVC/lcevc_dec.h>
#else
typedef uintptr_t LCEVC_DecoderHandle;
#endif

typedef struct FFLCEVCContext {
    LCEVC_DecoderHandle decoder;
    int initialized;
} FFLCEVCContext;

struct AVFrame;

int ff_lcevc_alloc(FFLCEVCContext **plcevc);
int ff_lcevc_process(void *logctx, struct AVFrame *frame);
void ff_lcevc_unref(void *opaque);
#endif /* AVCODEC_LCEVCDEC_H */
