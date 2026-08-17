/* gtktoolitem.c
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

#ifndef __GTK_TOOL_ITEM_H__
#define __GTK_TOOL_ITEM_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbin.h>
#include <gtk/gtkmenuitem.h>
#include <gtk/gtksizegroup.h>

G_BEGIN_DECLS

#define GTK_TYPE_TOOL_ITEM            (gtk_tool_item_get_type ())
#define GTK_TOOL_ITEM(o)              (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_TOOL_ITEM, GtkToolItem))
#define GTK_TOOL_ITEM_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TOOL_ITEM, GtkToolItemClass))
#define GTK_IS_TOOL_ITEM(o)           (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_TOOL_ITEM))
#define GTK_IS_TOOL_ITEM_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TOOL_ITEM))
#define GTK_TOOL_ITEM_GET_CLASS(o)    (G_TYPE_INSTANCE_GET_CLASS((o), GTK_TYPE_TOOL_ITEM, GtkToolItemClass))

typedef struct _GtkToolItem        GtkToolItem;
typedef struct _GtkToolItemClass   GtkToolItemClass;
typedef struct _GtkToolItemPrivate GtkToolItemPrivate;

struct _GtkToolItem
{
  GtkBin parent;

  /*< private >*/
  GtkToolItemPrivate *priv;
};

/**
 * GtkToolItemClass:
 * @parent_class: The parent class.
 * @create_menu_proxy: Signal emitted when the toolbar needs
 *    information from tool_item about whether the item should appear in
 *    the toolbar overflow menu.
 * @toolbar_reconfigured: Signal emitted when some property of the
 *    toolbar that the item is a child of changes.
 */
struct _GtkToolItemClass
{
  GtkBinClass parent_class;

  /* signals */
  gboolean   (* create_menu_proxy)    (GtkToolItem *tool_item);
  void       (* toolbar_reconfigured) (GtkToolItem *tool_item);

  /*< private >*/

  /* Padding for future expansion */
  void (* _gtk_reserved1) (void);
  void (* _gtk_reserved2) (void);
  void (* _gtk_reserved3) (void);
  void (* _gtk_reserved4) (void);
};


GDK_AVAILABLE_IN_ALL
GType        gtk_tool_item_get_type (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkToolItem *gtk_tool_item_new      (void);

GDK_AVAILABLE_IN_ALL
void            gtk_tool_item_set_homogeneous          (GtkToolItem *tool_item,
							gboolean     homogeneous);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_tool_item_get_homogeneous          (GtkToolItem *tool_item);

GDK_AVAILABLE_IN_ALL
void            gtk_tool_item_set_expand               (GtkToolItem *tool_item,
							gboolean     expand);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_tool_item_get_expand               (GtkToolItem *tool_item);
GDK_AVAILABLE_IN_ALL
void            gtk_tool_item_set_tooltip_text         (GtkToolItem *tool_item,
							const gchar *text);
GDK_AVAILABLE_IN_ALL
void            gtk_tool_item_set_tooltip_markup       (GtkToolItem *tool_item,
							const gchar *markup);

GDK_AVAILABLE_IN_ALL
void            gtk_tool_item_set_use_drag_window      (GtkToolItem *tool_item,
							gboolean     use_drag_window);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_tool_item_get_use_drag_window      (GtkToolItem *tool_item);

GDK_AVAILABLE_IN_ALL
void            gtk_tool_item_set_visible_horizontal   (GtkToolItem *tool_item,
							gboolean     visible_horizontal);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_tool_item_get_visible_horizontal   (GtkToolItem *tool_item);

GDK_AVAILABLE_IN_ALL
void            gtk_tool_item_set_visible_vertical     (GtkToolItem *tool_item,
							gboolean     visible_vertical);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_tool_item_get_visible_vertical     (GtkToolItem *tool_item);

GDK_AVAILABLE_IN_ALL
gboolean        gtk_tool_item_get_is_important         (GtkToolItem *tool_item);
GDK_AVAILABLE_IN_ALL
void            gtk_tool_item_set_is_important         (GtkToolItem *tool_item,
							gboolean     is_important);

GDK_AVAILABLE_IN_ALL
PangoEllipsizeMode gtk_tool_item_get_ellipsize_mode    (GtkToolItem *tool_item);
GDK_AVAILABLE_IN_ALL
GtkIconSize     gtk_tool_item_get_icon_size            (GtkToolItem *tool_item);
GDK_AVAILABLE_IN_ALL
GtkOrientation  gtk_tool_item_get_orientation          (GtkToolItem *tool_item);
GDK_AVAILABLE_IN_ALL
GtkToolbarStyle gtk_tool_item_get_toolbar_style        (GtkToolItem *tool_item);
GDK_AVAILABLE_IN_ALL
GtkReliefStyle  gtk_tool_item_get_relief_style         (GtkToolItem *tool_item);
GDK_AVAILABLE_IN_ALL
gfloat          gtk_tool_item_get_text_alignment       (GtkToolItem *tool_item);
GDK_AVAILABLE_IN_ALL
GtkOrientation  gtk_tool_item_get_text_orientation     (GtkToolItem *tool_item);
GDK_AVAILABLE_IN_ALL
GtkSizeGroup *  gtk_tool_item_get_text_size_group      (GtkToolItem *tool_item);

GDK_AVAILABLE_IN_ALL
GtkWidget *     gtk_tool_item_retrieve_proxy_menu_item (GtkToolItem *tool_item);
GDK_AVAILABLE_IN_ALL
GtkWidget *     gtk_tool_item_get_proxy_menu_item      (GtkToolItem *tool_item,
							const gchar *menu_item_id);
GDK_AVAILABLE_IN_ALL
void            gtk_tool_item_set_proxy_menu_item      (GtkToolItem *tool_item,
							const gchar *menu_item_id,
							GtkWidget   *menu_item);
GDK_AVAILABLE_IN_ALL
void		gtk_tool_item_rebuild_menu	       (GtkToolItem *tool_item);

GDK_AVAILABLE_IN_ALL
void            gtk_tool_item_toolbar_reconfigured     (GtkToolItem *tool_item);

/* private */

gboolean       _gtk_tool_item_create_menu_proxy        (GtkToolItem *tool_item);

G_END_DECLS

#endif /* __GTK_TOOL_ITEM_H__ */
