/*
 * Copyright © 2013 Keith Packard
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that copyright
 * notice and this permission notice appear in supporting documentation, and
 * that the name of the copyright holders not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  The copyright holders make no representations
 * about the suitability of this software for any purpose.  It is provided "as
 * is" without express or implied warranty.
 *
 * THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

#ifndef _XSHMFENCE_H_
#define _XSHMFENCE_H_

#include <X11/Xfuncproto.h>

#define HAVE_STRUCT_XSHMFENCE   1

struct xshmfence;

_X_EXPORT int 
xshmfence_trigger(struct xshmfence *f);

_X_EXPORT int
xshmfence_await(struct xshmfence *f);

_X_EXPORT int
xshmfence_query(struct xshmfence *f);

_X_EXPORT void
xshmfence_reset(struct xshmfence *f);

_X_EXPORT int
xshmfence_alloc_shm(void);

_X_EXPORT struct xshmfence *
xshmfence_map_shm(int fd);

_X_EXPORT void
xshmfence_unmap_shm(struct xshmfence *f);

#endif /* _XSHMFENCE_H_ */
