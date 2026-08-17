/* GTK - The GIMP Toolkit

   Copyright (C) 2001 CodeFactory AB
   Copyright (C) 2001 Anders Carlsson <andersca@codefactory.se>
   Copyright (C) 2003, 2004 Matthias Clasen <mclasen@redhat.com>

   This library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Library General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   This library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Library General Public License for more details.

   You should have received a copy of the GNU Library General Public
   License along with the Gnome Library; see the file COPYING.LIB.  If not,
   write to the Free Software Foundation, Inc., 59 Temple Place - Suite 330,
   Boston, MA 02111-1307, USA.

   Author: Anders Carlsson <andersca@codefactory.se>
*/

#ifndef __GTK_ABOUT_DIALOG_H__
#define __GTK_ABOUT_DIALOG_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkdialog.h>

G_BEGIN_DECLS

#define GTK_TYPE_ABOUT_DIALOG            (gtk_about_dialog_get_type ())
#define GTK_ABOUT_DIALOG(object)         (G_TYPE_CHECK_INSTANCE_CAST ((object), GTK_TYPE_ABOUT_DIALOG, GtkAboutDialog))
#define GTK_ABOUT_DIALOG_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_ABOUT_DIALOG, GtkAboutDialogClass))
#define GTK_IS_ABOUT_DIALOG(object)      (G_TYPE_CHECK_INSTANCE_TYPE ((object), GTK_TYPE_ABOUT_DIALOG))
#define GTK_IS_ABOUT_DIALOG_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_ABOUT_DIALOG))
#define GTK_ABOUT_DIALOG_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_ABOUT_DIALOG, GtkAboutDialogClass))

typedef struct _GtkAboutDialog        GtkAboutDialog;
typedef struct _GtkAboutDialogClass   GtkAboutDialogClass;

/**
 * GtkAboutDialog:
 *
 * The <structname>GtkAboutDialog</structname> struct contains
 * only private fields and should not be directly accessed.
 */
struct _GtkAboutDialog 
{
  GtkDialog parent_instance;

  /*< private >*/
  gpointer GSEAL (private_data);
};

struct _GtkAboutDialogClass 
{
  GtkDialogClass parent_class;

  gboolean (*activate_link) (GtkAboutDialog *dialog,
                             const gchar    *uri);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
};

GType                  gtk_about_dialog_get_type               (void) G_GNUC_CONST;
GtkWidget             *gtk_about_dialog_new                    (void);
void                   gtk_show_about_dialog                   (GtkWindow       *parent,
								const gchar     *first_property_name,
								...) G_GNUC_NULL_TERMINATED;

#ifndef GTK_DISABLE_DEPRECATED
const gchar *          gtk_about_dialog_get_name               (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_name               (GtkAboutDialog  *about,
								const gchar     *name);
#endif /* GTK_DISABLE_DEPRECATED */
const gchar *          gtk_about_dialog_get_program_name       (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_program_name       (GtkAboutDialog  *about,
								const gchar     *name);
const gchar *          gtk_about_dialog_get_version            (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_version            (GtkAboutDialog  *about,
								const gchar     *version);
const gchar *          gtk_about_dialog_get_copyright          (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_copyright          (GtkAboutDialog  *about,
								const gchar     *copyright);
const gchar *          gtk_about_dialog_get_comments           (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_comments           (GtkAboutDialog  *about,
								const gchar     *comments);
const gchar *          gtk_about_dialog_get_license            (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_license            (GtkAboutDialog  *about,
								const gchar     *license);

gboolean               gtk_about_dialog_get_wrap_license       (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_wrap_license       (GtkAboutDialog  *about,
                                                                gboolean         wrap_license);

const gchar *          gtk_about_dialog_get_website            (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_website            (GtkAboutDialog  *about,
								const gchar     *website);
const gchar *          gtk_about_dialog_get_website_label      (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_website_label      (GtkAboutDialog  *about,
								const gchar     *website_label);
const gchar* const *   gtk_about_dialog_get_authors            (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_authors            (GtkAboutDialog  *about,
								const gchar    **authors);
const gchar* const *   gtk_about_dialog_get_documenters        (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_documenters        (GtkAboutDialog  *about,
								const gchar    **documenters);
const gchar* const *   gtk_about_dialog_get_artists            (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_artists            (GtkAboutDialog  *about,
								const gchar    **artists);
const gchar *          gtk_about_dialog_get_translator_credits (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_translator_credits (GtkAboutDialog  *about,
								const gchar     *translator_credits);
GdkPixbuf             *gtk_about_dialog_get_logo               (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_logo               (GtkAboutDialog  *about,
								GdkPixbuf       *logo);
const gchar *          gtk_about_dialog_get_logo_icon_name     (GtkAboutDialog  *about);
void                   gtk_about_dialog_set_logo_icon_name     (GtkAboutDialog  *about,
								const gchar     *icon_name);

/**
 * GtkAboutDialogActivateLinkFunc:
 * @about: the #GtkAboutDialog in which the link was activated
 * @link_: the URL or email address to which the activated link points
 * @data: user data that was passed when the function was registered
 *  with gtk_about_dialog_set_email_hook() or
 *  gtk_about_dialog_set_url_hook()
 *
 * The type of a function which is called when a URL or email
 * link is activated.
 */
typedef void (* GtkAboutDialogActivateLinkFunc) (GtkAboutDialog *about,
						 const gchar    *link_,
						 gpointer        data);

#ifndef GTK_DISABLE_DEPRECATED
GtkAboutDialogActivateLinkFunc gtk_about_dialog_set_email_hook (GtkAboutDialogActivateLinkFunc func,
								gpointer                       data,
								GDestroyNotify                 destroy);
GtkAboutDialogActivateLinkFunc gtk_about_dialog_set_url_hook   (GtkAboutDialogActivateLinkFunc func,
								gpointer                       data,
								GDestroyNotify                 destroy);
#endif

G_END_DECLS

#endif /* __GTK_ABOUT_DIALOG_H__ */


