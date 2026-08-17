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
 * License along with this library; if not, write to the Free
 * Software Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_SOCKET_H__
#define __GTK_SOCKET_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>

G_BEGIN_DECLS

#define GTK_TYPE_SOCKET            (gtk_socket_get_type ())
#define GTK_SOCKET(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SOCKET, GtkSocket))
#define GTK_SOCKET_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SOCKET, GtkSocketClass))
#define GTK_IS_SOCKET(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SOCKET))
#define GTK_IS_SOCKET_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SOCKET))
#define GTK_SOCKET_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SOCKET, GtkSocketClass))


typedef struct _GtkSocket        GtkSocket;
typedef struct _GtkSocketClass   GtkSocketClass;

struct _GtkSocket
{
  GtkContainer container;

  guint16 GSEAL (request_width);
  guint16 GSEAL (request_height);
  guint16 GSEAL (current_width);
  guint16 GSEAL (current_height);

  GdkWindow *GSEAL (plug_window);
  GtkWidget *GSEAL (plug_widget);

  gshort GSEAL (xembed_version); /* -1 == not xembed */
  guint GSEAL (same_app) : 1;
  guint GSEAL (focus_in) : 1;
  guint GSEAL (have_size) : 1;
  guint GSEAL (need_map) : 1;
  guint GSEAL (is_mapped) : 1;
  guint GSEAL (active) : 1;

  GtkAccelGroup *GSEAL (accel_group);
  GtkWidget *GSEAL (toplevel);
};

struct _GtkSocketClass
{
  GtkContainerClass parent_class;

  void     (*plug_added)   (GtkSocket *socket_);
  gboolean (*plug_removed) (GtkSocket *socket_);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GType          gtk_socket_get_type (void) G_GNUC_CONST;
GtkWidget*     gtk_socket_new      (void);

void            gtk_socket_add_id (GtkSocket       *socket_,
				   GdkNativeWindow  window_id);
GdkNativeWindow gtk_socket_get_id (GtkSocket       *socket_);
GdkWindow*      gtk_socket_get_plug_window (GtkSocket       *socket_);

#ifndef GTK_DISABLE_DEPRECATED
void           gtk_socket_steal    (GtkSocket      *socket_,
				    GdkNativeWindow wid);
#endif /* GTK_DISABLE_DEPRECATED */

G_END_DECLS

#endif /* __GTK_SOCKET_H__ */
