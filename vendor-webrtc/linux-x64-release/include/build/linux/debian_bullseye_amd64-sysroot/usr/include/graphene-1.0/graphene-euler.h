/* graphene-euler.h: Euler angles
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
 * graphene_euler_order_t:
 * @GRAPHENE_EULER_ORDER_DEFAULT: Rotate in the default order; the
 *   default order is one of the following enumeration values
 * @GRAPHENE_EULER_ORDER_XYZ: Rotate in the X, Y, and Z order. Deprecated in
 *   Graphene 1.10, it's an alias for %GRAPHENE_EULER_ORDER_SXYZ
 * @GRAPHENE_EULER_ORDER_YZX: Rotate in the Y, Z, and X order. Deprecated in
 *   Graphene 1.10, it's an alias for %GRAPHENE_EULER_ORDER_SYZX
 * @GRAPHENE_EULER_ORDER_ZXY: Rotate in the Z, X, and Y order. Deprecated in
 *   Graphene 1.10, it's an alias for %GRAPHENE_EULER_ORDER_SZXY
 * @GRAPHENE_EULER_ORDER_XZY: Rotate in the X, Z, and Y order. Deprecated in
 *   Graphene 1.10, it's an alias for %GRAPHENE_EULER_ORDER_SXZY
 * @GRAPHENE_EULER_ORDER_YXZ: Rotate in the Y, X, and Z order. Deprecated in
 *   Graphene 1.10, it's an alias for %GRAPHENE_EULER_ORDER_SYXZ
 * @GRAPHENE_EULER_ORDER_ZYX: Rotate in the Z, Y, and X order. Deprecated in
 *   Graphene 1.10, it's an alias for %GRAPHENE_EULER_ORDER_SZYX
 * @GRAPHENE_EULER_ORDER_SXYZ: Defines a static rotation along the X, Y, and Z axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_SXYX: Defines a static rotation along the X, Y, and X axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_SXZY: Defines a static rotation along the X, Z, and Y axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_SXZX: Defines a static rotation along the X, Z, and X axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_SYZX: Defines a static rotation along the Y, Z, and X axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_SYZY: Defines a static rotation along the Y, Z, and Y axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_SYXZ: Defines a static rotation along the Y, X, and Z axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_SYXY: Defines a static rotation along the Y, X, and Y axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_SZXY: Defines a static rotation along the Z, X, and Y axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_SZXZ: Defines a static rotation along the Z, X, and Z axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_SZYX: Defines a static rotation along the Z, Y, and X axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_SZYZ: Defines a static rotation along the Z, Y, and Z axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_RZYX: Defines a relative rotation along the Z, Y, and X axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_RXYX: Defines a relative rotation along the X, Y, and X axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_RYZX: Defines a relative rotation along the Y, Z, and X axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_RXZX: Defines a relative rotation along the X, Z, and X axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_RXZY: Defines a relative rotation along the X, Z, and Y axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_RYZY: Defines a relative rotation along the Y, Z, and Y axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_RZXY: Defines a relative rotation along the Z, X, and Y axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_RYXY: Defines a relative rotation along the Y, X, and Y axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_RYXZ: Defines a relative rotation along the Y, X, and Z axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_RZXZ: Defines a relative rotation along the Z, X, and Z axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_RXYZ: Defines a relative rotation along the X, Y, and Z axes (Since: 1.10)
 * @GRAPHENE_EULER_ORDER_RZYZ: Defines a relative rotation along the Z, Y, and Z axes (Since: 1.10)
 *
 * Specify the order of the rotations on each axis.
 *
 * The %GRAPHENE_EULER_ORDER_DEFAULT value is special, and is used
 * as an alias for one of the other orders.
 *
 * Since: 1.2
 */
typedef enum {
  GRAPHENE_EULER_ORDER_DEFAULT = -1,

  /* Deprecated */
  GRAPHENE_EULER_ORDER_XYZ = 0,
  GRAPHENE_EULER_ORDER_YZX,
  GRAPHENE_EULER_ORDER_ZXY,
  GRAPHENE_EULER_ORDER_XZY,
  GRAPHENE_EULER_ORDER_YXZ,
  GRAPHENE_EULER_ORDER_ZYX,

  /* Static (extrinsic) coordinate axes */
  GRAPHENE_EULER_ORDER_SXYZ,
  GRAPHENE_EULER_ORDER_SXYX,
  GRAPHENE_EULER_ORDER_SXZY,
  GRAPHENE_EULER_ORDER_SXZX,
  GRAPHENE_EULER_ORDER_SYZX,
  GRAPHENE_EULER_ORDER_SYZY,
  GRAPHENE_EULER_ORDER_SYXZ,
  GRAPHENE_EULER_ORDER_SYXY,
  GRAPHENE_EULER_ORDER_SZXY,
  GRAPHENE_EULER_ORDER_SZXZ,
  GRAPHENE_EULER_ORDER_SZYX,
  GRAPHENE_EULER_ORDER_SZYZ,

  /* Relative (intrinsic) coordinate axes */
  GRAPHENE_EULER_ORDER_RZYX,
  GRAPHENE_EULER_ORDER_RXYX,
  GRAPHENE_EULER_ORDER_RYZX,
  GRAPHENE_EULER_ORDER_RXZX,
  GRAPHENE_EULER_ORDER_RXZY,
  GRAPHENE_EULER_ORDER_RYZY,
  GRAPHENE_EULER_ORDER_RZXY,
  GRAPHENE_EULER_ORDER_RYXY,
  GRAPHENE_EULER_ORDER_RYXZ,
  GRAPHENE_EULER_ORDER_RZXZ,
  GRAPHENE_EULER_ORDER_RXYZ,
  GRAPHENE_EULER_ORDER_RZYZ
} graphene_euler_order_t;

/**
 * graphene_euler_t:
 *
 * Describe a rotation using Euler angles.
 *
 * The contents of the #graphene_euler_t structure are private
 * and should never be accessed directly.
 *
 * Since: 1.2
 */
struct _graphene_euler_t
{
  /*< private >*/
  GRAPHENE_PRIVATE_FIELD (graphene_vec3_t, angles);
  GRAPHENE_PRIVATE_FIELD (graphene_euler_order_t, order);
};

GRAPHENE_AVAILABLE_IN_1_2
graphene_euler_t *      graphene_euler_alloc                    (void);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_euler_free                     (graphene_euler_t            *e);

GRAPHENE_AVAILABLE_IN_1_2
graphene_euler_t *      graphene_euler_init                     (graphene_euler_t            *e,
                                                                 float                        x,
                                                                 float                        y,
                                                                 float                        z);
GRAPHENE_AVAILABLE_IN_1_2
graphene_euler_t *      graphene_euler_init_with_order          (graphene_euler_t            *e,
                                                                 float                        x,
                                                                 float                        y,
                                                                 float                        z,
                                                                 graphene_euler_order_t       order);
GRAPHENE_AVAILABLE_IN_1_2
graphene_euler_t *      graphene_euler_init_from_matrix         (graphene_euler_t            *e,
                                                                 const graphene_matrix_t     *m,
                                                                 graphene_euler_order_t       order);
GRAPHENE_AVAILABLE_IN_1_2
graphene_euler_t *      graphene_euler_init_from_quaternion     (graphene_euler_t            *e,
                                                                 const graphene_quaternion_t *q,
                                                                 graphene_euler_order_t       order);
GRAPHENE_AVAILABLE_IN_1_2
graphene_euler_t *      graphene_euler_init_from_vec3           (graphene_euler_t            *e,
                                                                 const graphene_vec3_t       *v,
                                                                 graphene_euler_order_t       order);
GRAPHENE_AVAILABLE_IN_1_2
graphene_euler_t *      graphene_euler_init_from_euler          (graphene_euler_t            *e,
                                                                 const graphene_euler_t      *src);
GRAPHENE_AVAILABLE_IN_1_10
graphene_euler_t *      graphene_euler_init_from_radians        (graphene_euler_t            *e,
                                                                 float                        x,
                                                                 float                        y,
                                                                 float                        z,
                                                                 graphene_euler_order_t       order);

GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_euler_equal                    (const graphene_euler_t      *a,
                                                                 const graphene_euler_t      *b);

GRAPHENE_AVAILABLE_IN_1_2
float                   graphene_euler_get_x                    (const graphene_euler_t      *e);
GRAPHENE_AVAILABLE_IN_1_2
float                   graphene_euler_get_y                    (const graphene_euler_t      *e);
GRAPHENE_AVAILABLE_IN_1_2
float                   graphene_euler_get_z                    (const graphene_euler_t      *e);
GRAPHENE_AVAILABLE_IN_1_2
graphene_euler_order_t  graphene_euler_get_order                (const graphene_euler_t      *e);

GRAPHENE_AVAILABLE_IN_1_10
float                   graphene_euler_get_alpha                (const graphene_euler_t      *e);
GRAPHENE_AVAILABLE_IN_1_10
float                   graphene_euler_get_beta                 (const graphene_euler_t      *e);
GRAPHENE_AVAILABLE_IN_1_10
float                   graphene_euler_get_gamma                (const graphene_euler_t      *e);

GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_euler_to_vec3                  (const graphene_euler_t      *e,
                                                                 graphene_vec3_t             *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_euler_to_matrix                (const graphene_euler_t      *e,
                                                                 graphene_matrix_t           *res);
GRAPHENE_AVAILABLE_IN_1_10
void                    graphene_euler_to_quaternion            (const graphene_euler_t      *e,
                                                                 graphene_quaternion_t       *res);

GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_euler_reorder                  (const graphene_euler_t      *e,
                                                                 graphene_euler_order_t       order,
                                                                 graphene_euler_t            *res);

GRAPHENE_END_DECLS
