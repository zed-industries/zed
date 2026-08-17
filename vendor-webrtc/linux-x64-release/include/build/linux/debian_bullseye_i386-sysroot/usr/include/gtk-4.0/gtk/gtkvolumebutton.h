/* GTK - The GIMP Toolkit
 * Copyright (C) 2007 Red Hat, Inc.
 *
 * Authors:
 * - Bastien Nocera <bnocera@redhat.com>
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
 * Modified by the GTK+ Team and others 2007.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_VOLUME_BUTTON_H__
#define __GTK_VOLUME_BUTTON_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkscalebutton.h>

G_BEGIN_DECLS

#define GTK_TYPE_VOLUME_BUTTON                 (gtk_volume_button_get_type ())
#define GTK_VOLUME_BUTTON(obj)                 (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_VOLUME_BUTTON, GtkVolumeButton))
#define GTK_IS_VOLUME_BUTTON(obj)              (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_VOLUME_BUTTON))

typedef struct _GtkVolumeButton       GtkVolumeButton;

struct _GtkVolumeButton
{
  GtkScaleButton  parent;
};

GDK_AVAILABLE_IN_ALL
GType		gtk_volume_button_get_type	(void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget*	gtk_volume_button_new		(void);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkVolumeButton, g_object_unref)

G_END_DECLS

#endif /* __GTK_VOLUME_BUTTON_H__ */
