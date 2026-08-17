/*
 * Copyright (C) 2017 foo86
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

#ifndef AVCODEC_DOLBY_E_H
#define AVCODEC_DOLBY_E_H

#include <stdint.h>
#include "get_bits.h"

#define FRAME_SAMPLES   1792

#define MAX_PROG_CONF   23
#define MAX_PROGRAMS    8
#define MAX_CHANNELS    8

/**
 * @struct DolbyEHeaderInfo
 * Coded Dolby E header values up to end_gain element, plus derived values.
 */
typedef struct DolbyEHeaderInfo {
    /** @name Coded elements
     * @{
     */
    int         prog_conf;
    int         nb_channels;
    int         nb_programs;

    int         fr_code;
    int         fr_code_orig;

    int         ch_size[MAX_CHANNELS];
    int         mtd_ext_size;
    int         meter_size;

    int         rev_id[MAX_CHANNELS];
    int         begin_gain[MAX_CHANNELS];
    int         end_gain[MAX_CHANNELS];
    /** @} */

    /** @name Derived values
     * @{
     */
    int         multi_prog_warned;

    int         output_channel_order;

    int         sample_rate;
    /** @} */
} DolbyEHeaderInfo;

/**
 * @struct DBEContext
 * Dolby E reading context used by decoder and parser.
 */
typedef struct DBEContext {
    void        *avctx;
    GetBitContext   gb;

    const uint8_t *input;
    int         input_size;

    int         word_bits;
    int         word_bytes;
    int         key_present;

    DolbyEHeaderInfo metadata;

    uint8_t     buffer[1024 * 3 + AV_INPUT_BUFFER_PADDING_SIZE];
} DBEContext;

/**
 * Use the provided key to transform the input into data (put into s->buffer)
 * suitable for further processing and initialize s->gb to read said data.
 */
int ff_dolby_e_convert_input(DBEContext *s, int nb_words, int key);

/**
 * Initialize DBEContext and parse Dolby E metadata.
 * Set word_bits/word_bytes, input, input_size, key_present
 * and parse the header up to the end_gain element.
 * @param[out] s DBEContext.
 * @param[in]  buf raw input buffer.
 * @param[in]  buf_size must be 3 bytes at least.
 * @return Returns 0 on success, AVERROR_INVALIDDATA on error
 */
int ff_dolby_e_parse_header(DBEContext *s, const uint8_t *buf, int buf_size);

#endif
