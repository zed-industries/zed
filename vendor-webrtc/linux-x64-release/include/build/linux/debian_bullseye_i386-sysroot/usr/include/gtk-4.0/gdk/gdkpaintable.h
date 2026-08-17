/*
 * Copyright © 2018 Benjamin Otte
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors: Benjamin Otte <otte@gnome.org>
 */

#ifndef __GDK_PAINTABLE_H__
#define __GDK_PAINTABLE_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>
#include <gdk/gdkversionmacros.h>

G_BEGIN_DECLS

#define GDK_TYPE_PAINTABLE               (gdk_paintable_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_INTERFACE (GdkPaintable, gdk_paintable, GDK, PAINTABLE, GObject)

/**
 * GdkPaintableFlags:
 * @GDK_PAINTABLE_STATIC_SIZE: The size is immutable.
 *   The [signal@GdkPaintable::invalidate-size] signal will never be
 *   emitted.
 * @GDK_PAINTABLE_STATIC_CONTENTS: The content is immutable.
 *   The [signal@GdkPaintable::invalidate-contents] signal will never be
 *   emitted.
 *
 * Flags about a paintable object.
 *
 * Implementations use these for optimizations such as caching.
 */
typedef enum {
  GDK_PAINTABLE_STATIC_SIZE = 1 << 0,
  GDK_PAINTABLE_STATIC_CONTENTS = 1 << 1
} GdkPaintableFlags;

/**
 * GdkPaintableInterface:
 * @snapshot: Snapshot the paintable. The given @width and @height are
 *   guaranteed to be larger than 0.0. The resulting snapshot must modify
 *   only the area in the rectangle from (0,0) to (width, height).
 *   This is the only function that must be implemented for this interface.
 * @get_current_image: return a `GdkPaintable` that does not change over
 *   time. This means the `GDK_PAINTABLE_STATIC_SIZE` and
 *   `GDK_PAINTABLE_STATIC_CONTENTS` flag are set.
 * @get_flags: Get the flags for this instance. See [enum@Gdk.PaintableFlags]
 *   for details.
 * @get_intrinsic_width: The preferred width for this object to be
 *   snapshot at or 0 if none. This is purely a hint. The object must still
 *   be able to render at any size.
 * @get_intrinsic_height: The preferred height for this object to be
 *   snapshot at or 0 if none. This is purely a hint. The object must still
 *   be able to render at any size.
 * @get_intrinsic_aspect_ratio: The preferred aspect ratio for this object
 *   or 0 if none. If both [vfunc@Gdk.Paintable.get_intrinsic_width]
 *   and [vfunc@Gdk.Paintable.get_intrinsic_height] return non-zero
 *   values, this function should return the aspect ratio computed from those.
 *
 * The list of functions that can be implemented for the `GdkPaintable`
 * interface.
 *
 * Note that apart from the [vfunc@Gdk.Paintable.snapshot] function,
 * no virtual function of this interface is mandatory to implement, though it
 * is a good idea to implement [vfunc@Gdk.Paintable.get_current_image]
 * for non-static paintables and [vfunc@Gdk.Paintable.get_flags] if the
 * image is not dynamic as the default implementation returns no flags and
 * that will make the implementation likely quite slow.
 */
struct _GdkPaintableInterface
{
  /*< private >*/
  GTypeInterface g_iface;

  /*< public >*/
  /* draw to 0,0 with the given width and height */
  void                  (* snapshot)                    (GdkPaintable           *paintable,
                                                         GdkSnapshot            *snapshot,
                                                         double                  width,
                                                         double                  height);
  /* get the current contents in an immutable form (optional) */
  GdkPaintable *        (* get_current_image)           (GdkPaintable           *paintable);
   
  /* get flags for potential optimizations (optional) */
  GdkPaintableFlags     (* get_flags)                   (GdkPaintable           *paintable);
  /* preferred width of paintable or 0 if it has no width (optional) */
  int                   (* get_intrinsic_width)         (GdkPaintable           *paintable);
  /* preferred height of paintable or 0 if it has no height (optional) */
  int                   (* get_intrinsic_height)        (GdkPaintable           *paintable);
  /* aspect ratio (width / height) of paintable or 0 if it has no aspect ratio (optional) */
  double                (* get_intrinsic_aspect_ratio)  (GdkPaintable           *paintable);
};

GDK_AVAILABLE_IN_ALL
void            gdk_paintable_snapshot                  (GdkPaintable           *paintable,
                                                         GdkSnapshot            *snapshot,
                                                         double                  width,
                                                         double                  height);
GDK_AVAILABLE_IN_ALL
GdkPaintable *  gdk_paintable_get_current_image         (GdkPaintable           *paintable);

GDK_AVAILABLE_IN_ALL
GdkPaintableFlags
                gdk_paintable_get_flags                 (GdkPaintable           *paintable);
GDK_AVAILABLE_IN_ALL
int             gdk_paintable_get_intrinsic_width       (GdkPaintable           *paintable);
GDK_AVAILABLE_IN_ALL
int             gdk_paintable_get_intrinsic_height      (GdkPaintable           *paintable);
GDK_AVAILABLE_IN_ALL
double          gdk_paintable_get_intrinsic_aspect_ratio(GdkPaintable           *paintable);

GDK_AVAILABLE_IN_ALL
void            gdk_paintable_compute_concrete_size     (GdkPaintable           *paintable,
                                                         double                  specified_width,
                                                         double                  specified_height,
                                                         double                  default_width,
                                                         double                  default_height,
                                                         double                 *concrete_width,
                                                         double                 *concrete_height);
/* for implementations only */
GDK_AVAILABLE_IN_ALL
void            gdk_paintable_invalidate_contents       (GdkPaintable           *paintable);
GDK_AVAILABLE_IN_ALL
void            gdk_paintable_invalidate_size           (GdkPaintable           *paintable);
GDK_AVAILABLE_IN_ALL
GdkPaintable *  gdk_paintable_new_empty                 (int                     intrinsic_width,
                                                         int                     intrinsic_height);


G_END_DECLS

#endif /* __GDK_PAINTABLE_H__ */
