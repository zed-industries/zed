/* gdkbroadwayvisual.h
 *
 * Copyright (C) 2011  Alexander Larsson  <alexl@redhat.com>
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

#ifndef __GDK_BROADWAY_VISUAL_H__
#define __GDK_BROADWAY_VISUAL_H__

#include <gdk/gdk.h>

G_BEGIN_DECLS

#define GDK_TYPE_BROADWAY_VISUAL              (gdk_broadway_visual_get_type ())
#define GDK_BROADWAY_VISUAL(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_BROADWAY_VISUAL, GdkBroadwayVisual))
#define GDK_BROADWAY_VISUAL_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_BROADWAY_VISUAL, GdkBroadwayVisualClass))
#define GDK_IS_BROADWAY_VISUAL(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_BROADWAY_VISUAL))
#define GDK_IS_BROADWAY_VISUAL_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_BROADWAY_VISUAL))
#define GDK_BROADWAY_VISUAL_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_BROADWAY_VISUAL, GdkBroadwayVisualClass))

#ifdef GDK_COMPILATION
typedef struct _GdkBroadwayVisual GdkBroadwayVisual;
#else
typedef GdkVisual GdkBroadwayVisual;
#endif
typedef struct _GdkBroadwayVisualClass GdkBroadwayVisualClass;


GDK_AVAILABLE_IN_ALL
GType gdk_broadway_visual_get_type (void);

G_END_DECLS

#endif /* __GDK_BROADWAY_VISUAL_H__ */
