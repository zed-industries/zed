/* graphene-size.h: Size
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
 * GRAPHENE_SIZE_INIT:
 * @_w: the width
 * @_h: the height
 *
 * Initializes a #graphene_size_t with the given sizes when
 * declaring it, e.g.:
 *
 * |[<!-- language="C" -->
 *   graphene_size_t size = GRAPHENE_SIZE_INIT (100.f, 100.f);
 * ]|
 *
 * Since: 1.0
 */
#define GRAPHENE_SIZE_INIT(_w,_h)       (graphene_size_t) { .width = (_w), .height = (_h) }

/**
 * GRAPHENE_SIZE_INIT_ZERO:
 *
 * Initializes a #graphene_size_t to (0, 0) when declaring it.
 *
 * Since: 1.0
 */
#define GRAPHENE_SIZE_INIT_ZERO         GRAPHENE_SIZE_INIT (0.f, 0.f)

/**
 * graphene_size_t:
 * @width: the width
 * @height: the height
 *
 * A size.
 *
 * Since: 1.0
 */
struct _graphene_size_t
{
  float width;
  float height;
};

GRAPHENE_AVAILABLE_IN_1_0
graphene_size_t *               graphene_size_alloc             (void);
GRAPHENE_AVAILABLE_IN_1_0
void                            graphene_size_free              (graphene_size_t        *s);
GRAPHENE_AVAILABLE_IN_1_0
graphene_size_t *               graphene_size_init              (graphene_size_t        *s,
                                                                 float                   width,
                                                                 float                   height);
GRAPHENE_AVAILABLE_IN_1_0
graphene_size_t *               graphene_size_init_from_size    (graphene_size_t        *s,
                                                                 const graphene_size_t  *src);
GRAPHENE_AVAILABLE_IN_1_0
bool                            graphene_size_equal             (const graphene_size_t  *a,
                                                                 const graphene_size_t  *b);

GRAPHENE_AVAILABLE_IN_1_0
void                            graphene_size_scale             (const graphene_size_t  *s,
                                                                 float                   factor,
                                                                 graphene_size_t        *res);
GRAPHENE_AVAILABLE_IN_1_0
void                            graphene_size_interpolate       (const graphene_size_t  *a,
                                                                 const graphene_size_t  *b,
                                                                 double                  factor,
                                                                 graphene_size_t        *res);

GRAPHENE_AVAILABLE_IN_1_0
const graphene_size_t *         graphene_size_zero              (void);

GRAPHENE_END_DECLS
