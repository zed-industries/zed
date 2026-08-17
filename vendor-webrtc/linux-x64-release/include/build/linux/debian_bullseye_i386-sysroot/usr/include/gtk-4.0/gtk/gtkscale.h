/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_SCALE_H__
#define __GTK_SCALE_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkrange.h>


G_BEGIN_DECLS

#define GTK_TYPE_SCALE            (gtk_scale_get_type ())
#define GTK_SCALE(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SCALE, GtkScale))
#define GTK_SCALE_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SCALE, GtkScaleClass))
#define GTK_IS_SCALE(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SCALE))
#define GTK_IS_SCALE_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SCALE))
#define GTK_SCALE_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SCALE, GtkScaleClass))


typedef struct _GtkScale              GtkScale;
typedef struct _GtkScaleClass         GtkScaleClass;

struct _GtkScale
{
  GtkRange parent_instance;
};

struct _GtkScaleClass
{
  GtkRangeClass parent_class;

  void (* get_layout_offsets) (GtkScale *scale,
                               int      *x,
                               int      *y);

  /*< private >*/

  gpointer padding[8];
};


/**
 * GtkScaleFormatValueFunc:
 * @scale: The `GtkScale`
 * @value: The numeric value to format
 * @user_data: (closure): user data
 *
 * Returns: (not nullable): A newly allocated string describing a textual representation
 *   of the given numerical value.
 */
typedef char * (*GtkScaleFormatValueFunc) (GtkScale *scale,
                                           double    value,
                                           gpointer  user_data);


GDK_AVAILABLE_IN_ALL
GType             gtk_scale_get_type           (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget       * gtk_scale_new                (GtkOrientation   orientation,
                                                GtkAdjustment   *adjustment);
GDK_AVAILABLE_IN_ALL
GtkWidget       * gtk_scale_new_with_range     (GtkOrientation   orientation,
                                                double           min,
                                                double           max,
                                                double           step);
GDK_AVAILABLE_IN_ALL
void              gtk_scale_set_digits         (GtkScale        *scale,
                                                int              digits);
GDK_AVAILABLE_IN_ALL
int               gtk_scale_get_digits         (GtkScale        *scale);
GDK_AVAILABLE_IN_ALL
void              gtk_scale_set_draw_value     (GtkScale        *scale,
                                                gboolean         draw_value);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_scale_get_draw_value     (GtkScale        *scale);
GDK_AVAILABLE_IN_ALL
void              gtk_scale_set_has_origin     (GtkScale        *scale,
                                                gboolean         has_origin);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_scale_get_has_origin     (GtkScale        *scale);
GDK_AVAILABLE_IN_ALL
void              gtk_scale_set_value_pos      (GtkScale        *scale,
                                                GtkPositionType  pos);
GDK_AVAILABLE_IN_ALL
GtkPositionType   gtk_scale_get_value_pos      (GtkScale        *scale);

GDK_AVAILABLE_IN_ALL
PangoLayout     * gtk_scale_get_layout         (GtkScale        *scale);
GDK_AVAILABLE_IN_ALL
void              gtk_scale_get_layout_offsets (GtkScale        *scale,
                                                int             *x,
                                                int             *y);

GDK_AVAILABLE_IN_ALL
void              gtk_scale_add_mark           (GtkScale        *scale,
                                                double           value,
                                                GtkPositionType  position,
                                                const char      *markup);
GDK_AVAILABLE_IN_ALL
void              gtk_scale_clear_marks        (GtkScale        *scale);

GDK_AVAILABLE_IN_ALL
void              gtk_scale_set_format_value_func (GtkScale                *scale,
                                                   GtkScaleFormatValueFunc  func,
                                                   gpointer                 user_data,
                                                   GDestroyNotify           destroy_notify);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkScale, g_object_unref)

G_END_DECLS

#endif /* __GTK_SCALE_H__ */
