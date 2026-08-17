/*
Copyright 1996, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included
in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall
not be used in advertising or otherwise to promote the sale, use or
other dealings in this Software without prior written authorization
from The Open Group.
*/

#ifndef _XAG_H_
#define _XAG_H_

#include <X11/extensions/ag.h>
#include <X11/Xfuncproto.h>

#include <stdarg.h>

_XFUNCPROTOBEGIN

typedef XID XAppGroup;

Bool XagQueryVersion(
    Display*			/* dpy */,
    int*			/* major_version */,
    int*			/* minor_version */
);

Status XagCreateEmbeddedApplicationGroup(
    Display*			/* dpy */,
    VisualID			/* root_visual */,
    Colormap			/* default_colormap */,
    unsigned long		/* black_pixel */,
    unsigned long		/* white_pixel */,
    XAppGroup*			/* app_group_return */
);

Status XagCreateNonembeddedApplicationGroup(
    Display*			/* dpy */,
    XAppGroup*			/* app_group_return */
);

Status XagDestroyApplicationGroup(
    Display*			/* dpy */,
    XAppGroup			/* app_group */
);

Status XagGetApplicationGroupAttributes(
    Display*			/* dpy */,
    XAppGroup			/* app_group */,
    ...
);

Status XagQueryApplicationGroup(
    Display*			/* dpy */,
    XID				/* resource_base */,
    XAppGroup*			/* app_group_ret */
);

Status XagCreateAssociation(
    Display*			/* dpy */,
    Window*			/* window_ret */,
    void*			/* system_window */
);

Status XagDestroyAssociation(
    Display*			/* dpy */,
    Window			/* window */
);

_XFUNCPROTOEND

#endif /* _XAG_H_ */

