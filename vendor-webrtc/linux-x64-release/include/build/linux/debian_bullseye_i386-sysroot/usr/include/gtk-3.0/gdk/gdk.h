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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
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

#ifndef __GDK_H__
#define __GDK_H__

#define __GDK_H_INSIDE__

#include <gdk/gdkconfig.h>
#include <gdk/gdkversionmacros.h>
#include <gdk/gdkapplaunchcontext.h>
#include <gdk/gdkcairo.h>
#include <gdk/gdkcursor.h>
#include <gdk/gdkdevice.h>
#include <gdk/gdkdevicepad.h>
#include <gdk/gdkdevicetool.h>
#include <gdk/gdkdevicemanager.h>
#include <gdk/gdkdisplay.h>
#include <gdk/gdkdisplaymanager.h>
#include <gdk/gdkdnd.h>
#include <gdk/gdkdrawingcontext.h>
#include <gdk/gdkenumtypes.h>
#include <gdk/gdkevents.h>
#include <gdk/gdkframeclock.h>
#include <gdk/gdkframetimings.h>
#include <gdk/gdkglcontext.h>
#include <gdk/gdkkeys.h>
#include <gdk/gdkkeysyms.h>
#include <gdk/gdkmain.h>
#include <gdk/gdkmonitor.h>
#include <gdk/gdkpango.h>
#include <gdk/gdkpixbuf.h>
#include <gdk/gdkproperty.h>
#include <gdk/gdkrectangle.h>
#include <gdk/gdkrgba.h>
#include <gdk/gdkscreen.h>
#include <gdk/gdkseat.h>
#include <gdk/gdkselection.h>
#include <gdk/gdktestutils.h>
#include <gdk/gdkthreads.h>
#include <gdk/gdktypes.h>
#include <gdk/gdkvisual.h>
#include <gdk/gdkwindow.h>

#ifndef GDK_DISABLE_DEPRECATED
#include <gdk/deprecated/gdkcolor.h>
#endif

#include <gdk/gdk-autocleanup.h>

#undef __GDK_H_INSIDE__

#endif /* __GDK_H__ */
