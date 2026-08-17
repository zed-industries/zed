/* GDK - The GIMP Drawing Kit
 * Copyright (C) 2000 Red Hat, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GDK_DISPLAY_MANAGER_H__
#define __GDK_DISPLAY_MANAGER_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>
#include <gdk/gdkdisplay.h>

G_BEGIN_DECLS

typedef struct _GdkDisplayManager      GdkDisplayManager;
typedef struct _GdkDisplayManagerClass GdkDisplayManagerClass;

#define GDK_TYPE_DISPLAY_MANAGER              (gdk_display_manager_get_type ())
#define GDK_DISPLAY_MANAGER(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_DISPLAY_MANAGER, GdkDisplayManager))
#define GDK_DISPLAY_MANAGER_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_DISPLAY_MANAGER, GdkDisplayManagerClass))
#define GDK_IS_DISPLAY_MANAGER(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_DISPLAY_MANAGER))
#define GDK_IS_DISPLAY_MANAGER_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_DISPLAY_MANAGER))
#define GDK_DISPLAY_MANAGER_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_DISPLAY_MANAGER, GdkDisplayManagerClass))

struct _GdkDisplayManagerClass
{
  GObjectClass parent_class;

  void (*display_opened) (GdkDisplayManager *display_manager,
			  GdkDisplay *display);
};

GType gdk_display_manager_get_type (void) G_GNUC_CONST;

GdkDisplayManager *gdk_display_manager_get                 (void);
GdkDisplay *       gdk_display_manager_get_default_display (GdkDisplayManager *display_manager);
void               gdk_display_manager_set_default_display (GdkDisplayManager *display_manager,
							    GdkDisplay        *display);
GSList     *       gdk_display_manager_list_displays       (GdkDisplayManager *display_manager);

G_END_DECLS

#endif /* __GDK_DISPLAY_MANAGER_H__ */
