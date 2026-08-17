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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

/*
 * NOTE this widget is considered too specialized/little-used for
 * GTK+, and will in the future be moved to some other package.  If
 * your application needs this widget, feel free to use it, as the
 * widget does work and is useful in some applications; it's just not
 * of general interest. However, we are not accepting new features for
 * the widget, and it will eventually move out of the GTK+
 * distribution.
 */

#ifndef GTK_DISABLE_DEPRECATED

#ifndef __GTK_INPUTDIALOG_H__
#define __GTK_INPUTDIALOG_H__


#include <gtk/gtkdialog.h>


G_BEGIN_DECLS

#define GTK_TYPE_INPUT_DIALOG              (gtk_input_dialog_get_type ())
#define GTK_INPUT_DIALOG(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_INPUT_DIALOG, GtkInputDialog))
#define GTK_INPUT_DIALOG_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_INPUT_DIALOG, GtkInputDialogClass))
#define GTK_IS_INPUT_DIALOG(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_INPUT_DIALOG))
#define GTK_IS_INPUT_DIALOG_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_INPUT_DIALOG))
#define GTK_INPUT_DIALOG_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_INPUT_DIALOG, GtkInputDialogClass))


typedef struct _GtkInputDialog       GtkInputDialog;
typedef struct _GtkInputDialogClass  GtkInputDialogClass;

struct _GtkInputDialog
{
  GtkDialog dialog;

  GtkWidget *GSEAL (axis_list);
  GtkWidget *GSEAL (axis_listbox);
  GtkWidget *GSEAL (mode_optionmenu);

  GtkWidget *GSEAL (close_button);
  GtkWidget *GSEAL (save_button);

  GtkWidget *GSEAL (axis_items[GDK_AXIS_LAST]);
  GdkDevice *GSEAL (current_device);

  GtkWidget *GSEAL (keys_list);
  GtkWidget *GSEAL (keys_listbox);
};

struct _GtkInputDialogClass
{
  GtkDialogClass parent_class;

  void (* enable_device)               (GtkInputDialog    *inputd,
					GdkDevice         *device);
  void (* disable_device)              (GtkInputDialog    *inputd,
					GdkDevice         *device);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GType      gtk_input_dialog_get_type     (void) G_GNUC_CONST;
GtkWidget* gtk_input_dialog_new          (void);

G_END_DECLS

#endif /* __GTK_INPUTDIALOG_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
