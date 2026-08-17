/* graphene-rect.h: Rectangular type
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
#include "graphene-point.h"
#include "graphene-size.h"
#include "graphene-vec2.h"

GRAPHENE_BEGIN_DECLS

/**
 * GRAPHENE_RECT_INIT:
 * @_x: the X coordinate of the origin
 * @_y: the Y coordinate of the origin
 * @_w: the width
 * @_h: the height
 *
 * Initializes a #graphene_rect_t when declaring it.
 *
 * Since: 1.0
 */
#define GRAPHENE_RECT_INIT(_x,_y,_w,_h) \
  (graphene_rect_t) { .origin = { .x = (_x), .y = (_y) }, .size = { .width = (_w), .height = (_h) } }

/**
 * GRAPHENE_RECT_INIT_ZERO:
 *
 * Initializes a #graphene_rect_t to a degenerate rectangle with an origin
 * in (0, 0) and a size of 0.
 *
 * Since: 1.10
 */
#define GRAPHENE_RECT_INIT_ZERO         GRAPHENE_RECT_INIT (0.f, 0.f, 0.f, 0.f)

/**
 * graphene_rect_t:
 * @origin: the coordinates of the origin of the rectangle
 * @size: the size of the rectangle
 *
 * The location and size of a rectangle region.
 *
 * The width and height of a #graphene_rect_t can be negative; for instance,
 * a #graphene_rect_t with an origin of [ 0, 0 ] and a size of [ 10, 10 ] is
 * equivalent to a #graphene_rect_t with an origin of [ 10, 10 ] and a size
 * of [ -10, -10 ].
 *
 * Application code can normalize rectangles using graphene_rect_normalize();
 * this function will ensure that the width and height of a rectangle are
 * positive values. All functions taking a #graphene_rect_t as an argument
 * will internally operate on a normalized copy; all functions returning a
 * #graphene_rect_t will always return a normalized rectangle.
 *
 * Since: 1.0
 */
struct _graphene_rect_t
{
  graphene_point_t origin;
  graphene_size_t size;
};

GRAPHENE_AVAILABLE_IN_1_0
graphene_rect_t *       graphene_rect_alloc             (void);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_rect_free              (graphene_rect_t       *r);
GRAPHENE_AVAILABLE_IN_1_0
graphene_rect_t *       graphene_rect_init              (graphene_rect_t       *r,
                                                         float                  x,
                                                         float                  y,
                                                         float                  width,
                                                         float                  height);
GRAPHENE_AVAILABLE_IN_1_0
graphene_rect_t *       graphene_rect_init_from_rect    (graphene_rect_t       *r,
                                                         const graphene_rect_t *src);

GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_rect_equal             (const graphene_rect_t *a,
                                                         const graphene_rect_t *b);
GRAPHENE_AVAILABLE_IN_1_0
graphene_rect_t *       graphene_rect_normalize         (graphene_rect_t       *r);
GRAPHENE_AVAILABLE_IN_1_4
void                    graphene_rect_normalize_r       (const graphene_rect_t *r,
                                                         graphene_rect_t       *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_rect_get_center        (const graphene_rect_t *r,
                                                         graphene_point_t      *p);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_rect_get_top_left      (const graphene_rect_t *r,
                                                         graphene_point_t      *p);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_rect_get_top_right     (const graphene_rect_t *r,
                                                         graphene_point_t      *p);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_rect_get_bottom_right  (const graphene_rect_t *r,
                                                         graphene_point_t      *p);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_rect_get_bottom_left   (const graphene_rect_t *r,
                                                         graphene_point_t      *p);
GRAPHENE_AVAILABLE_IN_1_4
void                    graphene_rect_get_vertices      (const graphene_rect_t *r,
                                                         graphene_vec2_t        vertices[]);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_rect_get_x             (const graphene_rect_t *r);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_rect_get_y             (const graphene_rect_t *r);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_rect_get_width         (const graphene_rect_t *r);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_rect_get_height        (const graphene_rect_t *r);
GRAPHENE_AVAILABLE_IN_1_10
float                   graphene_rect_get_area          (const graphene_rect_t *r);

GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_rect_union             (const graphene_rect_t *a,
                                                         const graphene_rect_t *b,
                                                         graphene_rect_t       *res);
GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_rect_intersection      (const graphene_rect_t *a,
                                                         const graphene_rect_t *b,
                                                         graphene_rect_t       *res);
GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_rect_contains_point    (const graphene_rect_t  *r,
                                                         const graphene_point_t *p);
GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_rect_contains_rect     (const graphene_rect_t  *a,
                                                         const graphene_rect_t  *b);
GRAPHENE_AVAILABLE_IN_1_0
graphene_rect_t *       graphene_rect_offset            (graphene_rect_t        *r,
                                                         float                   d_x,
                                                         float                   d_y);
GRAPHENE_AVAILABLE_IN_1_4
void                    graphene_rect_offset_r          (const graphene_rect_t  *r,
                                                         float                   d_x,
                                                         float                   d_y,
                                                         graphene_rect_t        *res);
GRAPHENE_AVAILABLE_IN_1_0
graphene_rect_t *       graphene_rect_inset             (graphene_rect_t        *r,
                                                         float                   d_x,
                                                         float                   d_y);
GRAPHENE_AVAILABLE_IN_1_4
void                    graphene_rect_inset_r           (const graphene_rect_t  *r,
                                                         float                   d_x,
                                                         float                   d_y,
                                                         graphene_rect_t        *res);
GRAPHENE_DEPRECATED_IN_1_4_FOR (graphene_rect_round)
graphene_rect_t *       graphene_rect_round_to_pixel    (graphene_rect_t        *r);
GRAPHENE_DEPRECATED_IN_1_10_FOR (graphene_rect_round_extents)
void                    graphene_rect_round             (const graphene_rect_t  *r,
                                                         graphene_rect_t        *res);
GRAPHENE_AVAILABLE_IN_1_10
void                    graphene_rect_round_extents     (const graphene_rect_t  *r,
                                                         graphene_rect_t        *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_rect_interpolate       (const graphene_rect_t  *a,
                                                         const graphene_rect_t  *b,
                                                         double                  factor,
                                                         graphene_rect_t        *res);

GRAPHENE_AVAILABLE_IN_1_4
void                    graphene_rect_expand            (const graphene_rect_t  *r,
                                                         const graphene_point_t *p,
                                                         graphene_rect_t        *res);

GRAPHENE_AVAILABLE_IN_1_4
const graphene_rect_t * graphene_rect_zero              (void);

GRAPHENE_AVAILABLE_IN_1_10
void                    graphene_rect_scale             (const graphene_rect_t  *r,
                                                         float                   s_h,
                                                         float                   s_v,
                                                         graphene_rect_t        *res);

GRAPHENE_END_DECLS
