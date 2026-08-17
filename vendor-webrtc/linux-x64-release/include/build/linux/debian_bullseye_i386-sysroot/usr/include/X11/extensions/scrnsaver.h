/*
 *
Copyright (c) 1992  X Consortium

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
X CONSORTIUM BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of the X Consortium shall not be
used in advertising or otherwise to promote the sale, use or other dealings
in this Software without prior written authorization from the X Consortium.
 *
 * Author:  Keith Packard, MIT X Consortium
 */

#ifndef _SCRNSAVER_H_
#define _SCRNSAVER_H_

#include <X11/Xfuncproto.h>
#include <X11/Xlib.h>
#include <X11/extensions/saver.h>

typedef struct {
    int	type;		    /* of event */
    unsigned long serial;   /* # of last request processed by server */
    Bool send_event;	    /* true if this came frome a SendEvent request */
    Display *display;	    /* Display the event was read from */
    Window window;	    /* screen saver window */
    Window root;	    /* root window of event screen */
    int state;		    /* ScreenSaverOff, ScreenSaverOn, ScreenSaverCycle*/
    int kind;		    /* ScreenSaverBlanked, ...Internal, ...External */
    Bool forced;	    /* extents of new region */
    Time time;		    /* event timestamp */
} XScreenSaverNotifyEvent;

typedef struct {
    Window  window;	    /* screen saver window - may not exist */
    int	    state;	    /* ScreenSaverOff, ScreenSaverOn, ScreenSaverDisabled*/
    int	    kind;	    /* ScreenSaverBlanked, ...Internal, ...External */
    unsigned long    til_or_since;   /* time til or since screen saver */
    unsigned long    idle;	    /* total time since last user input */
    unsigned long   eventMask; /* currently selected events for this client */
} XScreenSaverInfo;

_XFUNCPROTOBEGIN

extern Bool XScreenSaverQueryExtension (
    Display*	/* display */,
    int*	/* event_base */,
    int*	/* error_base */
);

extern Status XScreenSaverQueryVersion (
    Display*	/* display */,
    int*	/* major_version */,
    int*	/* minor_version */
);

extern XScreenSaverInfo *XScreenSaverAllocInfo (
    void
);

extern Status XScreenSaverQueryInfo (
    Display*		/* display */,
    Drawable		/* drawable */,
    XScreenSaverInfo*	/* info */
);

extern void XScreenSaverSelectInput (
    Display*	/* display */,
    Drawable	/* drawable */,
    unsigned long   /* eventMask */
);

extern void XScreenSaverSetAttributes (
    Display*		    /* display */,
    Drawable		    /* drawable */,
    int			    /* x */,
    int			    /* y */,
    unsigned int	    /* width */,
    unsigned int	    /* height */,
    unsigned int	    /* border_width */,
    int			    /* depth */,
    unsigned int	    /* class */,
    Visual *		    /* visual */,
    unsigned long	    /* valuemask */,
    XSetWindowAttributes *  /* attributes */
);

extern void XScreenSaverUnsetAttributes (
    Display*	/* display */,
    Drawable	/* drawable */
);

extern Status XScreenSaverRegister (
    Display*	/* display */,
    int		/* screen */,
    XID		/* xid */,
    Atom	/* type */
);

extern Status XScreenSaverUnregister (
    Display*	/* display */,
    int		/* screen */
);

extern Status XScreenSaverGetRegistered (
    Display*	/* display */,
    int		/* screen */,
    XID*	/* xid */,
    Atom*	/* type */
);

extern void XScreenSaverSuspend (
    Display*	/* display */,
    Bool 	/* suspend */
);

_XFUNCPROTOEND

#endif /* _SCRNSAVER_H_ */
