/*

Copyright 1991, 1993, 1994, 1998  The Open Group

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

/***********************************************************
Copyright 1991,1993 by Digital Equipment Corporation, Maynard, Massachusetts,
and Olivetti Research Limited, Cambridge, England.

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the names of Digital or Olivetti
not be used in advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

DIGITAL AND OLIVETTI DISCLAIM ALL WARRANTIES WITH REGARD TO THIS
SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS, IN NO EVENT SHALL THEY BE LIABLE FOR ANY SPECIAL, INDIRECT OR
CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE.

******************************************************************/

#ifndef _SYNC_H_
#define _SYNC_H_

#include <X11/Xfuncproto.h>
#include <X11/extensions/syncconst.h>

#ifdef _SYNC_SERVER
#include <X11/extensions/syncproto.h>
#else

_XFUNCPROTOBEGIN
/* get rid of macros so we can define corresponding functions */
#undef XSyncIntToValue
#undef XSyncIntsToValue
#undef XSyncValueGreaterThan
#undef XSyncValueLessThan
#undef XSyncValueGreaterOrEqual
#undef XSyncValueLessOrEqual
#undef XSyncValueEqual
#undef XSyncValueIsNegative
#undef XSyncValueIsZero
#undef XSyncValueIsPositive
#undef XSyncValueLow32
#undef XSyncValueHigh32
#undef XSyncValueAdd
#undef XSyncValueSubtract
#undef XSyncMaxValue
#undef XSyncMinValue

extern void XSyncIntToValue(
    XSyncValue* /*pv*/,
    int /*i*/
);

extern void XSyncIntsToValue(
    XSyncValue* /*pv*/,
    unsigned int /*l*/,
    int /*h*/
);

extern Bool XSyncValueGreaterThan(
    XSyncValue /*a*/,
    XSyncValue /*b*/
);

extern Bool XSyncValueLessThan(
    XSyncValue /*a*/,
    XSyncValue /*b*/
);

extern Bool XSyncValueGreaterOrEqual(
    XSyncValue /*a*/,
    XSyncValue /*b*/
);

extern Bool XSyncValueLessOrEqual(
    XSyncValue /*a*/,
    XSyncValue /*b*/
);

extern Bool XSyncValueEqual(
    XSyncValue /*a*/,
    XSyncValue /*b*/
);

extern Bool XSyncValueIsNegative(
    XSyncValue /*v*/
);

extern Bool XSyncValueIsZero(
    XSyncValue /*a*/
);

extern Bool XSyncValueIsPositive(
    XSyncValue /*v*/
);

extern unsigned int XSyncValueLow32(
    XSyncValue /*v*/
);

extern int XSyncValueHigh32(
    XSyncValue /*v*/
);

extern void XSyncValueAdd(
    XSyncValue* /*presult*/,
    XSyncValue /*a*/,
    XSyncValue /*b*/,
    int* /*poverflow*/
);

extern void XSyncValueSubtract(
    XSyncValue* /*presult*/,
    XSyncValue /*a*/,
    XSyncValue /*b*/,
    int* /*poverflow*/
);

extern void XSyncMaxValue(
    XSyncValue* /*pv*/
);

extern void XSyncMinValue(
    XSyncValue* /*pv*/
);

_XFUNCPROTOEND


typedef struct _XSyncSystemCounter {
    char *name;			/* null-terminated name of system counter */
    XSyncCounter counter;	/* counter id of this system counter */
    XSyncValue resolution;	/* resolution of this system counter */
} XSyncSystemCounter;


typedef struct {
    XSyncCounter counter;	/* counter to trigger on */
    XSyncValueType value_type;	/* absolute/relative */
    XSyncValue wait_value;	/* value to compare counter to */
    XSyncTestType test_type;	/* pos/neg comparison/transtion */
} XSyncTrigger;

typedef struct {
    XSyncTrigger trigger;	/* trigger for await */
    XSyncValue event_threshold; /* send event if past threshold */
} XSyncWaitCondition;


typedef struct {
    XSyncTrigger trigger;
    XSyncValue  delta;
    Bool events;
    XSyncAlarmState state;
} XSyncAlarmAttributes;

/*
 *  Events
 */

typedef struct {
    int type;			/* event base + XSyncCounterNotify */
    unsigned long serial;	/* # of last request processed by server */
    Bool send_event;		/* true if this came from a SendEvent request */
    Display *display;		/* Display the event was read from */
    XSyncCounter counter;	/* counter involved in await */
    XSyncValue wait_value;	/* value being waited for */
    XSyncValue counter_value;	/* counter value when this event was sent */
    Time time;			/* milliseconds */
    int count;			/* how many more events to come */
    Bool destroyed;		/* True if counter was destroyed */
} XSyncCounterNotifyEvent;

typedef struct {
    int type;			/* event base + XSyncAlarmNotify */
    unsigned long serial;	/* # of last request processed by server */
    Bool send_event;		/* true if this came from a SendEvent request */
    Display *display;		/* Display the event was read from */
    XSyncAlarm alarm;		/* alarm that triggered */
    XSyncValue counter_value;	/* value that triggered the alarm */
    XSyncValue alarm_value;	/* test  value of trigger in alarm */
    Time time;			/* milliseconds */
    XSyncAlarmState state;	/* new state of alarm */
} XSyncAlarmNotifyEvent;

/*
 *  Errors
 */

typedef struct {
    int type;
    Display *display;		/* Display the event was read from */
    XSyncAlarm alarm;		/* resource id */
    unsigned long serial;	/* serial number of failed request */
    unsigned char error_code;	/* error base + XSyncBadAlarm */
    unsigned char request_code;	/* Major op-code of failed request */
    unsigned char minor_code;	/* Minor op-code of failed request */
} XSyncAlarmError;

typedef struct {
    int type;
    Display *display;		/* Display the event was read from */
    XSyncCounter counter;	/* resource id */
    unsigned long serial;	/* serial number of failed request */
    unsigned char error_code;	/* error base + XSyncBadCounter */
    unsigned char request_code;	/* Major op-code of failed request */
    unsigned char minor_code;	/* Minor op-code of failed request */
} XSyncCounterError;

/*
 *  Prototypes
 */

_XFUNCPROTOBEGIN

extern Status XSyncQueryExtension(
    Display* /*dpy*/,
    int* /*event_base_return*/,
    int* /*error_base_return*/
);

extern Status XSyncInitialize(
    Display* /*dpy*/,
    int* /*major_version_return*/,
    int* /*minor_version_return*/
);

extern XSyncSystemCounter *XSyncListSystemCounters(
    Display* /*dpy*/,
    int* /*n_counters_return*/
);

extern void XSyncFreeSystemCounterList(
    XSyncSystemCounter* /*list*/
);

extern XSyncCounter XSyncCreateCounter(
    Display* /*dpy*/,
    XSyncValue /*initial_value*/
);

extern Status XSyncSetCounter(
    Display* /*dpy*/,
    XSyncCounter /*counter*/,
    XSyncValue /*value*/
);

extern Status XSyncChangeCounter(
    Display* /*dpy*/,
    XSyncCounter /*counter*/,
    XSyncValue /*value*/
);

extern Status XSyncDestroyCounter(
    Display* /*dpy*/,
    XSyncCounter /*counter*/
);

extern Status XSyncQueryCounter(
    Display* /*dpy*/,
    XSyncCounter /*counter*/,
    XSyncValue* /*value_return*/
);

extern Status XSyncAwait(
    Display* /*dpy*/,
    XSyncWaitCondition* /*wait_list*/,
    int /*n_conditions*/
);

extern XSyncAlarm XSyncCreateAlarm(
    Display* /*dpy*/,
    unsigned long /*values_mask*/,
    XSyncAlarmAttributes* /*values*/
);

extern Status XSyncDestroyAlarm(
    Display* /*dpy*/,
    XSyncAlarm /*alarm*/
);

extern Status XSyncQueryAlarm(
    Display* /*dpy*/,
    XSyncAlarm /*alarm*/,
    XSyncAlarmAttributes* /*values_return*/
);

extern Status XSyncChangeAlarm(
    Display* /*dpy*/,
    XSyncAlarm /*alarm*/,
    unsigned long /*values_mask*/,
    XSyncAlarmAttributes* /*values*/
);

extern Status XSyncSetPriority(
    Display* /*dpy*/,
    XID /*client_resource_id*/,
    int /*priority*/
);

extern Status XSyncGetPriority(
    Display* /*dpy*/,
    XID /*client_resource_id*/,
    int* /*return_priority*/
);

extern XSyncFence XSyncCreateFence(
    Display* /*dpy*/,
    Drawable /*d*/,
    Bool /*initially_triggered*/
);

extern Bool XSyncTriggerFence(
    Display* /*dpy*/,
    XSyncFence /*fence*/
);

extern Bool XSyncResetFence(
    Display* /*dpy*/,
    XSyncFence /*fence*/
);

extern Bool XSyncDestroyFence(
    Display* /*dpy*/,
    XSyncFence /*fence*/
);

extern Bool XSyncQueryFence(
    Display* /*dpy*/,
    XSyncFence /*fence*/,
    Bool* /*triggered*/
);

extern Bool XSyncAwaitFence(
    Display* /*dpy*/,
    const XSyncFence* /*fence_list*/,
    int /*n_fences*/
);

_XFUNCPROTOEND

#endif /* _SYNC_SERVER */

#endif /* _SYNC_H_ */
