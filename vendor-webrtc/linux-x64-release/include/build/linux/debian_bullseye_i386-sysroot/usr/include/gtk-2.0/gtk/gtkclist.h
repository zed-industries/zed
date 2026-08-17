/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball, Josh MacDonald
 * Copyright (C) 1997-1998 Jay Painter <jpaint@serv.net><jpaint@gimp.org>
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

#ifndef __GTK_CLIST_H__
#define __GTK_CLIST_H__


#include <gtk/gtksignal.h>
#include <gtk/gtkalignment.h>
#include <gtk/gtklabel.h>
#include <gtk/gtkbutton.h>
#include <gtk/gtkhscrollbar.h>
#include <gtk/gtkvscrollbar.h>


G_BEGIN_DECLS


/* clist flags */
enum {
  GTK_CLIST_IN_DRAG             = 1 <<  0,
  GTK_CLIST_ROW_HEIGHT_SET      = 1 <<  1,
  GTK_CLIST_SHOW_TITLES         = 1 <<  2,
  /* Unused */
  GTK_CLIST_ADD_MODE            = 1 <<  4,
  GTK_CLIST_AUTO_SORT           = 1 <<  5,
  GTK_CLIST_AUTO_RESIZE_BLOCKED = 1 <<  6,
  GTK_CLIST_REORDERABLE         = 1 <<  7,
  GTK_CLIST_USE_DRAG_ICONS      = 1 <<  8,
  GTK_CLIST_DRAW_DRAG_LINE      = 1 <<  9,
  GTK_CLIST_DRAW_DRAG_RECT      = 1 << 10
};

/* cell types */
typedef enum
{
  GTK_CELL_EMPTY,
  GTK_CELL_TEXT,
  GTK_CELL_PIXMAP,
  GTK_CELL_PIXTEXT,
  GTK_CELL_WIDGET
} GtkCellType;

typedef enum
{
  GTK_CLIST_DRAG_NONE,
  GTK_CLIST_DRAG_BEFORE,
  GTK_CLIST_DRAG_INTO,
  GTK_CLIST_DRAG_AFTER
} GtkCListDragPos;

typedef enum
{
  GTK_BUTTON_IGNORED = 0,
  GTK_BUTTON_SELECTS = 1 << 0,
  GTK_BUTTON_DRAGS   = 1 << 1,
  GTK_BUTTON_EXPANDS = 1 << 2
} GtkButtonAction;

#define GTK_TYPE_CLIST            (gtk_clist_get_type ())
#define GTK_CLIST(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CLIST, GtkCList))
#define GTK_CLIST_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_CLIST, GtkCListClass))
#define GTK_IS_CLIST(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CLIST))
#define GTK_IS_CLIST_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_CLIST))
#define GTK_CLIST_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_CLIST, GtkCListClass))


#define GTK_CLIST_FLAGS(clist)             (GTK_CLIST (clist)->flags)
#define GTK_CLIST_SET_FLAG(clist,flag)     (GTK_CLIST_FLAGS (clist) |= (GTK_ ## flag))
#define GTK_CLIST_UNSET_FLAG(clist,flag)   (GTK_CLIST_FLAGS (clist) &= ~(GTK_ ## flag))

#define GTK_CLIST_IN_DRAG(clist)           (GTK_CLIST_FLAGS (clist) & GTK_CLIST_IN_DRAG)
#define GTK_CLIST_ROW_HEIGHT_SET(clist)    (GTK_CLIST_FLAGS (clist) & GTK_CLIST_ROW_HEIGHT_SET)
#define GTK_CLIST_SHOW_TITLES(clist)       (GTK_CLIST_FLAGS (clist) & GTK_CLIST_SHOW_TITLES)
#define GTK_CLIST_ADD_MODE(clist)          (GTK_CLIST_FLAGS (clist) & GTK_CLIST_ADD_MODE)
#define GTK_CLIST_AUTO_SORT(clist)         (GTK_CLIST_FLAGS (clist) & GTK_CLIST_AUTO_SORT)
#define GTK_CLIST_AUTO_RESIZE_BLOCKED(clist) (GTK_CLIST_FLAGS (clist) & GTK_CLIST_AUTO_RESIZE_BLOCKED)
#define GTK_CLIST_REORDERABLE(clist)       (GTK_CLIST_FLAGS (clist) & GTK_CLIST_REORDERABLE)
#define GTK_CLIST_USE_DRAG_ICONS(clist)    (GTK_CLIST_FLAGS (clist) & GTK_CLIST_USE_DRAG_ICONS)
#define GTK_CLIST_DRAW_DRAG_LINE(clist)    (GTK_CLIST_FLAGS (clist) & GTK_CLIST_DRAW_DRAG_LINE)
#define GTK_CLIST_DRAW_DRAG_RECT(clist)    (GTK_CLIST_FLAGS (clist) & GTK_CLIST_DRAW_DRAG_RECT)

#define GTK_CLIST_ROW(_glist_) ((GtkCListRow *)((_glist_)->data))

/* pointer casting for cells */
#define GTK_CELL_TEXT(cell)     (((GtkCellText *) &(cell)))
#define GTK_CELL_PIXMAP(cell)   (((GtkCellPixmap *) &(cell)))
#define GTK_CELL_PIXTEXT(cell)  (((GtkCellPixText *) &(cell)))
#define GTK_CELL_WIDGET(cell)   (((GtkCellWidget *) &(cell)))

typedef struct _GtkCList GtkCList;
typedef struct _GtkCListClass GtkCListClass;
typedef struct _GtkCListColumn GtkCListColumn;
typedef struct _GtkCListRow GtkCListRow;

typedef struct _GtkCell GtkCell;
typedef struct _GtkCellText GtkCellText;
typedef struct _GtkCellPixmap GtkCellPixmap;
typedef struct _GtkCellPixText GtkCellPixText;
typedef struct _GtkCellWidget GtkCellWidget;

typedef gint (*GtkCListCompareFunc) (GtkCList     *clist,
				     gconstpointer ptr1,
				     gconstpointer ptr2);

typedef struct _GtkCListCellInfo GtkCListCellInfo;
typedef struct _GtkCListDestInfo GtkCListDestInfo;

struct _GtkCListCellInfo
{
  gint row;
  gint column;
};

struct _GtkCListDestInfo
{
  GtkCListCellInfo cell;
  GtkCListDragPos  insert_pos;
};

struct _GtkCList
{
  GtkContainer container;
  
  guint16 flags;
  
  gpointer reserved1;
  gpointer reserved2;

  guint freeze_count;
  
  /* allocation rectangle after the container_border_width
   * and the width of the shadow border */
  GdkRectangle internal_allocation;
  
  /* rows */
  gint rows;
  gint row_height;
  GList *row_list;
  GList *row_list_end;
  
  /* columns */
  gint columns;
  GdkRectangle column_title_area;
  GdkWindow *title_window;
  
  /* dynamicly allocated array of column structures */
  GtkCListColumn *column;
  
  /* the scrolling window and its height and width to
   * make things a little speedier */
  GdkWindow *clist_window;
  gint clist_window_width;
  gint clist_window_height;
  
  /* offsets for scrolling */
  gint hoffset;
  gint voffset;
  
  /* border shadow style */
  GtkShadowType shadow_type;
  
  /* the list's selection mode (gtkenums.h) */
  GtkSelectionMode selection_mode;
  
  /* list of selected rows */
  GList *selection;
  GList *selection_end;
  
  GList *undo_selection;
  GList *undo_unselection;
  gint undo_anchor;
  
  /* mouse buttons */
  guint8 button_actions[5];

  guint8 drag_button;

  /* dnd */
  GtkCListCellInfo click_cell;

  /* scroll adjustments */
  GtkAdjustment *hadjustment;
  GtkAdjustment *vadjustment;
  
  /* xor GC for the vertical drag line */
  GdkGC *xor_gc;
  
  /* gc for drawing unselected cells */
  GdkGC *fg_gc;
  GdkGC *bg_gc;
  
  /* cursor used to indicate dragging */
  GdkCursor *cursor_drag;
  
  /* the current x-pixel location of the xor-drag line */
  gint x_drag;
  
  /* focus handling */
  gint focus_row;

  gint focus_header_column;
  
  /* dragging the selection */
  gint anchor;
  GtkStateType anchor_state;
  gint drag_pos;
  gint htimer;
  gint vtimer;
  
  GtkSortType sort_type;
  GtkCListCompareFunc compare;
  gint sort_column;

  gint drag_highlight_row;
  GtkCListDragPos drag_highlight_pos;
};

struct _GtkCListClass
{
  GtkContainerClass parent_class;
  
  void  (*set_scroll_adjustments) (GtkCList       *clist,
				   GtkAdjustment  *hadjustment,
				   GtkAdjustment  *vadjustment);
  void   (*refresh)             (GtkCList       *clist);
  void   (*select_row)          (GtkCList       *clist,
				 gint            row,
				 gint            column,
				 GdkEvent       *event);
  void   (*unselect_row)        (GtkCList       *clist,
				 gint            row,
				 gint            column,
				 GdkEvent       *event);
  void   (*row_move)            (GtkCList       *clist,
				 gint            source_row,
				 gint            dest_row);
  void   (*click_column)        (GtkCList       *clist,
				 gint            column);
  void   (*resize_column)       (GtkCList       *clist,
				 gint            column,
                                 gint            width);
  void   (*toggle_focus_row)    (GtkCList       *clist);
  void   (*select_all)          (GtkCList       *clist);
  void   (*unselect_all)        (GtkCList       *clist);
  void   (*undo_selection)      (GtkCList       *clist);
  void   (*start_selection)     (GtkCList       *clist);
  void   (*end_selection)       (GtkCList       *clist);
  void   (*extend_selection)    (GtkCList       *clist,
				 GtkScrollType   scroll_type,
				 gfloat          position,
				 gboolean        auto_start_selection);
  void   (*scroll_horizontal)   (GtkCList       *clist,
				 GtkScrollType   scroll_type,
				 gfloat          position);
  void   (*scroll_vertical)     (GtkCList       *clist,
				 GtkScrollType   scroll_type,
				 gfloat          position);
  void   (*toggle_add_mode)     (GtkCList       *clist);
  void   (*abort_column_resize) (GtkCList       *clist);
  void   (*resync_selection)    (GtkCList       *clist,
				 GdkEvent       *event);
  GList* (*selection_find)      (GtkCList       *clist,
				 gint            row_number,
				 GList          *row_list_element);
  void   (*draw_row)            (GtkCList       *clist,
				 GdkRectangle   *area,
				 gint            row,
				 GtkCListRow    *clist_row);
  void   (*draw_drag_highlight) (GtkCList        *clist,
				 GtkCListRow     *target_row,
				 gint             target_row_number,
				 GtkCListDragPos  drag_pos);
  void   (*clear)               (GtkCList       *clist);
  void   (*fake_unselect_all)   (GtkCList       *clist,
				 gint            row);
  void   (*sort_list)           (GtkCList       *clist);
  gint   (*insert_row)          (GtkCList       *clist,
				 gint            row,
				 gchar          *text[]);
  void   (*remove_row)          (GtkCList       *clist,
				 gint            row);
  void   (*set_cell_contents)   (GtkCList       *clist,
				 GtkCListRow    *clist_row,
				 gint            column,
				 GtkCellType     type,
				 const gchar    *text,
				 guint8          spacing,
				 GdkPixmap      *pixmap,
				 GdkBitmap      *mask);
  void   (*cell_size_request)   (GtkCList       *clist,
				 GtkCListRow    *clist_row,
				 gint            column,
				 GtkRequisition *requisition);

};

struct _GtkCListColumn
{
  gchar *title;
  GdkRectangle area;
  
  GtkWidget *button;
  GdkWindow *window;
  
  gint width;
  gint min_width;
  gint max_width;
  GtkJustification justification;
  
  guint visible        : 1;  
  guint width_set      : 1;
  guint resizeable     : 1;
  guint auto_resize    : 1;
  guint button_passive : 1;
};

struct _GtkCListRow
{
  GtkCell *cell;
  GtkStateType state;
  
  GdkColor foreground;
  GdkColor background;
  
  GtkStyle *style;

  gpointer data;
  GDestroyNotify destroy;
  
  guint fg_set     : 1;
  guint bg_set     : 1;
  guint selectable : 1;
};

/* Cell Structures */
struct _GtkCellText
{
  GtkCellType type;
  
  gint16 vertical;
  gint16 horizontal;
  
  GtkStyle *style;

  gchar *text;
};

struct _GtkCellPixmap
{
  GtkCellType type;
  
  gint16 vertical;
  gint16 horizontal;
  
  GtkStyle *style;

  GdkPixmap *pixmap;
  GdkBitmap *mask;
};

struct _GtkCellPixText
{
  GtkCellType type;
  
  gint16 vertical;
  gint16 horizontal;
  
  GtkStyle *style;

  gchar *text;
  guint8 spacing;
  GdkPixmap *pixmap;
  GdkBitmap *mask;
};

struct _GtkCellWidget
{
  GtkCellType type;
  
  gint16 vertical;
  gint16 horizontal;
  
  GtkStyle *style;

  GtkWidget *widget;
};

struct _GtkCell
{
  GtkCellType type;
  
  gint16 vertical;
  gint16 horizontal;
  
  GtkStyle *style;

  union {
    gchar *text;
    
    struct {
      GdkPixmap *pixmap;
      GdkBitmap *mask;
    } pm;
    
    struct {
      gchar *text;
      guint8 spacing;
      GdkPixmap *pixmap;
      GdkBitmap *mask;
    } pt;
    
    GtkWidget *widget;
  } u;
};

GType gtk_clist_get_type (void) G_GNUC_CONST;

/* create a new GtkCList */
GtkWidget* gtk_clist_new             (gint   columns);
GtkWidget* gtk_clist_new_with_titles (gint   columns,
				      gchar *titles[]);

/* set adjustments of clist */
void gtk_clist_set_hadjustment (GtkCList      *clist,
				GtkAdjustment *adjustment);
void gtk_clist_set_vadjustment (GtkCList      *clist,
				GtkAdjustment *adjustment);

/* get adjustments of clist */
GtkAdjustment* gtk_clist_get_hadjustment (GtkCList *clist);
GtkAdjustment* gtk_clist_get_vadjustment (GtkCList *clist);

/* set the border style of the clist */
void gtk_clist_set_shadow_type (GtkCList      *clist,
				GtkShadowType  type);

/* set the clist's selection mode */
void gtk_clist_set_selection_mode (GtkCList         *clist,
				   GtkSelectionMode  mode);

/* enable clists reorder ability */
void gtk_clist_set_reorderable (GtkCList *clist,
				gboolean  reorderable);
void gtk_clist_set_use_drag_icons (GtkCList *clist,
				   gboolean  use_icons);
void gtk_clist_set_button_actions (GtkCList *clist,
				   guint     button,
				   guint8    button_actions);

/* freeze all visual updates of the list, and then thaw the list after
 * you have made a number of changes and the updates wil occure in a
 * more efficent mannor than if you made them on a unfrozen list
 */
void gtk_clist_freeze (GtkCList *clist);
void gtk_clist_thaw   (GtkCList *clist);

/* show and hide the column title buttons */
void gtk_clist_column_titles_show (GtkCList *clist);
void gtk_clist_column_titles_hide (GtkCList *clist);

/* set the column title to be a active title (responds to button presses, 
 * prelights, and grabs keyboard focus), or passive where it acts as just
 * a title
 */
void gtk_clist_column_title_active   (GtkCList *clist,
				      gint      column);
void gtk_clist_column_title_passive  (GtkCList *clist,
				      gint      column);
void gtk_clist_column_titles_active  (GtkCList *clist);
void gtk_clist_column_titles_passive (GtkCList *clist);

/* set the title in the column title button */
void gtk_clist_set_column_title (GtkCList    *clist,
				 gint         column,
				 const gchar *title);

/* returns the title of column. Returns NULL if title is not set */
gchar * gtk_clist_get_column_title (GtkCList *clist,
				    gint      column);

/* set a widget instead of a title for the column title button */
void gtk_clist_set_column_widget (GtkCList  *clist,
				  gint       column,
				  GtkWidget *widget);

/* returns the column widget */
GtkWidget * gtk_clist_get_column_widget (GtkCList *clist,
					 gint      column);

/* set the justification on a column */
void gtk_clist_set_column_justification (GtkCList         *clist,
					 gint              column,
					 GtkJustification  justification);

/* set visibility of a column */
void gtk_clist_set_column_visibility (GtkCList *clist,
				      gint      column,
				      gboolean  visible);

/* enable/disable column resize operations by mouse */
void gtk_clist_set_column_resizeable (GtkCList *clist,
				      gint      column,
				      gboolean  resizeable);

/* resize column automatically to its optimal width */
void gtk_clist_set_column_auto_resize (GtkCList *clist,
				       gint      column,
				       gboolean  auto_resize);

gint gtk_clist_columns_autosize (GtkCList *clist);

/* return the optimal column width, i.e. maximum of all cell widths */
gint gtk_clist_optimal_column_width (GtkCList *clist,
				     gint      column);

/* set the pixel width of a column; this is a necessary step in
 * creating a CList because otherwise the column width is chozen from
 * the width of the column title, which will never be right
 */
void gtk_clist_set_column_width (GtkCList *clist,
				 gint      column,
				 gint      width);

/* set column minimum/maximum width. min/max_width < 0 => no restriction */
void gtk_clist_set_column_min_width (GtkCList *clist,
				     gint      column,
				     gint      min_width);
void gtk_clist_set_column_max_width (GtkCList *clist,
				     gint      column,
				     gint      max_width);

/* change the height of the rows, the default (height=0) is
 * the hight of the current font.
 */
void gtk_clist_set_row_height (GtkCList *clist,
			       guint     height);

/* scroll the viewing area of the list to the given column and row;
 * row_align and col_align are between 0-1 representing the location the
 * row should appear on the screnn, 0.0 being top or left, 1.0 being
 * bottom or right; if row or column is -1 then then there is no change
 */
void gtk_clist_moveto (GtkCList *clist,
		       gint      row,
		       gint      column,
		       gfloat    row_align,
		       gfloat    col_align);

/* returns whether the row is visible */
GtkVisibility gtk_clist_row_is_visible (GtkCList *clist,
					gint      row);

/* returns the cell type */
GtkCellType gtk_clist_get_cell_type (GtkCList *clist,
				     gint      row,
				     gint      column);

/* sets a given cell's text, replacing its current contents */
void gtk_clist_set_text (GtkCList    *clist,
			 gint         row,
			 gint         column,
			 const gchar *text);

/* for the "get" functions, any of the return pointer can be
 * NULL if you are not interested
 */
gint gtk_clist_get_text (GtkCList  *clist,
			 gint       row,
			 gint       column,
			 gchar    **text);

/* sets a given cell's pixmap, replacing its current contents */
void gtk_clist_set_pixmap (GtkCList  *clist,
			   gint       row,
			   gint       column,
			   GdkPixmap *pixmap,
			   GdkBitmap *mask);

gint gtk_clist_get_pixmap (GtkCList   *clist,
			   gint        row,
			   gint        column,
			   GdkPixmap **pixmap,
			   GdkBitmap **mask);

/* sets a given cell's pixmap and text, replacing its current contents */
void gtk_clist_set_pixtext (GtkCList    *clist,
			    gint         row,
			    gint         column,
			    const gchar *text,
			    guint8       spacing,
			    GdkPixmap   *pixmap,
			    GdkBitmap   *mask);

gint gtk_clist_get_pixtext (GtkCList   *clist,
			    gint        row,
			    gint        column,
			    gchar     **text,
			    guint8     *spacing,
			    GdkPixmap **pixmap,
			    GdkBitmap **mask);

/* sets the foreground color of a row, the color must already
 * be allocated
 */
void gtk_clist_set_foreground (GtkCList       *clist,
			       gint            row,
			       const GdkColor *color);

/* sets the background color of a row, the color must already
 * be allocated
 */
void gtk_clist_set_background (GtkCList       *clist,
			       gint            row,
			       const GdkColor *color);

/* set / get cell styles */
void gtk_clist_set_cell_style (GtkCList *clist,
			       gint      row,
			       gint      column,
			       GtkStyle *style);

GtkStyle *gtk_clist_get_cell_style (GtkCList *clist,
				    gint      row,
				    gint      column);

void gtk_clist_set_row_style (GtkCList *clist,
			      gint      row,
			      GtkStyle *style);

GtkStyle *gtk_clist_get_row_style (GtkCList *clist,
				   gint      row);

/* this sets a horizontal and vertical shift for drawing
 * the contents of a cell; it can be positive or negitive;
 * this is particulary useful for indenting items in a column
 */
void gtk_clist_set_shift (GtkCList *clist,
			  gint      row,
			  gint      column,
			  gint      vertical,
			  gint      horizontal);

/* set/get selectable flag of a single row */
void gtk_clist_set_selectable (GtkCList *clist,
			       gint      row,
			       gboolean  selectable);
gboolean gtk_clist_get_selectable (GtkCList *clist,
				   gint      row);

/* prepend/append returns the index of the row you just added,
 * making it easier to append and modify a row
 */
gint gtk_clist_prepend (GtkCList    *clist,
		        gchar       *text[]);
gint gtk_clist_append  (GtkCList    *clist,
			gchar       *text[]);

/* inserts a row at index row and returns the row where it was
 * actually inserted (may be different from "row" in auto_sort mode)
 */
gint gtk_clist_insert (GtkCList    *clist,
		       gint         row,
		       gchar       *text[]);

/* removes row at index row */
void gtk_clist_remove (GtkCList *clist,
		       gint      row);

/* sets a arbitrary data pointer for a given row */
void gtk_clist_set_row_data (GtkCList *clist,
			     gint      row,
			     gpointer  data);

/* sets a data pointer for a given row with destroy notification */
void gtk_clist_set_row_data_full (GtkCList       *clist,
			          gint            row,
			          gpointer        data,
				  GDestroyNotify  destroy);

/* returns the data set for a row */
gpointer gtk_clist_get_row_data (GtkCList *clist,
				 gint      row);

/* givin a data pointer, find the first (and hopefully only!)
 * row that points to that data, or -1 if none do
 */
gint gtk_clist_find_row_from_data (GtkCList *clist,
				   gpointer  data);

/* force selection of a row */
void gtk_clist_select_row (GtkCList *clist,
			   gint      row,
			   gint      column);

/* force unselection of a row */
void gtk_clist_unselect_row (GtkCList *clist,
			     gint      row,
			     gint      column);

/* undo the last select/unselect operation */
void gtk_clist_undo_selection (GtkCList *clist);

/* clear the entire list -- this is much faster than removing
 * each item with gtk_clist_remove
 */
void gtk_clist_clear (GtkCList *clist);

/* return the row column corresponding to the x and y coordinates,
 * the returned values are only valid if the x and y coordinates
 * are respectively to a window == clist->clist_window
 */
gint gtk_clist_get_selection_info (GtkCList *clist,
			     	   gint      x,
			     	   gint      y,
			     	   gint     *row,
			     	   gint     *column);

/* in multiple or extended mode, select all rows */
void gtk_clist_select_all (GtkCList *clist);

/* in all modes except browse mode, deselect all rows */
void gtk_clist_unselect_all (GtkCList *clist);

/* swap the position of two rows */
void gtk_clist_swap_rows (GtkCList *clist,
			  gint      row1,
			  gint      row2);

/* move row from source_row position to dest_row position */
void gtk_clist_row_move (GtkCList *clist,
			 gint      source_row,
			 gint      dest_row);

/* sets a compare function different to the default */
void gtk_clist_set_compare_func (GtkCList            *clist,
				 GtkCListCompareFunc  cmp_func);

/* the column to sort by */
void gtk_clist_set_sort_column (GtkCList *clist,
				gint      column);

/* how to sort : ascending or descending */
void gtk_clist_set_sort_type (GtkCList    *clist,
			      GtkSortType  sort_type);

/* sort the list with the current compare function */
void gtk_clist_sort (GtkCList *clist);

/* Automatically sort upon insertion */
void gtk_clist_set_auto_sort (GtkCList *clist,
			      gboolean  auto_sort);

/* Private function for clist, ctree */

PangoLayout *_gtk_clist_create_cell_layout (GtkCList       *clist,
					    GtkCListRow    *clist_row,
					    gint            column);


G_END_DECLS


#endif				/* __GTK_CLIST_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
