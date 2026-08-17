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

#ifndef GTK_DISABLE_DEPRECATED

#ifndef __GTK_PREVIEW_H__
#define __GTK_PREVIEW_H__

#include <gtk/gtk.h>


G_BEGIN_DECLS

#define GTK_TYPE_PREVIEW            (gtk_preview_get_type ())
#define GTK_PREVIEW(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_PREVIEW, GtkPreview))
#define GTK_PREVIEW_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_PREVIEW, GtkPreviewClass))
#define GTK_IS_PREVIEW(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_PREVIEW))
#define GTK_IS_PREVIEW_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_PREVIEW))
#define GTK_PREVIEW_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_PREVIEW, GtkPreviewClass))


typedef struct _GtkPreview       GtkPreview;
typedef struct _GtkPreviewInfo   GtkPreviewInfo;
typedef union  _GtkDitherInfo    GtkDitherInfo;
typedef struct _GtkPreviewClass  GtkPreviewClass;

struct _GtkPreview
{
  GtkWidget widget;

  guchar *buffer;
  guint16 buffer_width;
  guint16 buffer_height;

  guint16 bpp;
  guint16 rowstride;

  GdkRgbDither dither;

  guint type : 1;
  guint expand : 1;
};

struct _GtkPreviewInfo
{
  guchar *lookup;

  gdouble gamma;
};

union _GtkDitherInfo
{
  gushort s[2];
  guchar c[4];
};

struct _GtkPreviewClass
{
  GtkWidgetClass parent_class;

  GtkPreviewInfo info;

};


GType           gtk_preview_get_type           (void) G_GNUC_CONST;
void            gtk_preview_uninit             (void);
GtkWidget*      gtk_preview_new                (GtkPreviewType   type);
void            gtk_preview_size               (GtkPreview      *preview,
						gint             width,
						gint             height);
void            gtk_preview_put                (GtkPreview      *preview,
						GdkWindow       *window,
						GdkGC           *gc,
						gint             srcx,
						gint             srcy,
						gint             destx,
						gint             desty,
						gint             width,
						gint             height);
void            gtk_preview_draw_row           (GtkPreview      *preview,
						guchar          *data,
						gint             x,
						gint             y,
						gint             w);
void            gtk_preview_set_expand         (GtkPreview      *preview,
						gboolean         expand);

void            gtk_preview_set_gamma          (double           gamma_);
void            gtk_preview_set_color_cube     (guint            nred_shades,
						guint            ngreen_shades,
						guint            nblue_shades,
						guint            ngray_shades);
void            gtk_preview_set_install_cmap   (gint             install_cmap);
void            gtk_preview_set_reserved       (gint             nreserved);
void            gtk_preview_set_dither         (GtkPreview      *preview,
						GdkRgbDither     dither);
GdkVisual*      gtk_preview_get_visual         (void);
GdkColormap*    gtk_preview_get_cmap           (void);
GtkPreviewInfo* gtk_preview_get_info           (void);

/* This function reinitializes the preview colormap and visual from
 * the current gamma/color_cube/install_cmap settings. It must only
 * be called if there are no previews or users's of the preview
 * colormap in existence.
 */
void            gtk_preview_reset              (void);


G_END_DECLS

#endif /* __GTK_PREVIEW_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
