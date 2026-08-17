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

#ifndef __GDK_MEMORY_TEXTURE__H__
#define __GDK_MEMORY_TEXTURE__H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkenums.h>
#include <gdk/gdktexture.h>

G_BEGIN_DECLS

/**
 * GDK_MEMORY_DEFAULT:
 *
 * The default memory format used by GTK.
 *
 * This is the format provided by [method@Gdk.Texture.download].
 * It is equal to %CAIRO_FORMAT_ARGB32.
 *
 * Be aware that unlike the `GdkMemoryFormat` values, this format
 * is different for different endianness.
 */
#if G_BYTE_ORDER == G_LITTLE_ENDIAN
#define GDK_MEMORY_DEFAULT GDK_MEMORY_B8G8R8A8_PREMULTIPLIED
#elif G_BYTE_ORDER == G_BIG_ENDIAN
#define GDK_MEMORY_DEFAULT GDK_MEMORY_A8R8G8B8_PREMULTIPLIED
#else
#error "Unknown byte order for GDK_MEMORY_DEFAULT"
#endif

#define GDK_TYPE_MEMORY_TEXTURE (gdk_memory_texture_get_type ())

#define GDK_MEMORY_TEXTURE(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GDK_TYPE_MEMORY_TEXTURE, GdkMemoryTexture))
#define GDK_IS_MEMORY_TEXTURE(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GDK_TYPE_MEMORY_TEXTURE))

typedef struct _GdkMemoryTexture        GdkMemoryTexture;
typedef struct _GdkMemoryTextureClass   GdkMemoryTextureClass;

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkMemoryTexture, g_object_unref)


GDK_AVAILABLE_IN_ALL
GType                   gdk_memory_texture_get_type         (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GdkTexture *            gdk_memory_texture_new              (int                width,
                                                             int                height,
                                                             GdkMemoryFormat    format,
                                                             GBytes            *bytes,
                                                             gsize              stride);


G_END_DECLS

#endif /* __GDK_MEMORY_TEXTURE_H__ */
