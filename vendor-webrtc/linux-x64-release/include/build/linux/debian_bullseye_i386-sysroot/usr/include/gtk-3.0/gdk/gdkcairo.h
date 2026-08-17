/* GDK - The GIMP Drawing Kit
 * Copyright (C) 2005 Red Hat, Inc. 
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GDK_CAIRO_H__
#define __GDK_CAIRO_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/deprecated/gdkcolor.h>
#include <gdk/gdkrgba.h>
#include <gdk/gdkdrawingcontext.h>
#include <gdk/gdkpixbuf.h>
#include <pango/pangocairo.h>

G_BEGIN_DECLS

GDK_DEPRECATED_IN_3_22_FOR(gdk_window_begin_draw_frame() and gdk_drawing_context_get_cairo_context())
cairo_t  * gdk_cairo_create             (GdkWindow          *window);

GDK_AVAILABLE_IN_ALL
gboolean   gdk_cairo_get_clip_rectangle (cairo_t            *cr,
                                         GdkRectangle       *rect);

GDK_AVAILABLE_IN_ALL
void       gdk_cairo_set_source_rgba    (cairo_t              *cr,
                                         const GdkRGBA        *rgba);
GDK_AVAILABLE_IN_ALL
void       gdk_cairo_set_source_pixbuf  (cairo_t              *cr,
                                         const GdkPixbuf      *pixbuf,
                                         gdouble               pixbuf_x,
                                         gdouble               pixbuf_y);
GDK_AVAILABLE_IN_ALL
void       gdk_cairo_set_source_window  (cairo_t              *cr,
                                         GdkWindow            *window,
                                         gdouble               x,
                                         gdouble               y);

GDK_AVAILABLE_IN_ALL
void       gdk_cairo_rectangle          (cairo_t              *cr,
                                         const GdkRectangle   *rectangle);
GDK_AVAILABLE_IN_ALL
void       gdk_cairo_region             (cairo_t              *cr,
                                         const cairo_region_t *region);

GDK_AVAILABLE_IN_ALL
cairo_region_t *
           gdk_cairo_region_create_from_surface
                                        (cairo_surface_t      *surface);

GDK_DEPRECATED_IN_3_4_FOR(gdk_cairo_set_source_rgba)
void       gdk_cairo_set_source_color   (cairo_t              *cr,
                                         const GdkColor       *color);

GDK_AVAILABLE_IN_3_10
cairo_surface_t * gdk_cairo_surface_create_from_pixbuf      (const GdkPixbuf *pixbuf,
                                                             int scale,
                                                             GdkWindow *for_window);
GDK_AVAILABLE_IN_3_16
void       gdk_cairo_draw_from_gl (cairo_t              *cr,
                                   GdkWindow            *window,
                                   int                   source,
                                   int                   source_type,
                                   int                   buffer_scale,
                                   int                   x,
                                   int                   y,
                                   int                   width,
                                   int                   height);

GDK_AVAILABLE_IN_3_22
GdkDrawingContext *     gdk_cairo_get_drawing_context   (cairo_t *cr);

G_END_DECLS

#endif /* __GDK_CAIRO_H__ */
