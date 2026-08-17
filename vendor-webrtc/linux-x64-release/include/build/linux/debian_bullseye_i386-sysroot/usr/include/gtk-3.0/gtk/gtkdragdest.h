/* -*- Mode: C; c-file-style: "gnu"; tab-width: 8 -*- */
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

#ifndef __GTK_DRAG_DEST_H__
#define __GTK_DRAG_DEST_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkselection.h>
#include <gtk/gtkwidget.h>


G_BEGIN_DECLS

/**
 * GtkDestDefaults:
 * @GTK_DEST_DEFAULT_MOTION: If set for a widget, GTK+, during a drag over this
 *   widget will check if the drag matches this widget’s list of possible targets
 *   and actions.
 *   GTK+ will then call gdk_drag_status() as appropriate.
 * @GTK_DEST_DEFAULT_HIGHLIGHT: If set for a widget, GTK+ will draw a highlight on
 *   this widget as long as a drag is over this widget and the widget drag format
 *   and action are acceptable.
 * @GTK_DEST_DEFAULT_DROP: If set for a widget, when a drop occurs, GTK+ will
 *   will check if the drag matches this widget’s list of possible targets and
 *   actions. If so, GTK+ will call gtk_drag_get_data() on behalf of the widget.
 *   Whether or not the drop is successful, GTK+ will call gtk_drag_finish(). If
 *   the action was a move, then if the drag was successful, then %TRUE will be
 *   passed for the @delete parameter to gtk_drag_finish().
 * @GTK_DEST_DEFAULT_ALL: If set, specifies that all default actions should
 *   be taken.
 *
 * The #GtkDestDefaults enumeration specifies the various
 * types of action that will be taken on behalf
 * of the user for a drag destination site.
 */
typedef enum {
  GTK_DEST_DEFAULT_MOTION     = 1 << 0,
  GTK_DEST_DEFAULT_HIGHLIGHT  = 1 << 1,
  GTK_DEST_DEFAULT_DROP       = 1 << 2,
  GTK_DEST_DEFAULT_ALL        = 0x07
} GtkDestDefaults;

GDK_AVAILABLE_IN_ALL
void gtk_drag_dest_set   (GtkWidget            *widget,
                          GtkDestDefaults       flags,
                          const GtkTargetEntry *targets,
                          gint                  n_targets,
                          GdkDragAction         actions);

GDK_DEPRECATED_IN_3_22
void gtk_drag_dest_set_proxy (GtkWidget      *widget,
                              GdkWindow      *proxy_window,
                              GdkDragProtocol protocol,
                              gboolean        use_coordinates);

GDK_AVAILABLE_IN_ALL
void gtk_drag_dest_unset (GtkWidget          *widget);

GDK_AVAILABLE_IN_ALL
GdkAtom        gtk_drag_dest_find_target     (GtkWidget      *widget,
                                              GdkDragContext *context,
                                              GtkTargetList  *target_list);
GDK_AVAILABLE_IN_ALL
GtkTargetList* gtk_drag_dest_get_target_list (GtkWidget      *widget);
GDK_AVAILABLE_IN_ALL
void           gtk_drag_dest_set_target_list (GtkWidget      *widget,
                                              GtkTargetList  *target_list);
GDK_AVAILABLE_IN_ALL
void           gtk_drag_dest_add_text_targets  (GtkWidget    *widget);
GDK_AVAILABLE_IN_ALL
void           gtk_drag_dest_add_image_targets (GtkWidget    *widget);
GDK_AVAILABLE_IN_ALL
void           gtk_drag_dest_add_uri_targets   (GtkWidget    *widget);

GDK_AVAILABLE_IN_ALL
void           gtk_drag_dest_set_track_motion  (GtkWidget *widget,
                                                gboolean   track_motion);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_drag_dest_get_track_motion  (GtkWidget *widget);


G_END_DECLS

#endif /* __GTK_DRAG_DEST_H__ */
