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

#ifndef __GDK_PIXMAP_H__
#define __GDK_PIXMAP_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>
#include <gdk/gdkdrawable.h>

G_BEGIN_DECLS

typedef struct _GdkPixmapObject GdkPixmapObject;
typedef struct _GdkPixmapObjectClass GdkPixmapObjectClass;

#define GDK_TYPE_PIXMAP              (gdk_pixmap_get_type ())
#define GDK_PIXMAP(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_PIXMAP, GdkPixmap))
#define GDK_PIXMAP_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_PIXMAP, GdkPixmapObjectClass))
#define GDK_IS_PIXMAP(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_PIXMAP))
#define GDK_IS_PIXMAP_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_PIXMAP))
#define GDK_PIXMAP_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_PIXMAP, GdkPixmapObjectClass))
#define GDK_PIXMAP_OBJECT(object)    ((GdkPixmapObject *) GDK_PIXMAP (object))

struct _GdkPixmapObject
{
  GdkDrawable parent_instance;
  
  GdkDrawable *GSEAL (impl);  /* window-system-specific delegate object */

  gint GSEAL (depth);
};

struct _GdkPixmapObjectClass
{
  GdkDrawableClass parent_class;

};

GType      gdk_pixmap_get_type          (void) G_GNUC_CONST;

/* Pixmaps
 */
GdkPixmap* gdk_pixmap_new		(GdkDrawable *drawable,
					 gint	      width,
					 gint	      height,
					 gint	      depth);
#ifndef GDK_DISABLE_DEPRECATED
GdkBitmap* gdk_bitmap_create_from_data	(GdkDrawable *drawable,
					 const gchar *data,
					 gint	      width,
					 gint	      height);
GdkPixmap* gdk_pixmap_create_from_data	(GdkDrawable    *drawable,
					 const gchar 	*data,
					 gint	     	 width,
					 gint	     	 height,
					 gint	         depth,
					 const GdkColor *fg,
					 const GdkColor *bg);

GdkPixmap* gdk_pixmap_create_from_xpm            (GdkDrawable    *drawable,
						  GdkBitmap     **mask,
						  const GdkColor *transparent_color,
						  const gchar    *filename);
GdkPixmap* gdk_pixmap_colormap_create_from_xpm   (GdkDrawable    *drawable,
						  GdkColormap    *colormap,
						  GdkBitmap     **mask,
						  const GdkColor *transparent_color,
						  const gchar    *filename);
GdkPixmap* gdk_pixmap_create_from_xpm_d          (GdkDrawable    *drawable,
						  GdkBitmap     **mask,
						  const GdkColor *transparent_color,
						  gchar         **data);
GdkPixmap* gdk_pixmap_colormap_create_from_xpm_d (GdkDrawable    *drawable,
						  GdkColormap    *colormap,
						  GdkBitmap     **mask,
						  const GdkColor *transparent_color,
						  gchar         **data);
#endif

void          gdk_pixmap_get_size                (GdkPixmap      *pixmap,
                                                  gint	         *width,
                                                  gint  	 *height);

/* Functions to create/lookup pixmaps from their native equivalents
 */
#ifndef GDK_MULTIHEAD_SAFE
GdkPixmap*    gdk_pixmap_foreign_new (GdkNativeWindow anid);
GdkPixmap*    gdk_pixmap_lookup      (GdkNativeWindow anid);
#endif /* GDK_MULTIHEAD_SAFE */

GdkPixmap*    gdk_pixmap_foreign_new_for_display (GdkDisplay      *display,
						  GdkNativeWindow  anid);
GdkPixmap*    gdk_pixmap_lookup_for_display      (GdkDisplay      *display,
						  GdkNativeWindow  anid);
GdkPixmap*    gdk_pixmap_foreign_new_for_screen  (GdkScreen       *screen,
						  GdkNativeWindow  anid,
						  gint             width,
                                                  gint             height,
                                                  gint             depth);

#ifndef GDK_DISABLE_DEPRECATED
#define gdk_bitmap_ref                 g_object_ref
#define gdk_bitmap_unref               g_object_unref
#define gdk_pixmap_ref                 g_object_ref
#define gdk_pixmap_unref               g_object_unref
#endif /* GDK_DISABLE_DEPRECATED */

G_END_DECLS

#endif /* __GDK_PIXMAP_H__ */
