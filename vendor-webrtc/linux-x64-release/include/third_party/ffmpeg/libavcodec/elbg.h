/*
 * Copyright (C) 2007 Vitor Sessak <vitor1001@gmail.com>
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

#ifndef AVCODEC_ELBG_H
#define AVCODEC_ELBG_H

#include <stdint.h>
#include "libavutil/lfg.h"

struct ELBGContext;

/**
 * Implementation of the Enhanced LBG Algorithm
 * Based on the paper "Neural Networks 14:1219-1237" that can be found in
 * http://citeseer.ist.psu.edu/patan01enhanced.html .
 *
 * @param ctx  A pointer to a pointer to an already allocated ELBGContext
 *             or a pointer to NULL. In the latter case, this function
 *             will allocate an ELBGContext and put a pointer to it in `*ctx`.
 * @param points Input points.
 * @param dim Dimension of the points.
 * @param numpoints Num of points in **points.
 * @param codebook Pointer to the output codebook. Must be allocated.
 * @param num_cb Number of points in the codebook.
 * @param num_steps The maximum number of steps. One step is already a good compromise between time and quality.
 * @param closest_cb Return the closest codebook to each point. Must be allocated.
 * @param rand_state A random number generator state. Should be already initialized by av_lfg_init().
 * @param flags Currently unused; must be set to 0.
 * @return < 0 in case of error, 0 otherwise
 */
int avpriv_elbg_do(struct ELBGContext **ctx, int *points, int dim,
                   int numpoints, int *codebook, int num_cb, int num_steps,
                   int *closest_cb, AVLFG *rand_state, uintptr_t flags);

/**
 * Free an ELBGContext and reset the pointer to it.
 */
void avpriv_elbg_free(struct ELBGContext **ctx);

#endif /* AVCODEC_ELBG_H */
