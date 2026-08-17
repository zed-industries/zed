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

#ifndef __GTK_BUTTON_BOX_H__
#define __GTK_BUTTON_BOX_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbox.h>


G_BEGIN_DECLS  

#define GTK_TYPE_BUTTON_BOX             (gtk_button_box_get_type ())
#define GTK_BUTTON_BOX(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_BUTTON_BOX, GtkButtonBox))
#define GTK_BUTTON_BOX_CLASS(klass)     (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_BUTTON_BOX, GtkButtonBoxClass))
#define GTK_IS_BUTTON_BOX(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_BUTTON_BOX))
#define GTK_IS_BUTTON_BOX_CLASS(klass)  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_BUTTON_BOX))
#define GTK_BUTTON_BOX_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_BUTTON_BOX, GtkButtonBoxClass))
  

#define GTK_BUTTONBOX_DEFAULT -1
 
typedef struct _GtkButtonBox       GtkButtonBox;
typedef struct _GtkButtonBoxClass  GtkButtonBoxClass;

struct _GtkButtonBox
{
  GtkBox box;
  gint GSEAL (child_min_width);
  gint GSEAL (child_min_height);
  gint GSEAL (child_ipad_x);
  gint GSEAL (child_ipad_y);
  GtkButtonBoxStyle GSEAL (layout_style);
};

struct _GtkButtonBoxClass
{
  GtkBoxClass parent_class;
};


GType gtk_button_box_get_type (void) G_GNUC_CONST;

GtkButtonBoxStyle gtk_button_box_get_layout          (GtkButtonBox      *widget);
void              gtk_button_box_set_layout          (GtkButtonBox      *widget,
						      GtkButtonBoxStyle  layout_style);
gboolean          gtk_button_box_get_child_secondary (GtkButtonBox      *widget,
						      GtkWidget         *child);
void              gtk_button_box_set_child_secondary (GtkButtonBox      *widget,
						      GtkWidget         *child,
						      gboolean           is_secondary);

#ifndef GTK_DISABLE_DEPRECATED
#define gtk_button_box_set_spacing(b,s) gtk_box_set_spacing (GTK_BOX (b), s)
#define gtk_button_box_get_spacing(b)   gtk_box_get_spacing (GTK_BOX (b))

void gtk_button_box_set_child_size     (GtkButtonBox *widget,
					gint          min_width,
					gint          min_height);
void gtk_button_box_set_child_ipadding (GtkButtonBox *widget,
					gint          ipad_x,
					gint          ipad_y);
void gtk_button_box_get_child_size     (GtkButtonBox *widget,
					gint         *min_width,
					gint         *min_height);
void gtk_button_box_get_child_ipadding (GtkButtonBox *widget,
					gint         *ipad_x,
					gint         *ipad_y);
#endif

/* Internal method - do not use. */
void _gtk_button_box_child_requisition (GtkWidget *widget,
					int       *nvis_children,
					int       *nvis_secondaries,
					int       *width,
					int       *height);
G_END_DECLS

#endif /* __GTK_BUTTON_BOX_H__ */
