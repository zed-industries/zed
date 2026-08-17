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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_SCALE_H__
#define __GTK_SCALE_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
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


typedef struct _GtkScale        GtkScale;
typedef struct _GtkScaleClass   GtkScaleClass;

struct _GtkScale
{
  GtkRange range;

  gint  GSEAL (digits);
  guint GSEAL (draw_value) : 1;
  guint GSEAL (value_pos) : 2;
};

struct _GtkScaleClass
{
  GtkRangeClass parent_class;

  gchar* (* format_value) (GtkScale *scale,
                           gdouble   value);

  void (* draw_value) (GtkScale *scale);

  void (* get_layout_offsets) (GtkScale *scale,
                               gint     *x,
                               gint     *y);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
};

GType             gtk_scale_get_type           (void) G_GNUC_CONST;
void              gtk_scale_set_digits         (GtkScale        *scale,
                                                gint             digits);
gint              gtk_scale_get_digits         (GtkScale        *scale);
void              gtk_scale_set_draw_value     (GtkScale        *scale,
                                                gboolean         draw_value);
gboolean          gtk_scale_get_draw_value     (GtkScale        *scale);
void              gtk_scale_set_value_pos      (GtkScale        *scale,
                                                GtkPositionType  pos);
GtkPositionType   gtk_scale_get_value_pos      (GtkScale        *scale);

PangoLayout     * gtk_scale_get_layout         (GtkScale        *scale);
void              gtk_scale_get_layout_offsets (GtkScale        *scale,
                                                gint            *x,
                                                gint            *y);

void              gtk_scale_add_mark           (GtkScale        *scale,
                                                gdouble          value,
                                                GtkPositionType  position,
                                                const gchar     *markup);
void              gtk_scale_clear_marks        (GtkScale        *scale);

/* internal API */
void              _gtk_scale_clear_layout      (GtkScale        *scale);
void              _gtk_scale_get_value_size    (GtkScale        *scale,
                                                gint            *width,
                                                gint            *height);
gchar           * _gtk_scale_format_value      (GtkScale        *scale,
                                                gdouble          value);

G_END_DECLS

#endif /* __GTK_SCALE_H__ */
