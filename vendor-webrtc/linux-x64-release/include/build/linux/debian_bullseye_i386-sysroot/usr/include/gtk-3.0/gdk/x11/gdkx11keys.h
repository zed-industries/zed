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

#ifndef __GDK_X11_KEYS_H__
#define __GDK_X11_KEYS_H__

#if !defined (__GDKX_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdkx.h> can be included directly."
#endif

#include <gdk/gdk.h>

G_BEGIN_DECLS

#ifdef GDK_COMPILATION
typedef struct _GdkX11Keymap GdkX11Keymap;
#else
typedef GdkKeymap GdkX11Keymap;
#endif
typedef struct _GdkX11KeymapClass GdkX11KeymapClass;

#define GDK_TYPE_X11_KEYMAP              (gdk_x11_keymap_get_type())
#define GDK_X11_KEYMAP(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_X11_KEYMAP, GdkX11Keymap))
#define GDK_X11_KEYMAP_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_X11_KEYMAP, GdkX11KeymapClass))
#define GDK_IS_X11_KEYMAP(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_X11_KEYMAP))
#define GDK_IS_X11_KEYMAP_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_X11_KEYMAP))
#define GDK_X11_KEYMAP_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_X11_KEYMAP, GdkX11KeymapClass))

GDK_AVAILABLE_IN_ALL
GType gdk_x11_keymap_get_type (void);

GDK_AVAILABLE_IN_3_6
gint gdk_x11_keymap_get_group_for_state (GdkKeymap *keymap,
                                         guint      state);

GDK_AVAILABLE_IN_3_6
gboolean gdk_x11_keymap_key_is_modifier (GdkKeymap *keymap,
                                         guint      keycode);
G_END_DECLS

#endif /* __GDK_X11_KEYMAP_H__ */
