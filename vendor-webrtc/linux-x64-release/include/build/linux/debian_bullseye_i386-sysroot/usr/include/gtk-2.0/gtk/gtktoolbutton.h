/* gtktoolbutton.h
 *
 * Copyright (C) 2002 Anders Carlsson <andersca@gnome.org>
 * Copyright (C) 2002 James Henstridge <james@daa.com.au>
 * Copyright (C) 2003 Soeren Sandmann <sandmann@daimi.au.dk>
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

#ifndef __GTK_TOOL_BUTTON_H__
#define __GTK_TOOL_BUTTON_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtktoolitem.h>

G_BEGIN_DECLS

#define GTK_TYPE_TOOL_BUTTON            (gtk_tool_button_get_type ())
#define GTK_TOOL_BUTTON(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TOOL_BUTTON, GtkToolButton))
#define GTK_TOOL_BUTTON_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TOOL_BUTTON, GtkToolButtonClass))
#define GTK_IS_TOOL_BUTTON(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TOOL_BUTTON))
#define GTK_IS_TOOL_BUTTON_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TOOL_BUTTON))
#define GTK_TOOL_BUTTON_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS((obj), GTK_TYPE_TOOL_BUTTON, GtkToolButtonClass))

typedef struct _GtkToolButton        GtkToolButton;
typedef struct _GtkToolButtonClass   GtkToolButtonClass;
typedef struct _GtkToolButtonPrivate GtkToolButtonPrivate;

struct _GtkToolButton
{
  GtkToolItem parent;

  /*< private >*/
  GtkToolButtonPrivate *GSEAL (priv);
};

struct _GtkToolButtonClass
{
  GtkToolItemClass parent_class;

  GType button_type;

  /* signal */
  void       (* clicked)             (GtkToolButton    *tool_item);

  /* Padding for future expansion */
  void (* _gtk_reserved1) (void);
  void (* _gtk_reserved2) (void);
  void (* _gtk_reserved3) (void);
  void (* _gtk_reserved4) (void);
};

GType        gtk_tool_button_get_type       (void) G_GNUC_CONST;
GtkToolItem *gtk_tool_button_new            (GtkWidget   *icon_widget,
					     const gchar *label);
GtkToolItem *gtk_tool_button_new_from_stock (const gchar *stock_id);

void                  gtk_tool_button_set_label         (GtkToolButton *button,
							 const gchar   *label);
const gchar *         gtk_tool_button_get_label         (GtkToolButton *button);
void                  gtk_tool_button_set_use_underline (GtkToolButton *button,
							 gboolean       use_underline);
gboolean              gtk_tool_button_get_use_underline (GtkToolButton *button);
void                  gtk_tool_button_set_stock_id      (GtkToolButton *button,
							 const gchar   *stock_id);
const gchar *         gtk_tool_button_get_stock_id      (GtkToolButton *button);
void                  gtk_tool_button_set_icon_name     (GtkToolButton *button,
							 const gchar   *icon_name);
const gchar *         gtk_tool_button_get_icon_name     (GtkToolButton *button);
void                  gtk_tool_button_set_icon_widget   (GtkToolButton *button,
							 GtkWidget     *icon_widget);
GtkWidget *           gtk_tool_button_get_icon_widget   (GtkToolButton *button);
void                  gtk_tool_button_set_label_widget  (GtkToolButton *button,
							 GtkWidget     *label_widget);
GtkWidget *           gtk_tool_button_get_label_widget  (GtkToolButton *button);


/* internal function */
GtkWidget *_gtk_tool_button_get_button (GtkToolButton *button);

G_END_DECLS

#endif /* __GTK_TOOL_BUTTON_H__ */
