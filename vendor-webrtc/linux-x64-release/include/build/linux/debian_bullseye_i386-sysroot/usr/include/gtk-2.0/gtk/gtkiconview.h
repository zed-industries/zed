/* gtkiconview.h
 * Copyright (C) 2002, 2004  Anders Carlsson <andersca@gnome.org>
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

#ifndef __GTK_ICON_VIEW_H__
#define __GTK_ICON_VIEW_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>
#include <gtk/gtktreemodel.h>
#include <gtk/gtkcellrenderer.h>
#include <gtk/gtkselection.h>
#include <gtk/gtktooltip.h>

G_BEGIN_DECLS

#define GTK_TYPE_ICON_VIEW            (gtk_icon_view_get_type ())
#define GTK_ICON_VIEW(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ICON_VIEW, GtkIconView))
#define GTK_ICON_VIEW_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_ICON_VIEW, GtkIconViewClass))
#define GTK_IS_ICON_VIEW(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_ICON_VIEW))
#define GTK_IS_ICON_VIEW_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_ICON_VIEW))
#define GTK_ICON_VIEW_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_ICON_VIEW, GtkIconViewClass))

typedef struct _GtkIconView           GtkIconView;
typedef struct _GtkIconViewClass      GtkIconViewClass;
typedef struct _GtkIconViewPrivate    GtkIconViewPrivate;

typedef void (* GtkIconViewForeachFunc)     (GtkIconView      *icon_view,
					     GtkTreePath      *path,
					     gpointer          data);

typedef enum
{
  GTK_ICON_VIEW_NO_DROP,
  GTK_ICON_VIEW_DROP_INTO,
  GTK_ICON_VIEW_DROP_LEFT,
  GTK_ICON_VIEW_DROP_RIGHT,
  GTK_ICON_VIEW_DROP_ABOVE,
  GTK_ICON_VIEW_DROP_BELOW
} GtkIconViewDropPosition;

struct _GtkIconView
{
  GtkContainer parent;

  GtkIconViewPrivate *GSEAL (priv);
};

struct _GtkIconViewClass
{
  GtkContainerClass parent_class;

  void    (* set_scroll_adjustments) (GtkIconView      *icon_view,
				      GtkAdjustment    *hadjustment,
				      GtkAdjustment    *vadjustment);
  
  void    (* item_activated)         (GtkIconView      *icon_view,
				      GtkTreePath      *path);
  void    (* selection_changed)      (GtkIconView      *icon_view);

  /* Key binding signals */
  void    (* select_all)             (GtkIconView      *icon_view);
  void    (* unselect_all)           (GtkIconView      *icon_view);
  void    (* select_cursor_item)     (GtkIconView      *icon_view);
  void    (* toggle_cursor_item)     (GtkIconView      *icon_view);
  gboolean (* move_cursor)           (GtkIconView      *icon_view,
				      GtkMovementStep   step,
				      gint              count);
  gboolean (* activate_cursor_item)  (GtkIconView      *icon_view);
};

GType          gtk_icon_view_get_type          (void) G_GNUC_CONST;
GtkWidget *    gtk_icon_view_new               (void);
GtkWidget *    gtk_icon_view_new_with_model    (GtkTreeModel   *model);

void           gtk_icon_view_set_model         (GtkIconView    *icon_view,
 					        GtkTreeModel   *model);
GtkTreeModel * gtk_icon_view_get_model         (GtkIconView    *icon_view);
void           gtk_icon_view_set_text_column   (GtkIconView    *icon_view,
	 	 			        gint            column);
gint           gtk_icon_view_get_text_column   (GtkIconView    *icon_view);
void           gtk_icon_view_set_markup_column (GtkIconView    *icon_view,
					        gint            column);
gint           gtk_icon_view_get_markup_column (GtkIconView    *icon_view);
void           gtk_icon_view_set_pixbuf_column (GtkIconView    *icon_view,
					        gint            column);
gint           gtk_icon_view_get_pixbuf_column (GtkIconView    *icon_view);

void           gtk_icon_view_set_orientation   (GtkIconView    *icon_view,
	   			                GtkOrientation  orientation);
GtkOrientation gtk_icon_view_get_orientation   (GtkIconView    *icon_view);
void           gtk_icon_view_set_item_orientation (GtkIconView    *icon_view,
	   			                   GtkOrientation  orientation);
GtkOrientation gtk_icon_view_get_item_orientation (GtkIconView    *icon_view);
void           gtk_icon_view_set_columns       (GtkIconView    *icon_view,
		 			        gint            columns);
gint           gtk_icon_view_get_columns       (GtkIconView    *icon_view);
void           gtk_icon_view_set_item_width    (GtkIconView    *icon_view,
					        gint            item_width);
gint           gtk_icon_view_get_item_width    (GtkIconView    *icon_view);
void           gtk_icon_view_set_spacing       (GtkIconView    *icon_view, 
		 			        gint            spacing);
gint           gtk_icon_view_get_spacing       (GtkIconView    *icon_view);
void           gtk_icon_view_set_row_spacing   (GtkIconView    *icon_view, 
					        gint            row_spacing);
gint           gtk_icon_view_get_row_spacing   (GtkIconView    *icon_view);
void           gtk_icon_view_set_column_spacing (GtkIconView    *icon_view, 
					        gint            column_spacing);
gint           gtk_icon_view_get_column_spacing (GtkIconView    *icon_view);
void           gtk_icon_view_set_margin        (GtkIconView    *icon_view, 
					        gint            margin);
gint           gtk_icon_view_get_margin        (GtkIconView    *icon_view);
void           gtk_icon_view_set_item_padding  (GtkIconView    *icon_view, 
					        gint            item_padding);
gint           gtk_icon_view_get_item_padding  (GtkIconView    *icon_view);


GtkTreePath *  gtk_icon_view_get_path_at_pos   (GtkIconView     *icon_view,
						gint             x,
						gint             y);
gboolean       gtk_icon_view_get_item_at_pos   (GtkIconView     *icon_view,
						gint              x,
						gint              y,
						GtkTreePath     **path,
						GtkCellRenderer **cell);
gboolean       gtk_icon_view_get_visible_range (GtkIconView      *icon_view,
						GtkTreePath     **start_path,
						GtkTreePath     **end_path);

void           gtk_icon_view_selected_foreach   (GtkIconView            *icon_view,
						 GtkIconViewForeachFunc  func,
						 gpointer                data);
void           gtk_icon_view_set_selection_mode (GtkIconView            *icon_view,
						 GtkSelectionMode        mode);
GtkSelectionMode gtk_icon_view_get_selection_mode (GtkIconView            *icon_view);
void             gtk_icon_view_select_path        (GtkIconView            *icon_view,
						   GtkTreePath            *path);
void             gtk_icon_view_unselect_path      (GtkIconView            *icon_view,
						   GtkTreePath            *path);
gboolean         gtk_icon_view_path_is_selected   (GtkIconView            *icon_view,
						   GtkTreePath            *path);
gint             gtk_icon_view_get_item_row       (GtkIconView            *icon_view,
                                                   GtkTreePath            *path);
gint             gtk_icon_view_get_item_column    (GtkIconView            *icon_view,
                                                   GtkTreePath            *path);
GList           *gtk_icon_view_get_selected_items (GtkIconView            *icon_view);
void             gtk_icon_view_select_all         (GtkIconView            *icon_view);
void             gtk_icon_view_unselect_all       (GtkIconView            *icon_view);
void             gtk_icon_view_item_activated     (GtkIconView            *icon_view,
						   GtkTreePath            *path);
void             gtk_icon_view_set_cursor         (GtkIconView            *icon_view,
						   GtkTreePath            *path,
						   GtkCellRenderer        *cell,
						   gboolean                start_editing);
gboolean         gtk_icon_view_get_cursor         (GtkIconView            *icon_view,
						   GtkTreePath           **path,
						   GtkCellRenderer       **cell);
void             gtk_icon_view_scroll_to_path     (GtkIconView            *icon_view,
                                                   GtkTreePath            *path,
						   gboolean                use_align,
						   gfloat                  row_align,
                                                   gfloat                  col_align);

/* Drag-and-Drop support */
void                   gtk_icon_view_enable_model_drag_source (GtkIconView              *icon_view,
							       GdkModifierType           start_button_mask,
							       const GtkTargetEntry     *targets,
							       gint                      n_targets,
							       GdkDragAction             actions);
void                   gtk_icon_view_enable_model_drag_dest   (GtkIconView              *icon_view,
							       const GtkTargetEntry     *targets,
							       gint                      n_targets,
							       GdkDragAction             actions);
void                   gtk_icon_view_unset_model_drag_source  (GtkIconView              *icon_view);
void                   gtk_icon_view_unset_model_drag_dest    (GtkIconView              *icon_view);
void                   gtk_icon_view_set_reorderable          (GtkIconView              *icon_view,
							       gboolean                  reorderable);
gboolean               gtk_icon_view_get_reorderable          (GtkIconView              *icon_view);


/* These are useful to implement your own custom stuff. */
void                   gtk_icon_view_set_drag_dest_item       (GtkIconView              *icon_view,
							       GtkTreePath              *path,
							       GtkIconViewDropPosition   pos);
void                   gtk_icon_view_get_drag_dest_item       (GtkIconView              *icon_view,
							       GtkTreePath             **path,
							       GtkIconViewDropPosition  *pos);
gboolean               gtk_icon_view_get_dest_item_at_pos     (GtkIconView              *icon_view,
							       gint                      drag_x,
							       gint                      drag_y,
							       GtkTreePath             **path,
							       GtkIconViewDropPosition  *pos);
GdkPixmap             *gtk_icon_view_create_drag_icon         (GtkIconView              *icon_view,
							       GtkTreePath              *path);

void    gtk_icon_view_convert_widget_to_bin_window_coords     (GtkIconView *icon_view,
                                                               gint         wx,
                                                               gint         wy,
                                                               gint        *bx,
                                                               gint        *by);


void    gtk_icon_view_set_tooltip_item                        (GtkIconView     *icon_view,
                                                               GtkTooltip      *tooltip,
                                                               GtkTreePath     *path);
void    gtk_icon_view_set_tooltip_cell                        (GtkIconView     *icon_view,
                                                               GtkTooltip      *tooltip,
                                                               GtkTreePath     *path,
                                                               GtkCellRenderer *cell);
gboolean gtk_icon_view_get_tooltip_context                    (GtkIconView       *icon_view,
                                                               gint              *x,
                                                               gint              *y,
                                                               gboolean           keyboard_tip,
                                                               GtkTreeModel     **model,
                                                               GtkTreePath      **path,
                                                               GtkTreeIter       *iter);
void     gtk_icon_view_set_tooltip_column                     (GtkIconView       *icon_view,
                                                               gint               column);
gint     gtk_icon_view_get_tooltip_column                     (GtkIconView       *icon_view);


G_END_DECLS

#endif /* __GTK_ICON_VIEW_H__ */
