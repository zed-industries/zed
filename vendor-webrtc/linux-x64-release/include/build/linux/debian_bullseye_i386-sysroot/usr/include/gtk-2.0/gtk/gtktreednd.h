/* gtktreednd.h
 * Copyright (C) 2001  Red Hat, Inc.
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_TREE_DND_H__
#define __GTK_TREE_DND_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtktreemodel.h>
#include <gtk/gtkdnd.h>

G_BEGIN_DECLS

#define GTK_TYPE_TREE_DRAG_SOURCE            (gtk_tree_drag_source_get_type ())
#define GTK_TREE_DRAG_SOURCE(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TREE_DRAG_SOURCE, GtkTreeDragSource))
#define GTK_IS_TREE_DRAG_SOURCE(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TREE_DRAG_SOURCE))
#define GTK_TREE_DRAG_SOURCE_GET_IFACE(obj)  (G_TYPE_INSTANCE_GET_INTERFACE ((obj), GTK_TYPE_TREE_DRAG_SOURCE, GtkTreeDragSourceIface))

typedef struct _GtkTreeDragSource      GtkTreeDragSource; /* Dummy typedef */
typedef struct _GtkTreeDragSourceIface GtkTreeDragSourceIface;

struct _GtkTreeDragSourceIface
{
  GTypeInterface g_iface;

  /* VTable - not signals */

  gboolean     (* row_draggable)        (GtkTreeDragSource   *drag_source,
                                         GtkTreePath         *path);

  gboolean     (* drag_data_get)        (GtkTreeDragSource   *drag_source,
                                         GtkTreePath         *path,
                                         GtkSelectionData    *selection_data);

  gboolean     (* drag_data_delete)     (GtkTreeDragSource *drag_source,
                                         GtkTreePath       *path);
};

GType           gtk_tree_drag_source_get_type   (void) G_GNUC_CONST;

/* Returns whether the given row can be dragged */
gboolean gtk_tree_drag_source_row_draggable    (GtkTreeDragSource *drag_source,
                                                GtkTreePath       *path);

/* Deletes the given row, or returns FALSE if it can't */
gboolean gtk_tree_drag_source_drag_data_delete (GtkTreeDragSource *drag_source,
                                                GtkTreePath       *path);

/* Fills in selection_data with type selection_data->target based on
 * the row denoted by path, returns TRUE if it does anything
 */
gboolean gtk_tree_drag_source_drag_data_get    (GtkTreeDragSource *drag_source,
                                                GtkTreePath       *path,
                                                GtkSelectionData  *selection_data);

#define GTK_TYPE_TREE_DRAG_DEST            (gtk_tree_drag_dest_get_type ())
#define GTK_TREE_DRAG_DEST(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TREE_DRAG_DEST, GtkTreeDragDest))
#define GTK_IS_TREE_DRAG_DEST(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TREE_DRAG_DEST))
#define GTK_TREE_DRAG_DEST_GET_IFACE(obj)  (G_TYPE_INSTANCE_GET_INTERFACE ((obj), GTK_TYPE_TREE_DRAG_DEST, GtkTreeDragDestIface))

typedef struct _GtkTreeDragDest      GtkTreeDragDest; /* Dummy typedef */
typedef struct _GtkTreeDragDestIface GtkTreeDragDestIface;

struct _GtkTreeDragDestIface
{
  GTypeInterface g_iface;

  /* VTable - not signals */

  gboolean     (* drag_data_received) (GtkTreeDragDest   *drag_dest,
                                       GtkTreePath       *dest,
                                       GtkSelectionData  *selection_data);

  gboolean     (* row_drop_possible)  (GtkTreeDragDest   *drag_dest,
                                       GtkTreePath       *dest_path,
				       GtkSelectionData  *selection_data);
};

GType           gtk_tree_drag_dest_get_type   (void) G_GNUC_CONST;

/* Inserts a row before dest which contains data in selection_data,
 * or returns FALSE if it can't
 */
gboolean gtk_tree_drag_dest_drag_data_received (GtkTreeDragDest   *drag_dest,
						GtkTreePath       *dest,
						GtkSelectionData  *selection_data);


/* Returns TRUE if we can drop before path; path may not exist. */
gboolean gtk_tree_drag_dest_row_drop_possible  (GtkTreeDragDest   *drag_dest,
						GtkTreePath       *dest_path,
						GtkSelectionData  *selection_data);


/* The selection data would normally have target type GTK_TREE_MODEL_ROW in this
 * case. If the target is wrong these functions return FALSE.
 */
gboolean gtk_tree_set_row_drag_data            (GtkSelectionData  *selection_data,
						GtkTreeModel      *tree_model,
						GtkTreePath       *path);
gboolean gtk_tree_get_row_drag_data            (GtkSelectionData  *selection_data,
						GtkTreeModel     **tree_model,
						GtkTreePath      **path);

G_END_DECLS

#endif /* __GTK_TREE_DND_H__ */
