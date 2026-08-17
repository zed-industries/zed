/*
 * Copyright © 2018 Benjamin Otte
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors: Benjamin Otte <otte@gnome.org>
 */

#ifndef __GTK_MEDIA_CONTROLS_H__
#define __GTK_MEDIA_CONTROLS_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkmediastream.h>
#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_MEDIA_CONTROLS         (gtk_media_controls_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkMediaControls, gtk_media_controls, GTK, MEDIA_CONTROLS, GtkWidget)

GDK_AVAILABLE_IN_ALL
GtkWidget      *gtk_media_controls_new                  (GtkMediaStream         *stream);

GDK_AVAILABLE_IN_ALL
GtkMediaStream *gtk_media_controls_get_media_stream     (GtkMediaControls       *controls);
GDK_AVAILABLE_IN_ALL
void            gtk_media_controls_set_media_stream     (GtkMediaControls       *controls,
                                                         GtkMediaStream         *stream);


G_END_DECLS

#endif  /* __GTK_MEDIA_CONTROLS_H__ */
