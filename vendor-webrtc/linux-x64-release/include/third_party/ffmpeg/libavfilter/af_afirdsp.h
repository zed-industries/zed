/*
 * Copyright (c) 2017 Paul B Mahol
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

#ifndef AVFILTER_AFIRDSP_H
#define AVFILTER_AFIRDSP_H

#include <stddef.h>

#include "config.h"
#include "libavutil/attributes.h"

typedef struct AudioFIRDSPContext {
    void (*fcmul_add)(float *sum, const float *t, const float *c,
                      ptrdiff_t len);
    void (*dcmul_add)(double *sum, const double *t, const double *c,
                      ptrdiff_t len);
} AudioFIRDSPContext;

void ff_afir_init_riscv(AudioFIRDSPContext *s);
void ff_afir_init_x86(AudioFIRDSPContext *s);

static void fcmul_add_c(float *sum, const float *t, const float *c, ptrdiff_t len)
{
    int n;

    for (n = 0; n < len; n++) {
        const float cre = c[2 * n    ];
        const float cim = c[2 * n + 1];
        const float tre = t[2 * n    ];
        const float tim = t[2 * n + 1];

        sum[2 * n    ] += tre * cre - tim * cim;
        sum[2 * n + 1] += tre * cim + tim * cre;
    }

    sum[2 * n] += t[2 * n] * c[2 * n];
}

static void dcmul_add_c(double *sum, const double *t, const double *c, ptrdiff_t len)
{
    int n;

    for (n = 0; n < len; n++) {
        const double cre = c[2 * n    ];
        const double cim = c[2 * n + 1];
        const double tre = t[2 * n    ];
        const double tim = t[2 * n + 1];

        sum[2 * n    ] += tre * cre - tim * cim;
        sum[2 * n + 1] += tre * cim + tim * cre;
    }

    sum[2 * n] += t[2 * n] * c[2 * n];
}

static av_unused void ff_afir_init(AudioFIRDSPContext *dsp)
{
    dsp->fcmul_add = fcmul_add_c;
    dsp->dcmul_add = dcmul_add_c;

#if ARCH_RISCV
    ff_afir_init_riscv(dsp);
#elif ARCH_X86
    ff_afir_init_x86(dsp);
#endif
}

#endif /* AVFILTER_AFIRDSP_H */
