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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_EDITABLE_H__
#define __GTK_EDITABLE_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>


G_BEGIN_DECLS

#define GTK_TYPE_EDITABLE             (gtk_editable_get_type ())
#define GTK_EDITABLE(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_EDITABLE, GtkEditable))
#define GTK_IS_EDITABLE(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_EDITABLE))
#define GTK_EDITABLE_GET_IFACE(inst)  (G_TYPE_INSTANCE_GET_INTERFACE ((inst), GTK_TYPE_EDITABLE, GtkEditableInterface))

typedef struct _GtkEditable          GtkEditable;         /* Dummy typedef */
typedef struct _GtkEditableInterface GtkEditableInterface;

struct _GtkEditableInterface
{
  GTypeInterface		   base_iface;

  /* signals */
  void (* insert_text)              (GtkEditable    *editable,
				     const gchar    *new_text,
				     gint            new_text_length,
				     gint           *position);
  void (* delete_text)              (GtkEditable    *editable,
				     gint            start_pos,
				     gint            end_pos);
  void (* changed)                  (GtkEditable    *editable);

  /* vtable */
  void (* do_insert_text)           (GtkEditable    *editable,
				     const gchar    *new_text,
				     gint            new_text_length,
				     gint           *position);
  void (* do_delete_text)           (GtkEditable    *editable,
				     gint            start_pos,
				     gint            end_pos);

  gchar* (* get_chars)              (GtkEditable    *editable,
				     gint            start_pos,
				     gint            end_pos);
  void (* set_selection_bounds)     (GtkEditable    *editable,
				     gint            start_pos,
				     gint            end_pos);
  gboolean (* get_selection_bounds) (GtkEditable    *editable,
				     gint           *start_pos,
				     gint           *end_pos);
  void (* set_position)             (GtkEditable    *editable,
				     gint            position);
  gint (* get_position)             (GtkEditable    *editable);
};

GDK_AVAILABLE_IN_ALL
GType    gtk_editable_get_type             (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
void     gtk_editable_select_region        (GtkEditable *editable,
					    gint         start_pos,
					    gint         end_pos);
GDK_AVAILABLE_IN_ALL
gboolean gtk_editable_get_selection_bounds (GtkEditable *editable,
					    gint        *start_pos,
					    gint        *end_pos);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_insert_text          (GtkEditable *editable,
					    const gchar *new_text,
					    gint         new_text_length,
					    gint        *position);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_delete_text          (GtkEditable *editable,
					    gint         start_pos,
					    gint         end_pos);
GDK_AVAILABLE_IN_ALL
gchar*   gtk_editable_get_chars            (GtkEditable *editable,
					    gint         start_pos,
					    gint         end_pos);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_cut_clipboard        (GtkEditable *editable);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_copy_clipboard       (GtkEditable *editable);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_paste_clipboard      (GtkEditable *editable);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_delete_selection     (GtkEditable *editable);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_set_position         (GtkEditable *editable,
					    gint         position);
GDK_AVAILABLE_IN_ALL
gint     gtk_editable_get_position         (GtkEditable *editable);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_set_editable         (GtkEditable *editable,
					    gboolean     is_editable);
GDK_AVAILABLE_IN_ALL
gboolean gtk_editable_get_editable         (GtkEditable *editable);

G_END_DECLS

#endif /* __GTK_EDITABLE_H__ */
