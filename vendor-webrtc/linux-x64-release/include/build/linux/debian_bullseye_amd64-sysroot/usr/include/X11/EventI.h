/***********************************************************

Copyright 1987, 1988, 1998  The Open Group

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

/*
 * Event.h - exported types and functions for toolkit event handler
 *
 * Author:	Charles Haynes
 * 		Digital Equipment Corporation
 * 		Western Software Laboratory
 * Date:	Sun Dec  6 1987
 */

#ifndef _Event_h_
#define _Event_h_

typedef struct _XtGrabRec  *XtGrabList;

#include "PassivGraI.h"

_XFUNCPROTOBEGIN

extern void _XtEventInitialize(
    void
);

typedef struct _XtEventRec {
     XtEventTable	next;
     EventMask		mask;	/*  also select_data count for RecExt */
     XtEventHandler	proc;
     XtPointer		closure;
     unsigned int	select:1;
     unsigned int	has_type_specifier:1;
     unsigned int	async:1; /* not used, here for Digital extension? */
} XtEventRec;

typedef struct _XtGrabRec {
    XtGrabList next;
    Widget   widget;
    unsigned int exclusive:1;
    unsigned int spring_loaded:1;
}XtGrabRec;

typedef struct _BlockHookRec {
    struct _BlockHookRec* next;
    XtAppContext app;
    XtBlockHookProc proc;
    XtPointer closure;
} BlockHookRec, *BlockHook;

extern void _XtFreeEventTable(
    XtEventTable*	/* event_table */
);

extern Boolean _XtOnGrabList(
    Widget	/* widget */,
    XtGrabRec*	/* grabList */
);

extern void _XtRemoveAllInputs(
    XtAppContext /* app */
);

extern void _XtRefreshMapping(
    XEvent*	/* event */,
    _XtBoolean	/* dispatch */
);

extern void _XtSendFocusEvent(
    Widget	/* child */,
    int		/* type */);

extern EventMask _XtConvertTypeToMask(
    int		/* eventType */
);

/* EventUtil.c */
extern Widget _XtFindRemapWidget(XEvent *event, Widget widget,
				 EventMask mask, XtPerDisplayInput pdi);
extern void _XtUngrabBadGrabs(XEvent *event, Widget widget,
				 EventMask mask, XtPerDisplayInput pdi);
extern void _XtFillAncestorList(Widget **listPtr, int *maxElemsPtr,
				int *numElemsPtr, Widget start,
				Widget breakWidget);

/* NextEvent.c */
extern Boolean XtAppPeekEvent_SkipTimer;

_XFUNCPROTOEND

#endif /* _Event_h_ */
