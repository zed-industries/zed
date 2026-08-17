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

#ifndef __GTK_HSCALE_H__
#define __GTK_HSCALE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkscale.h>

G_BEGIN_DECLS

#define GTK_TYPE_HSCALE            (gtk_hscale_get_type ())
#define GTK_HSCALE(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_HSCALE, GtkHScale))
#define GTK_HSCALE_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_HSCALE, GtkHScaleClass))
#define GTK_IS_HSCALE(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_HSCALE))
#define GTK_IS_HSCALE_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_HSCALE))
#define GTK_HSCALE_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_HSCALE, GtkHScaleClass))


typedef struct _GtkHScale       GtkHScale;
typedef struct _GtkHScaleClass  GtkHScaleClass;

struct _GtkHScale
{
  GtkScale scale;
};

struct _GtkHScaleClass
{
  GtkScaleClass parent_class;
};


GDK_DEPRECATED_IN_3_2
GType      gtk_hscale_get_type       (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_2_FOR(gtk_scale_new)
GtkWidget* gtk_hscale_new            (GtkAdjustment *adjustment);
GDK_DEPRECATED_IN_3_2_FOR(gtk_scale_new_with_range)
GtkWidget* gtk_hscale_new_with_range (gdouble        min,
                                      gdouble        max,
                                      gdouble        step);

G_END_DECLS

#endif /* __GTK_HSCALE_H__ */
