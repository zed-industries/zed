/*
 * gtkoverlay.h
 * This file is part of gtk
 *
 * Copyright (C) 2011 - Ignacio Casal Quinteiro
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

#ifndef __GTK_OVERLAY_H__
#define __GTK_OVERLAY_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbin.h>

G_BEGIN_DECLS

#define GTK_TYPE_OVERLAY             (gtk_overlay_get_type ())
#define GTK_OVERLAY(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_OVERLAY, GtkOverlay))
#define GTK_OVERLAY_CLASS(klass)     (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_OVERLAY, GtkOverlayClass))
#define GTK_IS_OVERLAY(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_OVERLAY))
#define GTK_IS_OVERLAY_CLASS(klass)  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_OVERLAY))
#define GTK_OVERLAY_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_OVERLAY, GtkOverlayClass))

typedef struct _GtkOverlay         GtkOverlay;
typedef struct _GtkOverlayClass    GtkOverlayClass;
typedef struct _GtkOverlayPrivate  GtkOverlayPrivate;

struct _GtkOverlay
{
  GtkBin parent;

  GtkOverlayPrivate *priv;
};

/**
 * GtkOverlayClass:
 * @parent_class: The parent class.
 * @get_child_position: Signal emitted to determine the position and
 *    size of any overlay child widgets.
 */
struct _GtkOverlayClass
{
  GtkBinClass parent_class;

  /*< public >*/

  gboolean (*get_child_position) (GtkOverlay    *overlay,
                                  GtkWidget     *widget,
                                  GtkAllocation *allocation);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
  void (*_gtk_reserved5) (void);
  void (*_gtk_reserved6) (void);
  void (*_gtk_reserved7) (void);
  void (*_gtk_reserved8) (void);
};

GDK_AVAILABLE_IN_3_2
GType      gtk_overlay_get_type    (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_3_2
GtkWidget *gtk_overlay_new         (void);
GDK_AVAILABLE_IN_3_2
void       gtk_overlay_add_overlay (GtkOverlay *overlay,
                                    GtkWidget  *widget);
GDK_AVAILABLE_IN_3_18
void       gtk_overlay_reorder_overlay (GtkOverlay *overlay,
                                        GtkWidget  *child,
                                        int         index_);
GDK_AVAILABLE_IN_3_18
gboolean   gtk_overlay_get_overlay_pass_through (GtkOverlay *overlay,
						 GtkWidget  *widget);
GDK_AVAILABLE_IN_3_18
void       gtk_overlay_set_overlay_pass_through (GtkOverlay *overlay,
						 GtkWidget  *widget,
						 gboolean    pass_through);

G_END_DECLS

#endif /* __GTK_OVERLAY_H__ */
