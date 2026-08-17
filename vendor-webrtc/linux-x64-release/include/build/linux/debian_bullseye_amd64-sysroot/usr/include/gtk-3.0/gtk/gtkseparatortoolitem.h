/* gtktoggletoolbutton.h
 *
 * Copyright (C) 2002 Anders Carlsson <andersca@gnome.org>
 * Copyright (C) 2002 James Henstridge <james@daa.com.au>
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

#ifndef __GTK_SEPARATOR_TOOL_ITEM_H__
#define __GTK_SEPARATOR_TOOL_ITEM_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtktoolitem.h>

G_BEGIN_DECLS

#define GTK_TYPE_SEPARATOR_TOOL_ITEM            (gtk_separator_tool_item_get_type ())
#define GTK_SEPARATOR_TOOL_ITEM(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SEPARATOR_TOOL_ITEM, GtkSeparatorToolItem))
#define GTK_SEPARATOR_TOOL_ITEM_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SEPARATOR_TOOL_ITEM, GtkSeparatorToolItemClass))
#define GTK_IS_SEPARATOR_TOOL_ITEM(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SEPARATOR_TOOL_ITEM))
#define GTK_IS_SEPARATOR_TOOL_ITEM_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SEPARATOR_TOOL_ITEM))
#define GTK_SEPARATOR_TOOL_ITEM_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS((obj), GTK_TYPE_SEPARATOR_TOOL_ITEM, GtkSeparatorToolItemClass))

typedef struct _GtkSeparatorToolItem        GtkSeparatorToolItem;
typedef struct _GtkSeparatorToolItemClass   GtkSeparatorToolItemClass;
typedef struct _GtkSeparatorToolItemPrivate GtkSeparatorToolItemPrivate;

struct _GtkSeparatorToolItem
{
  GtkToolItem parent;

  /*< private >*/
  GtkSeparatorToolItemPrivate *priv;
};

/**
 * GtkSeparatorToolItemClass:
 * @parent_class: The parent class.
 */
struct _GtkSeparatorToolItemClass
{
  GtkToolItemClass parent_class;

  /*< private >*/

  /* Padding for future expansion */
  void (* _gtk_reserved1) (void);
  void (* _gtk_reserved2) (void);
  void (* _gtk_reserved3) (void);
  void (* _gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType        gtk_separator_tool_item_get_type (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkToolItem *gtk_separator_tool_item_new      (void);

GDK_AVAILABLE_IN_ALL
gboolean     gtk_separator_tool_item_get_draw (GtkSeparatorToolItem *item);
GDK_AVAILABLE_IN_ALL
void         gtk_separator_tool_item_set_draw (GtkSeparatorToolItem *item,
					       gboolean              draw);

G_END_DECLS

#endif /* __GTK_SEPARATOR_TOOL_ITEM_H__ */
