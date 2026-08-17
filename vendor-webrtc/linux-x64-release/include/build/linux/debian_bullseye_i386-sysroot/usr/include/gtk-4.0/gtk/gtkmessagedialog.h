/* GTK - The GIMP Toolkit
 * Copyright (C) 2000 Red Hat, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2003.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_MESSAGE_DIALOG_H__
#define __GTK_MESSAGE_DIALOG_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkdialog.h>
#include <gtk/gtkenums.h>

G_BEGIN_DECLS


#define GTK_TYPE_MESSAGE_DIALOG                  (gtk_message_dialog_get_type ())
#define GTK_MESSAGE_DIALOG(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_MESSAGE_DIALOG, GtkMessageDialog))
#define GTK_IS_MESSAGE_DIALOG(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_MESSAGE_DIALOG))

typedef struct _GtkMessageDialog              GtkMessageDialog;
typedef struct _GtkMessageDialogClass         GtkMessageDialogClass;

struct _GtkMessageDialog
{
  GtkDialog parent_instance;
};

/**
 * GtkButtonsType:
 * @GTK_BUTTONS_NONE: no buttons at all
 * @GTK_BUTTONS_OK: an OK button
 * @GTK_BUTTONS_CLOSE: a Close button
 * @GTK_BUTTONS_CANCEL: a Cancel button
 * @GTK_BUTTONS_YES_NO: Yes and No buttons
 * @GTK_BUTTONS_OK_CANCEL: OK and Cancel buttons
 *
 * Prebuilt sets of buttons for `GtkDialog`.
 *
 * If none of these choices are appropriate, simply use
 * %GTK_BUTTONS_NONE and call [method@Gtk.Dialog.add_buttons].
 *
 * > Please note that %GTK_BUTTONS_OK, %GTK_BUTTONS_YES_NO
 * > and %GTK_BUTTONS_OK_CANCEL are discouraged by the
 * > [GNOME Human Interface Guidelines](http://library.gnome.org/devel/hig-book/stable/).
 */
typedef enum
{
  GTK_BUTTONS_NONE,
  GTK_BUTTONS_OK,
  GTK_BUTTONS_CLOSE,
  GTK_BUTTONS_CANCEL,
  GTK_BUTTONS_YES_NO,
  GTK_BUTTONS_OK_CANCEL
} GtkButtonsType;

GDK_AVAILABLE_IN_ALL
GType      gtk_message_dialog_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_message_dialog_new      (GtkWindow      *parent,
                                        GtkDialogFlags  flags,
                                        GtkMessageType  type,
                                        GtkButtonsType  buttons,
                                        const char     *message_format,
                                        ...) G_GNUC_PRINTF (5, 6);

GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_message_dialog_new_with_markup   (GtkWindow      *parent,
                                                 GtkDialogFlags  flags,
                                                 GtkMessageType  type,
                                                 GtkButtonsType  buttons,
                                                 const char     *message_format,
                                                 ...) G_GNUC_PRINTF (5, 6);

GDK_AVAILABLE_IN_ALL
void       gtk_message_dialog_set_markup  (GtkMessageDialog *message_dialog,
                                           const char       *str);

GDK_AVAILABLE_IN_ALL
void       gtk_message_dialog_format_secondary_text (GtkMessageDialog *message_dialog,
                                                     const char       *message_format,
                                                     ...) G_GNUC_PRINTF (2, 3);

GDK_AVAILABLE_IN_ALL
void       gtk_message_dialog_format_secondary_markup (GtkMessageDialog *message_dialog,
                                                       const char       *message_format,
                                                       ...) G_GNUC_PRINTF (2, 3);

GDK_AVAILABLE_IN_ALL
GtkWidget *gtk_message_dialog_get_message_area (GtkMessageDialog *message_dialog);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkMessageDialog, g_object_unref)

G_END_DECLS

#endif /* __GTK_MESSAGE_DIALOG_H__ */
