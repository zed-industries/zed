/* GTK - The GIMP Toolkit
 * gtkfilechoosernative.h: Native File selector dialog
 * Copyright (C) 2015, Red Hat, Inc.
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

#ifndef __GTK_FILE_CHOOSER_NATIVE_H__
#define __GTK_FILE_CHOOSER_NATIVE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkfilechooser.h>
#include <gtk/gtknativedialog.h>

G_BEGIN_DECLS

#define GTK_TYPE_FILE_CHOOSER_NATIVE             (gtk_file_chooser_native_get_type ())

GDK_AVAILABLE_IN_3_20
G_DECLARE_FINAL_TYPE (GtkFileChooserNative, gtk_file_chooser_native, GTK, FILE_CHOOSER_NATIVE, GtkNativeDialog)

GDK_AVAILABLE_IN_3_20
GtkFileChooserNative *gtk_file_chooser_native_new (const gchar          *title,
                                                   GtkWindow            *parent,
                                                   GtkFileChooserAction  action,
                                                   const gchar          *accept_label,
                                                   const gchar          *cancel_label);

GDK_AVAILABLE_IN_3_20
const char *gtk_file_chooser_native_get_accept_label (GtkFileChooserNative *self);
GDK_AVAILABLE_IN_3_20
void        gtk_file_chooser_native_set_accept_label (GtkFileChooserNative *self,
                                                      const char           *accept_label);
GDK_AVAILABLE_IN_3_20
const char *gtk_file_chooser_native_get_cancel_label (GtkFileChooserNative *self);
GDK_AVAILABLE_IN_3_20
void        gtk_file_chooser_native_set_cancel_label (GtkFileChooserNative *self,
                                                      const char           *cancel_label);

G_END_DECLS

#endif /* __GTK_FILE_CHOOSER_NATIVE_H__ */
