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

#ifndef __GDK_RGB_H__
#define __GDK_RGB_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>

G_BEGIN_DECLS

typedef struct _GdkRgbCmap GdkRgbCmap;

typedef enum
{
  GDK_RGB_DITHER_NONE,
  GDK_RGB_DITHER_NORMAL,
  GDK_RGB_DITHER_MAX
} GdkRgbDither;

#ifndef GDK_DISABLE_DEPRECATED

struct _GdkRgbCmap {
  guint32 colors[256];
  gint n_colors;

  /*< private >*/
  GSList *info_list;
};

void gdk_rgb_init (void);

gulong gdk_rgb_xpixel_from_rgb   (guint32      rgb) G_GNUC_CONST;
void   gdk_rgb_gc_set_foreground (GdkGC       *gc,
				  guint32      rgb);
void   gdk_rgb_gc_set_background (GdkGC       *gc,
				  guint32      rgb);
#define gdk_rgb_get_cmap               gdk_rgb_get_colormap

void   gdk_rgb_find_color        (GdkColormap *colormap,
				  GdkColor    *color);

void        gdk_draw_rgb_image              (GdkDrawable  *drawable,
					     GdkGC        *gc,
					     gint          x,
					     gint          y,
					     gint          width,
					     gint          height,
					     GdkRgbDither  dith,
					     const guchar *rgb_buf,
					     gint          rowstride);
void        gdk_draw_rgb_image_dithalign    (GdkDrawable  *drawable,
					     GdkGC        *gc,
					     gint          x,
					     gint          y,
					     gint          width,
					     gint          height,
					     GdkRgbDither  dith,
					     const guchar *rgb_buf,
					     gint          rowstride,
					     gint          xdith,
					     gint          ydith);
void        gdk_draw_rgb_32_image           (GdkDrawable  *drawable,
					     GdkGC        *gc,
					     gint          x,
					     gint          y,
					     gint          width,
					     gint          height,
					     GdkRgbDither  dith,
					     const guchar *buf,
					     gint          rowstride);
void        gdk_draw_rgb_32_image_dithalign (GdkDrawable  *drawable,
					     GdkGC        *gc,
					     gint          x,
					     gint          y,
					     gint          width,
					     gint          height,
					     GdkRgbDither  dith,
					     const guchar *buf,
					     gint          rowstride,
					     gint          xdith,
					     gint          ydith);
void        gdk_draw_gray_image             (GdkDrawable  *drawable,
					     GdkGC        *gc,
					     gint          x,
					     gint          y,
					     gint          width,
					     gint          height,
					     GdkRgbDither  dith,
					     const guchar *buf,
					     gint          rowstride);
void        gdk_draw_indexed_image          (GdkDrawable  *drawable,
					     GdkGC        *gc,
					     gint          x,
					     gint          y,
					     gint          width,
					     gint          height,
					     GdkRgbDither  dith,
					     const guchar *buf,
					     gint          rowstride,
					     GdkRgbCmap   *cmap);
GdkRgbCmap *gdk_rgb_cmap_new                (guint32      *colors,
					     gint          n_colors);
void        gdk_rgb_cmap_free               (GdkRgbCmap   *cmap);

void     gdk_rgb_set_verbose (gboolean verbose);

/* experimental colormap stuff */
void gdk_rgb_set_install    (gboolean install);
void gdk_rgb_set_min_colors (gint     min_colors);

#ifndef GDK_MULTIHEAD_SAFE
GdkColormap *gdk_rgb_get_colormap (void);
GdkVisual *  gdk_rgb_get_visual   (void);
gboolean     gdk_rgb_ditherable   (void);
gboolean     gdk_rgb_colormap_ditherable (GdkColormap *cmap);
#endif
#endif /* GDK_DISABLE_DEPRECATED */

G_END_DECLS


#endif /* __GDK_RGB_H__ */
