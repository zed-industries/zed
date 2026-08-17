/***************************************************************************
 * Copyright 1995 Network Computing Devices
 *
 * Permission to use, copy, modify, distribute, and sell this software and
 * its documentation for any purpose is hereby granted without fee, provided
 * that the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of Network Computing Devices
 * not be used in advertising or publicity pertaining to distribution
 * of the software without specific, written prior permission.
 *
 * NETWORK COMPUTING DEVICES DISCLAIMs ALL WARRANTIES WITH REGARD TO
 * THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS, IN NO EVENT SHALL NETWORK COMPUTING DEVICES BE LIABLE
 * FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 **************************************************************************/

#ifndef _RECORD_H_
#define _RECORD_H_

#include <X11/extensions/recordconst.h>

typedef unsigned long   XRecordClientSpec;
typedef unsigned long	XRecordContext;

typedef struct
{
    unsigned char	first;
    unsigned char	last;
} XRecordRange8;

typedef struct
{
    unsigned short	first;
    unsigned short	last;
} XRecordRange16;

typedef struct
{
    XRecordRange8	ext_major;
    XRecordRange16	ext_minor;
} XRecordExtRange;

typedef struct
{
    XRecordRange8     core_requests;	/* core X requests */
    XRecordRange8     core_replies;	/* core X replies */
    XRecordExtRange   ext_requests;	/* extension requests */
    XRecordExtRange   ext_replies;	/* extension replies */
    XRecordRange8     delivered_events;	/* delivered core and ext events */
    XRecordRange8     device_events;	/* all core and ext device events */
    XRecordRange8     errors;		/* core X and ext errors */
    Bool	      client_started;	/* connection setup reply */
    Bool              client_died;	/* notice of client disconnect */
} XRecordRange;

typedef struct
{
    XRecordClientSpec	client;
    unsigned long	nranges;
    XRecordRange	**ranges;
} XRecordClientInfo;

typedef struct
{
    Bool		enabled;
    int			datum_flags;
    unsigned long	nclients;
    XRecordClientInfo	**client_info;
} XRecordState;

typedef struct
{
    XID		id_base;
    Time		server_time;
    unsigned long	client_seq;
    int			category;
    Bool		client_swapped;
    unsigned char	*data;
    unsigned long	data_len;	/* in 4-byte units */
} XRecordInterceptData;

_XFUNCPROTOBEGIN

/*********************************************************
 *
 * Prototypes
 *
 */

XID XRecordIdBaseMask(
    Display *dpy
);

extern Status XRecordQueryVersion(
    Display*			/* dpy */,
    int*			/* cmajor_return */,
    int*			/* cminor_return */
);

extern XRecordContext XRecordCreateContext(
    Display*			/* dpy */,
    int				/* datum_flags */,
    XRecordClientSpec*		/* clients */,
    int				/* nclients */,
    XRecordRange**              /* ranges */,
    int				/* nranges */
);

extern XRecordRange *XRecordAllocRange(
    void
);

extern Status XRecordRegisterClients(
    Display*			/* dpy */,
    XRecordContext		/* context */,
    int				/* datum_flags */,
    XRecordClientSpec*		/* clients */,
    int				/* nclients */,
    XRecordRange**		/* ranges */,
    int				/* nranges */
);

extern Status XRecordUnregisterClients(
    Display*			/* dpy */,
    XRecordContext		/* context */,
    XRecordClientSpec*		/* clients */,
    int				/* nclients */
);

extern Status XRecordGetContext(
    Display*			/* dpy */,
    XRecordContext		/* context */,
    XRecordState**		/* state_return */
);

extern void XRecordFreeState(
XRecordState*			/* state */
);

typedef void (*XRecordInterceptProc) (
    XPointer			/* closure */,
    XRecordInterceptData*	/* recorded_data */
);

extern Status XRecordEnableContext(
    Display*			/* dpy */,
    XRecordContext		/* context */,
    XRecordInterceptProc	/* callback */,
    XPointer			/* closure */
);

extern Status XRecordEnableContextAsync(
    Display*			/* dpy */,
    XRecordContext		/* context */,
    XRecordInterceptProc	/* callback */,
    XPointer			/* closure */
);

extern void XRecordProcessReplies(
    Display*			/* dpy */
);

extern void XRecordFreeData(
XRecordInterceptData*	/* data */
);

extern Status XRecordDisableContext(
    Display*			/* dpy */,
    XRecordContext		/* context */
);

extern Status XRecordFreeContext(
    Display*			/* dpy */,
    XRecordContext		/* context */
);

_XFUNCPROTOEND

#endif
