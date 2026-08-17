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

#ifndef _SYNCSTR_H_
#define _SYNCSTR_H_

#include <X11/extensions/syncproto.h>

#ifdef _SYNC_SERVER

#define CARD64 XSyncValue /* XXX temporary! need real 64 bit values for Alpha */

typedef struct _SyncCounter {
    ClientPtr		client;	/* Owning client. 0 for system counters */
    XSyncCounter	id;		/* resource ID */
    CARD64		value;		/* counter value */
    struct _SyncTriggerList *pTriglist;	/* list of triggers */
    Bool		beingDestroyed; /* in process of going away */
    struct _SysCounterInfo *pSysCounterInfo; /* NULL if not a system counter */
} SyncCounter;

/*
 * The System Counter interface
 */

typedef enum {
    XSyncCounterNeverChanges,
    XSyncCounterNeverIncreases,
    XSyncCounterNeverDecreases,
    XSyncCounterUnrestricted
} SyncCounterType;

typedef struct _SysCounterInfo {
    char	*name;
    CARD64	resolution;
    CARD64	bracket_greater;
    CARD64	bracket_less;
    SyncCounterType counterType;  /* how can this counter change */
    void        (*QueryValue)(
			      pointer /*pCounter*/,
			      CARD64 * /*freshvalue*/
);
    void	(*BracketValues)(
				 pointer /*pCounter*/,
				 CARD64 * /*lessthan*/,
				 CARD64 * /*greaterthan*/
);
} SysCounterInfo;



typedef struct _SyncTrigger {
    SyncCounter *pCounter;
    CARD64	wait_value;	/* wait value */
    unsigned int value_type;     /* Absolute or Relative */
    unsigned int test_type;	/* transition or Comparision type */
    CARD64	test_value;	/* trigger event threshold value */
    Bool	(*CheckTrigger)(
				struct _SyncTrigger * /*pTrigger*/,
				CARD64 /*newval*/
				);
    void	(*TriggerFired)(
				struct _SyncTrigger * /*pTrigger*/
				);
    void	(*CounterDestroyed)(
				struct _SyncTrigger * /*pTrigger*/
				    );
} SyncTrigger;

typedef struct _SyncTriggerList {
    SyncTrigger *pTrigger;
    struct _SyncTriggerList *next;
} SyncTriggerList;

typedef struct _SyncAlarmClientList {
    ClientPtr	client;
    XID		delete_id;
    struct _SyncAlarmClientList *next;
} SyncAlarmClientList;

typedef struct _SyncAlarm {
    SyncTrigger trigger;
    ClientPtr	client;
    XSyncAlarm 	alarm_id;
    CARD64	delta;
    int		events;
    int		state;
    SyncAlarmClientList *pEventClients;
} SyncAlarm;

typedef struct {
    ClientPtr	client;
    CARD32 	delete_id;
    int		num_waitconditions;
} SyncAwaitHeader;

typedef struct {
    SyncTrigger trigger;
    CARD64	event_threshold;
    SyncAwaitHeader *pHeader;
} SyncAwait;

typedef union {
    SyncAwaitHeader header;
    SyncAwait	    await;
} SyncAwaitUnion;


extern pointer SyncCreateSystemCounter(
    char *	/* name */,
    CARD64  	/* initial_value */,
    CARD64  	/* resolution */,
    SyncCounterType /* change characterization */,
    void        (* /*QueryValue*/ ) (
        pointer /* pCounter */,
        CARD64 * /* pValue_return */), /* XXX prototype */
    void        (* /*BracketValues*/) (
        pointer /* pCounter */,
        CARD64 * /* pbracket_less */,
        CARD64 * /* pbracket_greater */)
);

extern void SyncChangeCounter(
    SyncCounter *	/* pCounter*/,
    CARD64  		/* new_value */
);

extern void SyncDestroySystemCounter(
    pointer pCounter
);
extern void InitServertime(void);

#endif /* _SYNC_SERVER */

#endif /* _SYNCSTR_H_ */
