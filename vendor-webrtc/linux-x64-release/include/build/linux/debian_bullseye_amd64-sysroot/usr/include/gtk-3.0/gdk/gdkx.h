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

#ifndef __GDK_X_H__
#define __GDK_X_H__

#include <gdk/gdk.h>

#include <X11/Xlib.h>
#include <X11/Xutil.h>

#define __GDKX_H_INSIDE__

#include <gdk/x11/gdkx11applaunchcontext.h>
#include <gdk/x11/gdkx11cursor.h>
#include <gdk/x11/gdkx11device.h>
#include <gdk/x11/gdkx11device-core.h>
#include <gdk/x11/gdkx11device-xi2.h>
#include <gdk/x11/gdkx11devicemanager.h>
#include <gdk/x11/gdkx11devicemanager-core.h>
#include <gdk/x11/gdkx11devicemanager-xi2.h>
#include <gdk/x11/gdkx11display.h>
#include <gdk/x11/gdkx11displaymanager.h>
#include <gdk/x11/gdkx11dnd.h>
#include <gdk/x11/gdkx11glcontext.h>
#include <gdk/x11/gdkx11keys.h>
#include <gdk/x11/gdkx11monitor.h>
#include <gdk/x11/gdkx11property.h>
#include <gdk/x11/gdkx11screen.h>
#include <gdk/x11/gdkx11selection.h>
#include <gdk/x11/gdkx11utils.h>
#include <gdk/x11/gdkx11visual.h>
#include <gdk/x11/gdkx11window.h>

#include <gdk/x11/gdkx-autocleanups.h>

#undef __GDKX_H_INSIDE__

#endif /* __GDK_X_H__ */
