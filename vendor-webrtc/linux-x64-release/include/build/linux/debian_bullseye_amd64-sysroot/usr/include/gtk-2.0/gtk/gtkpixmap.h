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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
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

#if !defined (GTK_DISABLE_DEPRECATED) || defined (__GTK_PIXMAP_C__)

#ifndef __GTK_PIXMAP_H__
#define __GTK_PIXMAP_H__

#include <gtk/gtk.h>


G_BEGIN_DECLS

#define GTK_TYPE_PIXMAP			 (gtk_pixmap_get_type ())
#define GTK_PIXMAP(obj)			 (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_PIXMAP, GtkPixmap))
#define GTK_PIXMAP_CLASS(klass)		 (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_PIXMAP, GtkPixmapClass))
#define GTK_IS_PIXMAP(obj)		 (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_PIXMAP))
#define GTK_IS_PIXMAP_CLASS(klass)	 (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_PIXMAP))
#define GTK_PIXMAP_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_PIXMAP, GtkPixmapClass))


typedef struct _GtkPixmap	GtkPixmap;
typedef struct _GtkPixmapClass	GtkPixmapClass;

struct _GtkPixmap
{
  GtkMisc misc;

  GdkPixmap *pixmap;
  GdkBitmap *mask;

  GdkPixmap *pixmap_insensitive;
  guint build_insensitive : 1;
};

struct _GtkPixmapClass
{
  GtkMiscClass parent_class;
};


GType	   gtk_pixmap_get_type	 (void) G_GNUC_CONST;
GtkWidget* gtk_pixmap_new	 (GdkPixmap  *pixmap,
				  GdkBitmap  *mask);
void	   gtk_pixmap_set	 (GtkPixmap  *pixmap,
				  GdkPixmap  *val,
				  GdkBitmap  *mask);
void	   gtk_pixmap_get	 (GtkPixmap  *pixmap,
				  GdkPixmap **val,
				  GdkBitmap **mask);

void       gtk_pixmap_set_build_insensitive (GtkPixmap *pixmap,
		                             gboolean   build);


G_END_DECLS

#endif /* __GTK_PIXMAP_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
