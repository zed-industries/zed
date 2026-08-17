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

#ifndef GTK_DISABLE_DEPRECATED

#ifndef __GTK_OLD_EDITABLE_H__
#define __GTK_OLD_EDITABLE_H__

#include <gtk/gtk.h>


G_BEGIN_DECLS

#define GTK_TYPE_OLD_EDITABLE            (gtk_old_editable_get_type ())
#define GTK_OLD_EDITABLE(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_OLD_EDITABLE, GtkOldEditable))
#define GTK_OLD_EDITABLE_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_OLD_EDITABLE, GtkOldEditableClass))
#define GTK_IS_OLD_EDITABLE(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_OLD_EDITABLE))
#define GTK_IS_OLD_EDITABLE_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_OLD_EDITABLE))
#define GTK_OLD_EDITABLE_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_OLD_EDITABLE, GtkOldEditableClass))


typedef struct _GtkOldEditable       GtkOldEditable;
typedef struct _GtkOldEditableClass  GtkOldEditableClass;

typedef void (*GtkTextFunction) (GtkOldEditable  *editable, guint32 time_);

struct _GtkOldEditable
{
  GtkWidget widget;

  /*< public >*/
  guint      current_pos;

  guint      selection_start_pos;
  guint      selection_end_pos;
  guint      has_selection : 1;

  /*< private >*/
  guint      editable : 1;
  guint      visible : 1;
  
  gchar *clipboard_text;
};

struct _GtkOldEditableClass
{
  GtkWidgetClass parent_class;
  
  /* Bindings actions */
  void (* activate)        (GtkOldEditable *editable);
  void (* set_editable)    (GtkOldEditable *editable,
			    gboolean	    is_editable);
  void (* move_cursor)     (GtkOldEditable *editable,
			    gint            x,
			    gint            y);
  void (* move_word)       (GtkOldEditable *editable,
			    gint            n);
  void (* move_page)       (GtkOldEditable *editable,
			    gint            x,
			    gint            y);
  void (* move_to_row)     (GtkOldEditable *editable,
			    gint            row);
  void (* move_to_column)  (GtkOldEditable *editable,
			    gint            row);
  void (* kill_char)       (GtkOldEditable *editable,
			    gint            direction);
  void (* kill_word)       (GtkOldEditable *editable,
			    gint            direction);
  void (* kill_line)       (GtkOldEditable *editable,
			    gint            direction);
  void (* cut_clipboard)   (GtkOldEditable *editable);
  void (* copy_clipboard)  (GtkOldEditable *editable);
  void (* paste_clipboard) (GtkOldEditable *editable);

  /* Virtual functions. get_chars is in paricular not a signal because
   * it returns malloced memory. The others are not signals because
   * they would not be particularly useful as such. (All changes to
   * selection and position do not go through these functions)
   */
  void (* update_text)  (GtkOldEditable  *editable,
			 gint             start_pos,
			 gint             end_pos);
  gchar* (* get_chars)  (GtkOldEditable  *editable,
			 gint             start_pos,
			 gint             end_pos);
  void (* set_selection)(GtkOldEditable  *editable,
			 gint             start_pos,
			 gint             end_pos);
  void (* set_position) (GtkOldEditable  *editable,
			 gint             position);
};

GType      gtk_old_editable_get_type        (void) G_GNUC_CONST;
void       gtk_old_editable_claim_selection (GtkOldEditable *old_editable,
					     gboolean        claim,
					     guint32         time_);
void       gtk_old_editable_changed         (GtkOldEditable *old_editable);

G_END_DECLS

#endif /* __GTK_OLD_EDITABLE_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
