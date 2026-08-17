/***********************************************************

Copyright 1987, 1988, 1994, 1998  The Open Group

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


Copyright 1987, 1988 by Digital Equipment Corporation, Maynard, Massachusetts.

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the name of Digital not be
used in advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

DIGITAL DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING
ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL
DIGITAL BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR
ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
SOFTWARE.

******************************************************************/

#ifndef _XtselectionI_h
#define _XtselectionI_h

#include "Intrinsic.h"

typedef struct _RequestRec *Request;
typedef struct _SelectRec *Select;

typedef struct _RequestRec {
   Select ctx;		      /* logical owner */
   Widget widget;	      /* widget actually receiving Selection events */
   Window requestor;
   Atom property;
   Atom target;
   Atom type;
   int format;
   XtPointer value;
   unsigned long bytelength;
   unsigned long offset;
   XtIntervalId timeout;
   XSelectionRequestEvent event; /* for XtGetSelectionRequest */
   Boolean allSent;
} RequestRec;

typedef struct {
  Atom prop;
  Boolean avail;
} SelectionPropRec, *SelectionProp;

typedef struct {
  Display *dpy;
  Atom incr_atom, indirect_atom, timestamp_atom;
  int propCount;
  SelectionProp list;
} PropListRec, *PropList;

typedef struct _SelectRec {
    Atom selection; 			/* constant */
    Display *dpy; 			/* constant */
    Widget widget;
    Time time;
    unsigned long serial;
    XtConvertSelectionProc convert;
    XtLoseSelectionProc loses;
    XtSelectionDoneProc notify;
    XtCancelConvertSelectionProc owner_cancel;
    XtPointer owner_closure;
    PropList prop_list;
    Request req;			/* state for local non-incr xfer */
    int ref_count;			/* of active transfers */
    unsigned int incremental:1;
    unsigned int free_when_done:1;
    unsigned int was_disowned:1;
} SelectRec;

typedef struct _ParamRec {
    Atom selection;
    Atom param;
} ParamRec, *Param;

typedef struct _ParamInfoRec {
    unsigned int count;
    Param paramlist;
} ParamInfoRec, *ParamInfo;

typedef struct _QueuedRequestRec {
    Atom selection;
    Atom target;
    Atom param;
    XtSelectionCallbackProc callback;
    XtPointer closure;
    Time time;
    Boolean incremental;
} QueuedRequestRec, *QueuedRequest;

typedef struct _QueuedRequestInfoRec {
    int count;
    Atom *selections;
    QueuedRequest *requests;
} QueuedRequestInfoRec, *QueuedRequestInfo;

typedef struct {
    XtSelectionCallbackProc *callbacks;
    XtPointer *req_closure;
    Atom property;
    Atom *target;
    Atom type;
    int format;
    char *value;
    int bytelength;
    int offset;
    XtIntervalId timeout;
    XtEventHandler proc;
    Widget widget;
    Time time;
    Select ctx;
    Boolean *incremental;
    int current;
} CallBackInfoRec, *CallBackInfo;

typedef struct {
  Atom target;
  Atom property;
} IndirectPair;

#define IndirectPairWordSize 2

typedef struct {
  int active_transfer_count;
} RequestWindowRec;

#define MAX_SELECTION_INCR(dpy) (((65536 < XMaxRequestSize(dpy)) ? \
	(65536 << 2)  : (XMaxRequestSize(dpy) << 2))-100)

#define MATCH_SELECT(event, info) ((event->time == info->time) && \
	    (event->requestor == XtWindow(info->widget)) && \
	    (event->selection == info->ctx->selection) && \
	    (event->target == *info->target))

#endif /* _XtselectionI_h */
/* DON'T ADD STUFF AFTER THIS #endif */
