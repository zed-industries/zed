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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GDK_COLOR_H__
#define __GDK_COLOR_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <cairo.h>
#include <gdk/gdktypes.h>
#include <gdk/gdkversionmacros.h>

G_BEGIN_DECLS


/**
 * GdkColor:
 * @pixel: For allocated colors, the pixel value used to
 *     draw this color on the screen. Not used anymore.
 * @red: The red component of the color. This is
 *     a value between 0 and 65535, with 65535 indicating
 *     full intensity
 * @green: The green component of the color
 * @blue: The blue component of the color
 *
 * A #GdkColor is used to describe a color,
 * similar to the XColor struct used in the X11 drawing API.
 *
 * Deprecated: 3.14: Use #GdkRGBA
 */
struct _GdkColor
{
  guint32 pixel;
  guint16 red;
  guint16 green;
  guint16 blue;
};

#define GDK_TYPE_COLOR (gdk_color_get_type ())

GDK_DEPRECATED_IN_3_14_FOR(gdk_rgba_get_type)
GType     gdk_color_get_type (void) G_GNUC_CONST;

GDK_DEPRECATED_IN_3_14_FOR(gdk_rgba_copy)
GdkColor *gdk_color_copy      (const GdkColor *color);
GDK_DEPRECATED_IN_3_14_FOR(gdk_rgba_free)
void      gdk_color_free      (GdkColor       *color);

GDK_DEPRECATED_IN_3_14_FOR(gdk_rgba_hash)
guint     gdk_color_hash      (const GdkColor *color);
GDK_DEPRECATED_IN_3_14_FOR(gdk_rgba_equal)
gboolean  gdk_color_equal     (const GdkColor *colora,
                               const GdkColor *colorb);

GDK_DEPRECATED_IN_3_14_FOR(gdk_rgba_parse)
gboolean  gdk_color_parse     (const gchar    *spec,
                               GdkColor       *color);
GDK_DEPRECATED_IN_3_14_FOR(gdk_rgba_to_string)
gchar *   gdk_color_to_string (const GdkColor *color);


G_END_DECLS

#endif /* __GDK_COLOR_H__ */
