/* GDK - The GIMP Drawing Kit
 *
 * gdkglcontext-x11.c: X11 specific OpenGL wrappers
 * 
 * Copyright © 2014  Emmanuele Bassi
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

#ifndef __GDK_X11_GL_CONTEXT_H__
#define __GDK_X11_GL_CONTEXT_H__

#if !defined (__GDKX_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdkx.h> can be included directly."
#endif

#include <gdk/gdk.h>

G_BEGIN_DECLS

#define GDK_TYPE_X11_GL_CONTEXT		(gdk_x11_gl_context_get_type ())
#define GDK_X11_GL_CONTEXT(obj)		(G_TYPE_CHECK_INSTANCE_CAST ((obj), GDK_TYPE_X11_GL_CONTEXT, GdkX11GLContext))
#define GDK_X11_IS_GL_CONTEXT(obj)	(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GDK_TYPE_X11_GL_CONTEXT))

typedef struct _GdkX11GLContext		GdkX11GLContext;
typedef struct _GdkX11GLContextClass	GdkX11GLContextClass;

GDK_AVAILABLE_IN_3_16
GType gdk_x11_gl_context_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_16
gboolean        gdk_x11_display_get_glx_version (GdkDisplay *display,
                                                 gint       *major,
                                                 gint       *minor);

G_END_DECLS

#endif /* __GDK_X11_GL_CONTEXT_H__ */
