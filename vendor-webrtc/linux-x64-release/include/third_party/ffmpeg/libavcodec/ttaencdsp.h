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

#ifndef AVCODEC_TTAENCDSP_H
#define AVCODEC_TTAENCDSP_H

#include <stdint.h>

typedef struct TTAEncDSPContext {
    void (*filter_process)(int32_t *qm, int32_t *dx, int32_t *dl,
                           int32_t *error, int32_t *in, int32_t shift,
                           int32_t round);
} TTAEncDSPContext;

void ff_ttaencdsp_init(TTAEncDSPContext *c);
void ff_ttaencdsp_init_x86(TTAEncDSPContext *c);

#endif /* AVCODEC_TTAENCDSP_H */
