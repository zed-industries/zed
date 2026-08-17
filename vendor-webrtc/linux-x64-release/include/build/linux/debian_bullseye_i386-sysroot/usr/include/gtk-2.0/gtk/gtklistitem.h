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

#if !defined (GTK_DISABLE_DEPRECATED) || defined (__GTK_LIST_ITEM_C__)

#ifndef __GTK_LIST_ITEM_H__
#define __GTK_LIST_ITEM_H__

#include <gtk/gtk.h>


G_BEGIN_DECLS


#define GTK_TYPE_LIST_ITEM              (gtk_list_item_get_type ())
#define GTK_LIST_ITEM(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_LIST_ITEM, GtkListItem))
#define GTK_LIST_ITEM_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_LIST_ITEM, GtkListItemClass))
#define GTK_IS_LIST_ITEM(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_LIST_ITEM))
#define GTK_IS_LIST_ITEM_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_LIST_ITEM))
#define GTK_LIST_ITEM_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_LIST_ITEM, GtkListItemClass))


typedef struct _GtkListItem       GtkListItem;
typedef struct _GtkListItemClass  GtkListItemClass;

struct _GtkListItem
{
  GtkItem item;
};

struct _GtkListItemClass
{
  GtkItemClass parent_class;

  void (*toggle_focus_row)  (GtkListItem   *list_item);
  void (*select_all)        (GtkListItem   *list_item);
  void (*unselect_all)      (GtkListItem   *list_item);
  void (*undo_selection)    (GtkListItem   *list_item);
  void (*start_selection)   (GtkListItem   *list_item);
  void (*end_selection)     (GtkListItem   *list_item);
  void (*extend_selection)  (GtkListItem   *list_item,
			     GtkScrollType  scroll_type,
			     gfloat         position,
			     gboolean       auto_start_selection);
  void (*scroll_horizontal) (GtkListItem   *list_item,
			     GtkScrollType  scroll_type,
			     gfloat         position);
  void (*scroll_vertical)   (GtkListItem   *list_item,
			     GtkScrollType  scroll_type,
			     gfloat         position);
  void (*toggle_add_mode)   (GtkListItem   *list_item);
};


GType      gtk_list_item_get_type       (void) G_GNUC_CONST;
GtkWidget* gtk_list_item_new            (void);
GtkWidget* gtk_list_item_new_with_label (const gchar      *label);
void       gtk_list_item_select         (GtkListItem      *list_item);
void       gtk_list_item_deselect       (GtkListItem      *list_item);



G_END_DECLS


#endif /* __GTK_LIST_ITEM_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
