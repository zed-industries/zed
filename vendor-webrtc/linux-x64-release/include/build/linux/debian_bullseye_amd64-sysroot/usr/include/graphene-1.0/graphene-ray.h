/* graphene-ray.h: A ray emitted from an origin in a given direction
 *
 * SPDX-License-Identifier: MIT
 *
 * Copyright 2015  Emmanuele Bassi
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
 * graphene_ray_t:
 *
 * A ray emitted from an origin in a given direction.
 *
 * The contents of the `graphene_ray_t` structure are private, and should not
 * be modified directly.
 *
 * Since: 1.4
 */
struct _graphene_ray_t
{
  /*< private >*/
  GRAPHENE_PRIVATE_FIELD (graphene_vec3_t, origin);
  GRAPHENE_PRIVATE_FIELD (graphene_vec3_t, direction);
};

/**
 * graphene_ray_intersection_kind_t:
 * @GRAPHENE_RAY_INTERSECTION_KIND_NONE: No intersection
 * @GRAPHENE_RAY_INTERSECTION_KIND_ENTER: The ray is entering the intersected
 *   object
 * @GRAPHENE_RAY_INTERSECTION_KIND_LEAVE: The ray is leaving the intersected
 *   object
 *
 * The type of intersection.
 *
 * Since: 1.10
 */
typedef enum {
  GRAPHENE_RAY_INTERSECTION_KIND_NONE,
  GRAPHENE_RAY_INTERSECTION_KIND_ENTER,
  GRAPHENE_RAY_INTERSECTION_KIND_LEAVE,
} graphene_ray_intersection_kind_t;

GRAPHENE_AVAILABLE_IN_1_4
graphene_ray_t *                graphene_ray_alloc                  (void);
GRAPHENE_AVAILABLE_IN_1_4
void                            graphene_ray_free                   (graphene_ray_t           *r);

GRAPHENE_AVAILABLE_IN_1_4
graphene_ray_t *                graphene_ray_init                   (graphene_ray_t           *r,
                                                                     const graphene_point3d_t *origin,
                                                                     const graphene_vec3_t    *direction);
GRAPHENE_AVAILABLE_IN_1_4
graphene_ray_t *                graphene_ray_init_from_ray          (graphene_ray_t           *r,
                                                                     const graphene_ray_t     *src);
GRAPHENE_AVAILABLE_IN_1_4
graphene_ray_t *                graphene_ray_init_from_vec3         (graphene_ray_t           *r,
                                                                     const graphene_vec3_t    *origin,
                                                                     const graphene_vec3_t    *direction);
GRAPHENE_AVAILABLE_IN_1_4
void                            graphene_ray_get_origin             (const graphene_ray_t     *r,
                                                                     graphene_point3d_t       *origin);
GRAPHENE_AVAILABLE_IN_1_4
void                            graphene_ray_get_direction          (const graphene_ray_t     *r,
                                                                     graphene_vec3_t          *direction);

GRAPHENE_AVAILABLE_IN_1_4
void                            graphene_ray_get_position_at        (const graphene_ray_t     *r,
                                                                     float                     t,
                                                                     graphene_point3d_t       *position);
GRAPHENE_AVAILABLE_IN_1_4
float                           graphene_ray_get_distance_to_point  (const graphene_ray_t     *r,
                                                                     const graphene_point3d_t *p);
GRAPHENE_AVAILABLE_IN_1_4
float                           graphene_ray_get_distance_to_plane  (const graphene_ray_t     *r,
                                                                     const graphene_plane_t   *p);

GRAPHENE_AVAILABLE_IN_1_4
bool                            graphene_ray_equal                  (const graphene_ray_t     *a,
                                                                     const graphene_ray_t     *b);

GRAPHENE_AVAILABLE_IN_1_4
void                            graphene_ray_get_closest_point_to_point (const graphene_ray_t     *r,
                                                                         const graphene_point3d_t *p,
                                                                         graphene_point3d_t       *res);

GRAPHENE_AVAILABLE_IN_1_10
graphene_ray_intersection_kind_t graphene_ray_intersect_sphere          (const graphene_ray_t    *r,
                                                                         const graphene_sphere_t *s,
                                                                         float                   *t_out);
GRAPHENE_AVAILABLE_IN_1_10
bool                            graphene_ray_intersects_sphere          (const graphene_ray_t    *r,
                                                                         const graphene_sphere_t *s);
GRAPHENE_AVAILABLE_IN_1_10
graphene_ray_intersection_kind_t graphene_ray_intersect_box             (const graphene_ray_t    *r,
                                                                         const graphene_box_t    *b,
                                                                         float                   *t_out);
GRAPHENE_AVAILABLE_IN_1_10
bool                            graphene_ray_intersects_box             (const graphene_ray_t    *r,
                                                                         const graphene_box_t    *b);
GRAPHENE_AVAILABLE_IN_1_10
graphene_ray_intersection_kind_t graphene_ray_intersect_triangle        (const graphene_ray_t      *r,
                                                                         const graphene_triangle_t *t,
                                                                         float                     *t_out);
GRAPHENE_AVAILABLE_IN_1_10
bool                            graphene_ray_intersects_triangle        (const graphene_ray_t      *r,
                                                                         const graphene_triangle_t *t);

GRAPHENE_END_DECLS
