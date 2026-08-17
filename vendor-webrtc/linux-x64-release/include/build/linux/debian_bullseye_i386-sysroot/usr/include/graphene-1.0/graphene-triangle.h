/* graphene-triangle.h: A triangle
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
#include "graphene-vec3.h"

GRAPHENE_BEGIN_DECLS

/**
 * graphene_triangle_t:
 *
 * A triangle.
 *
 * Since: 1.2
 */
struct _graphene_triangle_t
{
  /*< private >*/
  GRAPHENE_PRIVATE_FIELD (graphene_vec3_t, a);
  GRAPHENE_PRIVATE_FIELD (graphene_vec3_t, b);
  GRAPHENE_PRIVATE_FIELD (graphene_vec3_t, c);
};

GRAPHENE_AVAILABLE_IN_1_2
graphene_triangle_t *   graphene_triangle_alloc                 (void);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_triangle_free                  (graphene_triangle_t *t);

GRAPHENE_AVAILABLE_IN_1_2
graphene_triangle_t *   graphene_triangle_init_from_point3d     (graphene_triangle_t       *t,
                                                                 const graphene_point3d_t  *a,
                                                                 const graphene_point3d_t  *b,
                                                                 const graphene_point3d_t  *c);
GRAPHENE_AVAILABLE_IN_1_2
graphene_triangle_t *   graphene_triangle_init_from_vec3        (graphene_triangle_t       *t,
                                                                 const graphene_vec3_t     *a,
                                                                 const graphene_vec3_t     *b,
                                                                 const graphene_vec3_t     *c);
GRAPHENE_AVAILABLE_IN_1_10
graphene_triangle_t *   graphene_triangle_init_from_float       (graphene_triangle_t       *t,
                                                                 const float               *a,
                                                                 const float               *b,
                                                                 const float               *c);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_triangle_get_points            (const graphene_triangle_t *t,
                                                                 graphene_point3d_t        *a,
                                                                 graphene_point3d_t        *b,
                                                                 graphene_point3d_t        *c);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_triangle_get_vertices          (const graphene_triangle_t *t,
                                                                 graphene_vec3_t           *a,
                                                                 graphene_vec3_t           *b,
                                                                 graphene_vec3_t           *c);
GRAPHENE_AVAILABLE_IN_1_2
float                   graphene_triangle_get_area              (const graphene_triangle_t *t);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_triangle_get_midpoint          (const graphene_triangle_t *t,
                                                                 graphene_point3d_t        *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_triangle_get_normal            (const graphene_triangle_t *t,
                                                                 graphene_vec3_t           *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_triangle_get_plane             (const graphene_triangle_t *t,
                                                                 graphene_plane_t          *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_triangle_get_bounding_box      (const graphene_triangle_t *t,
                                                                 graphene_box_t            *res);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_triangle_get_barycoords        (const graphene_triangle_t *t,
                                                                 const graphene_point3d_t  *p,
                                                                 graphene_vec2_t           *res);
GRAPHENE_AVAILABLE_IN_1_10
bool                    graphene_triangle_get_uv                (const graphene_triangle_t *t,
                                                                 const graphene_point3d_t  *p,
                                                                 const graphene_vec2_t     *uv_a,
                                                                 const graphene_vec2_t     *uv_b,
                                                                 const graphene_vec2_t     *uv_c,
                                                                 graphene_vec2_t           *res);

GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_triangle_contains_point        (const graphene_triangle_t *t,
                                                                 const graphene_point3d_t  *p);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_triangle_equal                 (const graphene_triangle_t *a,
                                                                 const graphene_triangle_t *b);

GRAPHENE_END_DECLS
