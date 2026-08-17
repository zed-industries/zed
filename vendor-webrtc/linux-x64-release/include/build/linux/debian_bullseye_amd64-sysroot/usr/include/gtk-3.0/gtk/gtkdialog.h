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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_DIALOG_H__
#define __GTK_DIALOG_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwindow.h>

G_BEGIN_DECLS

/**
 * GtkDialogFlags:
 * @GTK_DIALOG_MODAL: Make the constructed dialog modal,
 *     see gtk_window_set_modal()
 * @GTK_DIALOG_DESTROY_WITH_PARENT: Destroy the dialog when its
 *     parent is destroyed, see gtk_window_set_destroy_with_parent()
 * @GTK_DIALOG_USE_HEADER_BAR: Create dialog with actions in header
 *     bar instead of action area. Since 3.12.
 *
 * Flags used to influence dialog construction.
 */
typedef enum
{
  GTK_DIALOG_MODAL               = 1 << 0,
  GTK_DIALOG_DESTROY_WITH_PARENT = 1 << 1,
  GTK_DIALOG_USE_HEADER_BAR      = 1 << 2
} GtkDialogFlags;

/**
 * GtkResponseType:
 * @GTK_RESPONSE_NONE: Returned if an action widget has no response id,
 *     or if the dialog gets programmatically hidden or destroyed
 * @GTK_RESPONSE_REJECT: Generic response id, not used by GTK+ dialogs
 * @GTK_RESPONSE_ACCEPT: Generic response id, not used by GTK+ dialogs
 * @GTK_RESPONSE_DELETE_EVENT: Returned if the dialog is deleted
 * @GTK_RESPONSE_OK: Returned by OK buttons in GTK+ dialogs
 * @GTK_RESPONSE_CANCEL: Returned by Cancel buttons in GTK+ dialogs
 * @GTK_RESPONSE_CLOSE: Returned by Close buttons in GTK+ dialogs
 * @GTK_RESPONSE_YES: Returned by Yes buttons in GTK+ dialogs
 * @GTK_RESPONSE_NO: Returned by No buttons in GTK+ dialogs
 * @GTK_RESPONSE_APPLY: Returned by Apply buttons in GTK+ dialogs
 * @GTK_RESPONSE_HELP: Returned by Help buttons in GTK+ dialogs
 *
 * Predefined values for use as response ids in gtk_dialog_add_button().
 * All predefined values are negative; GTK+ leaves values of 0 or greater for
 * application-defined response ids.
 */
typedef enum
{
  GTK_RESPONSE_NONE         = -1,
  GTK_RESPONSE_REJECT       = -2,
  GTK_RESPONSE_ACCEPT       = -3,
  GTK_RESPONSE_DELETE_EVENT = -4,
  GTK_RESPONSE_OK           = -5,
  GTK_RESPONSE_CANCEL       = -6,
  GTK_RESPONSE_CLOSE        = -7,
  GTK_RESPONSE_YES          = -8,
  GTK_RESPONSE_NO           = -9,
  GTK_RESPONSE_APPLY        = -10,
  GTK_RESPONSE_HELP         = -11
} GtkResponseType;


#define GTK_TYPE_DIALOG                  (gtk_dialog_get_type ())
#define GTK_DIALOG(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_DIALOG, GtkDialog))
#define GTK_DIALOG_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_DIALOG, GtkDialogClass))
#define GTK_IS_DIALOG(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_DIALOG))
#define GTK_IS_DIALOG_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_DIALOG))
#define GTK_DIALOG_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_DIALOG, GtkDialogClass))


typedef struct _GtkDialog              GtkDialog;
typedef struct _GtkDialogPrivate       GtkDialogPrivate;
typedef struct _GtkDialogClass         GtkDialogClass;

/**
 * GtkDialog:
 *
 * The #GtkDialog-struct contains only private fields
 * and should not be directly accessed.
 */
struct _GtkDialog
{
  GtkWindow window;

  /*< private >*/
  GtkDialogPrivate *priv;
};

/**
 * GtkDialogClass:
 * @parent_class: The parent class.
 * @response: Signal emitted when an action widget is activated.
 * @close: Signal emitted when the user uses a keybinding to close the dialog.
 */
struct _GtkDialogClass
{
  GtkWindowClass parent_class;

  /*< public >*/

  void (* response) (GtkDialog *dialog, gint response_id);

  /* Keybinding signals */

  void (* close)    (GtkDialog *dialog);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GDK_AVAILABLE_IN_ALL
GType      gtk_dialog_get_type (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_dialog_new      (void);

GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_dialog_new_with_buttons (const gchar     *title,
                                        GtkWindow       *parent,
                                        GtkDialogFlags   flags,
                                        const gchar     *first_button_text,
                                        ...) G_GNUC_NULL_TERMINATED;

GDK_AVAILABLE_IN_ALL
void       gtk_dialog_add_action_widget (GtkDialog   *dialog,
                                         GtkWidget   *child,
                                         gint         response_id);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_dialog_add_button        (GtkDialog   *dialog,
                                         const gchar *button_text,
                                         gint         response_id);
GDK_AVAILABLE_IN_ALL
void       gtk_dialog_add_buttons       (GtkDialog   *dialog,
                                         const gchar *first_button_text,
                                         ...) G_GNUC_NULL_TERMINATED;

GDK_AVAILABLE_IN_ALL
void gtk_dialog_set_response_sensitive (GtkDialog *dialog,
                                        gint       response_id,
                                        gboolean   setting);
GDK_AVAILABLE_IN_ALL
void gtk_dialog_set_default_response   (GtkDialog *dialog,
                                        gint       response_id);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_dialog_get_widget_for_response (GtkDialog *dialog,
                                               gint       response_id);
GDK_AVAILABLE_IN_ALL
gint gtk_dialog_get_response_for_widget (GtkDialog *dialog,
                                         GtkWidget *widget);

GDK_DEPRECATED_IN_3_10
gboolean gtk_alternative_dialog_button_order (GdkScreen *screen);
GDK_DEPRECATED_IN_3_10
void     gtk_dialog_set_alternative_button_order (GtkDialog *dialog,
                                                  gint       first_response_id,
                                                  ...);
GDK_DEPRECATED_IN_3_10
void     gtk_dialog_set_alternative_button_order_from_array (GtkDialog *dialog,
                                                             gint       n_params,
                                                             gint      *new_order);

/* Emit response signal */
GDK_AVAILABLE_IN_ALL
void gtk_dialog_response           (GtkDialog *dialog,
                                    gint       response_id);

/* Returns response_id */
GDK_AVAILABLE_IN_ALL
gint gtk_dialog_run                (GtkDialog *dialog);

GDK_DEPRECATED_IN_3_10
GtkWidget * gtk_dialog_get_action_area  (GtkDialog *dialog);
GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_dialog_get_content_area (GtkDialog *dialog);
GDK_AVAILABLE_IN_3_12
GtkWidget * gtk_dialog_get_header_bar   (GtkDialog *dialog);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkDialog, g_object_unref)

G_END_DECLS

#endif /* __GTK_DIALOG_H__ */
