/*
 * gtkappchooserdialog.h: an app-chooser dialog
 *
 * Copyright (C) 2004 Novell, Inc.
 * Copyright (C) 2007, 2010 Red Hat, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public License as
 * published by the Free Software Foundation; either version 2 of the
 * License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors: Dave Camp <dave@novell.com>
 *          Alexander Larsson <alexl@redhat.com>
 *          Cosimo Cecchi <ccecchi@redhat.com>
 */

#ifndef __GTK_APP_CHOOSER_DIALOG_H__
#define __GTK_APP_CHOOSER_DIALOG_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkdialog.h>
#include <gio/gio.h>

G_BEGIN_DECLS

#define GTK_TYPE_APP_CHOOSER_DIALOG            (gtk_app_chooser_dialog_get_type ())
#define GTK_APP_CHOOSER_DIALOG(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_APP_CHOOSER_DIALOG, GtkAppChooserDialog))
#define GTK_IS_APP_CHOOSER_DIALOG(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_APP_CHOOSER_DIALOG))

typedef struct _GtkAppChooserDialog        GtkAppChooserDialog;

GDK_AVAILABLE_IN_ALL
GType         gtk_app_chooser_dialog_get_type             (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget *   gtk_app_chooser_dialog_new                  (GtkWindow           *parent,
                                                           GtkDialogFlags       flags,
                                                           GFile               *file);
GDK_AVAILABLE_IN_ALL
GtkWidget *   gtk_app_chooser_dialog_new_for_content_type (GtkWindow           *parent,
                                                           GtkDialogFlags       flags,
                                                           const char          *content_type);

GDK_AVAILABLE_IN_ALL
GtkWidget *   gtk_app_chooser_dialog_get_widget           (GtkAppChooserDialog *self);
GDK_AVAILABLE_IN_ALL
void          gtk_app_chooser_dialog_set_heading          (GtkAppChooserDialog *self,
                                                           const char          *heading);
GDK_AVAILABLE_IN_ALL
const char * gtk_app_chooser_dialog_get_heading          (GtkAppChooserDialog *self);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkAppChooserDialog, g_object_unref)

G_END_DECLS

#endif /* __GTK_APP_CHOOSER_DIALOG_H__ */
