/*
 * LZW decoder
 * Copyright (c) 2003 Fabrice Bellard
 * Copyright (c) 2006 Konstantin Shishkov
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
 * @brief LZW decoding routines
 * @author Fabrice Bellard
 * @author modified for use in TIFF by Konstantin Shishkov
 */

#ifndef AVCODEC_LZW_H
#define AVCODEC_LZW_H

#include <stdint.h>

struct PutBitContext;

enum FF_LZW_MODES{
    FF_LZW_GIF,
    FF_LZW_TIFF
};

/* clients should not know what LZWState is */
typedef void LZWState;

/* first two functions de/allocate memory for LZWState */
void ff_lzw_decode_open(LZWState **p);
void ff_lzw_decode_close(LZWState **p);
int ff_lzw_decode_init(LZWState *s, int csize, const uint8_t *buf, int buf_size, int mode);
int ff_lzw_decode(LZWState *s, uint8_t *buf, int len);
int ff_lzw_decode_tail(LZWState *lzw);

/** LZW encode state */
struct LZWEncodeState;
extern const int ff_lzw_encode_state_size;

void ff_lzw_encode_init(struct LZWEncodeState *s, uint8_t *outbuf, int outsize,
                        int maxbits, enum FF_LZW_MODES mode, int little_endian);
int ff_lzw_encode(struct LZWEncodeState * s, const uint8_t * inbuf, int insize);
int ff_lzw_encode_flush(struct LZWEncodeState *s);

#endif /* AVCODEC_LZW_H */
