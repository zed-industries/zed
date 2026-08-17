/*
 * Copyright (c) 2013 - 2014 Red Hat, Inc.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
 * License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 *
 */

#ifndef __GTK_ACTION_BAR_H__
#define __GTK_ACTION_BAR_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_ACTION_BAR            (gtk_action_bar_get_type ())
#define GTK_ACTION_BAR(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ACTION_BAR, GtkActionBar))
#define GTK_IS_ACTION_BAR(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_ACTION_BAR))

typedef struct _GtkActionBar              GtkActionBar;

GDK_AVAILABLE_IN_ALL
GType        gtk_action_bar_get_type          (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget   *gtk_action_bar_new               (void);
GDK_AVAILABLE_IN_ALL
GtkWidget   *gtk_action_bar_get_center_widget (GtkActionBar *action_bar);
GDK_AVAILABLE_IN_ALL
void         gtk_action_bar_set_center_widget (GtkActionBar *action_bar,
                                               GtkWidget    *center_widget);

GDK_AVAILABLE_IN_ALL
void         gtk_action_bar_pack_start        (GtkActionBar *action_bar,
                                               GtkWidget    *child);
GDK_AVAILABLE_IN_ALL
void         gtk_action_bar_pack_end          (GtkActionBar *action_bar,
                                               GtkWidget    *child);

GDK_AVAILABLE_IN_ALL
void         gtk_action_bar_remove            (GtkActionBar *action_bar,
                                               GtkWidget    *child);

GDK_AVAILABLE_IN_ALL
void        gtk_action_bar_set_revealed       (GtkActionBar *action_bar,
                                               gboolean      revealed);
GDK_AVAILABLE_IN_ALL
gboolean    gtk_action_bar_get_revealed       (GtkActionBar *action_bar);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkActionBar, g_object_unref)

G_END_DECLS

#endif /* __GTK_ACTION_BAR_H__ */
