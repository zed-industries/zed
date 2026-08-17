/* GTK - The GIMP Toolkit
 * gtkrecentchooserdialog.h: Recent files selector dialog
 * Copyright (C) 2006 Emmanuele Bassi
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

#ifndef __GTK_RECENT_CHOOSER_DIALOG_H__
#define __GTK_RECENT_CHOOSER_DIALOG_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkdialog.h>
#include <gtk/gtkrecentchooser.h>

G_BEGIN_DECLS

#define GTK_TYPE_RECENT_CHOOSER_DIALOG		  (gtk_recent_chooser_dialog_get_type ())
#define GTK_RECENT_CHOOSER_DIALOG(obj)		  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_RECENT_CHOOSER_DIALOG, GtkRecentChooserDialog))
#define GTK_IS_RECENT_CHOOSER_DIALOG(obj)	  (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_RECENT_CHOOSER_DIALOG))
#define GTK_RECENT_CHOOSER_DIALOG_CLASS(klass)	  (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_RECENT_CHOOSER_DIALOG, GtkRecentChooserDialogClass))
#define GTK_IS_RECENT_CHOOSER_DIALOG_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_RECENT_CHOOSER_DIALOG))
#define GTK_RECENT_CHOOSER_DIALOG_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_RECENT_CHOOSER_DIALOG, GtkRecentChooserDialogClass))

typedef struct _GtkRecentChooserDialog        GtkRecentChooserDialog;
typedef struct _GtkRecentChooserDialogClass   GtkRecentChooserDialogClass;

typedef struct _GtkRecentChooserDialogPrivate GtkRecentChooserDialogPrivate;


struct _GtkRecentChooserDialog
{
  GtkDialog parent_instance;

  /*< private >*/
  GtkRecentChooserDialogPrivate *priv;
};

struct _GtkRecentChooserDialogClass
{
  GtkDialogClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GDK_AVAILABLE_IN_ALL
GType      gtk_recent_chooser_dialog_get_type        (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget *gtk_recent_chooser_dialog_new             (const gchar      *title,
					              GtkWindow        *parent,
					              const gchar      *first_button_text,
					              ...) G_GNUC_NULL_TERMINATED;
GDK_AVAILABLE_IN_ALL
GtkWidget *gtk_recent_chooser_dialog_new_for_manager (const gchar      *title,
						      GtkWindow        *parent,
						      GtkRecentManager *manager,
						      const gchar      *first_button_text,
						      ...) G_GNUC_NULL_TERMINATED;

G_END_DECLS

#endif /* __GTK_RECENT_CHOOSER_DIALOG_H__ */
