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

#ifndef __GTK_VSCALE_H__
#define __GTK_VSCALE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkscale.h>

G_BEGIN_DECLS

#define GTK_TYPE_VSCALE            (gtk_vscale_get_type ())
#define GTK_VSCALE(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_VSCALE, GtkVScale))
#define GTK_VSCALE_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_VSCALE, GtkVScaleClass))
#define GTK_IS_VSCALE(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_VSCALE))
#define GTK_IS_VSCALE_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_VSCALE))
#define GTK_VSCALE_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_VSCALE, GtkVScaleClass))


typedef struct _GtkVScale       GtkVScale;
typedef struct _GtkVScaleClass  GtkVScaleClass;

/**
 * GtkVScale:
 *
 * The #GtkVScale struct contains private data only, and
 * should be accessed using the functions below.
 */
struct _GtkVScale
{
  GtkScale scale;
};

struct _GtkVScaleClass
{
  GtkScaleClass parent_class;
};


GDK_DEPRECATED_IN_3_2
GType      gtk_vscale_get_type       (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_2_FOR(gtk_scale_new)
GtkWidget* gtk_vscale_new            (GtkAdjustment *adjustment);
GDK_DEPRECATED_IN_3_2_FOR(gtk_scale_new_with_range)
GtkWidget* gtk_vscale_new_with_range (gdouble        min,
                                      gdouble        max,
                                      gdouble        step);

G_END_DECLS

#endif /* __GTK_VSCALE_H__ */
