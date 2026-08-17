/* GtkPageSetupUnixDialog
 * Copyright (C) 2006 Alexander Larsson <alexl@redhat.com>
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_PAGE_SETUP_UNIX_DIALOG_H__
#define __GTK_PAGE_SETUP_UNIX_DIALOG_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_UNIX_PRINT_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtkunixprint.h> can be included directly."
#endif

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define GTK_TYPE_PAGE_SETUP_UNIX_DIALOG                  (gtk_page_setup_unix_dialog_get_type ())
#define GTK_PAGE_SETUP_UNIX_DIALOG(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_PAGE_SETUP_UNIX_DIALOG, GtkPageSetupUnixDialog))
#define GTK_PAGE_SETUP_UNIX_DIALOG_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_PAGE_SETUP_UNIX_DIALOG, GtkPageSetupUnixDialogClass))
#define GTK_IS_PAGE_SETUP_UNIX_DIALOG(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_PAGE_SETUP_UNIX_DIALOG))
#define GTK_IS_PAGE_SETUP_UNIX_DIALOG_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_PAGE_SETUP_UNIX_DIALOG))
#define GTK_PAGE_SETUP_UNIX_DIALOG_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_PAGE_SETUP_UNIX_DIALOG, GtkPageSetupUnixDialogClass))


typedef struct _GtkPageSetupUnixDialog         GtkPageSetupUnixDialog;
typedef struct _GtkPageSetupUnixDialogClass    GtkPageSetupUnixDialogClass;
typedef struct GtkPageSetupUnixDialogPrivate   GtkPageSetupUnixDialogPrivate;

struct _GtkPageSetupUnixDialog
{
  GtkDialog parent_instance;

  GtkPageSetupUnixDialogPrivate *GSEAL (priv);
};

struct _GtkPageSetupUnixDialogClass
{
  GtkDialogClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
  void (*_gtk_reserved5) (void);
  void (*_gtk_reserved6) (void);
  void (*_gtk_reserved7) (void);
};

GType 		  gtk_page_setup_unix_dialog_get_type	        (void) G_GNUC_CONST;
GtkWidget *       gtk_page_setup_unix_dialog_new                (const gchar            *title,
								 GtkWindow              *parent);
void              gtk_page_setup_unix_dialog_set_page_setup     (GtkPageSetupUnixDialog *dialog,
								 GtkPageSetup           *page_setup);
GtkPageSetup *    gtk_page_setup_unix_dialog_get_page_setup     (GtkPageSetupUnixDialog *dialog);
void              gtk_page_setup_unix_dialog_set_print_settings (GtkPageSetupUnixDialog *dialog,
								 GtkPrintSettings       *print_settings);
GtkPrintSettings *gtk_page_setup_unix_dialog_get_print_settings (GtkPageSetupUnixDialog *dialog);

G_END_DECLS

#endif /* __GTK_PAGE_SETUP_UNIX_DIALOG_H__ */
