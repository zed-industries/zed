/*
 * Opus encoder
 * Copyright (c) 2017 Rostislav Pehlivanov <atomnuker@gmail.com>
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

#ifndef AVCODEC_OPUS_ENC_H
#define AVCODEC_OPUS_ENC_H

#include "libavutil/intmath.h"
#include "opus.h"

/* Determines the maximum delay the psychoacoustic system will use for lookahead */
#define FF_BUFQUEUE_SIZE 145
#include "libavfilter/bufferqueue.h"

#define OPUS_MAX_LOOKAHEAD ((FF_BUFQUEUE_SIZE - 1)*2.5f)

#define OPUS_MAX_CHANNELS 2

/* 120 ms / 2.5 ms = 48 frames (extremely improbable, but the encoder'll work) */
#define OPUS_MAX_FRAMES_PER_PACKET 48

#define OPUS_BLOCK_SIZE(x) (2 * 15 * (1 << ((x) + 2)))

#define OPUS_SAMPLES_TO_BLOCK_SIZE(x) (ff_log2((x) / (2 * 15)) - 2)

typedef struct OpusEncOptions {
    float max_delay_ms;
    int apply_phase_inv;
} OpusEncOptions;

typedef struct OpusPacketInfo {
    enum OpusMode mode;
    enum OpusBandwidth bandwidth;
    int framesize;
    int frames;
} OpusPacketInfo;

#endif /* AVCODEC_OPUS_ENC_H */
