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

#ifndef __GTK_VIEWPORT_H__
#define __GTK_VIEWPORT_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbin.h>


G_BEGIN_DECLS


#define GTK_TYPE_VIEWPORT            (gtk_viewport_get_type ())
#define GTK_VIEWPORT(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_VIEWPORT, GtkViewport))
#define GTK_VIEWPORT_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_VIEWPORT, GtkViewportClass))
#define GTK_IS_VIEWPORT(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_VIEWPORT))
#define GTK_IS_VIEWPORT_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_VIEWPORT))
#define GTK_VIEWPORT_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_VIEWPORT, GtkViewportClass))


typedef struct _GtkViewport              GtkViewport;
typedef struct _GtkViewportPrivate       GtkViewportPrivate;
typedef struct _GtkViewportClass         GtkViewportClass;

struct _GtkViewport
{
  GtkBin bin;

  /*< private >*/
  GtkViewportPrivate *priv;
};

/**
 * GtkViewportClass:
 * @parent_class: The parent class.
 */
struct _GtkViewportClass
{
  GtkBinClass parent_class;

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GDK_AVAILABLE_IN_ALL
GType          gtk_viewport_get_type        (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget*     gtk_viewport_new             (GtkAdjustment *hadjustment,
					     GtkAdjustment *vadjustment);

GDK_DEPRECATED_IN_3_0_FOR(gtk_scrollable_get_hadjustment)
GtkAdjustment* gtk_viewport_get_hadjustment (GtkViewport   *viewport);
GDK_DEPRECATED_IN_3_0_FOR(gtk_scrollable_get_vadjustment)
GtkAdjustment* gtk_viewport_get_vadjustment (GtkViewport   *viewport);
GDK_DEPRECATED_IN_3_0_FOR(gtk_scrollable_set_hadjustment)
void           gtk_viewport_set_hadjustment (GtkViewport   *viewport,
                                             GtkAdjustment *adjustment);
GDK_DEPRECATED_IN_3_0_FOR(gtk_scrollable_set_vadjustment)
void           gtk_viewport_set_vadjustment (GtkViewport   *viewport,
                                             GtkAdjustment *adjustment);

GDK_AVAILABLE_IN_ALL
void           gtk_viewport_set_shadow_type (GtkViewport   *viewport,
					     GtkShadowType  type);
GDK_AVAILABLE_IN_ALL
GtkShadowType  gtk_viewport_get_shadow_type (GtkViewport   *viewport);
GDK_AVAILABLE_IN_ALL
GdkWindow*     gtk_viewport_get_bin_window  (GtkViewport   *viewport);
GDK_AVAILABLE_IN_ALL
GdkWindow*     gtk_viewport_get_view_window (GtkViewport   *viewport);


G_END_DECLS


#endif /* __GTK_VIEWPORT_H__ */
