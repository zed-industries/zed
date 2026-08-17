/* GTK - The GIMP Toolkit
 * Copyright (C) 2010 Carlos Garnacho <carlosg@gnome.org>
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

#ifndef __GTK_RENDER_H__
#define __GTK_RENDER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <cairo.h>
#include <pango/pango.h>
#include <gdk/gdk.h>

#include <gtk/gtkenums.h>
#include <gtk/gtktypes.h>

G_BEGIN_DECLS

GDK_AVAILABLE_IN_ALL
void        gtk_render_check       (GtkStyleContext     *context,
                                    cairo_t             *cr,
                                    double               x,
                                    double               y,
                                    double               width,
                                    double               height);
GDK_AVAILABLE_IN_ALL
void        gtk_render_option      (GtkStyleContext     *context,
                                    cairo_t             *cr,
                                    double               x,
                                    double               y,
                                    double               width,
                                    double               height);
GDK_AVAILABLE_IN_ALL
void        gtk_render_arrow       (GtkStyleContext     *context,
                                    cairo_t             *cr,
                                    double               angle,
                                    double               x,
                                    double               y,
                                    double               size);
GDK_AVAILABLE_IN_ALL
void        gtk_render_background  (GtkStyleContext     *context,
                                    cairo_t             *cr,
                                    double               x,
                                    double               y,
                                    double               width,
                                    double               height);

GDK_AVAILABLE_IN_ALL
void        gtk_render_frame       (GtkStyleContext     *context,
                                    cairo_t             *cr,
                                    double               x,
                                    double               y,
                                    double               width,
                                    double               height);
GDK_AVAILABLE_IN_ALL
void        gtk_render_expander    (GtkStyleContext     *context,
                                    cairo_t             *cr,
                                    double               x,
                                    double               y,
                                    double               width,
                                    double               height);
GDK_AVAILABLE_IN_ALL
void        gtk_render_focus       (GtkStyleContext     *context,
                                    cairo_t             *cr,
                                    double               x,
                                    double               y,
                                    double               width,
                                    double               height);
GDK_AVAILABLE_IN_ALL
void        gtk_render_layout      (GtkStyleContext     *context,
                                    cairo_t             *cr,
                                    double               x,
                                    double               y,
                                    PangoLayout         *layout);
GDK_AVAILABLE_IN_ALL
void        gtk_render_line        (GtkStyleContext     *context,
                                    cairo_t             *cr,
                                    double               x0,
                                    double               y0,
                                    double               x1,
                                    double               y1);
GDK_AVAILABLE_IN_ALL
void        gtk_render_handle      (GtkStyleContext     *context,
                                    cairo_t             *cr,
                                    double               x,
                                    double               y,
                                    double               width,
                                    double               height);
GDK_AVAILABLE_IN_ALL
void        gtk_render_activity    (GtkStyleContext     *context,
                                    cairo_t             *cr,
                                    double               x,
                                    double               y,
                                    double               width,
                                    double               height);
GDK_AVAILABLE_IN_ALL
void        gtk_render_icon        (GtkStyleContext     *context,
                                    cairo_t             *cr,
                                    GdkTexture          *texture,
                                    double               x,
                                    double               y);
G_END_DECLS

#endif /* __GTK_RENDER_H__ */
