/*
 * Copyright © 2020 Red Hat, Inc.
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
 * Authors: Matthias Clasen <mclasen@redhat.com>
 */

#ifndef __GDK_TOPLEVEL_H__
#define __GDK_TOPLEVEL_H__

#if !defined(__GDK_H_INSIDE__) && !defined(GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkseat.h>
#include <gdk/gdksurface.h>
#include <gdk/gdktoplevellayout.h>

G_BEGIN_DECLS

/**
 * GdkSurfaceEdge:
 * @GDK_SURFACE_EDGE_NORTH_WEST: the top left corner.
 * @GDK_SURFACE_EDGE_NORTH: the top edge.
 * @GDK_SURFACE_EDGE_NORTH_EAST: the top right corner.
 * @GDK_SURFACE_EDGE_WEST: the left edge.
 * @GDK_SURFACE_EDGE_EAST: the right edge.
 * @GDK_SURFACE_EDGE_SOUTH_WEST: the lower left corner.
 * @GDK_SURFACE_EDGE_SOUTH: the lower edge.
 * @GDK_SURFACE_EDGE_SOUTH_EAST: the lower right corner.
 *
 * Determines a surface edge or corner.
 */
typedef enum
{
  GDK_SURFACE_EDGE_NORTH_WEST,
  GDK_SURFACE_EDGE_NORTH,
  GDK_SURFACE_EDGE_NORTH_EAST,
  GDK_SURFACE_EDGE_WEST,
  GDK_SURFACE_EDGE_EAST,
  GDK_SURFACE_EDGE_SOUTH_WEST,
  GDK_SURFACE_EDGE_SOUTH,
  GDK_SURFACE_EDGE_SOUTH_EAST
} GdkSurfaceEdge;

/**
 * GdkFullscreenMode:
 * @GDK_FULLSCREEN_ON_CURRENT_MONITOR: Fullscreen on current monitor only.
 * @GDK_FULLSCREEN_ON_ALL_MONITORS: Span across all monitors when fullscreen.
 *
 * Indicates which monitor a surface should span over when in fullscreen mode.
 */
typedef enum
{
  GDK_FULLSCREEN_ON_CURRENT_MONITOR,
  GDK_FULLSCREEN_ON_ALL_MONITORS
} GdkFullscreenMode;

/**
 * GdkToplevelState:
 * @GDK_TOPLEVEL_STATE_MINIMIZED: the surface is minimized
 * @GDK_TOPLEVEL_STATE_MAXIMIZED: the surface is maximized
 * @GDK_TOPLEVEL_STATE_STICKY: the surface is sticky
 * @GDK_TOPLEVEL_STATE_FULLSCREEN: the surface is maximized without decorations
 * @GDK_TOPLEVEL_STATE_ABOVE: the surface is kept above other surfaces
 * @GDK_TOPLEVEL_STATE_BELOW: the surface is kept below other surfaces
 * @GDK_TOPLEVEL_STATE_FOCUSED: the surface is presented as focused (with active decorations)
 * @GDK_TOPLEVEL_STATE_TILED: the surface is in a tiled state
 * @GDK_TOPLEVEL_STATE_TOP_TILED: whether the top edge is tiled
 * @GDK_TOPLEVEL_STATE_TOP_RESIZABLE: whether the top edge is resizable
 * @GDK_TOPLEVEL_STATE_RIGHT_TILED: whether the right edge is tiled
 * @GDK_TOPLEVEL_STATE_RIGHT_RESIZABLE: whether the right edge is resizable
 * @GDK_TOPLEVEL_STATE_BOTTOM_TILED: whether the bottom edge is tiled
 * @GDK_TOPLEVEL_STATE_BOTTOM_RESIZABLE: whether the bottom edge is resizable
 * @GDK_TOPLEVEL_STATE_LEFT_TILED: whether the left edge is tiled
 * @GDK_TOPLEVEL_STATE_LEFT_RESIZABLE: whether the left edge is resizable
 *
 * Specifies the state of a toplevel surface.
 *
 * On platforms that support information about individual edges, the
 * %GDK_TOPLEVEL_STATE_TILED state will be set whenever any of the individual
 * tiled states is set. On platforms that lack that support, the tiled state
 * will give an indication of tiledness without any of the per-edge states
 * being set.
 */
typedef enum
{
  GDK_TOPLEVEL_STATE_MINIMIZED        = 1 << 0,
  GDK_TOPLEVEL_STATE_MAXIMIZED        = 1 << 1,
  GDK_TOPLEVEL_STATE_STICKY           = 1 << 2,
  GDK_TOPLEVEL_STATE_FULLSCREEN       = 1 << 3,
  GDK_TOPLEVEL_STATE_ABOVE            = 1 << 4,
  GDK_TOPLEVEL_STATE_BELOW            = 1 << 5,
  GDK_TOPLEVEL_STATE_FOCUSED          = 1 << 6,
  GDK_TOPLEVEL_STATE_TILED            = 1 << 7,
  GDK_TOPLEVEL_STATE_TOP_TILED        = 1 << 8,
  GDK_TOPLEVEL_STATE_TOP_RESIZABLE    = 1 << 9,
  GDK_TOPLEVEL_STATE_RIGHT_TILED      = 1 << 10,
  GDK_TOPLEVEL_STATE_RIGHT_RESIZABLE  = 1 << 11,
  GDK_TOPLEVEL_STATE_BOTTOM_TILED     = 1 << 12,
  GDK_TOPLEVEL_STATE_BOTTOM_RESIZABLE = 1 << 13,
  GDK_TOPLEVEL_STATE_LEFT_TILED       = 1 << 14,
  GDK_TOPLEVEL_STATE_LEFT_RESIZABLE   = 1 << 15
} GdkToplevelState;

/**
 * GdkTitlebarGesture:
 * @GDK_TITLEBAR_GESTURE_DOUBLE_CLICK:
 * @GDK_TITLEBAR_GESTURE_RIGHT_CLICK:
 * @GDK_TITLEBAR_GESTURE_MIDDLE_CLICK:
 *
 * Since: 4.4
 */
typedef enum
{
  GDK_TITLEBAR_GESTURE_DOUBLE_CLICK   = 1,
  GDK_TITLEBAR_GESTURE_RIGHT_CLICK    = 2,
  GDK_TITLEBAR_GESTURE_MIDDLE_CLICK   = 3
} GdkTitlebarGesture;


#define GDK_TYPE_TOPLEVEL (gdk_toplevel_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_INTERFACE (GdkToplevel, gdk_toplevel, GDK, TOPLEVEL, GObject)

GDK_AVAILABLE_IN_ALL
void            gdk_toplevel_present            (GdkToplevel       *toplevel,
                                                 GdkToplevelLayout *layout);

GDK_AVAILABLE_IN_ALL
gboolean        gdk_toplevel_minimize           (GdkToplevel       *toplevel);

GDK_AVAILABLE_IN_ALL
gboolean        gdk_toplevel_lower              (GdkToplevel       *toplevel);

GDK_AVAILABLE_IN_ALL
void            gdk_toplevel_focus              (GdkToplevel       *toplevel,
                                                 guint32            timestamp);

GDK_AVAILABLE_IN_ALL
GdkToplevelState gdk_toplevel_get_state          (GdkToplevel       *toplevel);

GDK_AVAILABLE_IN_ALL
void            gdk_toplevel_set_title          (GdkToplevel       *toplevel,
                                                 const char        *title);

GDK_AVAILABLE_IN_ALL
void            gdk_toplevel_set_startup_id     (GdkToplevel       *toplevel,
                                                 const char        *startup_id);

GDK_AVAILABLE_IN_ALL
void            gdk_toplevel_set_transient_for  (GdkToplevel       *toplevel,
                                                 GdkSurface        *parent);

GDK_AVAILABLE_IN_ALL
void            gdk_toplevel_set_modal          (GdkToplevel       *toplevel,
                                                 gboolean           modal);

GDK_AVAILABLE_IN_ALL
void            gdk_toplevel_set_icon_list      (GdkToplevel       *toplevel,
                                                 GList             *surfaces);

GDK_AVAILABLE_IN_ALL
gboolean        gdk_toplevel_show_window_menu   (GdkToplevel       *toplevel,
                                                 GdkEvent          *event);

GDK_AVAILABLE_IN_ALL
void          gdk_toplevel_set_decorated         (GdkToplevel      *toplevel,
                                                  gboolean          decorated);

GDK_AVAILABLE_IN_ALL
void          gdk_toplevel_set_deletable         (GdkToplevel      *toplevel,
                                                  gboolean          deletable);
GDK_AVAILABLE_IN_ALL
gboolean      gdk_toplevel_supports_edge_constraints (GdkToplevel *toplevel);

GDK_AVAILABLE_IN_ALL
void          gdk_toplevel_inhibit_system_shortcuts  (GdkToplevel *toplevel,
                                                      GdkEvent    *event);

GDK_AVAILABLE_IN_ALL
void          gdk_toplevel_restore_system_shortcuts  (GdkToplevel *toplevel);

GDK_AVAILABLE_IN_ALL
void          gdk_toplevel_begin_resize              (GdkToplevel    *toplevel,
                                                      GdkSurfaceEdge  edge,
                                                      GdkDevice      *device,
                                                      int             button,
                                                      double          x,
                                                      double          y,
                                                      guint32         timestamp);

GDK_AVAILABLE_IN_ALL
void          gdk_toplevel_begin_move                (GdkToplevel    *toplevel,
                                                      GdkDevice      *device,
                                                      int             button,
                                                      double          x,
                                                      double          y,
                                                      guint32         timestamp);

GDK_AVAILABLE_IN_4_4
gboolean      gdk_toplevel_titlebar_gesture          (GdkToplevel        *toplevel,
                                                      GdkTitlebarGesture  gesture);

G_END_DECLS

#endif /* __GDK_TOPLEVEL_H__ */
