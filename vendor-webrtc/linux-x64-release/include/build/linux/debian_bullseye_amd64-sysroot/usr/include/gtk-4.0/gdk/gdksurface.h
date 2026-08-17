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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GDK_SURFACE_H__
#define __GDK_SURFACE_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>
#include <gdk/gdkevents.h>
#include <gdk/gdkframeclock.h>
#include <gdk/gdkmonitor.h>
#include <gdk/gdkpopuplayout.h>

G_BEGIN_DECLS

typedef struct _GdkSurfaceClass GdkSurfaceClass;

#define GDK_TYPE_SURFACE              (gdk_surface_get_type ())
#define GDK_SURFACE(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_SURFACE, GdkSurface))
#define GDK_SURFACE_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_SURFACE, GdkSurfaceClass))
#define GDK_IS_SURFACE(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_SURFACE))
#define GDK_IS_SURFACE_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_SURFACE))
#define GDK_SURFACE_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_SURFACE, GdkSurfaceClass))


GDK_AVAILABLE_IN_ALL
GType         gdk_surface_get_type              (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GdkSurface *   gdk_surface_new_toplevel         (GdkDisplay    *display);
GDK_AVAILABLE_IN_ALL
GdkSurface *   gdk_surface_new_popup            (GdkSurface    *parent,
                                                 gboolean       autohide);

GDK_AVAILABLE_IN_ALL
void          gdk_surface_destroy               (GdkSurface     *surface);
GDK_AVAILABLE_IN_ALL
gboolean      gdk_surface_is_destroyed          (GdkSurface     *surface);

GDK_AVAILABLE_IN_ALL
GdkDisplay *  gdk_surface_get_display           (GdkSurface     *surface);
GDK_AVAILABLE_IN_ALL
void          gdk_surface_hide                  (GdkSurface     *surface);

GDK_AVAILABLE_IN_ALL
void          gdk_surface_set_input_region      (GdkSurface     *surface,
                                                 cairo_region_t *region);

GDK_AVAILABLE_IN_ALL
gboolean      gdk_surface_get_mapped   (GdkSurface *surface);

GDK_AVAILABLE_IN_ALL
void          gdk_surface_set_cursor     (GdkSurface      *surface,
                                          GdkCursor       *cursor);
GDK_AVAILABLE_IN_ALL
GdkCursor    *gdk_surface_get_cursor      (GdkSurface       *surface);
GDK_AVAILABLE_IN_ALL
void          gdk_surface_set_device_cursor (GdkSurface   *surface,
                                             GdkDevice     *device,
                                             GdkCursor     *cursor);
GDK_AVAILABLE_IN_ALL
GdkCursor    *gdk_surface_get_device_cursor (GdkSurface     *surface,
                                             GdkDevice     *device);
GDK_AVAILABLE_IN_ALL
int           gdk_surface_get_width       (GdkSurface       *surface);
GDK_AVAILABLE_IN_ALL
int           gdk_surface_get_height      (GdkSurface       *surface);
GDK_AVAILABLE_IN_ALL
gboolean gdk_surface_translate_coordinates (GdkSurface *from,
                                            GdkSurface *to,
                                            double     *x,
                                            double     *y);

GDK_AVAILABLE_IN_ALL
int           gdk_surface_get_scale_factor  (GdkSurface     *surface);

GDK_AVAILABLE_IN_ALL
gboolean      gdk_surface_get_device_position (GdkSurface      *surface,
                                               GdkDevice       *device,
                                               double          *x,
                                               double          *y,
                                               GdkModifierType *mask);

GDK_AVAILABLE_IN_ALL
cairo_surface_t *
              gdk_surface_create_similar_surface (GdkSurface *surface,
                                                  cairo_content_t  content,
                                                  int              width,
                                                  int              height);

GDK_AVAILABLE_IN_ALL
void          gdk_surface_beep            (GdkSurface       *surface);

GDK_AVAILABLE_IN_ALL
void       gdk_surface_queue_render       (GdkSurface       *surface);

GDK_AVAILABLE_IN_ALL
void       gdk_surface_request_layout     (GdkSurface       *surface);

GDK_AVAILABLE_IN_ALL
GdkFrameClock* gdk_surface_get_frame_clock      (GdkSurface     *surface);

GDK_AVAILABLE_IN_ALL
void       gdk_surface_set_opaque_region        (GdkSurface      *surface,
                                                 cairo_region_t *region);

GDK_AVAILABLE_IN_ALL
GdkCairoContext *gdk_surface_create_cairo_context(GdkSurface    *surface);
GDK_AVAILABLE_IN_ALL
GdkGLContext * gdk_surface_create_gl_context    (GdkSurface     *surface,
                                                 GError        **error);
GDK_AVAILABLE_IN_ALL
GdkVulkanContext *
               gdk_surface_create_vulkan_context(GdkSurface     *surface,
                                                 GError        **error);

G_DEFINE_AUTOPTR_CLEANUP_FUNC (GdkSurface, g_object_unref)

G_END_DECLS

#endif /* __GDK_SURFACE_H__ */
