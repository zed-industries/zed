/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prthread_h___
#define prthread_h___

/*
** API for NSPR threads. On some architectures (Mac OS Classic
** notably) pre-emptibility is not guaranteed. Hard priority scheduling
** is not guaranteed, so programming using priority based synchronization
** is a no-no.
**
** NSPR threads are scheduled based loosely on their client set priority.
** In general, a thread of a higher priority has a statistically better
** chance of running relative to threads of lower priority. However,
** NSPR uses multiple strategies to provide execution vehicles for thread
** abstraction of various host platforms. As it turns out, there is little
** NSPR can do to affect the scheduling attributes of "GLOBAL" threads.
** However, a semblance of GLOBAL threads is used to implement "LOCAL"
** threads. An arbitrary number of such LOCAL threads can be assigned to
** a single GLOBAL thread.
**
** For scheduling, NSPR will attempt to run the highest priority LOCAL
** thread associated with a given GLOBAL thread. It is further assumed
** that the host OS will apply some form of "fair" scheduling on the
** GLOBAL threads.
**
** Threads have a "system flag" which when set indicates the thread
** doesn't count for determining when the process should exit (the
** process exits when the last user thread exits).
**
** Threads also have a "scope flag" which controls whether the threads
** are scheduled in the local scope or scheduled by the OS globally. This
** indicates whether a thread is permanently bound to a native OS thread.
** An unbound thread competes for scheduling resources in the same process.
**
** Another flag is "state flag" which control whether the thread is joinable.
** It allows other threads to wait for the created thread to reach completion.
**
** Threads can have "per-thread-data" attached to them. Each thread has a
** per-thread error number and error string which are updated when NSPR
** operations fail.
*/
#include "prtypes.h"
#include "prinrval.h"

PR_BEGIN_EXTERN_C

typedef struct PRThread PRThread;
typedef struct PRThreadStack PRThreadStack;

typedef enum PRThreadType {
    PR_USER_THREAD,
    PR_SYSTEM_THREAD
} PRThreadType;

typedef enum PRThreadScope {
    PR_LOCAL_THREAD,
    PR_GLOBAL_THREAD,
    PR_GLOBAL_BOUND_THREAD
} PRThreadScope;

typedef enum PRThreadState {
    PR_JOINABLE_THREAD,
    PR_UNJOINABLE_THREAD
} PRThreadState;

typedef enum PRThreadPriority
{
    PR_PRIORITY_FIRST = 0,      /* just a placeholder */
    PR_PRIORITY_LOW = 0,        /* the lowest possible priority */
    PR_PRIORITY_NORMAL = 1,     /* most common expected priority */
    PR_PRIORITY_HIGH = 2,       /* slightly more aggressive scheduling */
    PR_PRIORITY_URGENT = 3,     /* it does little good to have more than one */
    PR_PRIORITY_LAST = 3        /* this is just a placeholder */
} PRThreadPriority;

/*
** Create a new thread:
**     "type" is the type of thread to create
**     "start(arg)" will be invoked as the threads "main"
**     "priority" will be created thread's priority
**     "scope" will specify whether the thread is local or global
**     "state" will specify whether the thread is joinable or not
**     "stackSize" the size of the stack, in bytes. The value can be zero
**        and then a machine specific stack size will be chosen.
**
** This can return NULL if some kind of error occurs, such as if memory is
** tight.
**
** If you want the thread to start up waiting for the creator to do
** something, enter a lock before creating the thread and then have the
** threads start routine enter and exit the same lock. When you are ready
** for the thread to run, exit the lock.
**
** If you want to detect the completion of the created thread, the thread
** should be created joinable.  Then, use PR_JoinThread to synchrnoize the
** termination of another thread.
**
** When the start function returns the thread exits. If it is the last
** PR_USER_THREAD to exit then the process exits.
*/
NSPR_API(PRThread*) PR_CreateThread(PRThreadType type,
                                    void (PR_CALLBACK *start)(void *arg),
                                    void *arg,
                                    PRThreadPriority priority,
                                    PRThreadScope scope,
                                    PRThreadState state,
                                    PRUint32 stackSize);

/*
** Wait for thread termination:
**     "thread" is the target thread
**
** This can return PR_FAILURE if no joinable thread could be found
** corresponding to the specified target thread.
**
** The calling thread is blocked until the target thread completes.
** Several threads cannot wait for the same thread to complete; one thread
** will operate successfully and others will terminate with an error PR_FAILURE.
** The calling thread will not be blocked if the target thread has already
** terminated.
*/
NSPR_API(PRStatus) PR_JoinThread(PRThread *thread);

/*
** Return the current thread object for the currently running code.
** Never returns NULL.
*/
NSPR_API(PRThread*) PR_GetCurrentThread(void);
#ifndef NO_NSPR_10_SUPPORT
#define PR_CurrentThread() PR_GetCurrentThread() /* for nspr1.0 compat. */
#endif /* NO_NSPR_10_SUPPORT */

/*
** Get the priority of "thread".
*/
NSPR_API(PRThreadPriority) PR_GetThreadPriority(const PRThread *thread);

/*
** Change the priority of the "thread" to "priority".
**
** PR_SetThreadPriority works in a best-effort manner. On some platforms a
** special privilege, such as root access, is required to change thread
** priorities, especially to raise thread priorities. If the caller doesn't
** have enough privileges to change thread priorites, the function has no
** effect except causing a future PR_GetThreadPriority call to return
** |priority|.
*/
NSPR_API(void) PR_SetThreadPriority(PRThread *thread, PRThreadPriority priority);

/*
** Set the name of the current thread, which will be visible in a debugger
** and accessible via a call to PR_GetThreadName().
*/
NSPR_API(PRStatus) PR_SetCurrentThreadName(const char *name);

/*
** Return the name of "thread", if set.  Otherwise return NULL.
*/
NSPR_API(const char *) PR_GetThreadName(const PRThread *thread);

/*
** This routine returns a new index for per-thread-private data table.
** The index is visible to all threads within a process. This index can
** be used with the PR_SetThreadPrivate() and PR_GetThreadPrivate() routines
** to save and retrieve data associated with the index for a thread.
**
** Each index is associationed with a destructor function ('dtor'). The function
** may be specified as NULL when the index is created. If it is not NULL, the
** function will be called when:
**      - the thread exits and the private data for the associated index
**        is not NULL,
**      - new thread private data is set and the current private data is
**        not NULL.
**
** The index independently maintains specific values for each binding thread.
** A thread can only get access to its own thread-specific-data.
**
** Upon a new index return the value associated with the index for all threads
** is NULL, and upon thread creation the value associated with all indices for
** that thread is NULL.
**
** Returns PR_FAILURE if the total number of indices will exceed the maximun
** allowed.
*/
typedef void (PR_CALLBACK *PRThreadPrivateDTOR)(void *priv);

NSPR_API(PRStatus) PR_NewThreadPrivateIndex(
    PRUintn *newIndex, PRThreadPrivateDTOR destructor);

/*
** Define some per-thread-private data.
**     "tpdIndex" is an index into the per-thread private data table
**     "priv" is the per-thread-private data
**
** If the per-thread private data table has a previously registered
** destructor function and a non-NULL per-thread-private data value,
** the destructor function is invoked.
**
** This can return PR_FAILURE if the index is invalid.
*/
NSPR_API(PRStatus) PR_SetThreadPrivate(PRUintn tpdIndex, void *priv);

/*
** Recover the per-thread-private data for the current thread. "tpdIndex" is
** the index into the per-thread private data table.
**
** The returned value may be NULL which is indistinguishable from an error
** condition.
**
** A thread can only get access to its own thread-specific-data.
*/
NSPR_API(void*) PR_GetThreadPrivate(PRUintn tpdIndex);

/*
** This routine sets the interrupt request for a target thread. The interrupt
** request remains in the thread's state until it is delivered exactly once
** or explicitly canceled.
**
** A thread that has been interrupted will fail all NSPR blocking operations
** that return a PRStatus (I/O, waiting on a condition, etc).
**
** PR_Interrupt may itself fail if the target thread is invalid.
*/
NSPR_API(PRStatus) PR_Interrupt(PRThread *thread);

/*
** Clear the interrupt request for the calling thread. If no such request
** is pending, this operation is a noop.
*/
NSPR_API(void) PR_ClearInterrupt(void);

/*
** Block the interrupt for the calling thread.
*/
NSPR_API(void) PR_BlockInterrupt(void);

/*
** Unblock the interrupt for the calling thread.
*/
NSPR_API(void) PR_UnblockInterrupt(void);

/*
** Make the current thread sleep until "ticks" time amount of time
** has expired. If "ticks" is PR_INTERVAL_NO_WAIT then the call is
** equivalent to calling PR_Yield. Calling PR_Sleep with an argument
** equivalent to PR_INTERVAL_NO_TIMEOUT is an error and will result
** in a PR_FAILURE error return.
*/
NSPR_API(PRStatus) PR_Sleep(PRIntervalTime ticks);

/*
** Get the scoping of this thread.
*/
NSPR_API(PRThreadScope) PR_GetThreadScope(const PRThread *thread);

/*
** Get the type of this thread.
*/
NSPR_API(PRThreadType) PR_GetThreadType(const PRThread *thread);

/*
** Get the join state of this thread.
*/
NSPR_API(PRThreadState) PR_GetThreadState(const PRThread *thread);

PR_END_EXTERN_C

#endif /* prthread_h___ */
