/* cairo - a vector graphics library with display and print output
 *
 * Copyright © 2002 University of Southern California
 *
 * This library is free software; you can redistribute it and/or
 * modify it either under the terms of the GNU Lesser General Public
 * License version 2.1 as published by the Free Software Foundation
 * (the "LGPL") or, at your option, under the terms of the Mozilla
 * Public License Version 1.1 (the "MPL"). If you do not alter this
 * notice, a recipient may use your version of this file under either
 * the MPL or the LGPL.
 *
 * You should have received a copy of the LGPL along with this library
 * in the file COPYING-LGPL-2.1; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Suite 500, Boston, MA 02110-1335, USA
 * You should have received a copy of the MPL along with this library
 * in the file COPYING-MPL-1.1
 *
 * The contents of this file are subject to the Mozilla Public License
 * Version 1.1 (the "License"); you may not use this file except in
 * compliance with the License. You may obtain a copy of the License at
 * http://www.mozilla.org/MPL/
 *
 * This software is distributed on an "AS IS" basis, WITHOUT WARRANTY
 * OF ANY KIND, either express or implied. See the LGPL or the MPL for
 * the specific language governing rights and limitations.
 *
 * The Original Code is the cairo graphics library.
 *
 * The Initial Developer of the Original Code is University of Southern
 * California.
 *
 * Contributor(s):
 *	Carl D. Worth <cworth@cworth.org>
 */

#ifndef CAIRO_XLIB_H
#define CAIRO_XLIB_H

#include "cairo.h"

#if CAIRO_HAS_XLIB_SURFACE

#include <X11/Xlib.h>

CAIRO_BEGIN_DECLS

cairo_public cairo_surface_t *
cairo_xlib_surface_create (Display     *dpy,
			   Drawable	drawable,
			   Visual      *visual,
			   int		width,
			   int		height);

cairo_public cairo_surface_t *
cairo_xlib_surface_create_for_bitmap (Display  *dpy,
				      Pixmap	bitmap,
				      Screen	*screen,
				      int	width,
				      int	height);

cairo_public void
cairo_xlib_surface_set_size (cairo_surface_t *surface,
			     int              width,
			     int              height);

cairo_public void
cairo_xlib_surface_set_drawable (cairo_surface_t *surface,
				 Drawable	  drawable,
				 int              width,
				 int              height);

cairo_public Display *
cairo_xlib_surface_get_display (cairo_surface_t *surface);

cairo_public Drawable
cairo_xlib_surface_get_drawable (cairo_surface_t *surface);

cairo_public Screen *
cairo_xlib_surface_get_screen (cairo_surface_t *surface);

cairo_public Visual *
cairo_xlib_surface_get_visual (cairo_surface_t *surface);

cairo_public int
cairo_xlib_surface_get_depth (cairo_surface_t *surface);

cairo_public int
cairo_xlib_surface_get_width (cairo_surface_t *surface);

cairo_public int
cairo_xlib_surface_get_height (cairo_surface_t *surface);

/* debug interface */

cairo_public void
cairo_xlib_device_debug_cap_xrender_version (cairo_device_t *device,
					     int major_version,
					     int minor_version);

/*
 * @precision: -1 implies automatically choose based on antialiasing mode,
 *            any other value overrides and sets the corresponding PolyMode.
 */
cairo_public void
cairo_xlib_device_debug_set_precision (cairo_device_t *device,
				       int precision);

cairo_public int
cairo_xlib_device_debug_get_precision (cairo_device_t *device);

CAIRO_END_DECLS

#else  /* CAIRO_HAS_XLIB_SURFACE */
# error Cairo was not compiled with support for the xlib backend
#endif /* CAIRO_HAS_XLIB_SURFACE */

#endif /* CAIRO_XLIB_H */
