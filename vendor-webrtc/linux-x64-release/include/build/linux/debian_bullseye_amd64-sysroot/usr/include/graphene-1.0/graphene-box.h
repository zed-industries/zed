/* graphene-box.h: An axis aligned bounding box
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
 * graphene_box_t:
 *
 * A 3D box, described as the volume between a minimum and
 * a maximum vertices.
 *
 * Since: 1.2
 */
struct _graphene_box_t
{
  /*< private >*/
  GRAPHENE_PRIVATE_FIELD (graphene_vec3_t, min);
  GRAPHENE_PRIVATE_FIELD (graphene_vec3_t, max);
};

GRAPHENE_AVAILABLE_IN_1_2
graphene_box_t *        graphene_box_alloc                      (void);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_box_free                       (graphene_box_t           *box);

GRAPHENE_AVAILABLE_IN_1_2
graphene_box_t *        graphene_box_init                       (graphene_box_t           *box,
                                                                 const graphene_point3d_t *min,
                                                                 const graphene_point3d_t *max);
GRAPHENE_AVAILABLE_IN_1_2
graphene_box_t *        graphene_box_init_from_points           (graphene_box_t           *box,
                                                                 unsigned int              n_points,
                                                                 const graphene_point3d_t *points);
GRAPHENE_AVAILABLE_IN_1_2
graphene_box_t *        graphene_box_init_from_vectors          (graphene_box_t           *box,
                                                                 unsigned int              n_vectors,
                                                                 const graphene_vec3_t    *vectors);
GRAPHENE_AVAILABLE_IN_1_2
graphene_box_t *        graphene_box_init_from_box              (graphene_box_t           *box,
                                                                 const graphene_box_t     *src);
GRAPHENE_AVAILABLE_IN_1_2
graphene_box_t *        graphene_box_init_from_vec3             (graphene_box_t           *box,
                                                                 const graphene_vec3_t    *min,
                                                                 const graphene_vec3_t    *max);

GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_box_expand                     (const graphene_box_t     *box,
                                                                 const graphene_point3d_t *point,
                                                                 graphene_box_t           *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_box_expand_vec3                (const graphene_box_t     *box,
                                                                 const graphene_vec3_t    *vec,
                                                                 graphene_box_t           *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_box_expand_scalar              (const graphene_box_t     *box,
                                                                 float                     scalar,
                                                                 graphene_box_t           *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_box_union                      (const graphene_box_t     *a,
                                                                 const graphene_box_t     *b,
                                                                 graphene_box_t           *res);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_box_intersection               (const graphene_box_t     *a,
                                                                 const graphene_box_t     *b,
                                                                 graphene_box_t           *res);

GRAPHENE_AVAILABLE_IN_1_2
float                   graphene_box_get_width                  (const graphene_box_t     *box);
GRAPHENE_AVAILABLE_IN_1_2
float                   graphene_box_get_height                 (const graphene_box_t     *box);
GRAPHENE_AVAILABLE_IN_1_2
float                   graphene_box_get_depth                  (const graphene_box_t     *box);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_box_get_size                   (const graphene_box_t     *box,
                                                                 graphene_vec3_t          *size);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_box_get_center                 (const graphene_box_t     *box,
                                                                 graphene_point3d_t       *center);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_box_get_min                    (const graphene_box_t     *box,
                                                                 graphene_point3d_t       *min);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_box_get_max                    (const graphene_box_t     *box,
                                                                 graphene_point3d_t       *max);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_box_get_vertices               (const graphene_box_t     *box,
                                                                 graphene_vec3_t           vertices[]);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_box_get_bounding_sphere        (const graphene_box_t     *box,
                                                                 graphene_sphere_t        *sphere);

GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_box_contains_point             (const graphene_box_t     *box,
                                                                 const graphene_point3d_t *point);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_box_contains_box               (const graphene_box_t     *a,
                                                                 const graphene_box_t     *b);

GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_box_equal                      (const graphene_box_t     *a,
                                                                 const graphene_box_t     *b);

GRAPHENE_AVAILABLE_IN_1_2
const graphene_box_t *  graphene_box_zero                       (void);
GRAPHENE_AVAILABLE_IN_1_2
const graphene_box_t *  graphene_box_one                        (void);
GRAPHENE_AVAILABLE_IN_1_2
const graphene_box_t *  graphene_box_minus_one                  (void);
GRAPHENE_AVAILABLE_IN_1_2
const graphene_box_t *  graphene_box_one_minus_one              (void);
GRAPHENE_AVAILABLE_IN_1_2
const graphene_box_t *  graphene_box_infinite                   (void);
GRAPHENE_AVAILABLE_IN_1_2
const graphene_box_t *  graphene_box_empty                      (void);

GRAPHENE_END_DECLS
