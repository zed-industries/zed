/*****************************************************************
Copyright (c) 1991, 1997 Digital Equipment Corporation, Maynard, Massachusetts.
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software.

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
DIGITAL EQUIPMENT CORPORATION BE LIABLE FOR ANY CLAIM, DAMAGES, INCLUDING,
BUT NOT LIMITED TO CONSEQUENTIAL OR INCIDENTAL DAMAGES, OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of Digital Equipment Corporation
shall not be used in advertising or otherwise to promote the sale, use or other
dealings in this Software without prior written authorization from Digital
Equipment Corporation.
******************************************************************/
/*
 *	PanoramiX definitions
 */

/* THIS IS NOT AN X PROJECT TEAM SPECIFICATION */

#ifndef _panoramiXext_h
#define _panoramiXext_h

#include <X11/Xfuncproto.h>

typedef struct {
    Window  window;         /* PanoramiX window - may not exist */
    int	    screen;
    int     State;          /* PanoramiXOff, PanoramiXOn */
    int	    width;	    /* width of this screen */
    int     height;	    /* height of this screen */
    int     ScreenCount;    /* real physical number of screens */
    XID     eventMask;      /* selected events for this client */
} XPanoramiXInfo;

_XFUNCPROTOBEGIN

extern Bool XPanoramiXQueryExtension (
    Display *		/* dpy */,
    int *		/* event_base_return */,
    int *		/* error_base_return */
);

extern Status XPanoramiXQueryVersion(
    Display *		/* dpy */,
    int *		/* major_version_return */,
    int *		/* minor_version_return */
);

extern XPanoramiXInfo *XPanoramiXAllocInfo (
    void
);

extern Status XPanoramiXGetState (
    Display *		/* dpy */,
    Drawable		/* drawable */,
    XPanoramiXInfo *	/* panoramiX_info */
);

extern Status XPanoramiXGetScreenCount (
    Display *		/* dpy */,
    Drawable		/* drawable */,
    XPanoramiXInfo *	/* panoramiX_info */
);

extern Status XPanoramiXGetScreenSize (
    Display *		/* dpy */,
    Drawable		/* drawable */,
    int			/* screen_num */,
    XPanoramiXInfo *	/* panoramiX_info */
);

_XFUNCPROTOEND

#endif /* _panoramiXext_h */
