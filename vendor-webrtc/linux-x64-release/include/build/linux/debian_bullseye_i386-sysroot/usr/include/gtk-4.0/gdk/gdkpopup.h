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

#ifndef __GDK_POPUP_H__
#define __GDK_POPUP_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkpopuplayout.h>
#include <gdk/gdksurface.h>

G_BEGIN_DECLS

#define GDK_TYPE_POPUP (gdk_popup_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_INTERFACE (GdkPopup, gdk_popup, GDK, POPUP, GObject)

GDK_AVAILABLE_IN_ALL
gboolean        gdk_popup_present               (GdkPopup       *popup,
                                                 int             width,
                                                 int             height,
                                                 GdkPopupLayout *layout);

GDK_AVAILABLE_IN_ALL
GdkGravity      gdk_popup_get_surface_anchor    (GdkPopup       *popup);

GDK_AVAILABLE_IN_ALL
GdkGravity      gdk_popup_get_rect_anchor       (GdkPopup       *popup);

GDK_AVAILABLE_IN_ALL
GdkSurface *    gdk_popup_get_parent            (GdkPopup       *popup);

GDK_AVAILABLE_IN_ALL
int             gdk_popup_get_position_x        (GdkPopup       *popup);

GDK_AVAILABLE_IN_ALL
int             gdk_popup_get_position_y        (GdkPopup       *popup);

GDK_AVAILABLE_IN_ALL
gboolean        gdk_popup_get_autohide          (GdkPopup       *popup);

G_END_DECLS

#endif /* __GDK_POPUP_H__ */
