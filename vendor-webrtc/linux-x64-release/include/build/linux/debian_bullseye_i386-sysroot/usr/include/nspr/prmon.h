/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prmon_h___
#define prmon_h___

#include "prtypes.h"
#include "prinrval.h"

PR_BEGIN_EXTERN_C

typedef struct PRMonitor PRMonitor;

/*
** Create a new monitor. Monitors are re-entrant locks with a single built-in
** condition variable.
**
** This may fail if memory is tight or if some operating system resource
** is low.
*/
NSPR_API(PRMonitor*) PR_NewMonitor(void);

/*
** Destroy a monitor. The caller is responsible for guaranteeing that the
** monitor is no longer in use. There must be no thread waiting on the monitor's
** condition variable and that the lock is not held.
**
*/
NSPR_API(void) PR_DestroyMonitor(PRMonitor *mon);

/*
** Enter the lock associated with the monitor. If the calling thread currently
** is in the monitor, the call to enter will silently succeed. In either case,
** it will increment the entry count by one.
*/
NSPR_API(void) PR_EnterMonitor(PRMonitor *mon);

/*
** Decrement the entry count associated with the monitor. If the decremented
** entry count is zero, the monitor is exited. Returns PR_FAILURE if the
** calling thread has not entered the monitor.
*/
NSPR_API(PRStatus) PR_ExitMonitor(PRMonitor *mon);

/*
** Wait for a notify on the monitor's condition variable. Sleep for "ticks"
** amount of time (if "ticks" is PR_INTERVAL_NO_TIMEOUT then the sleep is
** indefinite).
**
** While the thread is waiting it exits the monitor (as if it called
** PR_ExitMonitor as many times as it had called PR_EnterMonitor).  When
** the wait has finished the thread regains control of the monitors lock
** with the same entry count as before the wait began.
**
** The thread waiting on the monitor will be resumed when the monitor is
** notified (assuming the thread is the next in line to receive the
** notify) or when the "ticks" timeout elapses.
**
** Returns PR_FAILURE if the caller has not entered the monitor.
*/
NSPR_API(PRStatus) PR_Wait(PRMonitor *mon, PRIntervalTime ticks);

/*
** Notify a thread waiting on the monitor's condition variable. If a thread
** is waiting on the condition variable (using PR_Wait) then it is awakened
** and attempts to reenter the monitor.
*/
NSPR_API(PRStatus) PR_Notify(PRMonitor *mon);

/*
** Notify all of the threads waiting on the monitor's condition variable.
** All of threads waiting on the condition are scheduled to reenter the
** monitor.
*/
NSPR_API(PRStatus) PR_NotifyAll(PRMonitor *mon);

/*
** PR_ASSERT_CURRENT_THREAD_IN_MONITOR
** If the current thread is in |mon|, this assertion is guaranteed to
** succeed.  Otherwise, the behavior of this function is undefined.
*/
#if defined(DEBUG) || defined(FORCE_PR_ASSERT)
#define PR_ASSERT_CURRENT_THREAD_IN_MONITOR(/* PRMonitor* */ mon) \
    PR_AssertCurrentThreadInMonitor(mon)
#else
#define PR_ASSERT_CURRENT_THREAD_IN_MONITOR(/* PRMonitor* */ mon)
#endif

/* Don't call this function directly. */
NSPR_API(void) PR_AssertCurrentThreadInMonitor(PRMonitor *mon);

PR_END_EXTERN_C

#endif /* prmon_h___ */
