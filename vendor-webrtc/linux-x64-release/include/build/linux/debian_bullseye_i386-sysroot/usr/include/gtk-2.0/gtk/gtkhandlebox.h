/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 * Copyright (C) 1998 Elliot Lee
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

/* The GtkHandleBox is to allow widgets to be dragged in and out of
 * their parents.
 */

#ifndef __GTK_HANDLE_BOX_H__
#define __GTK_HANDLE_BOX_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbin.h>


G_BEGIN_DECLS

#define GTK_TYPE_HANDLE_BOX            (gtk_handle_box_get_type ())
#define GTK_HANDLE_BOX(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_HANDLE_BOX, GtkHandleBox))
#define GTK_HANDLE_BOX_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_HANDLE_BOX, GtkHandleBoxClass))
#define GTK_IS_HANDLE_BOX(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_HANDLE_BOX))
#define GTK_IS_HANDLE_BOX_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_HANDLE_BOX))
#define GTK_HANDLE_BOX_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_HANDLE_BOX, GtkHandleBoxClass))


typedef struct _GtkHandleBox       GtkHandleBox;
typedef struct _GtkHandleBoxClass  GtkHandleBoxClass;

struct _GtkHandleBox
{
  GtkBin bin;

  GdkWindow      *GSEAL (bin_window);	/* parent window for children */
  GdkWindow      *GSEAL (float_window);
  GtkShadowType   GSEAL (shadow_type);
  guint           GSEAL (handle_position) : 2;
  guint           GSEAL (float_window_mapped) : 1;
  guint           GSEAL (child_detached) : 1;
  guint           GSEAL (in_drag) : 1;
  guint           GSEAL (shrink_on_detach) : 1;

  signed int      GSEAL (snap_edge : 3); /* -1 == unset */

  /* Variables used during a drag
   */
  gint            GSEAL (deskoff_x); /* Offset between root relative coords */
  gint            GSEAL (deskoff_y); /* and deskrelative coords             */

  GtkAllocation   GSEAL (attach_allocation);
  GtkAllocation   GSEAL (float_allocation);
};

struct _GtkHandleBoxClass
{
  GtkBinClass parent_class;

  void	(*child_attached)	(GtkHandleBox	*handle_box,
				 GtkWidget	*child);
  void	(*child_detached)	(GtkHandleBox	*handle_box,
				 GtkWidget	*child);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GType         gtk_handle_box_get_type             (void) G_GNUC_CONST;
GtkWidget*    gtk_handle_box_new                  (void);
void          gtk_handle_box_set_shadow_type      (GtkHandleBox    *handle_box,
                                                   GtkShadowType    type);
GtkShadowType gtk_handle_box_get_shadow_type      (GtkHandleBox    *handle_box);
void          gtk_handle_box_set_handle_position  (GtkHandleBox    *handle_box,
					           GtkPositionType  position);
GtkPositionType gtk_handle_box_get_handle_position(GtkHandleBox    *handle_box);
void          gtk_handle_box_set_snap_edge        (GtkHandleBox    *handle_box,
						   GtkPositionType  edge);
GtkPositionType gtk_handle_box_get_snap_edge      (GtkHandleBox    *handle_box);
gboolean      gtk_handle_box_get_child_detached   (GtkHandleBox    *handle_box);

G_END_DECLS

#endif /* __GTK_HANDLE_BOX_H__ */
