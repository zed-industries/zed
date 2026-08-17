/* GDK - The GIMP Drawing Kit
 * Copyright (C) 2020 Red Hat
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
 *
 */

#ifndef __GDK_POPUP_LAYOUT_H__
#define __GDK_POPUP_LAYOUT_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkenums.h>
#include <gdk/gdktypes.h>
#include <gdk/gdkversionmacros.h>

G_BEGIN_DECLS

/**
 * GdkAnchorHints:
 * @GDK_ANCHOR_FLIP_X: allow flipping anchors horizontally
 * @GDK_ANCHOR_FLIP_Y: allow flipping anchors vertically
 * @GDK_ANCHOR_SLIDE_X: allow sliding surface horizontally
 * @GDK_ANCHOR_SLIDE_Y: allow sliding surface vertically
 * @GDK_ANCHOR_RESIZE_X: allow resizing surface horizontally
 * @GDK_ANCHOR_RESIZE_Y: allow resizing surface vertically
 * @GDK_ANCHOR_FLIP: allow flipping anchors on both axes
 * @GDK_ANCHOR_SLIDE: allow sliding surface on both axes
 * @GDK_ANCHOR_RESIZE: allow resizing surface on both axes
 *
 * Positioning hints for aligning a surface relative to a rectangle.
 *
 * These hints determine how the surface should be positioned in the case that
 * the surface would fall off-screen if placed in its ideal position.
 *
 * For example, %GDK_ANCHOR_FLIP_X will replace %GDK_GRAVITY_NORTH_WEST with
 * %GDK_GRAVITY_NORTH_EAST and vice versa if the surface extends beyond the left
 * or right edges of the monitor.
 *
 * If %GDK_ANCHOR_SLIDE_X is set, the surface can be shifted horizontally to fit
 * on-screen. If %GDK_ANCHOR_RESIZE_X is set, the surface can be shrunken
 * horizontally to fit.
 *
 * In general, when multiple flags are set, flipping should take precedence over
 * sliding, which should take precedence over resizing.
 */
typedef enum
{
  GDK_ANCHOR_FLIP_X   = 1 << 0,
  GDK_ANCHOR_FLIP_Y   = 1 << 1,
  GDK_ANCHOR_SLIDE_X  = 1 << 2,
  GDK_ANCHOR_SLIDE_Y  = 1 << 3,
  GDK_ANCHOR_RESIZE_X = 1 << 4,
  GDK_ANCHOR_RESIZE_Y = 1 << 5,
  GDK_ANCHOR_FLIP     = GDK_ANCHOR_FLIP_X | GDK_ANCHOR_FLIP_Y,
  GDK_ANCHOR_SLIDE    = GDK_ANCHOR_SLIDE_X | GDK_ANCHOR_SLIDE_Y,
  GDK_ANCHOR_RESIZE   = GDK_ANCHOR_RESIZE_X | GDK_ANCHOR_RESIZE_Y,
} GdkAnchorHints;

typedef struct _GdkPopupLayout GdkPopupLayout;

#define GDK_TYPE_POPUP_LAYOUT (gdk_popup_layout_get_type ())

GDK_AVAILABLE_IN_ALL
GType                   gdk_popup_layout_get_type               (void);

GDK_AVAILABLE_IN_ALL
GdkPopupLayout *        gdk_popup_layout_new                    (const GdkRectangle     *anchor_rect,
                                                                 GdkGravity              rect_anchor,
                                                                 GdkGravity              surface_anchor);

GDK_AVAILABLE_IN_ALL
GdkPopupLayout *        gdk_popup_layout_ref                    (GdkPopupLayout         *layout);

GDK_AVAILABLE_IN_ALL
void                    gdk_popup_layout_unref                  (GdkPopupLayout         *layout);

GDK_AVAILABLE_IN_ALL
GdkPopupLayout *        gdk_popup_layout_copy                   (GdkPopupLayout         *layout);

GDK_AVAILABLE_IN_ALL
gboolean                gdk_popup_layout_equal                  (GdkPopupLayout         *layout,
                                                                 GdkPopupLayout         *other);

GDK_AVAILABLE_IN_ALL
void                    gdk_popup_layout_set_anchor_rect        (GdkPopupLayout         *layout,
                                                                 const GdkRectangle     *anchor_rect);

GDK_AVAILABLE_IN_ALL
const GdkRectangle *    gdk_popup_layout_get_anchor_rect        (GdkPopupLayout         *layout);

GDK_AVAILABLE_IN_ALL
void                    gdk_popup_layout_set_rect_anchor        (GdkPopupLayout         *layout,
                                                                 GdkGravity              anchor);

GDK_AVAILABLE_IN_ALL
GdkGravity              gdk_popup_layout_get_rect_anchor        (GdkPopupLayout         *layout);

GDK_AVAILABLE_IN_ALL
void                    gdk_popup_layout_set_surface_anchor     (GdkPopupLayout         *layout,
                                                                 GdkGravity              anchor);

GDK_AVAILABLE_IN_ALL
GdkGravity              gdk_popup_layout_get_surface_anchor     (GdkPopupLayout         *layout);

GDK_AVAILABLE_IN_ALL
void                    gdk_popup_layout_set_anchor_hints       (GdkPopupLayout         *layout,
                                                                 GdkAnchorHints          anchor_hints);

GDK_AVAILABLE_IN_ALL
GdkAnchorHints          gdk_popup_layout_get_anchor_hints       (GdkPopupLayout         *layout);

GDK_AVAILABLE_IN_ALL
void                    gdk_popup_layout_set_offset             (GdkPopupLayout         *layout,
                                                                 int                     dx,
                                                                 int                     dy);

GDK_AVAILABLE_IN_ALL
void                    gdk_popup_layout_get_offset             (GdkPopupLayout         *layout,
                                                                 int                    *dx,
                                                                 int                    *dy);

GDK_AVAILABLE_IN_4_2
void                    gdk_popup_layout_set_shadow_width       (GdkPopupLayout     *layout,
                                                                 int                 left,
                                                                 int                 right,
                                                                 int                 top,
                                                                 int                 bottom);
GDK_AVAILABLE_IN_4_2
void                    gdk_popup_layout_get_shadow_width       (GdkPopupLayout     *layout,
                                                                 int                *left,
                                                                 int                *right,
                                                                 int                *top,
                                                                 int                *bottom);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkPopupLayout, gdk_popup_layout_unref)

G_END_DECLS

#endif /* __GDK_POPUP_LAYOUT_H__ */
