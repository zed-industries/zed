/*
 * Common code between Nellymoser encoder and decoder
 * Copyright (c) 2007 a840bda5870ba11f19698ff6eb9581dfb0f95fa5,
 *                    539459aeb7d425140b62a3ec7dbf6dc8e408a306, and
 *                    520e17cd55896441042b14df2566a6eb610ed444
 * Copyright (c) 2007 Loic Minier <lool at dooz.org>
 *                    Benjamin Larsson
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

/**
 * @file
 * The 3 alphanumeric copyright notices are md5summed they are from the original
 * implementors. The original code is available from http://code.google.com/p/nelly2pcm/
 */

#ifndef AVCODEC_NELLYMOSER_H
#define AVCODEC_NELLYMOSER_H

#include <stdint.h>

#define NELLY_BANDS       23
#define NELLY_BLOCK_LEN   64
#define NELLY_HEADER_BITS 116
#define NELLY_DETAIL_BITS 198
#define NELLY_BUF_LEN     128
#define NELLY_FILL_LEN    124
#define NELLY_BIT_CAP     6
#define NELLY_BASE_OFF    4228
#define NELLY_BASE_SHIFT  19
#define NELLY_SAMPLES     (2 * NELLY_BUF_LEN)

extern const float    ff_nelly_dequantization_table[127];
extern const uint8_t  ff_nelly_band_sizes_table[NELLY_BANDS];
extern const uint16_t ff_nelly_init_table[64];
extern const int16_t  ff_nelly_delta_table[32];

void ff_nelly_get_sample_bits(const float *buf, int *bits);

#endif /* AVCODEC_NELLYMOSER_H */
