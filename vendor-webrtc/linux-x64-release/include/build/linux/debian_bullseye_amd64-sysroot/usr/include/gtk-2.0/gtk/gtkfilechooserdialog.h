/* GTK - The GIMP Toolkit
 * gtkfilechooserdialog.h: File selector dialog
 * Copyright (C) 2003, Red Hat, Inc.
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_FILE_CHOOSER_DIALOG_H__
#define __GTK_FILE_CHOOSER_DIALOG_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkdialog.h>
#include <gtk/gtkfilechooser.h>

G_BEGIN_DECLS

#define GTK_TYPE_FILE_CHOOSER_DIALOG             (gtk_file_chooser_dialog_get_type ())
#define GTK_FILE_CHOOSER_DIALOG(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_FILE_CHOOSER_DIALOG, GtkFileChooserDialog))
#define GTK_FILE_CHOOSER_DIALOG_CLASS(klass)     (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_FILE_CHOOSER_DIALOG, GtkFileChooserDialogClass))
#define GTK_IS_FILE_CHOOSER_DIALOG(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_FILE_CHOOSER_DIALOG))
#define GTK_IS_FILE_CHOOSER_DIALOG_CLASS(klass)  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_FILE_CHOOSER_DIALOG))
#define GTK_FILE_CHOOSER_DIALOG_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_FILE_CHOOSER_DIALOG, GtkFileChooserDialogClass))

typedef struct _GtkFileChooserDialog        GtkFileChooserDialog;
typedef struct _GtkFileChooserDialogPrivate GtkFileChooserDialogPrivate;
typedef struct _GtkFileChooserDialogClass   GtkFileChooserDialogClass;

struct _GtkFileChooserDialog
{
  GtkDialog parent_instance;

  GtkFileChooserDialogPrivate *GSEAL (priv);
};

struct _GtkFileChooserDialogClass
{
  GtkDialogClass parent_class;
};

GType      gtk_file_chooser_dialog_get_type         (void) G_GNUC_CONST;
GtkWidget *gtk_file_chooser_dialog_new              (const gchar          *title,
						     GtkWindow            *parent,
						     GtkFileChooserAction  action,
						     const gchar          *first_button_text,
						     ...) G_GNUC_NULL_TERMINATED;

#ifndef GTK_DISABLE_DEPRECATED
GtkWidget *gtk_file_chooser_dialog_new_with_backend (const gchar          *title,
						     GtkWindow            *parent,
						     GtkFileChooserAction  action,
						     const gchar          *backend,
						     const gchar          *first_button_text,
						     ...) G_GNUC_NULL_TERMINATED;
#endif /* GTK_DISABLE_DEPRECATED */

G_END_DECLS

#endif /* __GTK_FILE_CHOOSER_DIALOG_H__ */
