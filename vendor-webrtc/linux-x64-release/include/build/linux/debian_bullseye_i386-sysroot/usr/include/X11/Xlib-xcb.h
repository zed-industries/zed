/* Copyright (C) 2003-2006 Jamey Sharp, Josh Triplett
 * This file is licensed under the MIT license. See the file COPYING. */

#ifndef _X11_XLIB_XCB_H_
#define _X11_XLIB_XCB_H_

#include <xcb/xcb.h>
#include <X11/Xlib.h>
#include <X11/Xfuncproto.h>

_XFUNCPROTOBEGIN

xcb_connection_t *XGetXCBConnection(Display *dpy);

enum XEventQueueOwner { XlibOwnsEventQueue = 0, XCBOwnsEventQueue };
void XSetEventQueueOwner(Display *dpy, enum XEventQueueOwner owner);

_XFUNCPROTOEND

#endif /* _X11_XLIB_XCB_H_ */
