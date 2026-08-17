/* GTK - The GIMP Toolkit
 * gtklinkbutton.h - an hyperlink-enabled button
 *
 * Copyright (C) 2005 Emmanuele Bassi <ebassi@gmail.com>
 * All rights reserved.
 *
 * Based on gnome-href code by:
 * 	James Henstridge <james@daa.com.au>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_LINK_BUTTON_H__
#define __GTK_LINK_BUTTON_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbutton.h>

G_BEGIN_DECLS

#define GTK_TYPE_LINK_BUTTON		(gtk_link_button_get_type ())
#define GTK_LINK_BUTTON(obj)		(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_LINK_BUTTON, GtkLinkButton))
#define GTK_IS_LINK_BUTTON(obj)		(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_LINK_BUTTON))
#define GTK_LINK_BUTTON_CLASS(klass)	(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_LINK_BUTTON, GtkLinkButtonClass))
#define GTK_IS_LINK_BUTTON_CLASS(klass)	(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_LINK_BUTTON))
#define GTK_LINK_BUTTON_GET_CLASS(obj)	(G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_LINK_BUTTON, GtkLinkButtonClass))

typedef struct _GtkLinkButton		GtkLinkButton;
typedef struct _GtkLinkButtonClass	GtkLinkButtonClass;
typedef struct _GtkLinkButtonPrivate	GtkLinkButtonPrivate;

/**
 * GtkLinkButton:
 *
 * The #GtkLinkButton-struct contains only
 * private data and should be accessed using the provided API.
 */
struct _GtkLinkButton
{
  /*< private >*/
  GtkButton parent_instance;

  GtkLinkButtonPrivate *priv;
};

/**
 * GtkLinkButtonClass:
 * @activate_link: class handler for the #GtkLinkButton::activate-link signal
 *
 * The #GtkLinkButtonClass contains only
 * private data.
 */
struct _GtkLinkButtonClass
{
  /*< private >*/
  GtkButtonClass parent_class;

  /*< public >*/
  gboolean (* activate_link) (GtkLinkButton *button);

  /*< private >*/
  /* Padding for future expansion */
  void (*_gtk_padding1) (void);
  void (*_gtk_padding2) (void);
  void (*_gtk_padding3) (void);
  void (*_gtk_padding4) (void);
};

GDK_AVAILABLE_IN_ALL
GType                 gtk_link_button_get_type          (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget *           gtk_link_button_new               (const gchar   *uri);
GDK_AVAILABLE_IN_ALL
GtkWidget *           gtk_link_button_new_with_label    (const gchar   *uri,
						         const gchar   *label);

GDK_AVAILABLE_IN_ALL
const gchar *         gtk_link_button_get_uri           (GtkLinkButton *link_button);
GDK_AVAILABLE_IN_ALL
void                  gtk_link_button_set_uri           (GtkLinkButton *link_button,
						         const gchar   *uri);

GDK_AVAILABLE_IN_ALL
gboolean              gtk_link_button_get_visited       (GtkLinkButton *link_button);
GDK_AVAILABLE_IN_ALL
void                  gtk_link_button_set_visited       (GtkLinkButton *link_button,
                                                         gboolean       visited);


G_END_DECLS

#endif /* __GTK_LINK_BUTTON_H__ */
