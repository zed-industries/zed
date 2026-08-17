/* gdktexture.h
 *
 * Copyright 2016  Benjamin Otte
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

#ifndef __GDK_GL_TEXTURE_H__
#define __GDK_GL_TEXTURE_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkglcontext.h>
#include <gdk/gdktexture.h>

G_BEGIN_DECLS

#define GDK_TYPE_GL_TEXTURE (gdk_gl_texture_get_type ())

#define GDK_GL_TEXTURE(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GDK_TYPE_GL_TEXTURE, GdkGLTexture))
#define GDK_IS_GL_TEXTURE(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GDK_TYPE_GL_TEXTURE))

typedef struct _GdkGLTexture            GdkGLTexture;
typedef struct _GdkGLTextureClass       GdkGLTextureClass;

GDK_AVAILABLE_IN_ALL
GType                   gdk_gl_texture_get_type                (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GdkTexture *            gdk_gl_texture_new                     (GdkGLContext    *context,
                                                                guint            id,
                                                                int              width,
                                                                int              height,
                                                                GDestroyNotify   destroy,
                                                                gpointer         data);

GDK_AVAILABLE_IN_ALL
void                    gdk_gl_texture_release                 (GdkGLTexture    *self);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkGLTexture, g_object_unref)

G_END_DECLS

#endif /* __GDK_GL_TEXTURE_H__ */
