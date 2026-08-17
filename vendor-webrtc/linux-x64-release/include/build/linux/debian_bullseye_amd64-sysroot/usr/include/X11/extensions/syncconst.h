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

#ifndef _SYNCCONST_H_
#define _SYNCCONST_H_

#define SYNC_NAME "SYNC"

#define SYNC_MAJOR_VERSION	3
#define SYNC_MINOR_VERSION	1


#define XSyncCounterNotify              0
#define XSyncAlarmNotify		1
#define XSyncAlarmNotifyMask 		(1L << XSyncAlarmNotify)

#define XSyncNumberEvents		2L

#define XSyncBadCounter			0L
#define XSyncBadAlarm			1L
#define XSyncBadFence           2L
#define XSyncNumberErrors		(XSyncBadFence + 1)

/*
 * Flags for Alarm Attributes
 */
#define XSyncCACounter			(1L<<0)
#define XSyncCAValueType		(1L<<1)
#define XSyncCAValue			(1L<<2)
#define XSyncCATestType			(1L<<3)
#define XSyncCADelta			(1L<<4)
#define XSyncCAEvents			(1L<<5)

/*  The _XSync macros below are for library internal use only.  They exist
 *  so that if we have to make a fix, we can change it in this one place
 *  and have both the macro and function variants inherit the fix.
 */

#define _XSyncIntToValue(pv, i)     ((pv)->hi=((i<0)?~0:0),(pv)->lo=(i))
#define _XSyncIntsToValue(pv, l, h) ((pv)->lo = (l), (pv)->hi = (h))
#define _XSyncValueGreaterThan(a, b)\
    ((a).hi>(b).hi || ((a).hi==(b).hi && (a).lo>(b).lo))
#define _XSyncValueLessThan(a, b)\
    ((a).hi<(b).hi || ((a).hi==(b).hi && (a).lo<(b).lo))
#define _XSyncValueGreaterOrEqual(a, b)\
    ((a).hi>(b).hi || ((a).hi==(b).hi && (a).lo>=(b).lo))
#define _XSyncValueLessOrEqual(a, b)\
    ((a).hi<(b).hi || ((a).hi==(b).hi && (a).lo<=(b).lo))
#define _XSyncValueEqual(a, b)	((a).lo==(b).lo && (a).hi==(b).hi)
#define _XSyncValueIsNegative(v) (((v).hi & 0x80000000) ? 1 : 0)
#define _XSyncValueIsZero(a)	((a).lo==0 && (a).hi==0)
#define _XSyncValueIsPositive(v) (((v).hi & 0x80000000) ? 0 : 1)
#define _XSyncValueLow32(v)	((v).lo)
#define _XSyncValueHigh32(v)	((v).hi)
#define _XSyncValueAdd(presult,a,b,poverflow) {\
	int t = (a).lo;\
	Bool signa = XSyncValueIsNegative(a);\
	Bool signb = XSyncValueIsNegative(b);\
	((presult)->lo = (a).lo + (b).lo);\
	((presult)->hi = (a).hi + (b).hi);\
	if (t>(presult)->lo) (presult)->hi++;\
	*poverflow = ((signa == signb) && !(signa == XSyncValueIsNegative(*presult)));\
     }
#define _XSyncValueSubtract(presult,a,b,poverflow) {\
	int t = (a).lo;\
	Bool signa = XSyncValueIsNegative(a);\
	Bool signb = XSyncValueIsNegative(b);\
	((presult)->lo = (a).lo - (b).lo);\
	((presult)->hi = (a).hi - (b).hi);\
	if (t<(presult)->lo) (presult)->hi--;\
	*poverflow = ((signa == signb) && !(signa == XSyncValueIsNegative(*presult)));\
     }
#define _XSyncMaxValue(pv) ((pv)->hi = 0x7fffffff, (pv)->lo = 0xffffffff)
#define _XSyncMinValue(pv) ((pv)->hi = 0x80000000, (pv)->lo = 0)

/*
 *  These are the publically usable macros.  If you want the function version
 *  of one of these, just #undef the macro to uncover the function.
 *  (This is the same convention that the ANSI C library uses.)
 */

#define XSyncIntToValue(pv, i) _XSyncIntToValue(pv, i)
#define XSyncIntsToValue(pv, l, h) _XSyncIntsToValue(pv, l, h)
#define XSyncValueGreaterThan(a, b) _XSyncValueGreaterThan(a, b)
#define XSyncValueLessThan(a, b) _XSyncValueLessThan(a, b)
#define XSyncValueGreaterOrEqual(a, b) _XSyncValueGreaterOrEqual(a, b)
#define XSyncValueLessOrEqual(a, b) _XSyncValueLessOrEqual(a, b)
#define XSyncValueEqual(a, b) _XSyncValueEqual(a, b)
#define XSyncValueIsNegative(v) _XSyncValueIsNegative(v)
#define XSyncValueIsZero(a) _XSyncValueIsZero(a)
#define XSyncValueIsPositive(v) _XSyncValueIsPositive(v)
#define XSyncValueLow32(v) _XSyncValueLow32(v)
#define XSyncValueHigh32(v) _XSyncValueHigh32(v)
#define XSyncValueAdd(presult,a,b,poverflow) _XSyncValueAdd(presult,a,b,poverflow)
#define XSyncValueSubtract(presult,a,b,poverflow) _XSyncValueSubtract(presult,a,b,poverflow)
#define XSyncMaxValue(pv) _XSyncMaxValue(pv)
#define XSyncMinValue(pv) _XSyncMinValue(pv)

/*
 * Constants for the value_type argument of various requests
 */
typedef enum {
    XSyncAbsolute,
    XSyncRelative
} XSyncValueType;

/*
 * Alarm Test types
 */
typedef enum {
    XSyncPositiveTransition,
    XSyncNegativeTransition,
    XSyncPositiveComparison,
    XSyncNegativeComparison
} XSyncTestType;

/*
 * Alarm state constants
 */
typedef enum {
    XSyncAlarmActive,
    XSyncAlarmInactive,
    XSyncAlarmDestroyed
} XSyncAlarmState;


typedef XID XSyncCounter;
typedef XID XSyncAlarm;
typedef XID XSyncFence;
typedef struct _XSyncValue {
    int hi;
    unsigned int lo;
} XSyncValue;
#endif /* _SYNCCONST_H_ */
