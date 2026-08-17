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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
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

#if !defined (GTK_DISABLE_DEPRECATED) || defined (__GTK_LIST_C__)

#ifndef __GTK_LIST_H__
#define __GTK_LIST_H__

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define GTK_TYPE_LIST                  (gtk_list_get_type ())
#define GTK_LIST(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_LIST, GtkList))
#define GTK_LIST_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_LIST, GtkListClass))
#define GTK_IS_LIST(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_LIST))
#define GTK_IS_LIST_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_LIST))
#define GTK_LIST_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_LIST, GtkListClass))


typedef struct _GtkList	      GtkList;
typedef struct _GtkListClass  GtkListClass;

struct _GtkList
{
  GtkContainer container;

  GList *children;
  GList *selection;

  GList *undo_selection;
  GList *undo_unselection;

  GtkWidget *last_focus_child;
  GtkWidget *undo_focus_child;

  guint htimer;
  guint vtimer;

  gint anchor;
  gint drag_pos;
  GtkStateType anchor_state;

  guint selection_mode : 2;
  guint drag_selection:1;
  guint add_mode:1;
};

struct _GtkListClass
{
  GtkContainerClass parent_class;

  void (* selection_changed) (GtkList	*list);
  void (* select_child)	     (GtkList	*list,
			      GtkWidget *child);
  void (* unselect_child)    (GtkList	*list,
			      GtkWidget *child);
};


GType      gtk_list_get_type		  (void) G_GNUC_CONST;
GtkWidget* gtk_list_new			  (void);
void	   gtk_list_insert_items	  (GtkList	    *list,
					   GList	    *items,
					   gint		     position);
void	   gtk_list_append_items	  (GtkList	    *list,
					   GList	    *items);
void	   gtk_list_prepend_items	  (GtkList	    *list,
					   GList	    *items);
void	   gtk_list_remove_items	  (GtkList	    *list,
					   GList	    *items);
void	   gtk_list_remove_items_no_unref (GtkList	    *list,
					   GList	    *items);
void	   gtk_list_clear_items		  (GtkList	    *list,
					   gint		     start,
					   gint		     end);
void	   gtk_list_select_item		  (GtkList	    *list,
					   gint		     item);
void	   gtk_list_unselect_item	  (GtkList	    *list,
					   gint		     item);
void	   gtk_list_select_child	  (GtkList	    *list,
					   GtkWidget	    *child);
void	   gtk_list_unselect_child	  (GtkList	    *list,
					   GtkWidget	    *child);
gint	   gtk_list_child_position	  (GtkList	    *list,
					   GtkWidget	    *child);
void	   gtk_list_set_selection_mode	  (GtkList	    *list,
					   GtkSelectionMode  mode);

void       gtk_list_extend_selection      (GtkList          *list,
					   GtkScrollType     scroll_type,
					   gfloat            position,
					   gboolean          auto_start_selection);
void       gtk_list_start_selection       (GtkList          *list);
void       gtk_list_end_selection         (GtkList          *list);
void       gtk_list_select_all            (GtkList          *list);
void       gtk_list_unselect_all          (GtkList          *list);
void       gtk_list_scroll_horizontal     (GtkList          *list,
					   GtkScrollType     scroll_type,
					   gfloat            position);
void       gtk_list_scroll_vertical       (GtkList          *list,
					   GtkScrollType     scroll_type,
					   gfloat            position);
void       gtk_list_toggle_add_mode       (GtkList          *list);
void       gtk_list_toggle_focus_row      (GtkList          *list);
void       gtk_list_toggle_row            (GtkList          *list,
					   GtkWidget        *item);
void       gtk_list_undo_selection        (GtkList          *list);
void       gtk_list_end_drag_selection    (GtkList          *list);

G_END_DECLS

#endif /* __GTK_LIST_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
