/******************************************************************************
 *
 * Copyright (c) 1994, 1995  Hewlett-Packard Company
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL HEWLETT-PACKARD COMPANY BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR
 * THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * Except as contained in this notice, the name of the Hewlett-Packard
 * Company shall not be used in advertising or otherwise to promote the
 * sale, use or other dealings in this Software without prior written
 * authorization from the Hewlett-Packard Company.
 *
 *     Header file for Xlib-related DBE
 *
 *****************************************************************************/

#ifndef XDBE_H
#define XDBE_H

#include <X11/Xfuncproto.h>
#include <X11/extensions/dbe.h>

typedef struct
{
    VisualID    visual;    /* one visual ID that supports double-buffering */
    int         depth;     /* depth of visual in bits                      */
    int         perflevel; /* performance level of visual                  */
}
XdbeVisualInfo;

typedef struct
{
    int                 count;          /* number of items in visual_depth   */
    XdbeVisualInfo      *visinfo;       /* list of visuals & depths for scrn */
}
XdbeScreenVisualInfo;


typedef Drawable XdbeBackBuffer;

typedef unsigned char XdbeSwapAction;

typedef struct
{
    Window		swap_window;    /* window for which to swap buffers   */
    XdbeSwapAction	swap_action;    /* swap action to use for swap_window */
}
XdbeSwapInfo;

typedef struct
{
    Window	window;			/* window that buffer belongs to */
}
XdbeBackBufferAttributes;

typedef struct
{
    int			type;
    Display		*display;	/* display the event was read from */
    XdbeBackBuffer	buffer;		/* resource id                     */
    unsigned long	serial;		/* serial number of failed request */
    unsigned char	error_code;	/* error base + XdbeBadBuffer      */
    unsigned char	request_code;	/* major opcode of failed request  */
    unsigned char	minor_code;	/* minor opcode of failed request  */
}
XdbeBufferError;

/* _XFUNCPROTOBEGIN and _XFUNCPROTOEND are defined as noops
 * (for non-C++ builds) in X11/Xfuncproto.h.
 */
_XFUNCPROTOBEGIN

extern Status XdbeQueryExtension(
    Display*		/* dpy                  */,
    int*		/* major_version_return */,
    int*		/* minor_version_return */
);

extern XdbeBackBuffer XdbeAllocateBackBufferName(
    Display*		/* dpy         */,
    Window		/* window      */,
    XdbeSwapAction	/* swap_action */
);

extern Status XdbeDeallocateBackBufferName(
    Display*		/* dpy    */,
    XdbeBackBuffer	/* buffer */
);

extern Status XdbeSwapBuffers(
    Display*		/* dpy         */,
    XdbeSwapInfo*	/* swap_info   */,
    int			/* num_windows */
);

extern Status XdbeBeginIdiom(
    Display*		/* dpy */
);

extern Status XdbeEndIdiom(
    Display*		/* dpy */
);

extern XdbeScreenVisualInfo *XdbeGetVisualInfo(
    Display*		/* dpy               */,
    Drawable*		/* screen_specifiers */,
    int*		/* num_screens       */
);

extern void XdbeFreeVisualInfo(
    XdbeScreenVisualInfo*	/* visual_info */
);

extern XdbeBackBufferAttributes *XdbeGetBackBufferAttributes(
    Display*		/* dpy    */,
    XdbeBackBuffer	/* buffer */
);

_XFUNCPROTOEND

#endif /* XDBE_H */

