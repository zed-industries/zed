/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
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

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifdef GTK_ENABLE_BROKEN

#ifndef __GTK_TREE_ITEM_H__
#define __GTK_TREE_ITEM_H__


#include <gtk/gtkitem.h>


G_BEGIN_DECLS

#define GTK_TYPE_TREE_ITEM              (gtk_tree_item_get_type ())
#define GTK_TREE_ITEM(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TREE_ITEM, GtkTreeItem))
#define GTK_TREE_ITEM_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TREE_ITEM, GtkTreeItemClass))
#define GTK_IS_TREE_ITEM(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TREE_ITEM))
#define GTK_IS_TREE_ITEM_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TREE_ITEM))
#define GTK_TREE_ITEM_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TREE_ITEM, GtkTreeItemClass))


#define GTK_TREE_ITEM_SUBTREE(obj)      (GTK_TREE_ITEM(obj)->subtree)


typedef struct _GtkTreeItem       GtkTreeItem;
typedef struct _GtkTreeItemClass  GtkTreeItemClass;

struct _GtkTreeItem
{
  GtkItem item;

  GtkWidget *subtree;
  GtkWidget *pixmaps_box;
  GtkWidget *plus_pix_widget, *minus_pix_widget;

  GList *pixmaps;		/* pixmap node for this items color depth */

  guint expanded : 1;
};

struct _GtkTreeItemClass
{
  GtkItemClass parent_class;

  void (* expand)   (GtkTreeItem *tree_item);
  void (* collapse) (GtkTreeItem *tree_item);
};


GType      gtk_tree_item_get_type       (void) G_GNUC_CONST;
GtkWidget* gtk_tree_item_new            (void);
GtkWidget* gtk_tree_item_new_with_label (const gchar *label);
void       gtk_tree_item_set_subtree    (GtkTreeItem *tree_item,
					 GtkWidget   *subtree);
void       gtk_tree_item_remove_subtree (GtkTreeItem *tree_item);
void       gtk_tree_item_select         (GtkTreeItem *tree_item);
void       gtk_tree_item_deselect       (GtkTreeItem *tree_item);
void       gtk_tree_item_expand         (GtkTreeItem *tree_item);
void       gtk_tree_item_collapse       (GtkTreeItem *tree_item);


G_END_DECLS

#endif /* __GTK_TREE_ITEM_H__ */

#endif /* GTK_ENABLE_BROKEN */
