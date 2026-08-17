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

#ifndef __GTK_PLUG_H__
#define __GTK_PLUG_H__

#if !defined (__GTKX_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtkx.h> can be included directly."
#endif

#include <gdk/gdk.h>

#ifdef GDK_WINDOWING_X11

#include <gdk/gdkx.h>

#include <gtk/gtksocket.h>
#include <gtk/gtkwindow.h>

G_BEGIN_DECLS

#define GTK_TYPE_PLUG            (gtk_plug_get_type ())
#define GTK_PLUG(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_PLUG, GtkPlug))
#define GTK_PLUG_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_PLUG, GtkPlugClass))
#define GTK_IS_PLUG(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_PLUG))
#define GTK_IS_PLUG_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_PLUG))
#define GTK_PLUG_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_PLUG, GtkPlugClass))


typedef struct _GtkPlug        GtkPlug;
typedef struct _GtkPlugPrivate GtkPlugPrivate;
typedef struct _GtkPlugClass   GtkPlugClass;


struct _GtkPlug
{
  GtkWindow window;

  GtkPlugPrivate *priv;
};

struct _GtkPlugClass
{
  GtkWindowClass parent_class;

  void (*embedded) (GtkPlug *plug);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType      gtk_plug_get_type              (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
void       gtk_plug_construct             (GtkPlug    *plug,
                                           Window      socket_id);
GDK_AVAILABLE_IN_ALL
GtkWidget *gtk_plug_new                   (Window      socket_id);

GDK_AVAILABLE_IN_ALL
void       gtk_plug_construct_for_display (GtkPlug    *plug,
                                           GdkDisplay *display,
                                           Window      socket_id);
GDK_AVAILABLE_IN_ALL
GtkWidget *gtk_plug_new_for_display       (GdkDisplay *display,
                                           Window      socket_id);
GDK_AVAILABLE_IN_ALL
Window     gtk_plug_get_id                (GtkPlug    *plug);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_plug_get_embedded          (GtkPlug    *plug);
GDK_AVAILABLE_IN_ALL
GdkWindow *gtk_plug_get_socket_window     (GtkPlug    *plug);

G_END_DECLS

#endif /* GDK_WINDOWING_X11 */

#endif /* __GTK_PLUG_H__ */
