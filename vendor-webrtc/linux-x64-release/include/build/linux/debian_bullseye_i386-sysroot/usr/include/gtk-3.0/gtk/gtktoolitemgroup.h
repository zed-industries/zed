/* GtkToolPalette -- A tool palette with categories and DnD support
 * Copyright (C) 2008  Openismus GmbH
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors:
 *      Mathias Hasselmann
 */

#ifndef __GTK_TOOL_ITEM_GROUP_H__
#define __GTK_TOOL_ITEM_GROUP_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>
#include <gtk/gtktoolitem.h>

G_BEGIN_DECLS

#define GTK_TYPE_TOOL_ITEM_GROUP           (gtk_tool_item_group_get_type ())
#define GTK_TOOL_ITEM_GROUP(obj)           (G_TYPE_CHECK_INSTANCE_CAST (obj, GTK_TYPE_TOOL_ITEM_GROUP, GtkToolItemGroup))
#define GTK_TOOL_ITEM_GROUP_CLASS(cls)     (G_TYPE_CHECK_CLASS_CAST (cls, GTK_TYPE_TOOL_ITEM_GROUP, GtkToolItemGroupClass))
#define GTK_IS_TOOL_ITEM_GROUP(obj)        (G_TYPE_CHECK_INSTANCE_TYPE (obj, GTK_TYPE_TOOL_ITEM_GROUP))
#define GTK_IS_TOOL_ITEM_GROUP_CLASS(obj)  (G_TYPE_CHECK_CLASS_TYPE (obj, GTK_TYPE_TOOL_ITEM_GROUP))
#define GTK_TOOL_ITEM_GROUP_GET_CLASS(obj) (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TOOL_ITEM_GROUP, GtkToolItemGroupClass))

typedef struct _GtkToolItemGroup        GtkToolItemGroup;
typedef struct _GtkToolItemGroupClass   GtkToolItemGroupClass;
typedef struct _GtkToolItemGroupPrivate GtkToolItemGroupPrivate;

/**
 * GtkToolItemGroup:
 *
 * This should not be accessed directly. Use the accessor functions below.
 */
struct _GtkToolItemGroup
{
  GtkContainer parent_instance;
  GtkToolItemGroupPrivate *priv;
};

/**
 * GtkToolItemGroupClass:
 * @parent_class: The parent class.
 */
struct _GtkToolItemGroupClass
{
  GtkContainerClass parent_class;

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType                 gtk_tool_item_group_get_type          (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget*            gtk_tool_item_group_new               (const gchar        *label);

GDK_AVAILABLE_IN_ALL
void                  gtk_tool_item_group_set_label         (GtkToolItemGroup   *group,
                                                             const gchar        *label);
GDK_AVAILABLE_IN_ALL
void                  gtk_tool_item_group_set_label_widget  (GtkToolItemGroup   *group,
                                                             GtkWidget          *label_widget);
GDK_AVAILABLE_IN_ALL
void                  gtk_tool_item_group_set_collapsed      (GtkToolItemGroup  *group,
                                                             gboolean            collapsed);
GDK_AVAILABLE_IN_ALL
void                  gtk_tool_item_group_set_ellipsize     (GtkToolItemGroup   *group,
                                                             PangoEllipsizeMode  ellipsize);
GDK_AVAILABLE_IN_ALL
void                  gtk_tool_item_group_set_header_relief (GtkToolItemGroup   *group,
                                                             GtkReliefStyle      style);

GDK_AVAILABLE_IN_ALL
const gchar *         gtk_tool_item_group_get_label         (GtkToolItemGroup   *group);
GDK_AVAILABLE_IN_ALL
GtkWidget            *gtk_tool_item_group_get_label_widget  (GtkToolItemGroup   *group);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_tool_item_group_get_collapsed     (GtkToolItemGroup   *group);
GDK_AVAILABLE_IN_ALL
PangoEllipsizeMode    gtk_tool_item_group_get_ellipsize     (GtkToolItemGroup   *group);
GDK_AVAILABLE_IN_ALL
GtkReliefStyle        gtk_tool_item_group_get_header_relief (GtkToolItemGroup   *group);

GDK_AVAILABLE_IN_ALL
void                  gtk_tool_item_group_insert            (GtkToolItemGroup   *group,
                                                             GtkToolItem        *item,
                                                             gint                position);
GDK_AVAILABLE_IN_ALL
void                  gtk_tool_item_group_set_item_position (GtkToolItemGroup   *group,
                                                             GtkToolItem        *item,
                                                             gint                position);
GDK_AVAILABLE_IN_ALL
gint                  gtk_tool_item_group_get_item_position (GtkToolItemGroup   *group,
                                                             GtkToolItem        *item);

GDK_AVAILABLE_IN_ALL
guint                 gtk_tool_item_group_get_n_items       (GtkToolItemGroup   *group);
GDK_AVAILABLE_IN_ALL
GtkToolItem*          gtk_tool_item_group_get_nth_item      (GtkToolItemGroup   *group,
                                                             guint               index);
GDK_AVAILABLE_IN_ALL
GtkToolItem*          gtk_tool_item_group_get_drop_item     (GtkToolItemGroup   *group,
                                                             gint                x,
                                                             gint                y);

G_END_DECLS

#endif /* __GTK_TOOL_ITEM_GROUP_H__ */
