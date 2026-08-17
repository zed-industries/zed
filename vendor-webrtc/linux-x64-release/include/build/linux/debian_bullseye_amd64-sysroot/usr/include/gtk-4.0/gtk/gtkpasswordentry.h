/* GTK - The GIMP Toolkit
 * Copyright (C) 2019 Red Hat, Inc.
 *
 * Authors:
 * - MAtthias Clasen <mclasen@redhat.com>
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

#ifndef __GTK_PASSWORD_ENTRY_H__
#define __GTK_PASSWORD_ENTRY_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkentry.h>

G_BEGIN_DECLS

#define GTK_TYPE_PASSWORD_ENTRY                 (gtk_password_entry_get_type ())
#define GTK_PASSWORD_ENTRY(obj)                 (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_PASSWORD_ENTRY, GtkPasswordEntry))
#define GTK_IS_PASSWORD_ENTRY(obj)              (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_PASSWORD_ENTRY))

typedef struct _GtkPasswordEntry       GtkPasswordEntry;
typedef struct _GtkPasswordEntryClass  GtkPasswordEntryClass;

GDK_AVAILABLE_IN_ALL
GType           gtk_password_entry_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget *     gtk_password_entry_new      (void);

GDK_AVAILABLE_IN_ALL
void            gtk_password_entry_set_show_peek_icon (GtkPasswordEntry *entry,
                                                       gboolean          show_peek_icon);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_password_entry_get_show_peek_icon (GtkPasswordEntry *entry);

GDK_AVAILABLE_IN_ALL
void            gtk_password_entry_set_extra_menu     (GtkPasswordEntry *entry,
                                                       GMenuModel       *model);
GDK_AVAILABLE_IN_ALL
GMenuModel *    gtk_password_entry_get_extra_menu     (GtkPasswordEntry *entry);

G_END_DECLS

#endif /* __GTK_PASSWORD_ENTRY_H__ */
