/*
 * Copyright © 2019 Benjamin Otte
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors: Benjamin Otte <otte@gnome.org>
 */


#ifndef __GSK_TRANSFORM_H__
#define __GSK_TRANSFORM_H__

#if !defined (__GSK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gsk/gsk.h> can be included directly."
#endif

#include <gsk/gskenums.h>
#include <gsk/gsktypes.h>

G_BEGIN_DECLS

#define GSK_TYPE_TRANSFORM (gsk_transform_get_type ())

GDK_AVAILABLE_IN_ALL
GType                   gsk_transform_get_type                  (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GskTransform *          gsk_transform_ref                       (GskTransform                   *self);
GDK_AVAILABLE_IN_ALL
void                    gsk_transform_unref                     (GskTransform                   *self);

GDK_AVAILABLE_IN_ALL
void                    gsk_transform_print                     (GskTransform                   *self,
                                                                 GString                        *string);
GDK_AVAILABLE_IN_ALL
char *                  gsk_transform_to_string                 (GskTransform                   *self);
GDK_AVAILABLE_IN_ALL
gboolean                gsk_transform_parse                     (const char                     *string,
                                                                 GskTransform                  **out_transform);
GDK_AVAILABLE_IN_ALL
void                    gsk_transform_to_matrix                 (GskTransform                   *self,
                                                                 graphene_matrix_t              *out_matrix);
GDK_AVAILABLE_IN_ALL
void                    gsk_transform_to_2d                     (GskTransform                   *self,
                                                                 float                          *out_xx,
                                                                 float                          *out_yx,
                                                                 float                          *out_xy,
                                                                 float                          *out_yy,
                                                                 float                          *out_dx,
                                                                 float                          *out_dy);
GDK_AVAILABLE_IN_4_6
void                    gsk_transform_to_2d_components          (GskTransform                   *self,
                                                                 float                          *out_skew_x,
                                                                 float                          *out_skew_y,
                                                                 float                          *out_scale_x,
                                                                 float                          *out_scale_y,
                                                                 float                          *out_angle,
                                                                 float                          *out_dx,
                                                                 float                          *out_dy);
GDK_AVAILABLE_IN_ALL
void                    gsk_transform_to_affine                 (GskTransform                   *self,
                                                                 float                          *out_scale_x,
                                                                 float                          *out_scale_y,
                                                                 float                          *out_dx,
                                                                 float                          *out_dy);
GDK_AVAILABLE_IN_ALL
void                    gsk_transform_to_translate              (GskTransform                   *self,
                                                                 float                          *out_dx,
                                                                 float                          *out_dy);

GDK_AVAILABLE_IN_ALL
GskTransformCategory    gsk_transform_get_category              (GskTransform                   *self) G_GNUC_PURE;
GDK_AVAILABLE_IN_ALL
gboolean                gsk_transform_equal                     (GskTransform                   *first,
                                                                 GskTransform                   *second) G_GNUC_PURE;

GDK_AVAILABLE_IN_ALL
GskTransform *          gsk_transform_new                       (void);
GDK_AVAILABLE_IN_ALL
GskTransform *          gsk_transform_transform                 (GskTransform                   *next,
                                                                 GskTransform                   *other) G_GNUC_WARN_UNUSED_RESULT;
GDK_AVAILABLE_IN_ALL
GskTransform *          gsk_transform_invert                    (GskTransform                   *self) G_GNUC_WARN_UNUSED_RESULT;
GDK_AVAILABLE_IN_ALL
GskTransform *          gsk_transform_matrix                    (GskTransform                   *next,
                                                                 const graphene_matrix_t        *matrix) G_GNUC_WARN_UNUSED_RESULT;
GDK_AVAILABLE_IN_ALL
GskTransform *          gsk_transform_translate                 (GskTransform                   *next,
                                                                 const graphene_point_t         *point) G_GNUC_WARN_UNUSED_RESULT;
GDK_AVAILABLE_IN_ALL
GskTransform *          gsk_transform_translate_3d              (GskTransform                   *next,
                                                                 const graphene_point3d_t       *point) G_GNUC_WARN_UNUSED_RESULT;
GDK_AVAILABLE_IN_4_6
GskTransform *          gsk_transform_skew                      (GskTransform                   *next,
                                                                 float                           skew_x,
                                                                 float                           skew_y) G_GNUC_WARN_UNUSED_RESULT;
GDK_AVAILABLE_IN_ALL
GskTransform *          gsk_transform_rotate                    (GskTransform                   *next,
                                                                 float                           angle) G_GNUC_WARN_UNUSED_RESULT;
GDK_AVAILABLE_IN_ALL
GskTransform *          gsk_transform_rotate_3d                 (GskTransform                   *next,
                                                                 float                           angle,
                                                                 const graphene_vec3_t          *axis) G_GNUC_WARN_UNUSED_RESULT;
GDK_AVAILABLE_IN_ALL
GskTransform *          gsk_transform_scale                     (GskTransform                   *next,
                                                                 float                           factor_x,
                                                                 float                           factor_y) G_GNUC_WARN_UNUSED_RESULT;
GDK_AVAILABLE_IN_ALL
GskTransform *          gsk_transform_scale_3d                  (GskTransform                   *next,
                                                                 float                           factor_x,
                                                                 float                           factor_y,
                                                                 float                           factor_z) G_GNUC_WARN_UNUSED_RESULT;
GDK_AVAILABLE_IN_ALL
GskTransform *          gsk_transform_perspective               (GskTransform                   *next,
                                                                 float                           depth) G_GNUC_WARN_UNUSED_RESULT;

GDK_AVAILABLE_IN_ALL
void                    gsk_transform_transform_bounds          (GskTransform                   *self,
                                                                 const graphene_rect_t          *rect,
                                                                 graphene_rect_t                *out_rect);
GDK_AVAILABLE_IN_ALL
void                    gsk_transform_transform_point           (GskTransform                   *self,
                                                                 const graphene_point_t          *point,
                                                                 graphene_point_t                *out_point);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GskTransform, gsk_transform_unref)

G_END_DECLS

#endif /* __GSK_TRANSFORM_H__ */
