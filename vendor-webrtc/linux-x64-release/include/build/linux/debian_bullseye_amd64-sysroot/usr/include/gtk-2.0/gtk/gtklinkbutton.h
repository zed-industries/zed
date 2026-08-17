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
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Cambridge, MA 02139, USA.
 */

#ifndef __GTK_LINK_BUTTON_H__
#define __GTK_LINK_BUTTON_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
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

typedef void (*GtkLinkButtonUriFunc) (GtkLinkButton *button,
				      const gchar   *link_,
				      gpointer       user_data);

struct _GtkLinkButton
{
  GtkButton parent_instance;

  GtkLinkButtonPrivate *GSEAL (priv);
};

struct _GtkLinkButtonClass
{
  GtkButtonClass parent_class;

  void (*_gtk_padding1) (void);
  void (*_gtk_padding2) (void);
  void (*_gtk_padding3) (void);
  void (*_gtk_padding4) (void);
};

GType                 gtk_link_button_get_type          (void) G_GNUC_CONST;

GtkWidget *           gtk_link_button_new               (const gchar   *uri);
GtkWidget *           gtk_link_button_new_with_label    (const gchar   *uri,
						         const gchar   *label);

const gchar *         gtk_link_button_get_uri           (GtkLinkButton *link_button);
void                  gtk_link_button_set_uri           (GtkLinkButton *link_button,
						         const gchar   *uri);

#ifndef GTK_DISABLE_DEPRECATED
GtkLinkButtonUriFunc  gtk_link_button_set_uri_hook      (GtkLinkButtonUriFunc func,
							 gpointer             data,
							 GDestroyNotify       destroy);
#endif

gboolean              gtk_link_button_get_visited       (GtkLinkButton *link_button);
void                  gtk_link_button_set_visited       (GtkLinkButton *link_button,
                                                         gboolean       visited);


G_END_DECLS

#endif /* __GTK_LINK_BUTTON_H__ */
