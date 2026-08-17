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

/**
 * @file
 * Scene SAD functions
 */

#ifndef AVFILTER_SCENE_SAD_H
#define AVFILTER_SCENE_SAD_H

#include "avfilter.h"

#define SCENE_SAD_PARAMS const uint8_t *src1, ptrdiff_t stride1, \
                         const uint8_t *src2, ptrdiff_t stride2, \
                         ptrdiff_t width, ptrdiff_t height, \
                         uint64_t *sum

typedef void (*ff_scene_sad_fn)(SCENE_SAD_PARAMS);

void ff_scene_sad_c(SCENE_SAD_PARAMS);

void ff_scene_sad16_c(SCENE_SAD_PARAMS);

ff_scene_sad_fn ff_scene_sad_get_fn_x86(int depth);

ff_scene_sad_fn ff_scene_sad_get_fn(int depth);

#endif /* AVFILTER_SCENE_SAD_H */
