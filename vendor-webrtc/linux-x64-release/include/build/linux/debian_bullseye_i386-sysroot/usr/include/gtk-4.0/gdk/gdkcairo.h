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

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdkrgba.h>
#include <gdk/gdkpixbuf.h>
#include <pango/pangocairo.h>

G_BEGIN_DECLS

GDK_AVAILABLE_IN_ALL
void       gdk_cairo_set_source_rgba    (cairo_t              *cr,
                                         const GdkRGBA        *rgba);
GDK_AVAILABLE_IN_ALL
void       gdk_cairo_set_source_pixbuf  (cairo_t              *cr,
                                         const GdkPixbuf      *pixbuf,
                                         double                pixbuf_x,
                                         double                pixbuf_y);

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

GDK_DEPRECATED_IN_4_6_FOR(gdk_gl_texture_new)
void       gdk_cairo_draw_from_gl (cairo_t              *cr,
                                   GdkSurface            *surface,
                                   int                   source,
                                   int                   source_type,
                                   int                   buffer_scale,
                                   int                   x,
                                   int                   y,
                                   int                   width,
                                   int                   height);

G_END_DECLS

#endif /* __GDK_CAIRO_H__ */
