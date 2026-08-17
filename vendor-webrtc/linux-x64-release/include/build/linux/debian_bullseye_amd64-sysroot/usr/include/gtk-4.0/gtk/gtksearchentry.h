/* GTK - The GIMP Toolkit
 * Copyright (C) 2012 Red Hat, Inc.
 *
 * Authors:
 * - Bastien Nocera <bnocera@redhat.com>
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
 * Modified by the GTK+ Team and others 2012.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_SEARCH_ENTRY_H__
#define __GTK_SEARCH_ENTRY_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkentry.h>

G_BEGIN_DECLS

#define GTK_TYPE_SEARCH_ENTRY                 (gtk_search_entry_get_type ())
#define GTK_SEARCH_ENTRY(obj)                 (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SEARCH_ENTRY, GtkSearchEntry))
#define GTK_IS_SEARCH_ENTRY(obj)              (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SEARCH_ENTRY))

typedef struct _GtkSearchEntry       GtkSearchEntry;

GDK_AVAILABLE_IN_ALL
GType           gtk_search_entry_get_type       (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget*      gtk_search_entry_new            (void);

GDK_AVAILABLE_IN_ALL
void            gtk_search_entry_set_key_capture_widget (GtkSearchEntry *entry,
                                                         GtkWidget      *widget);
GDK_AVAILABLE_IN_ALL
GtkWidget*      gtk_search_entry_get_key_capture_widget (GtkSearchEntry *entry);

GDK_AVAILABLE_IN_4_8
void gtk_search_entry_set_search_delay (GtkSearchEntry *entry,
                                        guint delay);
GDK_AVAILABLE_IN_4_8
guint gtk_search_entry_get_search_delay (GtkSearchEntry *entry);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkSearchEntry, g_object_unref)

G_END_DECLS

#endif /* __GTK_SEARCH_ENTRY_H__ */
