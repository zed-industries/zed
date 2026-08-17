/* graphene-plane.h: A plane in 3D space
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
 * graphene_plane_t:
 *
 * A 2D plane that extends infinitely in a 3D volume.
 *
 * The contents of the `graphene_plane_t` are private, and should not be
 * modified directly.
 *
 * Since: 1.2
 */
struct _graphene_plane_t
{
  /*< private >*/
  GRAPHENE_PRIVATE_FIELD (graphene_vec3_t, normal);
  GRAPHENE_PRIVATE_FIELD (float, constant);
};

GRAPHENE_AVAILABLE_IN_1_2
graphene_plane_t *              graphene_plane_alloc            (void);
GRAPHENE_AVAILABLE_IN_1_2
void                            graphene_plane_free             (graphene_plane_t         *p);

GRAPHENE_AVAILABLE_IN_1_2
graphene_plane_t *              graphene_plane_init             (graphene_plane_t         *p,
                                                                 const graphene_vec3_t    *normal,
                                                                 float                     constant);
GRAPHENE_AVAILABLE_IN_1_2
graphene_plane_t *              graphene_plane_init_from_vec4   (graphene_plane_t         *p,
                                                                 const graphene_vec4_t    *src);
GRAPHENE_AVAILABLE_IN_1_2
graphene_plane_t *              graphene_plane_init_from_plane  (graphene_plane_t         *p,
                                                                 const graphene_plane_t   *src);
GRAPHENE_AVAILABLE_IN_1_2
graphene_plane_t *              graphene_plane_init_from_point  (graphene_plane_t         *p,
                                                                 const graphene_vec3_t    *normal,
                                                                 const graphene_point3d_t *point);
GRAPHENE_AVAILABLE_IN_1_2
graphene_plane_t *              graphene_plane_init_from_points (graphene_plane_t         *p,
                                                                 const graphene_point3d_t *a,
                                                                 const graphene_point3d_t *b,
                                                                 const graphene_point3d_t *c);

GRAPHENE_AVAILABLE_IN_1_2
void                            graphene_plane_normalize        (const graphene_plane_t   *p,
                                                                 graphene_plane_t         *res);
GRAPHENE_AVAILABLE_IN_1_2
void                            graphene_plane_negate           (const graphene_plane_t   *p,
                                                                 graphene_plane_t         *res);
GRAPHENE_AVAILABLE_IN_1_2
bool                            graphene_plane_equal            (const graphene_plane_t   *a,
                                                                 const graphene_plane_t   *b);
GRAPHENE_AVAILABLE_IN_1_2
float                           graphene_plane_distance         (const graphene_plane_t   *p,
                                                                 const graphene_point3d_t *point);

GRAPHENE_AVAILABLE_IN_1_2
void                            graphene_plane_get_normal       (const graphene_plane_t   *p,
                                                                 graphene_vec3_t          *normal);
GRAPHENE_AVAILABLE_IN_1_2
float                           graphene_plane_get_constant     (const graphene_plane_t   *p);

GRAPHENE_AVAILABLE_IN_1_10
void                            graphene_plane_transform        (const graphene_plane_t   *p,
                                                                 const graphene_matrix_t  *matrix,
                                                                 const graphene_matrix_t  *normal_matrix,
                                                                 graphene_plane_t         *res);

GRAPHENE_END_DECLS
