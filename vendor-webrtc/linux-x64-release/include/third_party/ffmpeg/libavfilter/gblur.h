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

#ifndef AVFILTER_GBLUR_H
#define AVFILTER_GBLUR_H
#include "avfilter.h"

typedef struct GBlurContext {
    const AVClass *class;

    float sigma;
    float sigmaV;
    int steps;
    int planes;

    int flt;
    int depth;
    int stride;
    int planewidth[4];
    int planeheight[4];
    float *buffer;
    float *localbuf;  ///< temporary buffer for horiz_slice. NULL if not used
    float boundaryscale;
    float boundaryscaleV;
    float postscale;
    float postscaleV;
    float nu;
    float nuV;
    int nb_planes;
    void (*horiz_slice)(float *buffer, int width, int height, int steps, float nu, float bscale, float *localbuf);
    void (*verti_slice)(float *buffer, int width, int height, int slice_start, int slice_end, int steps,
                            float nu, float bscale);
    void (*postscale_slice)(float *buffer, int length, float postscale, float min, float max);
} GBlurContext;

void ff_gblur_init_x86(GBlurContext *s);
#endif
