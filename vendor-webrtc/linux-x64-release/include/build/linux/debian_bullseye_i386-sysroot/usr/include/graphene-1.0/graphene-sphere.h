/* graphene-sphere.h: A sphere
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
 * graphene_sphere_t:
 *
 * A sphere, represented by its center and radius.
 *
 * Since: 1.2
 */
struct _graphene_sphere_t
{
  /*< private >*/
  GRAPHENE_PRIVATE_FIELD (graphene_vec3_t, center);
  GRAPHENE_PRIVATE_FIELD (float, radius);
};

GRAPHENE_AVAILABLE_IN_1_2
graphene_sphere_t *     graphene_sphere_alloc                   (void);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_sphere_free                    (graphene_sphere_t        *s);

GRAPHENE_AVAILABLE_IN_1_2
graphene_sphere_t *     graphene_sphere_init                    (graphene_sphere_t        *s,
                                                                 const graphene_point3d_t *center,
                                                                 float                     radius);

GRAPHENE_AVAILABLE_IN_1_2
graphene_sphere_t *     graphene_sphere_init_from_points        (graphene_sphere_t        *s,
                                                                 unsigned int              n_points,
                                                                 const graphene_point3d_t *points,
                                                                 const graphene_point3d_t *center);
GRAPHENE_AVAILABLE_IN_1_2
graphene_sphere_t *     graphene_sphere_init_from_vectors       (graphene_sphere_t        *s,
                                                                 unsigned int              n_vectors,
                                                                 const graphene_vec3_t    *vectors,
                                                                 const graphene_point3d_t *center);

GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_sphere_get_center              (const graphene_sphere_t  *s,
                                                                 graphene_point3d_t       *center);
GRAPHENE_AVAILABLE_IN_1_2
float                   graphene_sphere_get_radius              (const graphene_sphere_t  *s);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_sphere_is_empty                (const graphene_sphere_t  *s);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_sphere_equal                   (const graphene_sphere_t  *a,
                                                                 const graphene_sphere_t  *b);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_sphere_contains_point          (const graphene_sphere_t  *s,
                                                                 const graphene_point3d_t *point);
GRAPHENE_AVAILABLE_IN_1_2
float                   graphene_sphere_distance                (const graphene_sphere_t  *s,
                                                                 const graphene_point3d_t *point);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_sphere_get_bounding_box        (const graphene_sphere_t  *s,
                                                                 graphene_box_t           *box);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_sphere_translate               (const graphene_sphere_t  *s,
                                                                 const graphene_point3d_t *point,
                                                                 graphene_sphere_t        *res);

GRAPHENE_END_DECLS
