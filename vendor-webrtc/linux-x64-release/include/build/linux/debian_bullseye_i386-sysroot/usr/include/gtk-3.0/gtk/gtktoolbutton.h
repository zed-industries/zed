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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_TOOL_BUTTON_H__
#define __GTK_TOOL_BUTTON_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
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
  GtkToolButtonPrivate *priv;
};

/**
 * GtkToolButtonClass:
 * @parent_class: The parent class.
 * @button_type: 
 * @clicked: Signal emitted when the tool button is clicked with the
 *    mouse or activated with the keyboard.
 */
struct _GtkToolButtonClass
{
  GtkToolItemClass parent_class;

  GType button_type;

  /*< public >*/

  /* signal */
  void       (* clicked)             (GtkToolButton    *tool_item);

  /*< private >*/

  /* Padding for future expansion */
  void (* _gtk_reserved1) (void);
  void (* _gtk_reserved2) (void);
  void (* _gtk_reserved3) (void);
  void (* _gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType        gtk_tool_button_get_type       (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkToolItem *gtk_tool_button_new            (GtkWidget   *icon_widget,
					     const gchar *label);
GDK_DEPRECATED_IN_3_10_FOR(gtk_tool_button_new)
GtkToolItem *gtk_tool_button_new_from_stock (const gchar *stock_id);

GDK_AVAILABLE_IN_ALL
void                  gtk_tool_button_set_label         (GtkToolButton *button,
							 const gchar   *label);
GDK_AVAILABLE_IN_ALL
const gchar *         gtk_tool_button_get_label         (GtkToolButton *button);
GDK_AVAILABLE_IN_ALL
void                  gtk_tool_button_set_use_underline (GtkToolButton *button,
							 gboolean       use_underline);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_tool_button_get_use_underline (GtkToolButton *button);
GDK_DEPRECATED_IN_3_10_FOR(gtk_tool_button_set_icon_name)
void                  gtk_tool_button_set_stock_id      (GtkToolButton *button,
							 const gchar   *stock_id);
GDK_DEPRECATED_IN_3_10_FOR(gtk_tool_button_get_icon_name)
const gchar *         gtk_tool_button_get_stock_id      (GtkToolButton *button);
GDK_AVAILABLE_IN_ALL
void                  gtk_tool_button_set_icon_name     (GtkToolButton *button,
							 const gchar   *icon_name);
GDK_AVAILABLE_IN_ALL
const gchar *         gtk_tool_button_get_icon_name     (GtkToolButton *button);
GDK_AVAILABLE_IN_ALL
void                  gtk_tool_button_set_icon_widget   (GtkToolButton *button,
							 GtkWidget     *icon_widget);
GDK_AVAILABLE_IN_ALL
GtkWidget *           gtk_tool_button_get_icon_widget   (GtkToolButton *button);
GDK_AVAILABLE_IN_ALL
void                  gtk_tool_button_set_label_widget  (GtkToolButton *button,
							 GtkWidget     *label_widget);
GDK_AVAILABLE_IN_ALL
GtkWidget *           gtk_tool_button_get_label_widget  (GtkToolButton *button);


/* internal function */
GtkWidget *_gtk_tool_button_get_button (GtkToolButton *button);

G_END_DECLS

#endif /* __GTK_TOOL_BUTTON_H__ */
