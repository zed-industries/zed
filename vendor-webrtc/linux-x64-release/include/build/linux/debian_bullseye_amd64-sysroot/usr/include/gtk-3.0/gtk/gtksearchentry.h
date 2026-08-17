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
#define GTK_SEARCH_ENTRY_CLASS(klass)         (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SEARCH_ENTRY, GtkSearchEntryClass))
#define GTK_IS_SEARCH_ENTRY(obj)              (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SEARCH_ENTRY))
#define GTK_IS_SEARCH_ENTRY_CLASS(klass)      (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SEARCH_ENTRY))
#define GTK_SEARCH_ENTRY_GET_CLASS(obj)       (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SEARCH_ENTRY, GtkSearchEntryClass))

typedef struct _GtkSearchEntry       GtkSearchEntry;
typedef struct _GtkSearchEntryClass  GtkSearchEntryClass;

struct _GtkSearchEntry
{
  GtkEntry parent;
};

struct _GtkSearchEntryClass
{
  GtkEntryClass parent_class;

  void (*search_changed) (GtkSearchEntry *entry);
  void (*next_match)     (GtkSearchEntry *entry);
  void (*previous_match) (GtkSearchEntry *entry);
  void (*stop_search)    (GtkSearchEntry *entry);
};

GDK_AVAILABLE_IN_3_6
GType           gtk_search_entry_get_type       (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_6
GtkWidget*      gtk_search_entry_new            (void);

GDK_AVAILABLE_IN_3_16
gboolean        gtk_search_entry_handle_event   (GtkSearchEntry *entry,
                                                 GdkEvent       *event);

G_END_DECLS

#endif /* __GTK_SEARCH_ENTRY_H__ */
