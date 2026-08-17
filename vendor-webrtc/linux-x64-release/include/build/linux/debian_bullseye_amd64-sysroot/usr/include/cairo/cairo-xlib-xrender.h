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

#ifndef CAIRO_XLIB_XRENDER_H
#define CAIRO_XLIB_XRENDER_H

#include "cairo.h"

#if CAIRO_HAS_XLIB_XRENDER_SURFACE

#include <X11/Xlib.h>
#include <X11/extensions/Xrender.h>

CAIRO_BEGIN_DECLS

cairo_public cairo_surface_t *
cairo_xlib_surface_create_with_xrender_format (Display		 *dpy,
                                               Drawable		  drawable,
					       Screen		 *screen,
                                               XRenderPictFormat *format,
                                               int		  width,
                                               int		  height);

cairo_public XRenderPictFormat *
cairo_xlib_surface_get_xrender_format (cairo_surface_t *surface);

CAIRO_END_DECLS

#else  /* CAIRO_HAS_XLIB_XRENDER_SURFACE */
# error Cairo was not compiled with support for the xlib XRender backend
#endif /* CAIRO_HAS_XLIB_XRENDER_SURFACE */

#endif /* CAIRO_XLIB_XRENDER_H */
