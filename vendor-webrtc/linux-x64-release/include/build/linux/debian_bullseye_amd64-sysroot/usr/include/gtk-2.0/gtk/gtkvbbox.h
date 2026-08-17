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

#ifndef __GTK_VBBOX_H__
#define __GTK_VBBOX_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbbox.h>


G_BEGIN_DECLS


#define GTK_TYPE_VBUTTON_BOX            (gtk_vbutton_box_get_type ())
#define GTK_VBUTTON_BOX(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_VBUTTON_BOX, GtkVButtonBox))
#define GTK_VBUTTON_BOX_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_VBUTTON_BOX, GtkVButtonBoxClass))
#define GTK_IS_VBUTTON_BOX(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_VBUTTON_BOX))
#define GTK_IS_VBUTTON_BOX_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_VBUTTON_BOX))
#define GTK_VBUTTON_BOX_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_VBUTTON_BOX, GtkVButtonBoxClass))


typedef struct _GtkVButtonBox       GtkVButtonBox;
typedef struct _GtkVButtonBoxClass  GtkVButtonBoxClass;

struct _GtkVButtonBox
{
  GtkButtonBox button_box;
};

struct _GtkVButtonBoxClass
{
  GtkButtonBoxClass parent_class;
};


GType      gtk_vbutton_box_get_type (void) G_GNUC_CONST;
GtkWidget *gtk_vbutton_box_new      (void);

/* buttons can be added by gtk_container_add() */

#ifndef GTK_DISABLE_DEPRECATED
gint gtk_vbutton_box_get_spacing_default (void);
void gtk_vbutton_box_set_spacing_default (gint spacing);

GtkButtonBoxStyle gtk_vbutton_box_get_layout_default (void);
void gtk_vbutton_box_set_layout_default (GtkButtonBoxStyle layout);
#endif

/* private API */
GtkButtonBoxStyle _gtk_vbutton_box_get_layout_default (void);

G_END_DECLS


#endif /* __GTK_VBBOX_H__ */
