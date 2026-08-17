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

#ifndef __GDK_VISUAL_H__
#define __GDK_VISUAL_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>

G_BEGIN_DECLS

#define GDK_TYPE_VISUAL              (gdk_visual_get_type ())
#define GDK_VISUAL(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_VISUAL, GdkVisual))
#define GDK_VISUAL_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_VISUAL, GdkVisualClass))
#define GDK_IS_VISUAL(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_VISUAL))
#define GDK_IS_VISUAL_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_VISUAL))
#define GDK_VISUAL_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_VISUAL, GdkVisualClass))

typedef struct _GdkVisualClass    GdkVisualClass;

/* Types of visuals.
 *   StaticGray:
 *   Grayscale:
 *   StaticColor:
 *   PseudoColor:
 *   TrueColor:
 *   DirectColor:
 */
typedef enum
{
  GDK_VISUAL_STATIC_GRAY,
  GDK_VISUAL_GRAYSCALE,
  GDK_VISUAL_STATIC_COLOR,
  GDK_VISUAL_PSEUDO_COLOR,
  GDK_VISUAL_TRUE_COLOR,
  GDK_VISUAL_DIRECT_COLOR
} GdkVisualType;

/* The visual type.
 *   "type" is the type of visual this is (PseudoColor, TrueColor, etc).
 *   "depth" is the bit depth of this visual.
 *   "colormap_size" is the size of a colormap for this visual.
 *   "bits_per_rgb" is the number of significant bits per red, green and blue.
 *  The red, green and blue masks, shifts and precisions refer
 *   to value needed to calculate pixel values in TrueColor and DirectColor
 *   visuals. The "mask" is the significant bits within the pixel. The
 *   "shift" is the number of bits left we must shift a primary for it
 *   to be in position (according to the "mask"). "prec" refers to how
 *   much precision the pixel value contains for a particular primary.
 */
struct _GdkVisual
{
  GObject parent_instance;
  
  GdkVisualType GSEAL (type);
  gint GSEAL (depth);
  GdkByteOrder GSEAL (byte_order);
  gint GSEAL (colormap_size);
  gint GSEAL (bits_per_rgb);

  guint32 GSEAL (red_mask);
  gint GSEAL (red_shift);
  gint GSEAL (red_prec);

  guint32 GSEAL (green_mask);
  gint GSEAL (green_shift);
  gint GSEAL (green_prec);

  guint32 GSEAL (blue_mask);
  gint GSEAL (blue_shift);
  gint GSEAL (blue_prec);
};

GType         gdk_visual_get_type            (void) G_GNUC_CONST;

#ifndef GDK_MULTIHEAD_SAFE
gint	      gdk_visual_get_best_depth	     (void);
GdkVisualType gdk_visual_get_best_type	     (void);
GdkVisual*    gdk_visual_get_system	     (void);
GdkVisual*    gdk_visual_get_best	     (void);
GdkVisual*    gdk_visual_get_best_with_depth (gint	     depth);
GdkVisual*    gdk_visual_get_best_with_type  (GdkVisualType  visual_type);
GdkVisual*    gdk_visual_get_best_with_both  (gint	     depth,
					      GdkVisualType  visual_type);

void gdk_query_depths	    (gint	    **depths,
			     gint	     *count);
void gdk_query_visual_types (GdkVisualType  **visual_types,
			     gint	     *count);

GList* gdk_list_visuals (void);
#endif

GdkScreen *gdk_visual_get_screen (GdkVisual *visual);

GdkVisualType gdk_visual_get_visual_type         (GdkVisual *visual);
gint          gdk_visual_get_depth               (GdkVisual *visual);
GdkByteOrder  gdk_visual_get_byte_order          (GdkVisual *visual);
gint          gdk_visual_get_colormap_size       (GdkVisual *visual);
gint          gdk_visual_get_bits_per_rgb        (GdkVisual *visual);
void          gdk_visual_get_red_pixel_details   (GdkVisual *visual,
                                                  guint32   *mask,
                                                  gint      *shift,
                                                  gint      *precision);
void          gdk_visual_get_green_pixel_details (GdkVisual *visual,
                                                  guint32   *mask,
                                                  gint      *shift,
                                                  gint      *precision);
void          gdk_visual_get_blue_pixel_details  (GdkVisual *visual,
                                                  guint32   *mask,
                                                  gint      *shift,
                                                  gint      *precision);

#ifndef GDK_DISABLE_DEPRECATED
#define gdk_visual_ref(v) g_object_ref(v)
#define gdk_visual_unref(v) g_object_unref(v)
#endif

G_END_DECLS

#endif /* __GDK_VISUAL_H__ */
