/* graphene-frustum.h: A 3D field of view
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
#include "graphene-plane.h"

GRAPHENE_BEGIN_DECLS

/**
 * graphene_frustum_t:
 *
 * A 3D volume delimited by 2D clip planes.
 *
 * The contents of the `graphene_frustum_t` are private, and should not be
 * modified directly.
 *
 * Since: 1.2
 */
struct _graphene_frustum_t
{
  /*< private >*/
  GRAPHENE_PRIVATE_FIELD (graphene_plane_t, planes[6]);
};

GRAPHENE_AVAILABLE_IN_1_2
graphene_frustum_t *    graphene_frustum_alloc                  (void);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_frustum_free                   (graphene_frustum_t *f);

GRAPHENE_AVAILABLE_IN_1_2
graphene_frustum_t *    graphene_frustum_init                   (graphene_frustum_t       *f,
                                                                 const graphene_plane_t   *p0,
                                                                 const graphene_plane_t   *p1,
                                                                 const graphene_plane_t   *p2,
                                                                 const graphene_plane_t   *p3,
                                                                 const graphene_plane_t   *p4,
                                                                 const graphene_plane_t   *p5);
GRAPHENE_AVAILABLE_IN_1_2
graphene_frustum_t *    graphene_frustum_init_from_frustum      (graphene_frustum_t       *f,
                                                                 const graphene_frustum_t *src);
GRAPHENE_AVAILABLE_IN_1_2
graphene_frustum_t *    graphene_frustum_init_from_matrix       (graphene_frustum_t       *f,
                                                                 const graphene_matrix_t  *matrix);

GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_frustum_contains_point         (const graphene_frustum_t *f,
                                                                 const graphene_point3d_t *point);

GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_frustum_intersects_sphere      (const graphene_frustum_t *f,
                                                                 const graphene_sphere_t  *sphere);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_frustum_intersects_box         (const graphene_frustum_t *f,
                                                                 const graphene_box_t     *box);

GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_frustum_get_planes             (const graphene_frustum_t *f,
                                                                 graphene_plane_t          planes[]);

GRAPHENE_AVAILABLE_IN_1_6
bool                    graphene_frustum_equal                  (const graphene_frustum_t *a,
                                                                 const graphene_frustum_t *b);

GRAPHENE_END_DECLS
