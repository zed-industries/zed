/* graphene-quaternion.h: Quaternion
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
#include "graphene-vec4.h"

GRAPHENE_BEGIN_DECLS

/**
 * graphene_quaternion_t:
 *
 * A quaternion.
 *
 * The contents of the #graphene_quaternion_t structure are private
 * and should never be accessed directly.
 *
 * Since: 1.0
 */
struct _graphene_quaternion_t
{
  /*< private >*/
  GRAPHENE_PRIVATE_FIELD (float, x);
  GRAPHENE_PRIVATE_FIELD (float, y);
  GRAPHENE_PRIVATE_FIELD (float, z);
  GRAPHENE_PRIVATE_FIELD (float, w);
};

GRAPHENE_AVAILABLE_IN_1_0
graphene_quaternion_t * graphene_quaternion_alloc                       (void);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_quaternion_free                        (graphene_quaternion_t       *q);

GRAPHENE_AVAILABLE_IN_1_0
graphene_quaternion_t * graphene_quaternion_init                        (graphene_quaternion_t       *q,
                                                                         float                        x,
                                                                         float                        y,
                                                                         float                        z,
                                                                         float                        w);
GRAPHENE_AVAILABLE_IN_1_0
graphene_quaternion_t * graphene_quaternion_init_identity               (graphene_quaternion_t       *q);
GRAPHENE_AVAILABLE_IN_1_0
graphene_quaternion_t * graphene_quaternion_init_from_quaternion        (graphene_quaternion_t       *q,
                                                                         const graphene_quaternion_t *src);
GRAPHENE_AVAILABLE_IN_1_0
graphene_quaternion_t * graphene_quaternion_init_from_vec4              (graphene_quaternion_t       *q,
                                                                         const graphene_vec4_t       *src);
GRAPHENE_AVAILABLE_IN_1_0
graphene_quaternion_t * graphene_quaternion_init_from_matrix            (graphene_quaternion_t       *q,
                                                                         const graphene_matrix_t     *m);
GRAPHENE_AVAILABLE_IN_1_0
graphene_quaternion_t * graphene_quaternion_init_from_angles            (graphene_quaternion_t       *q,
                                                                         float                        deg_x,
                                                                         float                        deg_y,
                                                                         float                        deg_z);
GRAPHENE_AVAILABLE_IN_1_4
graphene_quaternion_t * graphene_quaternion_init_from_radians           (graphene_quaternion_t       *q,
                                                                         float                        rad_x,
                                                                         float                        rad_y,
                                                                         float                        rad_z);
GRAPHENE_AVAILABLE_IN_1_0
graphene_quaternion_t * graphene_quaternion_init_from_angle_vec3        (graphene_quaternion_t       *q,
                                                                         float                        angle,
                                                                         const graphene_vec3_t       *axis);
GRAPHENE_AVAILABLE_IN_1_2
graphene_quaternion_t * graphene_quaternion_init_from_euler             (graphene_quaternion_t       *q,
                                                                         const graphene_euler_t      *e);

GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_quaternion_to_vec4                     (const graphene_quaternion_t *q,
                                                                         graphene_vec4_t             *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_quaternion_to_matrix                   (const graphene_quaternion_t *q,
                                                                         graphene_matrix_t           *m);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_quaternion_to_angles                   (const graphene_quaternion_t *q,
                                                                         float                       *deg_x,
                                                                         float                       *deg_y,
                                                                         float                       *deg_z);
GRAPHENE_AVAILABLE_IN_1_4
void                    graphene_quaternion_to_radians                  (const graphene_quaternion_t *q,
                                                                         float                       *rad_x,
                                                                         float                       *rad_y,
                                                                         float                       *rad_z);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_quaternion_to_angle_vec3               (const graphene_quaternion_t *q,
                                                                         float                       *angle,
                                                                         graphene_vec3_t             *axis);

GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_quaternion_equal                       (const graphene_quaternion_t *a,
                                                                         const graphene_quaternion_t *b);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_quaternion_dot                         (const graphene_quaternion_t *a,
                                                                         const graphene_quaternion_t *b);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_quaternion_invert                      (const graphene_quaternion_t *q,
                                                                         graphene_quaternion_t       *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_quaternion_normalize                   (const graphene_quaternion_t *q,
                                                                         graphene_quaternion_t       *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_quaternion_slerp                       (const graphene_quaternion_t *a,
                                                                         const graphene_quaternion_t *b,
                                                                         float                        factor,
                                                                         graphene_quaternion_t       *res);
GRAPHENE_AVAILABLE_IN_1_10
void                    graphene_quaternion_multiply                    (const graphene_quaternion_t *a,
                                                                         const graphene_quaternion_t *b,
                                                                         graphene_quaternion_t       *res);
GRAPHENE_AVAILABLE_IN_1_10
void                    graphene_quaternion_scale                       (const graphene_quaternion_t *q,
                                                                         float                        factor,
                                                                         graphene_quaternion_t       *res);
GRAPHENE_AVAILABLE_IN_1_10
void                    graphene_quaternion_add                         (const graphene_quaternion_t *a,
                                                                         const graphene_quaternion_t *b,
                                                                         graphene_quaternion_t       *res);

GRAPHENE_END_DECLS
