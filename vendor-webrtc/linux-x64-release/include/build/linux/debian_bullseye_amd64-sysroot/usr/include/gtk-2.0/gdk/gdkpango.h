/* GDK - The GIMP Drawing Kit
 * Copyright (C) 2000 Red Hat, Inc. 
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

#ifndef __GDK_PANGO_H__
#define __GDK_PANGO_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>

G_BEGIN_DECLS

/* Pango interaction */

typedef struct _GdkPangoRenderer        GdkPangoRenderer;
typedef struct _GdkPangoRendererClass   GdkPangoRendererClass;
typedef struct _GdkPangoRendererPrivate GdkPangoRendererPrivate;

#define GDK_TYPE_PANGO_RENDERER            (gdk_pango_renderer_get_type())
#define GDK_PANGO_RENDERER(object)         (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_PANGO_RENDERER, GdkPangoRenderer))
#define GDK_IS_PANGO_RENDERER(object)      (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_PANGO_RENDERER))
#define GDK_PANGO_RENDERER_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_PANGO_RENDERER, GdkPangoRendererClass))
#define GDK_IS_PANGO_RENDERER_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_PANGO_RENDERER))
#define GDK_PANGO_RENDERER_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_PANGO_RENDERER, GdkPangoRendererClass))

/**
 * GdkPangoRenderer:
 *
 * #GdkPangoRenderer is a subclass of #PangoRenderer used for rendering
 * Pango objects into GDK drawables. The default renderer for a particular
 * screen is obtained with gdk_pango_renderer_get_default(); Pango
 * functions like pango_renderer_draw_layout() and
 * pango_renderer_draw_layout_line() are then used to draw objects with
 * the renderer.
 *
 * In most simple cases, applications can just use gdk_draw_layout(), and
 * don't need to directly use #GdkPangoRenderer at all. Using the
 * #GdkPangoRenderer directly is most useful when working with a
 * transformation such as a rotation, because the Pango drawing functions
 * take user space coordinates (coordinates before the transformation)
 * instead of device coordinates.
 *
 * In certain cases it can be useful to subclass #GdkPangoRenderer. Examples
 * of reasons to do this are to add handling of custom attributes by
 * overriding 'prepare_run' or to do custom drawing of embedded objects
 * by overriding 'draw_shape'.
 *
 * Since: 2.6
 **/
struct _GdkPangoRenderer
{
  /*< private >*/
  PangoRenderer parent_instance;

  GdkPangoRendererPrivate *priv;
};

/**
 * GdkPangoRendererClass:
 *
 * #GdkPangoRenderer is the class structure for #GdkPangoRenderer.
 *
 * Since: 2.6
 **/
struct _GdkPangoRendererClass
{
  /*< private >*/
  PangoRendererClass parent_class;
};

GType gdk_pango_renderer_get_type (void) G_GNUC_CONST;

PangoRenderer *gdk_pango_renderer_new         (GdkScreen *screen);
PangoRenderer *gdk_pango_renderer_get_default (GdkScreen *screen);

void gdk_pango_renderer_set_drawable       (GdkPangoRenderer *gdk_renderer,
					    GdkDrawable      *drawable);
void gdk_pango_renderer_set_gc             (GdkPangoRenderer *gdk_renderer,
					    GdkGC            *gc);
void gdk_pango_renderer_set_stipple        (GdkPangoRenderer *gdk_renderer,
					    PangoRenderPart   part,
					    GdkBitmap        *stipple);
void gdk_pango_renderer_set_override_color (GdkPangoRenderer *gdk_renderer,
					    PangoRenderPart   part,
					    const GdkColor   *color);

/************************************************************************/

PangoContext *gdk_pango_context_get_for_screen (GdkScreen    *screen);
#ifndef GDK_MULTIHEAD_SAFE
PangoContext *gdk_pango_context_get            (void);
#endif
#ifndef GDK_DISABLE_DEPRECATED
void          gdk_pango_context_set_colormap   (PangoContext *context,
                                                GdkColormap  *colormap);
#endif 


/* Get a clip region to draw only part of a layout or
 * line. index_ranges contains alternating range starts/stops. The
 * region is the region which contains the given ranges, i.e. if you
 * draw with the region as clip, only the given ranges are drawn.
 */

GdkRegion    *gdk_pango_layout_line_get_clip_region (PangoLayoutLine *line,
                                                     gint             x_origin,
                                                     gint             y_origin,
                                                     const gint      *index_ranges,
                                                     gint             n_ranges);
GdkRegion    *gdk_pango_layout_get_clip_region      (PangoLayout     *layout,
                                                     gint             x_origin,
                                                     gint             y_origin,
                                                     const gint      *index_ranges,
                                                     gint             n_ranges);



/* Attributes use to render insensitive text in GTK+. */

typedef struct _GdkPangoAttrStipple GdkPangoAttrStipple;
typedef struct _GdkPangoAttrEmbossed GdkPangoAttrEmbossed;
typedef struct _GdkPangoAttrEmbossColor GdkPangoAttrEmbossColor;

struct _GdkPangoAttrStipple
{
  PangoAttribute attr;
  GdkBitmap *stipple;
};

struct _GdkPangoAttrEmbossed
{
  PangoAttribute attr;
  gboolean embossed;
};

struct _GdkPangoAttrEmbossColor
{
  PangoAttribute attr;
  PangoColor color;
};

PangoAttribute *gdk_pango_attr_stipple_new  (GdkBitmap *stipple);
PangoAttribute *gdk_pango_attr_embossed_new (gboolean embossed);
PangoAttribute *gdk_pango_attr_emboss_color_new (const GdkColor *color);

G_END_DECLS

#endif /* __GDK_FONT_H__ */
