/*
 *
Copyright 1989, 1998  The Open Group

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

#ifndef _MULTIBUF_H_
#define _MULTIBUF_H_

#include <X11/Xfuncproto.h>

#include <X11/extensions/multibufconst.h>

#define MbufGetReq(name,req,info) GetReq (name, req); \
	req->reqType = info->codes->major_opcode; \
	req->mbufReqType = X_##name;

/*
 * Extra definitions that will only be needed in the client
 */
typedef XID Multibuffer;

typedef struct {
    int	type;		    /* of event */
    unsigned long serial;   /* # of last request processed by server */
    int send_event;	    /* true if this came frome a SendEvent request */
    Display *display;	    /* Display the event was read from */
    Multibuffer buffer;	    /* buffer of event */
    int	state;		    /* see Clobbered constants above */
} XmbufClobberNotifyEvent;

typedef struct {
    int	type;		    /* of event */
    unsigned long serial;   /* # of last request processed by server */
    int send_event;	    /* true if this came frome a SendEvent request */
    Display *display;	    /* Display the event was read from */
    Multibuffer buffer;	    /* buffer of event */
} XmbufUpdateNotifyEvent;


/*
 * per-window attributes that can be got
 */
typedef struct {
    int displayed_index;	/* which buffer is being displayed */
    int update_action;		/* Undefined, Background, Untouched, Copied */
    int update_hint;		/* Frequent, Intermittent, Static */
    int window_mode;		/* Mono, Stereo */
    int nbuffers;		/* Number of buffers */
    Multibuffer *buffers;	/* Buffers */
} XmbufWindowAttributes;

/*
 * per-window attributes that can be set
 */
typedef struct {
    int update_hint;		/* Frequent, Intermittent, Static */
} XmbufSetWindowAttributes;


/*
 * per-buffer attributes that can be got
 */
typedef struct {
    Window window;		/* which window this belongs to */
    unsigned long event_mask;	/* events that have been selected */
    int buffer_index;		/* which buffer is this */
    int side;			/* Mono, Left, Right */
} XmbufBufferAttributes;

/*
 * per-buffer attributes that can be set
 */
typedef struct {
    unsigned long event_mask;	/* events that have been selected */
} XmbufSetBufferAttributes;


/*
 * per-screen buffer info (there will be lists of them)
 */
typedef struct {
    VisualID visualid;		/* visual usuable at this depth */
    int max_buffers;		/* most buffers for this visual */
    int depth;			/* depth of buffers to be created */
} XmbufBufferInfo;

_XFUNCPROTOBEGIN

extern Bool XmbufQueryExtension(
    Display*		/* dpy */,
    int*		/* event_base_return */,
    int*		/* error_base_return */
);

extern Status XmbufGetVersion(
    Display*		/* dpy */,
    int*		/* major_version_return */,
    int*		/* minor_version_return */
);

extern int XmbufCreateBuffers(
    Display*		/* dpy */,
    Window		/* w */,
    int			/* count */,
    int			/* update_action */,
    int			/* update_hint */,
    Multibuffer*	/* buffers */
);

extern void XmbufDestroyBuffers(
    Display*		/* dpy */,
    Window		/* window */
);

extern void XmbufDisplayBuffers(
    Display*		/* dpy */,
    int			/* count */,
    Multibuffer*	/* buffers */,
    int			/* min_delay */,
    int			/* max_delay */
);

extern Status XmbufGetWindowAttributes(
    Display*			/* dpy */,
    Window			/* w */,
    XmbufWindowAttributes*	/* attr */
);

extern void XmbufChangeWindowAttributes(
    Display*			/* dpy */,
    Window			/* w */,
    unsigned long		/* valuemask */,
    XmbufSetWindowAttributes*	/* attr */
);

extern Status XmbufGetBufferAttributes(
    Display*			/* dpy */,
    Multibuffer			/* b */,
    XmbufBufferAttributes*	/* attr */
);

extern void XmbufChangeBufferAttributes(
    Display*			/* dpy */,
    Multibuffer			/* b */,
    unsigned long		/* valuemask */,
    XmbufSetBufferAttributes*	/* attr */
);

extern Status XmbufGetScreenInfo(
    Display*			/* dpy */,
    Drawable			/* d */,
    int*			/* nmono_return */,
    XmbufBufferInfo**		/* mono_info_return */,
    int*			/* nstereo_return */,
    XmbufBufferInfo**		/* stereo_info_return */
);

extern Window XmbufCreateStereoWindow(
    Display*			/* dpy */,
    Window			/* parent */,
    int				/* x */,
    int				/* y */,
    unsigned int		/* width */,
    unsigned int		/* height */,
    unsigned int		/* border_width */,
    int				/* depth */,
    unsigned int		/* class */,
    Visual*			/* visual */,
    unsigned long		/* valuemask */,
    XSetWindowAttributes*	/* attr */,
    Multibuffer*		/* leftp */,
    Multibuffer*		/* rightp */
);

extern void XmbufClearBufferArea(
    Display*			/* dpy */,
    Multibuffer			/* buffer */,
    int				/* x */,
    int				/* y */,
    unsigned int		/* width */,
    unsigned int		/* height */,
    Bool			/* exposures */
);

_XFUNCPROTOEND

#endif /* _MULTIBUF_H_ */
