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

#ifndef __GTK_TREE_H__
#define __GTK_TREE_H__


#include <gtk/gtkcontainer.h>


G_BEGIN_DECLS

/* set this flag to enable tree debugging output */
/* #define TREE_DEBUG */

#define GTK_TYPE_TREE                  (gtk_tree_get_type ())
#define GTK_TREE(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TREE, GtkTree))
#define GTK_TREE_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TREE, GtkTreeClass))
#define GTK_IS_TREE(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TREE))
#define GTK_IS_TREE_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TREE))
#define GTK_TREE_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TREE, GtkTreeClass))


#define GTK_IS_ROOT_TREE(obj)   ((GtkObject*) GTK_TREE(obj)->root_tree == (GtkObject*)obj)
#define GTK_TREE_ROOT_TREE(obj) (GTK_TREE(obj)->root_tree ? GTK_TREE(obj)->root_tree : GTK_TREE(obj))
#define GTK_TREE_SELECTION_OLD(obj) (GTK_TREE_ROOT_TREE(obj)->selection)

typedef enum 
{
  GTK_TREE_VIEW_LINE,  /* default view mode */
  GTK_TREE_VIEW_ITEM
} GtkTreeViewMode;

typedef struct _GtkTree       GtkTree;
typedef struct _GtkTreeClass  GtkTreeClass;

struct _GtkTree
{
  GtkContainer container;
  
  GList *children;
  
  GtkTree* root_tree; /* owner of selection list */
  GtkWidget* tree_owner;
  GList *selection;
  guint level;
  guint indent_value;
  guint current_indent;
  guint selection_mode : 2;
  guint view_mode : 1;
  guint view_line : 1;
};

struct _GtkTreeClass
{
  GtkContainerClass parent_class;
  
  void (* selection_changed) (GtkTree   *tree);
  void (* select_child)      (GtkTree   *tree,
			      GtkWidget *child);
  void (* unselect_child)    (GtkTree   *tree,
			      GtkWidget *child);
};


GType      gtk_tree_get_type           (void) G_GNUC_CONST;
GtkWidget* gtk_tree_new                (void);
void       gtk_tree_append             (GtkTree          *tree,
				        GtkWidget        *tree_item);
void       gtk_tree_prepend            (GtkTree          *tree,
				        GtkWidget        *tree_item);
void       gtk_tree_insert             (GtkTree          *tree,
				        GtkWidget        *tree_item,
				        gint              position);
void       gtk_tree_remove_items       (GtkTree          *tree,
				        GList            *items);
void       gtk_tree_clear_items        (GtkTree          *tree,
				        gint              start,
				        gint              end);
void       gtk_tree_select_item        (GtkTree          *tree,
				        gint              item);
void       gtk_tree_unselect_item      (GtkTree          *tree,
				        gint              item);
void       gtk_tree_select_child       (GtkTree          *tree,
				        GtkWidget        *tree_item);
void       gtk_tree_unselect_child     (GtkTree          *tree,
				        GtkWidget        *tree_item);
gint       gtk_tree_child_position     (GtkTree          *tree,
				        GtkWidget        *child);
void       gtk_tree_set_selection_mode (GtkTree          *tree,
				        GtkSelectionMode  mode);
void       gtk_tree_set_view_mode      (GtkTree          *tree,
				        GtkTreeViewMode   mode); 
void       gtk_tree_set_view_lines     (GtkTree          *tree,
					gboolean	  flag);

/* deprecated function, use gtk_container_remove instead.
 */
void       gtk_tree_remove_item        (GtkTree          *tree,
				        GtkWidget        *child);


G_END_DECLS

#endif /* __GTK_TREE_H__ */

#endif /* GTK_ENABLE_BROKEN */
