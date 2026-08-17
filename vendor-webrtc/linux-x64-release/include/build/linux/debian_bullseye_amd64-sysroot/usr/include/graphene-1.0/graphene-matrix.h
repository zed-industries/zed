/* graphene-matrix.h: 4x4 matrix
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

#include "graphene-types.h"

GRAPHENE_BEGIN_DECLS

/**
 * graphene_matrix_t:
 *
 * A structure capable of holding a 4x4 matrix.
 *
 * The contents of the #graphene_matrix_t structure are private and
 * should never be accessed directly.
 */
struct _graphene_matrix_t
{
  /*< private >*/
  GRAPHENE_ALIGNED_DECL (GRAPHENE_PRIVATE_FIELD (graphene_simd4x4f_t, value), 16);
};

GRAPHENE_AVAILABLE_IN_1_0
graphene_matrix_t *     graphene_matrix_alloc                   (void);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_free                    (graphene_matrix_t        *m);

GRAPHENE_AVAILABLE_IN_1_0
graphene_matrix_t *     graphene_matrix_init_identity           (graphene_matrix_t        *m);
GRAPHENE_AVAILABLE_IN_1_0
graphene_matrix_t *     graphene_matrix_init_from_float         (graphene_matrix_t        *m,
                                                                 const float              *v);
GRAPHENE_AVAILABLE_IN_1_0
graphene_matrix_t *     graphene_matrix_init_from_vec4          (graphene_matrix_t        *m,
                                                                 const graphene_vec4_t    *v0,
                                                                 const graphene_vec4_t    *v1,
                                                                 const graphene_vec4_t    *v2,
                                                                 const graphene_vec4_t    *v3);
GRAPHENE_AVAILABLE_IN_1_0
graphene_matrix_t *     graphene_matrix_init_from_matrix        (graphene_matrix_t        *m,
                                                                 const graphene_matrix_t  *src);
GRAPHENE_AVAILABLE_IN_1_0
graphene_matrix_t *     graphene_matrix_init_perspective        (graphene_matrix_t        *m,
                                                                 float                     fovy,
                                                                 float                     aspect,
                                                                 float                     z_near,
                                                                 float                     z_far);
GRAPHENE_AVAILABLE_IN_1_0
graphene_matrix_t *     graphene_matrix_init_ortho              (graphene_matrix_t        *m,
                                                                 float                     left,
                                                                 float                     right,
                                                                 float                     top,
                                                                 float                     bottom,
                                                                 float                     z_near,
                                                                 float                     z_far);
GRAPHENE_AVAILABLE_IN_1_0
graphene_matrix_t *     graphene_matrix_init_look_at            (graphene_matrix_t        *m,
                                                                 const graphene_vec3_t    *eye,
                                                                 const graphene_vec3_t    *center,
                                                                 const graphene_vec3_t    *up);
GRAPHENE_AVAILABLE_IN_1_2
graphene_matrix_t *     graphene_matrix_init_frustum            (graphene_matrix_t        *m,
                                                                 float                     left,
                                                                 float                     right,
                                                                 float                     bottom,
                                                                 float                     top,
                                                                 float                     z_near,
                                                                 float                     z_far);
GRAPHENE_AVAILABLE_IN_1_0
graphene_matrix_t *     graphene_matrix_init_scale              (graphene_matrix_t        *m,
                                                                 float                     x,
                                                                 float                     y,
                                                                 float                     z);
GRAPHENE_AVAILABLE_IN_1_0
graphene_matrix_t *     graphene_matrix_init_translate          (graphene_matrix_t        *m,
                                                                 const graphene_point3d_t *p);
GRAPHENE_AVAILABLE_IN_1_0
graphene_matrix_t *     graphene_matrix_init_rotate             (graphene_matrix_t        *m,
                                                                 float                     angle,
                                                                 const graphene_vec3_t    *axis);
GRAPHENE_AVAILABLE_IN_1_0
graphene_matrix_t *     graphene_matrix_init_skew               (graphene_matrix_t        *m,
                                                                 float                     x_skew,
                                                                 float                     y_skew);
GRAPHENE_AVAILABLE_IN_1_0
graphene_matrix_t *     graphene_matrix_init_from_2d            (graphene_matrix_t        *m,
                                                                 double                    xx,
                                                                 double                    yx,
                                                                 double                    xy,
                                                                 double                    yy,
                                                                 double                    x_0,
                                                                 double                    y_0);

GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_matrix_is_identity             (const graphene_matrix_t  *m);
GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_matrix_is_2d                   (const graphene_matrix_t  *m);
GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_matrix_is_backface_visible     (const graphene_matrix_t  *m);
GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_matrix_is_singular             (const graphene_matrix_t  *m);

GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_to_float                (const graphene_matrix_t  *m,
                                                                 float                    *v);
GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_matrix_to_2d                   (const graphene_matrix_t  *m,
                                                                 double                   *xx,
                                                                 double                   *yx,
                                                                 double                   *xy,
                                                                 double                   *yy,
                                                                 double                   *x_0,
                                                                 double                   *y_0);


GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_get_row                 (const graphene_matrix_t  *m,
                                                                 unsigned int              index_,
                                                                 graphene_vec4_t          *res);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_matrix_get_value               (const graphene_matrix_t  *m,
                                                                 unsigned int              row,
                                                                 unsigned int              col);

GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_multiply                (const graphene_matrix_t  *a,
                                                                 const graphene_matrix_t  *b,
                                                                 graphene_matrix_t        *res);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_matrix_determinant             (const graphene_matrix_t  *m);

GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_transform_vec4          (const graphene_matrix_t  *m,
                                                                 const graphene_vec4_t    *v,
                                                                 graphene_vec4_t          *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_transform_vec3          (const graphene_matrix_t  *m,
                                                                 const graphene_vec3_t    *v,
                                                                 graphene_vec3_t          *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_transform_point         (const graphene_matrix_t  *m,
                                                                 const graphene_point_t   *p,
                                                                 graphene_point_t         *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_matrix_transform_point3d       (const graphene_matrix_t  *m,
                                                                 const graphene_point3d_t *p,
                                                                 graphene_point3d_t       *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_transform_rect          (const graphene_matrix_t  *m,
                                                                 const graphene_rect_t    *r,
                                                                 graphene_quad_t          *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_transform_bounds        (const graphene_matrix_t  *m,
                                                                 const graphene_rect_t    *r,
                                                                 graphene_rect_t          *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_matrix_transform_sphere        (const graphene_matrix_t  *m,
                                                                 const graphene_sphere_t  *s,
                                                                 graphene_sphere_t        *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_matrix_transform_box           (const graphene_matrix_t  *m,
                                                                 const graphene_box_t     *b,
                                                                 graphene_box_t           *res);
GRAPHENE_AVAILABLE_IN_1_4
void                    graphene_matrix_transform_ray           (const graphene_matrix_t  *m,
                                                                 const graphene_ray_t     *r,
                                                                 graphene_ray_t           *res);

GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_project_point           (const graphene_matrix_t  *m,
                                                                 const graphene_point_t   *p,
                                                                 graphene_point_t         *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_project_rect_bounds     (const graphene_matrix_t  *m,
                                                                 const graphene_rect_t    *r,
                                                                 graphene_rect_t          *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_matrix_project_rect            (const graphene_matrix_t  *m,
                                                                 const graphene_rect_t    *r,
                                                                 graphene_quad_t          *res);
GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_matrix_untransform_point       (const graphene_matrix_t  *m,
                                                                 const graphene_point_t   *p,
                                                                 const graphene_rect_t    *bounds,
                                                                 graphene_point_t         *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_untransform_bounds      (const graphene_matrix_t  *m,
                                                                 const graphene_rect_t    *r,
                                                                 const graphene_rect_t    *bounds,
                                                                 graphene_rect_t          *res);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_matrix_unproject_point3d       (const graphene_matrix_t  *projection,
                                                                 const graphene_matrix_t  *modelview,
                                                                 const graphene_point3d_t *point,
                                                                 graphene_point3d_t       *res);

GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_translate               (graphene_matrix_t        *m,
                                                                 const graphene_point3d_t *pos);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_rotate                  (graphene_matrix_t        *m,
                                                                 float                     angle,
                                                                 const graphene_vec3_t    *axis);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_rotate_x                (graphene_matrix_t        *m,
                                                                 float                     angle);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_rotate_y                (graphene_matrix_t        *m,
                                                                 float                     angle);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_rotate_z                (graphene_matrix_t        *m,
                                                                 float                     angle);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_matrix_rotate_quaternion       (graphene_matrix_t        *m,
                                                                 const graphene_quaternion_t *q);
GRAPHENE_AVAILABLE_IN_1_2
void                    graphene_matrix_rotate_euler            (graphene_matrix_t        *m,
                                                                 const graphene_euler_t   *e);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_scale                   (graphene_matrix_t        *m,
                                                                 float                     factor_x,
                                                                 float                     factor_y,
                                                                 float                     factor_z);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_skew_xy                 (graphene_matrix_t        *m,
                                                                 float                     factor);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_skew_xz                 (graphene_matrix_t        *m,
                                                                 float                     factor);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_skew_yz                 (graphene_matrix_t        *m,
                                                                 float                     factor);

GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_transpose               (const graphene_matrix_t  *m,
                                                                 graphene_matrix_t        *res);
GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_matrix_inverse                 (const graphene_matrix_t  *m,
                                                                 graphene_matrix_t        *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_perspective             (const graphene_matrix_t  *m,
                                                                 float                     depth,
                                                                 graphene_matrix_t        *res);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_normalize               (const graphene_matrix_t  *m,
                                                                 graphene_matrix_t        *res);

GRAPHENE_AVAILABLE_IN_1_10
float                   graphene_matrix_get_x_translation       (const graphene_matrix_t  *m);
GRAPHENE_AVAILABLE_IN_1_10
float                   graphene_matrix_get_y_translation       (const graphene_matrix_t  *m);
GRAPHENE_AVAILABLE_IN_1_10
float                   graphene_matrix_get_z_translation       (const graphene_matrix_t  *m);

GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_matrix_get_x_scale             (const graphene_matrix_t  *m);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_matrix_get_y_scale             (const graphene_matrix_t  *m);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_matrix_get_z_scale             (const graphene_matrix_t  *m);

GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_interpolate             (const graphene_matrix_t  *a,
                                                                 const graphene_matrix_t  *b,
                                                                 double                    factor,
                                                                 graphene_matrix_t        *res);

GRAPHENE_AVAILABLE_IN_1_10
bool                    graphene_matrix_near                    (const graphene_matrix_t  *a,
                                                                 const graphene_matrix_t  *b,
                                                                 float                     epsilon);
GRAPHENE_AVAILABLE_IN_1_10
bool                    graphene_matrix_equal                   (const graphene_matrix_t  *a,
                                                                 const graphene_matrix_t  *b);
GRAPHENE_AVAILABLE_IN_1_10
bool                    graphene_matrix_equal_fast              (const graphene_matrix_t  *a,
                                                                 const graphene_matrix_t  *b);

GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_matrix_print                   (const graphene_matrix_t  *m);

GRAPHENE_AVAILABLE_IN_1_10
bool                    graphene_matrix_decompose               (const graphene_matrix_t  *m,
                                                                 graphene_vec3_t          *translate,
                                                                 graphene_vec3_t          *scale,
                                                                 graphene_quaternion_t    *rotate,
                                                                 graphene_vec3_t          *shear,
                                                                 graphene_vec4_t          *perspective);

GRAPHENE_END_DECLS
