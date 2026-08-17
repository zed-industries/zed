/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball, Josh MacDonald
 * Copyright (C) 1997-1998 Jay Painter <jpaint@serv.net><jpaint@gimp.org>
 *
 * GtkCTree widget for GTK+
 * Copyright (C) 1998 Lars Hamann and Stefan Jeske
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

#if !defined (GTK_DISABLE_DEPRECATED) || defined (__GTK_CLIST_C__) || defined (__GTK_CTREE_C__)

#ifndef __GTK_CTREE_H__
#define __GTK_CTREE_H__

#include <gtk/gtkclist.h>

G_BEGIN_DECLS

#define GTK_TYPE_CTREE            (gtk_ctree_get_type ())
#define GTK_CTREE(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CTREE, GtkCTree))
#define GTK_CTREE_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_CTREE, GtkCTreeClass))
#define GTK_IS_CTREE(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CTREE))
#define GTK_IS_CTREE_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_CTREE))
#define GTK_CTREE_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_CTREE, GtkCTreeClass))

#define GTK_CTREE_ROW(_node_) ((GtkCTreeRow *)(((GList *)(_node_))->data))
#define GTK_CTREE_NODE(_node_) ((GtkCTreeNode *)((_node_)))
#define GTK_CTREE_NODE_NEXT(_nnode_) ((GtkCTreeNode *)(((GList *)(_nnode_))->next))
#define GTK_CTREE_NODE_PREV(_pnode_) ((GtkCTreeNode *)(((GList *)(_pnode_))->prev))
#define GTK_CTREE_FUNC(_func_) ((GtkCTreeFunc)(_func_))

#define GTK_TYPE_CTREE_NODE (gtk_ctree_node_get_type ())

typedef enum
{
  GTK_CTREE_POS_BEFORE,
  GTK_CTREE_POS_AS_CHILD,
  GTK_CTREE_POS_AFTER
} GtkCTreePos;

typedef enum
{
  GTK_CTREE_LINES_NONE,
  GTK_CTREE_LINES_SOLID,
  GTK_CTREE_LINES_DOTTED,
  GTK_CTREE_LINES_TABBED
} GtkCTreeLineStyle;

typedef enum
{
  GTK_CTREE_EXPANDER_NONE,
  GTK_CTREE_EXPANDER_SQUARE,
  GTK_CTREE_EXPANDER_TRIANGLE,
  GTK_CTREE_EXPANDER_CIRCULAR
} GtkCTreeExpanderStyle;

typedef enum
{
  GTK_CTREE_EXPANSION_EXPAND,
  GTK_CTREE_EXPANSION_EXPAND_RECURSIVE,
  GTK_CTREE_EXPANSION_COLLAPSE,
  GTK_CTREE_EXPANSION_COLLAPSE_RECURSIVE,
  GTK_CTREE_EXPANSION_TOGGLE,
  GTK_CTREE_EXPANSION_TOGGLE_RECURSIVE
} GtkCTreeExpansionType;

typedef struct _GtkCTree      GtkCTree;
typedef struct _GtkCTreeClass GtkCTreeClass;
typedef struct _GtkCTreeRow   GtkCTreeRow;
typedef struct _GtkCTreeNode  GtkCTreeNode;

typedef void (*GtkCTreeFunc) (GtkCTree     *ctree,
			      GtkCTreeNode *node,
			      gpointer      data);

typedef gboolean (*GtkCTreeGNodeFunc) (GtkCTree     *ctree,
                                       guint         depth,
                                       GNode        *gnode,
				       GtkCTreeNode *cnode,
                                       gpointer      data);

typedef gboolean (*GtkCTreeCompareDragFunc) (GtkCTree     *ctree,
                                             GtkCTreeNode *source_node,
                                             GtkCTreeNode *new_parent,
                                             GtkCTreeNode *new_sibling);

struct _GtkCTree
{
  GtkCList clist;
  
  GdkGC *lines_gc;
  
  gint tree_indent;
  gint tree_spacing;
  gint tree_column;

  guint line_style     : 2;
  guint expander_style : 2;
  guint show_stub      : 1;

  GtkCTreeCompareDragFunc drag_compare;
};

struct _GtkCTreeClass
{
  GtkCListClass parent_class;
  
  void (*tree_select_row)   (GtkCTree     *ctree,
			     GtkCTreeNode *row,
			     gint          column);
  void (*tree_unselect_row) (GtkCTree     *ctree,
			     GtkCTreeNode *row,
			     gint          column);
  void (*tree_expand)       (GtkCTree     *ctree,
			     GtkCTreeNode *node);
  void (*tree_collapse)     (GtkCTree     *ctree,
			     GtkCTreeNode *node);
  void (*tree_move)         (GtkCTree     *ctree,
			     GtkCTreeNode *node,
			     GtkCTreeNode *new_parent,
			     GtkCTreeNode *new_sibling);
  void (*change_focus_row_expansion) (GtkCTree *ctree,
				      GtkCTreeExpansionType action);
};

struct _GtkCTreeRow
{
  GtkCListRow row;
  
  GtkCTreeNode *parent;
  GtkCTreeNode *sibling;
  GtkCTreeNode *children;
  
  GdkPixmap *pixmap_closed;
  GdkBitmap *mask_closed;
  GdkPixmap *pixmap_opened;
  GdkBitmap *mask_opened;
  
  guint16 level;
  
  guint is_leaf  : 1;
  guint expanded : 1;
};

struct _GtkCTreeNode {
  GList list;
};


/***********************************************************
 *           Creation, insertion, deletion                 *
 ***********************************************************/

GType   gtk_ctree_get_type                       (void) G_GNUC_CONST;
GtkWidget * gtk_ctree_new_with_titles            (gint          columns, 
						  gint          tree_column,
						  gchar        *titles[]);
GtkWidget * gtk_ctree_new                        (gint          columns, 
						  gint          tree_column);
GtkCTreeNode * gtk_ctree_insert_node             (GtkCTree     *ctree,
						  GtkCTreeNode *parent, 
						  GtkCTreeNode *sibling,
						  gchar        *text[],
						  guint8        spacing,
						  GdkPixmap    *pixmap_closed,
						  GdkBitmap    *mask_closed,
						  GdkPixmap    *pixmap_opened,
						  GdkBitmap    *mask_opened,
						  gboolean      is_leaf,
						  gboolean      expanded);
void gtk_ctree_remove_node                       (GtkCTree     *ctree, 
						  GtkCTreeNode *node);
GtkCTreeNode * gtk_ctree_insert_gnode            (GtkCTree          *ctree,
						  GtkCTreeNode      *parent,
						  GtkCTreeNode      *sibling,
						  GNode             *gnode,
						  GtkCTreeGNodeFunc  func,
						  gpointer           data);
GNode * gtk_ctree_export_to_gnode                (GtkCTree          *ctree,
						  GNode             *parent,
						  GNode             *sibling,
						  GtkCTreeNode      *node,
						  GtkCTreeGNodeFunc  func,
						  gpointer           data);

/***********************************************************
 *  Generic recursive functions, querying / finding tree   *
 *  information                                            *
 ***********************************************************/

void gtk_ctree_post_recursive                    (GtkCTree     *ctree, 
						  GtkCTreeNode *node,
						  GtkCTreeFunc  func,
						  gpointer      data);
void gtk_ctree_post_recursive_to_depth           (GtkCTree     *ctree, 
						  GtkCTreeNode *node,
						  gint          depth,
						  GtkCTreeFunc  func,
						  gpointer      data);
void gtk_ctree_pre_recursive                     (GtkCTree     *ctree, 
						  GtkCTreeNode *node,
						  GtkCTreeFunc  func,
						  gpointer      data);
void gtk_ctree_pre_recursive_to_depth            (GtkCTree     *ctree, 
						  GtkCTreeNode *node,
						  gint          depth,
						  GtkCTreeFunc  func,
						  gpointer      data);
gboolean gtk_ctree_is_viewable                   (GtkCTree     *ctree, 
					          GtkCTreeNode *node);
GtkCTreeNode * gtk_ctree_last                    (GtkCTree     *ctree,
					          GtkCTreeNode *node);
GtkCTreeNode * gtk_ctree_find_node_ptr           (GtkCTree     *ctree,
					          GtkCTreeRow  *ctree_row);
GtkCTreeNode * gtk_ctree_node_nth                (GtkCTree     *ctree,
						  guint         row);
gboolean gtk_ctree_find                          (GtkCTree     *ctree,
					          GtkCTreeNode *node,
					          GtkCTreeNode *child);
gboolean gtk_ctree_is_ancestor                   (GtkCTree     *ctree,
					          GtkCTreeNode *node,
					          GtkCTreeNode *child);
GtkCTreeNode * gtk_ctree_find_by_row_data        (GtkCTree     *ctree,
					          GtkCTreeNode *node,
					          gpointer      data);
/* returns a GList of all GtkCTreeNodes with row->data == data. */
GList * gtk_ctree_find_all_by_row_data           (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gpointer      data);
GtkCTreeNode * gtk_ctree_find_by_row_data_custom (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gpointer      data,
						  GCompareFunc  func);
/* returns a GList of all GtkCTreeNodes with row->data == data. */
GList * gtk_ctree_find_all_by_row_data_custom    (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gpointer      data,
						  GCompareFunc  func);
gboolean gtk_ctree_is_hot_spot                   (GtkCTree     *ctree,
					          gint          x,
					          gint          y);

/***********************************************************
 *   Tree signals : move, expand, collapse, (un)select     *
 ***********************************************************/

void gtk_ctree_move                              (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  GtkCTreeNode *new_parent, 
						  GtkCTreeNode *new_sibling);
void gtk_ctree_expand                            (GtkCTree     *ctree,
						  GtkCTreeNode *node);
void gtk_ctree_expand_recursive                  (GtkCTree     *ctree,
						  GtkCTreeNode *node);
void gtk_ctree_expand_to_depth                   (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gint          depth);
void gtk_ctree_collapse                          (GtkCTree     *ctree,
						  GtkCTreeNode *node);
void gtk_ctree_collapse_recursive                (GtkCTree     *ctree,
						  GtkCTreeNode *node);
void gtk_ctree_collapse_to_depth                 (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gint          depth);
void gtk_ctree_toggle_expansion                  (GtkCTree     *ctree,
						  GtkCTreeNode *node);
void gtk_ctree_toggle_expansion_recursive        (GtkCTree     *ctree,
						  GtkCTreeNode *node);
void gtk_ctree_select                            (GtkCTree     *ctree, 
						  GtkCTreeNode *node);
void gtk_ctree_select_recursive                  (GtkCTree     *ctree, 
						  GtkCTreeNode *node);
void gtk_ctree_unselect                          (GtkCTree     *ctree, 
						  GtkCTreeNode *node);
void gtk_ctree_unselect_recursive                (GtkCTree     *ctree, 
						  GtkCTreeNode *node);
void gtk_ctree_real_select_recursive             (GtkCTree     *ctree, 
						  GtkCTreeNode *node, 
						  gint          state);

/***********************************************************
 *           Analogons of GtkCList functions               *
 ***********************************************************/

void gtk_ctree_node_set_text                     (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gint          column,
						  const gchar  *text);
void gtk_ctree_node_set_pixmap                   (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gint          column,
						  GdkPixmap    *pixmap,
						  GdkBitmap    *mask);
void gtk_ctree_node_set_pixtext                  (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gint          column,
						  const gchar  *text,
						  guint8        spacing,
						  GdkPixmap    *pixmap,
						  GdkBitmap    *mask);
void gtk_ctree_set_node_info                     (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  const gchar  *text,
						  guint8        spacing,
						  GdkPixmap    *pixmap_closed,
						  GdkBitmap    *mask_closed,
						  GdkPixmap    *pixmap_opened,
						  GdkBitmap    *mask_opened,
						  gboolean      is_leaf,
						  gboolean      expanded);
void gtk_ctree_node_set_shift                    (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gint          column,
						  gint          vertical,
						  gint          horizontal);
void gtk_ctree_node_set_selectable               (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gboolean      selectable);
gboolean gtk_ctree_node_get_selectable           (GtkCTree     *ctree,
						  GtkCTreeNode *node);
GtkCellType gtk_ctree_node_get_cell_type         (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gint          column);
gboolean gtk_ctree_node_get_text                 (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gint          column,
						  gchar       **text);
gboolean gtk_ctree_node_get_pixmap               (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gint          column,
						  GdkPixmap   **pixmap,
						  GdkBitmap   **mask);
gboolean gtk_ctree_node_get_pixtext              (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gint          column,
						  gchar       **text,
						  guint8       *spacing,
						  GdkPixmap   **pixmap,
						  GdkBitmap   **mask);
gboolean gtk_ctree_get_node_info                 (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gchar       **text,
						  guint8       *spacing,
						  GdkPixmap   **pixmap_closed,
						  GdkBitmap   **mask_closed,
						  GdkPixmap   **pixmap_opened,
						  GdkBitmap   **mask_opened,
						  gboolean     *is_leaf,
						  gboolean     *expanded);
void gtk_ctree_node_set_row_style                (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  GtkStyle     *style);
GtkStyle * gtk_ctree_node_get_row_style          (GtkCTree     *ctree,
						  GtkCTreeNode *node);
void gtk_ctree_node_set_cell_style               (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gint          column,
						  GtkStyle     *style);
GtkStyle * gtk_ctree_node_get_cell_style         (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gint          column);
void gtk_ctree_node_set_foreground               (GtkCTree       *ctree,
						  GtkCTreeNode   *node,
						  const GdkColor *color);
void gtk_ctree_node_set_background               (GtkCTree       *ctree,
						  GtkCTreeNode   *node,
						  const GdkColor *color);
void gtk_ctree_node_set_row_data                 (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gpointer      data);
void gtk_ctree_node_set_row_data_full            (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gpointer      data,
						  GDestroyNotify destroy);
gpointer gtk_ctree_node_get_row_data             (GtkCTree     *ctree,
						  GtkCTreeNode *node);
void gtk_ctree_node_moveto                       (GtkCTree     *ctree,
						  GtkCTreeNode *node,
						  gint          column,
						  gfloat        row_align,
						  gfloat        col_align);
GtkVisibility gtk_ctree_node_is_visible          (GtkCTree     *ctree,
						  GtkCTreeNode *node);

/***********************************************************
 *             GtkCTree specific functions                 *
 ***********************************************************/

void gtk_ctree_set_indent            (GtkCTree                *ctree, 
				      gint                     indent);
void gtk_ctree_set_spacing           (GtkCTree                *ctree, 
				      gint                     spacing);
void gtk_ctree_set_show_stub         (GtkCTree                *ctree, 
				      gboolean                 show_stub);
void gtk_ctree_set_line_style        (GtkCTree                *ctree, 
				      GtkCTreeLineStyle        line_style);
void gtk_ctree_set_expander_style    (GtkCTree                *ctree, 
				      GtkCTreeExpanderStyle    expander_style);
void gtk_ctree_set_drag_compare_func (GtkCTree     	      *ctree,
				      GtkCTreeCompareDragFunc  cmp_func);

/***********************************************************
 *             Tree sorting functions                      *
 ***********************************************************/

void gtk_ctree_sort_node                         (GtkCTree     *ctree, 
						  GtkCTreeNode *node);
void gtk_ctree_sort_recursive                    (GtkCTree     *ctree, 
						  GtkCTreeNode *node);


#define gtk_ctree_set_reorderable(t,r)                    gtk_clist_set_reorderable((GtkCList*) (t),(r))

/* GType for the GtkCTreeNode.  This is a boxed type, although it uses
 * no-op's for the copy and free routines.  It is defined in order to
 * provide type information for the signal arguments
 */
GType   gtk_ctree_node_get_type                  (void) G_GNUC_CONST;

G_END_DECLS

#endif				/* __GTK_CTREE_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
