/*
 * Copyright (c) 2024 Lynne <dev@lynne.ee>
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

#ifndef AVCODEC_AAC_AACDEC_USAC_H
#define AVCODEC_AAC_AACDEC_USAC_H

#include "aacdec.h"

#include "libavcodec/get_bits.h"

int ff_aac_usac_config_decode(AACDecContext *ac, AVCodecContext *avctx,
                              GetBitContext *gb, OutputConfiguration *oc,
                              int channel_config);

int ff_aac_usac_reset_state(AACDecContext *ac, OutputConfiguration *oc);

int ff_aac_usac_decode_frame(AVCodecContext *avctx, AACDecContext *ac,
                             GetBitContext *gb, int *got_frame_ptr);

#endif /* AVCODEC_AAC_AACDEC_USAC_H */
