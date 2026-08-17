/* GtkPrintUnixDialog
 * Copyright (C) 2006 John (J5) Palmieri <johnp@redhat.com>
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

#ifndef __GTK_PRINT_UNIX_DIALOG_H__
#define __GTK_PRINT_UNIX_DIALOG_H__

#if !defined (__GTK_UNIX_PRINT_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtkunixprint.h> can be included directly."
#endif

#include <gtk/gtk.h>
#include <gtk/gtkprinter.h>
#include <gtk/gtkprintjob.h>

G_BEGIN_DECLS

#define GTK_TYPE_PRINT_UNIX_DIALOG                  (gtk_print_unix_dialog_get_type ())
#define GTK_PRINT_UNIX_DIALOG(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_PRINT_UNIX_DIALOG, GtkPrintUnixDialog))
#define GTK_PRINT_UNIX_DIALOG_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_PRINT_UNIX_DIALOG, GtkPrintUnixDialogClass))
#define GTK_IS_PRINT_UNIX_DIALOG(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_PRINT_UNIX_DIALOG))
#define GTK_IS_PRINT_UNIX_DIALOG_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_PRINT_UNIX_DIALOG))
#define GTK_PRINT_UNIX_DIALOG_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_PRINT_UNIX_DIALOG, GtkPrintUnixDialogClass))


typedef struct _GtkPrintUnixDialog         GtkPrintUnixDialog;
typedef struct _GtkPrintUnixDialogClass    GtkPrintUnixDialogClass;
typedef struct GtkPrintUnixDialogPrivate   GtkPrintUnixDialogPrivate;

struct _GtkPrintUnixDialog
{
  GtkDialog parent_instance;

  /*< private >*/
  GtkPrintUnixDialogPrivate *priv;
};

struct _GtkPrintUnixDialogClass
{
  GtkDialogClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType                gtk_print_unix_dialog_get_type                (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget *          gtk_print_unix_dialog_new                     (const gchar *title,
                                                                    GtkWindow   *parent);

GDK_AVAILABLE_IN_ALL
void                 gtk_print_unix_dialog_set_page_setup          (GtkPrintUnixDialog *dialog,
								    GtkPageSetup       *page_setup);
GDK_AVAILABLE_IN_ALL
GtkPageSetup *       gtk_print_unix_dialog_get_page_setup          (GtkPrintUnixDialog *dialog);
GDK_AVAILABLE_IN_ALL
void                 gtk_print_unix_dialog_set_current_page        (GtkPrintUnixDialog *dialog,
								    gint                current_page);
GDK_AVAILABLE_IN_ALL
gint                 gtk_print_unix_dialog_get_current_page        (GtkPrintUnixDialog *dialog);
GDK_AVAILABLE_IN_ALL
void                 gtk_print_unix_dialog_set_settings            (GtkPrintUnixDialog *dialog,
								    GtkPrintSettings   *settings);
GDK_AVAILABLE_IN_ALL
GtkPrintSettings *   gtk_print_unix_dialog_get_settings            (GtkPrintUnixDialog *dialog);
GDK_AVAILABLE_IN_ALL
GtkPrinter *         gtk_print_unix_dialog_get_selected_printer    (GtkPrintUnixDialog *dialog);
GDK_AVAILABLE_IN_ALL
void                 gtk_print_unix_dialog_add_custom_tab          (GtkPrintUnixDialog *dialog,
								    GtkWidget          *child,
								    GtkWidget          *tab_label);
GDK_AVAILABLE_IN_ALL
void                 gtk_print_unix_dialog_set_manual_capabilities (GtkPrintUnixDialog *dialog,
								    GtkPrintCapabilities capabilities);
GDK_AVAILABLE_IN_ALL
GtkPrintCapabilities gtk_print_unix_dialog_get_manual_capabilities (GtkPrintUnixDialog  *dialog);
GDK_AVAILABLE_IN_ALL
void                 gtk_print_unix_dialog_set_support_selection   (GtkPrintUnixDialog  *dialog,
								    gboolean             support_selection);
GDK_AVAILABLE_IN_ALL
gboolean             gtk_print_unix_dialog_get_support_selection   (GtkPrintUnixDialog  *dialog);
GDK_AVAILABLE_IN_ALL
void                 gtk_print_unix_dialog_set_has_selection       (GtkPrintUnixDialog  *dialog,
								    gboolean             has_selection);
GDK_AVAILABLE_IN_ALL
gboolean             gtk_print_unix_dialog_get_has_selection       (GtkPrintUnixDialog  *dialog);
GDK_AVAILABLE_IN_ALL
void                 gtk_print_unix_dialog_set_embed_page_setup    (GtkPrintUnixDialog *dialog,
								    gboolean            embed);
GDK_AVAILABLE_IN_ALL
gboolean             gtk_print_unix_dialog_get_embed_page_setup    (GtkPrintUnixDialog *dialog);
GDK_AVAILABLE_IN_ALL
gboolean             gtk_print_unix_dialog_get_page_setup_set      (GtkPrintUnixDialog *dialog);

G_END_DECLS

#endif /* __GTK_PRINT_UNIX_DIALOG_H__ */
