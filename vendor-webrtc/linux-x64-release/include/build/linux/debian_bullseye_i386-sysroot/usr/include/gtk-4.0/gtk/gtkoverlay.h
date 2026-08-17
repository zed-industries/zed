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

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_OVERLAY             (gtk_overlay_get_type ())
#define GTK_OVERLAY(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_OVERLAY, GtkOverlay))
#define GTK_IS_OVERLAY(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_OVERLAY))

typedef struct _GtkOverlay         GtkOverlay;

GDK_AVAILABLE_IN_ALL
GType      gtk_overlay_get_type    (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget *gtk_overlay_new         (void);
GDK_AVAILABLE_IN_ALL
void       gtk_overlay_add_overlay (GtkOverlay *overlay,
                                    GtkWidget  *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_overlay_remove_overlay (GtkOverlay *overlay,
                                       GtkWidget  *widget);

GDK_AVAILABLE_IN_ALL
void                  gtk_overlay_set_child (GtkOverlay *overlay,
                                             GtkWidget  *child);
GDK_AVAILABLE_IN_ALL
GtkWidget *           gtk_overlay_get_child (GtkOverlay *overlay);

GDK_AVAILABLE_IN_ALL
gboolean   gtk_overlay_get_measure_overlay (GtkOverlay *overlay,
                                            GtkWidget  *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_overlay_set_measure_overlay (GtkOverlay *overlay,
                                            GtkWidget  *widget,
                                            gboolean    measure);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_overlay_get_clip_overlay    (GtkOverlay *overlay,
                                            GtkWidget  *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_overlay_set_clip_overlay    (GtkOverlay *overlay,
                                            GtkWidget  *widget,
                                            gboolean    clip_overlay);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkOverlay, g_object_unref)

G_END_DECLS

#endif /* __GTK_OVERLAY_H__ */
