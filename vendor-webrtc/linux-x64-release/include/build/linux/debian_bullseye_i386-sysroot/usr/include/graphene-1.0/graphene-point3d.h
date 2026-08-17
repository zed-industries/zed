/* graphene-point3d.h: Point in 3D space
 *
 * SPDX-License-Identifier: MIT
 *
 * Copyright 2014  Emmanuele Bassi
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

#pragma once

#if !defined(GRAPHENE_H_INSIDE) && !defined(GRAPHENE_COMPILATION)
#error "Only graphene.h can be included directly."
#endif

#include "graphene-types.h"

GRAPHENE_BEGIN_DECLS

/**
 * GRAPHENE_POINT3D_INIT:
 * @_x: the X coordinate
 * @_y: the Y coordinate
 * @_z: the Z coordinate
 *
 * Initializes a #graphene_point3d_t to the given coordinates when declaring it.
 *
 * Since: 1.0
 */
#define GRAPHENE_POINT3D_INIT(_x,_y,_z) (graphene_point3d_t) { .x = (_x), .y = (_y), .z = (_z) }

/**
 * GRAPHENE_POINT3D_INIT_ZERO:
 *
 * Initializes a #graphene_point3d_t to (0, 0, 0) when declaring it.
 *
 * Since: 1.0
 */
#define GRAPHENE_POINT3D_INIT_ZERO      GRAPHENE_POINT3D_INIT (0.f, 0.f, 0.f)

/**
 * graphene_point3d_t:
 * @x: the X coordinate
 * @y: the Y coordinate
 * @z: the Z coordinate
 *
 * A point with three components: X, Y, and Z.
 *
 * Since: 1.0
 */
struct _graphene_point3d_t
{
  float x;
  float y;
  float z;
};

GRAPHENE_AVAILABLE_IN_1_0
graphene_point3d_t *            graphene_point3d_alloc                  (void);
GRAPHENE_AVAILABLE_IN_1_0
void                            graphene_point3d_free                   (graphene_point3d_t       *p);

GRAPHENE_AVAILABLE_IN_1_0
graphene_point3d_t *            graphene_point3d_init                   (graphene_point3d_t       *p,
                                                                         float                     x,
                                                                         float                     y,
                                                                         float                     z);
GRAPHENE_AVAILABLE_IN_1_0
graphene_point3d_t *            graphene_point3d_init_from_point        (graphene_point3d_t       *p,
                                                                         const graphene_point3d_t *src);
GRAPHENE_AVAILABLE_IN_1_0
graphene_point3d_t *            graphene_point3d_init_from_vec3         (graphene_point3d_t       *p,
                                                                         const graphene_vec3_t    *v);
GRAPHENE_AVAILABLE_IN_1_0
void                            graphene_point3d_to_vec3                (const graphene_point3d_t *p,
                                                                         graphene_vec3_t          *v);

GRAPHENE_AVAILABLE_IN_1_0
bool                            graphene_point3d_equal                  (const graphene_point3d_t *a,
                                                                         const graphene_point3d_t *b);
GRAPHENE_AVAILABLE_IN_1_0
bool                            graphene_point3d_near                   (const graphene_point3d_t *a,
                                                                         const graphene_point3d_t *b,
                                                                         float                     epsilon);

GRAPHENE_AVAILABLE_IN_1_0
void                            graphene_point3d_scale                  (const graphene_point3d_t *p,
                                                                         float                     factor,
                                                                         graphene_point3d_t       *res);
GRAPHENE_AVAILABLE_IN_1_0
void                            graphene_point3d_cross                  (const graphene_point3d_t *a,
                                                                         const graphene_point3d_t *b,
                                                                         graphene_point3d_t       *res);
GRAPHENE_AVAILABLE_IN_1_0
float                           graphene_point3d_dot                    (const graphene_point3d_t *a,
                                                                         const graphene_point3d_t *b);
GRAPHENE_AVAILABLE_IN_1_0
float                           graphene_point3d_length                 (const graphene_point3d_t *p);
GRAPHENE_AVAILABLE_IN_1_0
void                            graphene_point3d_normalize              (const graphene_point3d_t *p,
                                                                         graphene_point3d_t       *res);
GRAPHENE_AVAILABLE_IN_1_4
float                           graphene_point3d_distance               (const graphene_point3d_t *a,
                                                                         const graphene_point3d_t *b,
                                                                         graphene_vec3_t          *delta);

GRAPHENE_AVAILABLE_IN_1_0
void                            graphene_point3d_interpolate            (const graphene_point3d_t *a,
                                                                         const graphene_point3d_t *b,
                                                                         double                    factor,
                                                                         graphene_point3d_t       *res);

GRAPHENE_AVAILABLE_IN_1_4
void                            graphene_point3d_normalize_viewport     (const graphene_point3d_t *p,
                                                                         const graphene_rect_t    *viewport,
                                                                         float                     z_near,
                                                                         float                     z_far,
                                                                         graphene_point3d_t       *res);

GRAPHENE_AVAILABLE_IN_1_0
const graphene_point3d_t *      graphene_point3d_zero                   (void);

GRAPHENE_END_DECLS
