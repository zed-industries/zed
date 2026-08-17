/* graphene-vec2.h: 2-coords vector
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
 * graphene_vec2_t:
 *
 * A structure capable of holding a vector with two dimensions, x and y.
 *
 * The contents of the #graphene_vec2_t structure are private and should
 * never be accessed directly.
 */
struct _graphene_vec2_t
{
  /*< private >*/
  GRAPHENE_ALIGNED_DECL (GRAPHENE_PRIVATE_FIELD (graphene_simd4f_t, value), 16);
};

GRAPHENE_AVAILABLE_IN_1_0
graphene_vec2_t *       graphene_vec2_alloc             (void);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_vec2_free              (graphene_vec2_t       *v);
GRAPHENE_AVAILABLE_IN_1_0
graphene_vec2_t *       graphene_vec2_init              (graphene_vec2_t       *v,
                                                         float                  x,
                                                         float                  y);
GRAPHENE_AVAILABLE_IN_1_0
graphene_vec2_t *       graphene_vec2_init_from_vec2    (graphene_vec2_t       *v,
                                                         const graphene_vec2_t *src);
GRAPHENE_AVAILABLE_IN_1_0
graphene_vec2_t *       graphene_vec2_init_from_float   (graphene_vec2_t       *v,
                                                         const float           *src);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_vec2_to_float          (const graphene_vec2_t *v,
                                                         float                 *dest);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_vec2_add               (const graphene_vec2_t *a,
                                                         const graphene_vec2_t *b,
                                                         graphene_vec2_t       *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_vec2_subtract          (const graphene_vec2_t *a,
                                                         const graphene_vec2_t *b,
                                                         graphene_vec2_t       *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_vec2_multiply          (const graphene_vec2_t *a,
                                                         const graphene_vec2_t *b,
                                                         graphene_vec2_t       *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_vec2_divide            (const graphene_vec2_t *a,
                                                         const graphene_vec2_t *b,
                                                         graphene_vec2_t       *res);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_vec2_dot               (const graphene_vec2_t *a,
                                                         const graphene_vec2_t *b);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_vec2_length            (const graphene_vec2_t *v);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_vec2_normalize         (const graphene_vec2_t *v,
                                                         graphene_vec2_t       *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_vec2_scale             (const graphene_vec2_t *v,
                                                         float                  factor,
                                                         graphene_vec2_t       *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_vec2_negate            (const graphene_vec2_t *v,
                                                         graphene_vec2_t       *res);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_vec2_near              (const graphene_vec2_t *v1,
                                                         const graphene_vec2_t *v2,
                                                         float                  epsilon);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_vec2_equal             (const graphene_vec2_t *v1,
                                                         const graphene_vec2_t *v2);

GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_vec2_min               (const graphene_vec2_t *a,
                                                         const graphene_vec2_t *b,
                                                         graphene_vec2_t       *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_vec2_max               (const graphene_vec2_t *a,
                                                         const graphene_vec2_t *b,
                                                         graphene_vec2_t       *res);
GRAPHENE_AVAILABLE_IN_1_10
void                    graphene_vec2_interpolate       (const graphene_vec2_t *v1,
                                                         const graphene_vec2_t *v2,
                                                         double                 factor,
                                                         graphene_vec2_t       *res);

GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_vec2_get_x             (const graphene_vec2_t *v);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_vec2_get_y             (const graphene_vec2_t *v);

GRAPHENE_AVAILABLE_IN_1_0
const graphene_vec2_t * graphene_vec2_zero              (void);
GRAPHENE_AVAILABLE_IN_1_0
const graphene_vec2_t * graphene_vec2_one               (void);
GRAPHENE_AVAILABLE_IN_1_0
const graphene_vec2_t * graphene_vec2_x_axis            (void);
GRAPHENE_AVAILABLE_IN_1_0
const graphene_vec2_t * graphene_vec2_y_axis            (void);

GRAPHENE_END_DECLS
