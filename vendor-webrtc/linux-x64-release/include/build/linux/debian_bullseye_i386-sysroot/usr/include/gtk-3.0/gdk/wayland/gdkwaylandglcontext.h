/* GDK - The GIMP Drawing Kit
 *
 * gdkglcontext-wayland.c: Wayland specific OpenGL wrappers
 *
 * Copyright © 2014  Emmanuele Bassi
 * Copyright © 2014  Red Hat, Inc
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GDK_WAYLAND_GL_CONTEXT_H__
#define __GDK_WAYLAND_GL_CONTEXT_H__

#if !defined (__GDKWAYLAND_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdkwayland.h> can be included directly."
#endif

#include <gdk/gdk.h>

G_BEGIN_DECLS

#define GDK_TYPE_WAYLAND_GL_CONTEXT		(gdk_wayland_gl_context_get_type ())
#define GDK_WAYLAND_GL_CONTEXT(obj)		(G_TYPE_CHECK_INSTANCE_CAST ((obj), GDK_TYPE_WAYLAND_GL_CONTEXT, GdkWaylandGLContext))
#define GDK_WAYLAND_IS_GL_CONTEXT(obj)	(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GDK_TYPE_WAYLAND_GL_CONTEXT))

typedef struct _GdkWaylandGLContext		GdkWaylandGLContext;
typedef struct _GdkWaylandGLContextClass	GdkWaylandGLContextClass;

GDK_AVAILABLE_IN_3_16
GType gdk_wayland_gl_context_get_type (void) G_GNUC_CONST;

G_END_DECLS

#endif /* __GDK_WAYLAND_GL_CONTEXT_H__ */
