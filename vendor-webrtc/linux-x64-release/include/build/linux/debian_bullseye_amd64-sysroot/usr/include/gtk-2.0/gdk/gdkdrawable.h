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

#ifndef __GDK_DRAWABLE_H__
#define __GDK_DRAWABLE_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>
#include <gdk/gdkgc.h>
#include <gdk/gdkrgb.h>
#include <gdk-pixbuf/gdk-pixbuf.h>

#include <cairo.h>

G_BEGIN_DECLS

typedef struct _GdkDrawableClass GdkDrawableClass;
typedef struct _GdkTrapezoid     GdkTrapezoid;

#define GDK_TYPE_DRAWABLE              (gdk_drawable_get_type ())
#define GDK_DRAWABLE(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_DRAWABLE, GdkDrawable))
#define GDK_DRAWABLE_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_DRAWABLE, GdkDrawableClass))
#define GDK_IS_DRAWABLE(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_DRAWABLE))
#define GDK_IS_DRAWABLE_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_DRAWABLE))
#define GDK_DRAWABLE_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_DRAWABLE, GdkDrawableClass))

struct _GdkDrawable
{
  GObject parent_instance;
};
 
struct _GdkDrawableClass 
{
  GObjectClass parent_class;
  
  GdkGC *(*create_gc)    (GdkDrawable    *drawable,
		          GdkGCValues    *values,
		          GdkGCValuesMask mask);
  void (*draw_rectangle) (GdkDrawable  *drawable,
			  GdkGC	       *gc,
			  gboolean	filled,
			  gint		x,
			  gint		y,
			  gint		width,
			  gint		height);
  void (*draw_arc)       (GdkDrawable  *drawable,
			  GdkGC	       *gc,
			  gboolean	filled,
			  gint		x,
			  gint		y,
			  gint		width,
			  gint		height,
			  gint		angle1,
			  gint		angle2);
  void (*draw_polygon)   (GdkDrawable  *drawable,
			  GdkGC	       *gc,
			  gboolean	filled,
			  GdkPoint     *points,
			  gint		npoints);
  void (*draw_text)      (GdkDrawable  *drawable,
			  GdkFont      *font,
			  GdkGC	       *gc,
			  gint		x,
			  gint		y,
			  const gchar  *text,
			  gint		text_length);
  void (*draw_text_wc)   (GdkDrawable	 *drawable,
			  GdkFont	 *font,
			  GdkGC		 *gc,
			  gint		  x,
			  gint		  y,
			  const GdkWChar *text,
			  gint		  text_length);
  void (*draw_drawable)  (GdkDrawable  *drawable,
			  GdkGC	       *gc,
			  GdkDrawable  *src,
			  gint		xsrc,
			  gint		ysrc,
			  gint		xdest,
			  gint		ydest,
			  gint		width,
			  gint		height);
  void (*draw_points)	 (GdkDrawable  *drawable,
			  GdkGC	       *gc,
			  GdkPoint     *points,
			  gint		npoints);
  void (*draw_segments)	 (GdkDrawable  *drawable,
			  GdkGC	       *gc,
			  GdkSegment   *segs,
			  gint		nsegs);
  void (*draw_lines)     (GdkDrawable  *drawable,
			  GdkGC        *gc,
			  GdkPoint     *points,
			  gint          npoints);

  void (*draw_glyphs)    (GdkDrawable      *drawable,
			  GdkGC	           *gc,
			  PangoFont        *font,
			  gint              x,
			  gint              y,
			  PangoGlyphString *glyphs);

  void (*draw_image)     (GdkDrawable *drawable,
                          GdkGC	      *gc,
                          GdkImage    *image,
                          gint	       xsrc,
                          gint	       ysrc,
                          gint	       xdest,
                          gint	       ydest,
                          gint	       width,
                          gint	       height);
  
  gint (*get_depth)      (GdkDrawable  *drawable);
  void (*get_size)       (GdkDrawable  *drawable,
                          gint         *width,
                          gint         *height);

  void (*set_colormap)   (GdkDrawable  *drawable,
                          GdkColormap  *cmap);

  GdkColormap* (*get_colormap)	(GdkDrawable  *drawable);
  GdkVisual*   (*get_visual)	(GdkDrawable  *drawable);
  GdkScreen*   (*get_screen)	(GdkDrawable  *drawable);

  GdkImage*    (*get_image)  (GdkDrawable  *drawable,
                              gint          x,
                              gint          y,
                              gint          width,
                              gint          height);

  GdkRegion*   (*get_clip_region)    (GdkDrawable  *drawable);
  GdkRegion*   (*get_visible_region) (GdkDrawable  *drawable);

  GdkDrawable* (*get_composite_drawable) (GdkDrawable *drawable,
                                          gint         x,
                                          gint         y,
                                          gint         width,
                                          gint         height,
                                          gint        *composite_x_offset,
                                          gint        *composite_y_offset);

  void         (*draw_pixbuf) (GdkDrawable *drawable,
			       GdkGC       *gc,
			       GdkPixbuf   *pixbuf,
			       gint         src_x,
			       gint         src_y,
			       gint         dest_x,
			       gint         dest_y,
			       gint         width,
			       gint         height,
			       GdkRgbDither dither,
			       gint         x_dither,
			       gint         y_dither);
  GdkImage*    (*_copy_to_image) (GdkDrawable    *drawable,
				  GdkImage       *image,
				  gint            src_x,
				  gint            src_y,
				  gint            dest_x,
				  gint            dest_y,
				  gint            width,
				  gint            height);
  
  void (*draw_glyphs_transformed) (GdkDrawable      *drawable,
				   GdkGC	    *gc,
				   PangoMatrix      *matrix,
				   PangoFont        *font,
				   gint              x,
				   gint              y,
				   PangoGlyphString *glyphs);
  void (*draw_trapezoids)         (GdkDrawable      *drawable,
				   GdkGC	    *gc,
				   GdkTrapezoid     *trapezoids,
				   gint              n_trapezoids);

  cairo_surface_t *(*ref_cairo_surface) (GdkDrawable *drawable);

  GdkDrawable *(*get_source_drawable) (GdkDrawable *drawable);

  void         (*set_cairo_clip)      (GdkDrawable *drawable,
				       cairo_t *cr);

  cairo_surface_t * (*create_cairo_surface) (GdkDrawable *drawable,
					     int width,
					     int height);

  void (*draw_drawable_with_src)  (GdkDrawable  *drawable,
				   GdkGC	       *gc,
				   GdkDrawable  *src,
				   gint		xsrc,
				   gint		ysrc,
				   gint		xdest,
				   gint		ydest,
				   gint		width,
				   gint		height,
				   GdkDrawable  *original_src);

  /* Padding for future expansion */
  void         (*_gdk_reserved7)  (void);
  void         (*_gdk_reserved9)  (void);
  void         (*_gdk_reserved10) (void);
  void         (*_gdk_reserved11) (void);
  void         (*_gdk_reserved12) (void);
  void         (*_gdk_reserved13) (void);
  void         (*_gdk_reserved14) (void);
  void         (*_gdk_reserved15) (void);
};

struct _GdkTrapezoid
{
  double y1, x11, x21, y2, x12, x22;
};

GType           gdk_drawable_get_type     (void) G_GNUC_CONST;

/* Manipulation of drawables
 */

#ifndef GDK_DISABLE_DEPRECATED
void            gdk_drawable_set_data     (GdkDrawable    *drawable,
					   const gchar    *key,
					   gpointer	  data,
					   GDestroyNotify  destroy_func);
gpointer        gdk_drawable_get_data     (GdkDrawable    *drawable,
					   const gchar    *key);
#endif /* GDK_DISABLE_DEPRECATED */

void	        gdk_drawable_set_colormap (GdkDrawable	  *drawable,
					   GdkColormap	  *colormap);
GdkColormap*    gdk_drawable_get_colormap (GdkDrawable	  *drawable);
gint            gdk_drawable_get_depth    (GdkDrawable	  *drawable);

#if !defined (GDK_DISABLE_DEPRECATED)
void            gdk_drawable_get_size     (GdkDrawable	  *drawable,
					   gint	          *width,
					   gint  	  *height);
GdkVisual*      gdk_drawable_get_visual   (GdkDrawable	  *drawable);
GdkScreen*	gdk_drawable_get_screen   (GdkDrawable    *drawable);
GdkDisplay*	gdk_drawable_get_display  (GdkDrawable    *drawable);
#endif /* GDK_DISABLE_DEPRECATED */

#ifndef GDK_DISABLE_DEPRECATED
GdkDrawable*    gdk_drawable_ref          (GdkDrawable    *drawable);
void            gdk_drawable_unref        (GdkDrawable    *drawable);
#endif /* GDK_DISABLE_DEPRECATED */

/* Drawing
 */
#ifndef GDK_DISABLE_DEPRECATED
void gdk_draw_point     (GdkDrawable      *drawable,
			 GdkGC            *gc,
			 gint              x,
			 gint              y);
void gdk_draw_line      (GdkDrawable      *drawable,
			 GdkGC            *gc,
			 gint              x1_,
			 gint              y1_,
			 gint              x2_,
			 gint              y2_);
void gdk_draw_rectangle (GdkDrawable      *drawable,
			 GdkGC            *gc,
			 gboolean          filled,
			 gint              x,
			 gint              y,
			 gint              width,
			 gint              height);
void gdk_draw_arc       (GdkDrawable      *drawable,
			 GdkGC            *gc,
			 gboolean          filled,
			 gint              x,
			 gint              y,
			 gint              width,
			 gint              height,
			 gint              angle1,
			 gint              angle2);
void gdk_draw_polygon   (GdkDrawable      *drawable,
			 GdkGC            *gc,
			 gboolean          filled,
			 const GdkPoint   *points,
			 gint              n_points);
void gdk_draw_string    (GdkDrawable      *drawable,
			 GdkFont          *font,
			 GdkGC            *gc,
			 gint              x,
			 gint              y,
			 const gchar      *string);
void gdk_draw_text      (GdkDrawable      *drawable,
			 GdkFont          *font,
			 GdkGC            *gc,
			 gint              x,
			 gint              y,
			 const gchar      *text,
			 gint              text_length);
void gdk_draw_text_wc   (GdkDrawable      *drawable,
			 GdkFont          *font,
			 GdkGC            *gc,
			 gint              x,
			 gint              y,
			 const GdkWChar   *text,
			 gint              text_length);
void gdk_draw_drawable  (GdkDrawable      *drawable,
			 GdkGC            *gc,
			 GdkDrawable      *src,
			 gint              xsrc,
			 gint              ysrc,
			 gint              xdest,
			 gint              ydest,
			 gint              width,
			 gint              height);
void gdk_draw_image     (GdkDrawable      *drawable,
			 GdkGC            *gc,
			 GdkImage         *image,
			 gint              xsrc,
			 gint              ysrc,
			 gint              xdest,
			 gint              ydest,
			 gint              width,
			 gint              height);
void gdk_draw_points    (GdkDrawable      *drawable,
			 GdkGC            *gc,
			 const GdkPoint   *points,
			 gint              n_points);
void gdk_draw_segments  (GdkDrawable      *drawable,
			 GdkGC            *gc,
			 const GdkSegment *segs,
			 gint              n_segs);
void gdk_draw_lines     (GdkDrawable      *drawable,
			 GdkGC            *gc,
			 const GdkPoint   *points,
			 gint              n_points);
void gdk_draw_pixbuf    (GdkDrawable      *drawable,
			 GdkGC            *gc,
			 const GdkPixbuf  *pixbuf,
			 gint              src_x,
			 gint              src_y,
			 gint              dest_x,
			 gint              dest_y,
			 gint              width,
			 gint              height,
			 GdkRgbDither      dither,
			 gint              x_dither,
			 gint              y_dither);

void gdk_draw_glyphs      (GdkDrawable      *drawable,
			   GdkGC            *gc,
			   PangoFont        *font,
			   gint              x,
			   gint              y,
			   PangoGlyphString *glyphs);
void gdk_draw_layout_line (GdkDrawable      *drawable,
			   GdkGC            *gc,
			   gint              x,
			   gint              y,
			   PangoLayoutLine  *line);
void gdk_draw_layout      (GdkDrawable      *drawable,
			   GdkGC            *gc,
			   gint              x,
			   gint              y,
			   PangoLayout      *layout);

void gdk_draw_layout_line_with_colors (GdkDrawable     *drawable,
                                       GdkGC           *gc,
                                       gint             x,
                                       gint             y,
                                       PangoLayoutLine *line,
                                       const GdkColor  *foreground,
                                       const GdkColor  *background);
void gdk_draw_layout_with_colors      (GdkDrawable     *drawable,
                                       GdkGC           *gc,
                                       gint             x,
                                       gint             y,
                                       PangoLayout     *layout,
                                       const GdkColor  *foreground,
                                       const GdkColor  *background);

void gdk_draw_glyphs_transformed (GdkDrawable        *drawable,
				  GdkGC	             *gc,
				  const PangoMatrix  *matrix,
				  PangoFont          *font,
				  gint                x,
				  gint                y,
				  PangoGlyphString   *glyphs);
void gdk_draw_trapezoids         (GdkDrawable        *drawable,
				  GdkGC	             *gc,
				  const GdkTrapezoid *trapezoids,
				  gint                n_trapezoids);

#define gdk_draw_pixmap                gdk_draw_drawable
#define gdk_draw_bitmap                gdk_draw_drawable

GdkImage* gdk_drawable_get_image      (GdkDrawable *drawable,
                                       gint         x,
                                       gint         y,
                                       gint         width,
                                       gint         height);
GdkImage *gdk_drawable_copy_to_image (GdkDrawable  *drawable,
				      GdkImage     *image,
				      gint          src_x,
				      gint          src_y,
				      gint          dest_x,
				      gint          dest_y,
				      gint          width,
				      gint          height);
#endif /* GDK_DISABLE_DEPRECATED */

GdkRegion *gdk_drawable_get_clip_region    (GdkDrawable *drawable);
GdkRegion *gdk_drawable_get_visible_region (GdkDrawable *drawable);

G_END_DECLS

#endif /* __GDK_DRAWABLE_H__ */
