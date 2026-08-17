/* GDK - The GIMP Drawing Kit
 * Copyright (C) 2010 Red Hat, Inc.
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

#ifndef __GDK_X11_DND_H__
#define __GDK_X11_DND_H__

#if !defined (__GDKX_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdkx.h> can be included directly."
#endif

#include <gdk/gdk.h>

G_BEGIN_DECLS

#define GDK_TYPE_X11_DRAG_CONTEXT              (gdk_x11_drag_context_get_type ())
#define GDK_X11_DRAG_CONTEXT(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_X11_DRAG_CONTEXT, GdkX11DragContext))
#define GDK_X11_DRAG_CONTEXT_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_X11_DRAG_CONTEXT, GdkX11DragContextClass))
#define GDK_IS_X11_DRAG_CONTEXT(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_X11_DRAG_CONTEXT))
#define GDK_IS_X11_DRAG_CONTEXT_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_X11_DRAG_CONTEXT))
#define GDK_X11_DRAG_CONTEXT_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_X11_DRAG_CONTEXT, GdkX11DragContextClass))

#ifdef GDK_COMPILATION
typedef struct _GdkX11DragContext GdkX11DragContext;
#else
typedef GdkDragContext GdkX11DragContext;
#endif
typedef struct _GdkX11DragContextClass GdkX11DragContextClass;

GDK_AVAILABLE_IN_ALL
GType    gdk_x11_drag_context_get_type (void);

G_END_DECLS

#endif /* __GDK_X11_DRAG_CONTEXT_H__ */
