/* GDK - The GIMP Drawing Kit
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

#ifndef __GDK_PIXBUF_H__
#define __GDK_PIXBUF_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <cairo.h>
#include <gdk/gdktypes.h>
#include <gdk/gdkrgb.h>
#include <gdk-pixbuf/gdk-pixbuf.h>

G_BEGIN_DECLS

/* Rendering to a drawable */

void gdk_pixbuf_render_threshold_alpha   (GdkPixbuf           *pixbuf,
					  GdkBitmap           *bitmap,
					  int                  src_x,
					  int                  src_y,
					  int                  dest_x,
					  int                  dest_y,
					  int                  width,
					  int                  height,
					  int                  alpha_threshold);
#ifndef GDK_DISABLE_DEPRECATED
void gdk_pixbuf_render_to_drawable       (GdkPixbuf           *pixbuf,
					  GdkDrawable         *drawable,
					  GdkGC               *gc,
					  int                  src_x,
					  int                  src_y,
					  int                  dest_x,
					  int                  dest_y,
					  int                  width,
					  int                  height,
					  GdkRgbDither         dither,
					  int                  x_dither,
					  int                  y_dither);
void gdk_pixbuf_render_to_drawable_alpha (GdkPixbuf           *pixbuf,
					  GdkDrawable         *drawable,
					  int                  src_x,
					  int                  src_y,
					  int                  dest_x,
					  int                  dest_y,
					  int                  width,
					  int                  height,
					  GdkPixbufAlphaMode   alpha_mode,
					  int                  alpha_threshold,
					  GdkRgbDither         dither,
					  int                  x_dither,
					  int                  y_dither);
#endif /* GDK_DISABLE_DEPRECATED */
void gdk_pixbuf_render_pixmap_and_mask_for_colormap (GdkPixbuf    *pixbuf,
						     GdkColormap  *colormap,
						     GdkPixmap   **pixmap_return,
						     GdkBitmap   **mask_return,
						     int           alpha_threshold);
#ifndef GDK_MULTIHEAD_SAFE
void gdk_pixbuf_render_pixmap_and_mask              (GdkPixbuf    *pixbuf,
						     GdkPixmap   **pixmap_return,
						     GdkBitmap   **mask_return,
						     int           alpha_threshold);
#endif


/* Fetching a region from a drawable */
GdkPixbuf *gdk_pixbuf_get_from_drawable (GdkPixbuf   *dest,
					 GdkDrawable *src,
					 GdkColormap *cmap,
					 int          src_x,
					 int          src_y,
					 int          dest_x,
					 int          dest_y,
					 int          width,
					 int          height);

GdkPixbuf *gdk_pixbuf_get_from_image    (GdkPixbuf   *dest,
                                         GdkImage    *src,
                                         GdkColormap *cmap,
                                         int          src_x,
                                         int          src_y,
                                         int          dest_x,
                                         int          dest_y,
                                         int          width,
                                         int          height);

G_END_DECLS

#endif /* __GDK_PIXBUF_H__ */
