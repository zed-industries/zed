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

#ifndef __GDK_REGION_H__
#define __GDK_REGION_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>

G_BEGIN_DECLS

#ifndef GDK_DISABLE_DEPRECATED
/* GC fill rule for polygons
 *  EvenOddRule
 *  WindingRule
 */
typedef enum
{
  GDK_EVEN_ODD_RULE,
  GDK_WINDING_RULE
} GdkFillRule;
#endif

/* Types of overlapping between a rectangle and a region
 * GDK_OVERLAP_RECTANGLE_IN: rectangle is in region
 * GDK_OVERLAP_RECTANGLE_OUT: rectangle in not in region
 * GDK_OVERLAP_RECTANGLE_PART: rectangle in partially in region
 */
typedef enum
{
  GDK_OVERLAP_RECTANGLE_IN,
  GDK_OVERLAP_RECTANGLE_OUT,
  GDK_OVERLAP_RECTANGLE_PART
} GdkOverlapType;

#ifndef GDK_DISABLE_DEPRECATED
typedef void (* GdkSpanFunc) (GdkSpan *span,
                              gpointer data);
#endif

GdkRegion    * gdk_region_new             (void);
#ifndef GDK_DISABLE_DEPRECATED
GdkRegion    * gdk_region_polygon         (const GdkPoint     *points,
                                           gint                n_points,
                                           GdkFillRule         fill_rule);
#endif
GdkRegion    * gdk_region_copy            (const GdkRegion    *region);
GdkRegion    * gdk_region_rectangle       (const GdkRectangle *rectangle);
void           gdk_region_destroy         (GdkRegion          *region);

void	       gdk_region_get_clipbox     (const GdkRegion    *region,
                                           GdkRectangle       *rectangle);
void           gdk_region_get_rectangles  (const GdkRegion    *region,
                                           GdkRectangle      **rectangles,
                                           gint               *n_rectangles);

gboolean       gdk_region_empty           (const GdkRegion    *region);
gboolean       gdk_region_equal           (const GdkRegion    *region1,
                                           const GdkRegion    *region2);
#ifndef GDK_DISABLE_DEPRECATED
gboolean       gdk_region_rect_equal      (const GdkRegion    *region,
                                           const GdkRectangle *rectangle);
#endif
gboolean       gdk_region_point_in        (const GdkRegion    *region,
                                           int                 x,
                                           int                 y);
GdkOverlapType gdk_region_rect_in         (const GdkRegion    *region,
                                           const GdkRectangle *rectangle);

void           gdk_region_offset          (GdkRegion          *region,
                                           gint                dx,
                                           gint                dy);
#ifndef GDK_DISABLE_DEPRECATED
void           gdk_region_shrink          (GdkRegion          *region,
                                           gint                dx,
                                           gint                dy);
#endif
void           gdk_region_union_with_rect (GdkRegion          *region,
                                           const GdkRectangle *rect);
void           gdk_region_intersect       (GdkRegion          *source1,
                                           const GdkRegion    *source2);
void           gdk_region_union           (GdkRegion          *source1,
                                           const GdkRegion    *source2);
void           gdk_region_subtract        (GdkRegion          *source1,
                                           const GdkRegion    *source2);
void           gdk_region_xor             (GdkRegion          *source1,
                                           const GdkRegion    *source2);

#ifndef GDK_DISABLE_DEPRECATED
void   gdk_region_spans_intersect_foreach (GdkRegion          *region,
                                           const GdkSpan      *spans,
                                           int                 n_spans,
                                           gboolean            sorted,
                                           GdkSpanFunc         function,
                                           gpointer            data);
#endif

G_END_DECLS

#endif /* __GDK_REGION_H__ */

