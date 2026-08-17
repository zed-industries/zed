/*

Copyright 1992, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall not be
used in advertising or otherwise to promote the sale, use or other dealings
in this Software without prior written authorization from The Open Group.

*/

#ifndef _XTEST_H_
#define _XTEST_H_

#include <X11/Xfuncproto.h>
#include <X11/extensions/xtestconst.h>
#include <X11/extensions/XInput.h>

_XFUNCPROTOBEGIN

Bool XTestQueryExtension(
    Display*		/* dpy */,
    int*		/* event_basep */,
    int*		/* error_basep */,
    int*		/* majorp */,
    int*		/* minorp */
);

Bool XTestCompareCursorWithWindow(
    Display*		/* dpy */,
    Window		/* window */,
    Cursor		/* cursor */
);

Bool XTestCompareCurrentCursorWithWindow(
    Display*		/* dpy */,
    Window		/* window */
);

extern int XTestFakeKeyEvent(
    Display*		/* dpy */,
    unsigned int	/* keycode */,
    Bool		/* is_press */,
    unsigned long	/* delay */
);

extern int XTestFakeButtonEvent(
    Display*		/* dpy */,
    unsigned int	/* button */,
    Bool		/* is_press */,
    unsigned long	/* delay */
);

extern int XTestFakeMotionEvent(
    Display*		/* dpy */,
    int			/* screen */,
    int			/* x */,
    int			/* y */,
    unsigned long	/* delay */
);

extern int XTestFakeRelativeMotionEvent(
    Display*		/* dpy */,
    int			/* x */,
    int			/* y */,
    unsigned long	/* delay */
);

extern int XTestFakeDeviceKeyEvent(
    Display*		/* dpy */,
    XDevice*		/* dev */,
    unsigned int	/* keycode */,
    Bool		/* is_press */,
    int*		/* axes */,
    int			/* n_axes */,
    unsigned long	/* delay */
);

extern int XTestFakeDeviceButtonEvent(
    Display*		/* dpy */,
    XDevice*		/* dev */,
    unsigned int	/* button */,
    Bool		/* is_press */,
    int*		/* axes */,
    int			/* n_axes */,
    unsigned long	/* delay */
);

extern int XTestFakeProximityEvent(
    Display*		/* dpy */,
    XDevice*		/* dev */,
    Bool		/* in_prox */,
    int*		/* axes */,
    int			/* n_axes */,
    unsigned long	/* delay */
);

extern int XTestFakeDeviceMotionEvent(
    Display*		/* dpy */,
    XDevice*		/* dev */,
    Bool		/* is_relative */,
    int			/* first_axis */,
    int*		/* axes */,
    int			/* n_axes */,
    unsigned long	/* delay */
);

extern int XTestGrabControl(
    Display*		/* dpy */,
    Bool		/* impervious */
);

void XTestSetGContextOfGC(
    GC			/* gc */,
    GContext		/* gid */
);

void XTestSetVisualIDOfVisual(
    Visual*		/* visual */,
    VisualID		/* visualid */
);

Status XTestDiscard(
    Display*		/* dpy */
);

_XFUNCPROTOEND

#endif
