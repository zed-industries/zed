/*
 * Copyright (c) 2011 Pascal Getreuer
 * Copyright (c) 2016 Paul B Mahol
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer in the documentation and/or other materials provided
 *    with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef AVFILTER_GBLUR_INIT_H
#define AVFILTER_GBLUR_INIT_H

#include "config.h"
#include "libavutil/attributes.h"
#include "libavutil/common.h"
#include "gblur.h"

static void postscale_c(float *buffer, int length,
                        float postscale, float min, float max)
{
    for (int i = 0; i < length; i++) {
        buffer[i] *= postscale;
        buffer[i] = av_clipf(buffer[i], min, max);
    }
}

static void horiz_slice_c(float *buffer, int width, int height, int steps,
                          float nu, float bscale, float *localbuf)
{
    int x;
    for (int y = 0; y < height; y++) {
        for (int step = 0; step < steps; step++) {
            float *ptr = buffer + width * y;
            ptr[0] *= bscale;

            /* Filter rightwards */
            for (x = 1; x < width; x++)
                ptr[x] += nu * ptr[x - 1];
            ptr[x = width - 1] *= bscale;

            /* Filter leftwards */
            for (; x > 0; x--)
                ptr[x - 1] += nu * ptr[x];
        }
    }
}

static void do_vertical_columns(float *buffer, int width, int height,
                                int column_begin, int column_end, int steps,
                                float nu, float boundaryscale, int column_step)
{
    const int numpixels = width * height;
    int i;
    for (int x = column_begin; x < column_end;) {
        for (int step = 0; step < steps; step++) {
            float *ptr = buffer + x;
            for (int k = 0; k < column_step; k++) {
                ptr[k] *= boundaryscale;
            }
            /* Filter downwards */
            for (i = width; i < numpixels; i += width) {
                for (int k = 0; k < column_step; k++) {
                    ptr[i + k] += nu * ptr[i - width + k];
                }
            }
            i = numpixels - width;

            for (int k = 0; k < column_step; k++)
                ptr[i + k] *= boundaryscale;

            /* Filter upwards */
            for (; i > 0; i -= width) {
                for (int k = 0; k < column_step; k++)
                    ptr[i - width + k] += nu * ptr[i + k];
            }
        }
        x += column_step;
    }
}

static void verti_slice_c(float *buffer, int width, int height,
                          int slice_start, int slice_end, int steps,
                          float nu, float boundaryscale)
{
    int aligned_end = slice_start + (((slice_end - slice_start) >> 3) << 3);
    /* Filter vertically along columns (process 8 columns in each step) */
    do_vertical_columns(buffer, width, height, slice_start, aligned_end,
                        steps, nu, boundaryscale, 8);
    /* Filter un-aligned columns one by one */
    do_vertical_columns(buffer, width, height, aligned_end, slice_end,
                        steps, nu, boundaryscale, 1);
}

static av_unused void ff_gblur_init(GBlurContext *s)
{
    s->localbuf = NULL;
    s->horiz_slice = horiz_slice_c;
    s->verti_slice = verti_slice_c;
    s->postscale_slice = postscale_c;
#if ARCH_X86
    ff_gblur_init_x86(s);
#endif
}

#endif /* AVFILTER_GBLUR_INIT_H */
