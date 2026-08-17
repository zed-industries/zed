/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef pprthred_h___
#define pprthred_h___

/*
** API for PR private functions.  These calls are to be used by internal
** developers only.
*/
#include "nspr.h"

#if defined(XP_OS2)
#define INCL_DOS
#define INCL_DOSERRORS
#define INCL_WIN
#include <os2.h>
#endif

PR_BEGIN_EXTERN_C

/*---------------------------------------------------------------------------
** THREAD PRIVATE FUNCTIONS
---------------------------------------------------------------------------*/

/*
** Associate a thread object with an existing native thread.
**  "type" is the type of thread object to attach
**  "priority" is the priority to assign to the thread
**  "stack" defines the shape of the threads stack
**
** This can return NULL if some kind of error occurs, or if memory is
** tight. This call invokes "start(obj,arg)" and returns when the
** function returns. The thread object is automatically destroyed.
**
** This call is not normally needed unless you create your own native
** thread. PR_Init does this automatically for the primordial thread.
*/
NSPR_API(PRThread*) PR_AttachThread(PRThreadType type,
                                    PRThreadPriority priority,
                                    PRThreadStack *stack);

/*
** Detach the nspr thread from the currently executing native thread.
** The thread object will be destroyed and all related data attached
** to it. The exit procs will be invoked.
**
** This call is not normally needed unless you create your own native
** thread. PR_Exit will automatially detach the nspr thread object
** created by PR_Init for the primordial thread.
**
** This call returns after the nspr thread object is destroyed.
*/
NSPR_API(void) PR_DetachThread(void);

/*
** Get the id of the named thread. Each thread is assigned a unique id
** when it is created or attached.
*/
NSPR_API(PRUint32) PR_GetThreadID(PRThread *thread);

/*
** Set the procedure that is called when a thread is dumped. The procedure
** will be applied to the argument, arg, when called. Setting the procedure
** to NULL effectively removes it.
*/
typedef void (*PRThreadDumpProc)(PRFileDesc *fd, PRThread *t, void *arg);
NSPR_API(void) PR_SetThreadDumpProc(
    PRThread* thread, PRThreadDumpProc dump, void *arg);

/*
** Get this thread's affinity mask.  The affinity mask is a 32 bit quantity
** marking a bit for each processor this process is allowed to run on.
** The processor mask is returned in the mask argument.
** The least-significant-bit represents processor 0.
**
** Returns 0 on success, -1 on failure.
*/
NSPR_API(PRInt32) PR_GetThreadAffinityMask(PRThread *thread, PRUint32 *mask);

/*
** Set this thread's affinity mask.
**
** Returns 0 on success, -1 on failure.
*/
NSPR_API(PRInt32) PR_SetThreadAffinityMask(PRThread *thread, PRUint32 mask );

/*
** Set the default CPU Affinity mask.
**
*/
NSPR_API(PRInt32) PR_SetCPUAffinityMask(PRUint32 mask);

/*
** Show status of all threads to standard error output.
*/
NSPR_API(void) PR_ShowStatus(void);

/*
** Set thread recycle mode to on (1) or off (0)
*/
NSPR_API(void) PR_SetThreadRecycleMode(PRUint32 flag);


/*---------------------------------------------------------------------------
** THREAD PRIVATE FUNCTIONS FOR GARBAGE COLLECTIBLE THREADS
---------------------------------------------------------------------------*/

/*
** Only Garbage collectible threads participate in resume all, suspend all and
** enumeration operations.  They are also different during creation when
** platform specific action may be needed (For example, all Solaris GC able
** threads are bound threads).
*/

/*
** Same as PR_CreateThread except that the thread is marked as garbage
** collectible.
*/
NSPR_API(PRThread*) PR_CreateThreadGCAble(PRThreadType type,
        void (*start)(void *arg),
        void *arg,
        PRThreadPriority priority,
        PRThreadScope scope,
        PRThreadState state,
        PRUint32 stackSize);

/*
** Same as PR_AttachThread except that the thread being attached is marked as
** garbage collectible.
*/
NSPR_API(PRThread*) PR_AttachThreadGCAble(PRThreadType type,
        PRThreadPriority priority,
        PRThreadStack *stack);

/*
** Mark the thread as garbage collectible.
*/
NSPR_API(void) PR_SetThreadGCAble(void);

/*
** Unmark the thread as garbage collectible.
*/
NSPR_API(void) PR_ClearThreadGCAble(void);

/*
** This routine prevents all other GC able threads from running. This call is needed by
** the garbage collector.
*/
NSPR_API(void) PR_SuspendAll(void);

/*
** This routine unblocks all other GC able threads that were suspended from running by
** PR_SuspendAll(). This call is needed by the garbage collector.
*/
NSPR_API(void) PR_ResumeAll(void);

/*
** Return the thread stack pointer of the given thread.
** Needed by the garbage collector.
*/
NSPR_API(void *) PR_GetSP(PRThread *thread);

/*
** Save the registers that the GC would find interesting into the thread
** "t". isCurrent will be non-zero if the thread state that is being
** saved is the currently executing thread. Return the address of the
** first register to be scanned as well as the number of registers to
** scan in "np".
**
** If "isCurrent" is non-zero then it is allowed for the thread context
** area to be used as scratch storage to hold just the registers
** necessary for scanning.
**
** This function simply calls the internal function _MD_HomeGCRegisters().
*/
NSPR_API(PRWord *) PR_GetGCRegisters(PRThread *t, int isCurrent, int *np);

/*
** (Get|Set)ExecutionEnvironent
**
** Used by Java to associate it's execution environment so garbage collector
** can find it. If return is NULL, then it's probably not a collectable thread.
**
** There's no locking required around these calls.
*/
NSPR_API(void*) GetExecutionEnvironment(PRThread *thread);
NSPR_API(void) SetExecutionEnvironment(PRThread* thread, void *environment);

/*
** Enumeration function that applies "func(thread,i,arg)" to each active
** thread in the process. The enumerator returns PR_SUCCESS if the enumeration
** should continue, any other value is considered failure, and enumeration
** stops, returning the failure value from PR_EnumerateThreads.
** Needed by the garbage collector.
*/
typedef PRStatus (PR_CALLBACK *PREnumerator)(PRThread *t, int i, void *arg);
NSPR_API(PRStatus) PR_EnumerateThreads(PREnumerator func, void *arg);

/*
** Signature of a thread stack scanning function. It is applied to every
** contiguous group of potential pointers within a thread. Count denotes the
** number of pointers.
*/
typedef PRStatus
(PR_CALLBACK *PRScanStackFun)(PRThread* t,
                              void** baseAddr, PRUword count, void* closure);

/*
** Applies scanFun to all contiguous groups of potential pointers
** within a thread. This includes the stack, registers, and thread-local
** data. If scanFun returns a status value other than PR_SUCCESS the scan
** is aborted, and the status value is returned.
*/
NSPR_API(PRStatus)
PR_ThreadScanStackPointers(PRThread* t,
                           PRScanStackFun scanFun, void* scanClosure);

/*
** Calls PR_ThreadScanStackPointers for every thread.
*/
NSPR_API(PRStatus)
PR_ScanStackPointers(PRScanStackFun scanFun, void* scanClosure);

/*
** Returns a conservative estimate on the amount of stack space left
** on a thread in bytes, sufficient for making decisions about whether
** to continue recursing or not.
*/
NSPR_API(PRUword)
PR_GetStackSpaceLeft(PRThread* t);

/*---------------------------------------------------------------------------
** THREAD CPU PRIVATE FUNCTIONS
---------------------------------------------------------------------------*/

/*
** Get a pointer to the primordial CPU.
*/
NSPR_API(struct _PRCPU *) _PR_GetPrimordialCPU(void);

/*---------------------------------------------------------------------------
** THREAD SYNCHRONIZATION PRIVATE FUNCTIONS
---------------------------------------------------------------------------*/

/*
** Create a new named monitor (named for debugging purposes).
**  Monitors are re-entrant locks with a built-in condition variable.
**
** This may fail if memory is tight or if some operating system resource
** is low.
*/
NSPR_API(PRMonitor*) PR_NewNamedMonitor(const char* name);

/*
** Test and then lock the lock if it's not already locked by some other
** thread. Return PR_FALSE if some other thread owned the lock at the
** time of the call.
*/
NSPR_API(PRBool) PR_TestAndLock(PRLock *lock);

/*
** Test and then enter the mutex associated with the monitor if it's not
** already entered by some other thread. Return PR_FALSE if some other
** thread owned the mutex at the time of the call.
*/
NSPR_API(PRBool) PR_TestAndEnterMonitor(PRMonitor *mon);

/*
** Return the number of times that the current thread has entered the
** mutex. Returns zero if the current thread has not entered the mutex.
*/
NSPR_API(PRIntn) PR_GetMonitorEntryCount(PRMonitor *mon);

/*
** Just like PR_CEnterMonitor except that if the monitor is owned by
** another thread NULL is returned.
*/
NSPR_API(PRMonitor*) PR_CTestAndEnterMonitor(void *address);

/*---------------------------------------------------------------------------
** PLATFORM-SPECIFIC INITIALIZATION FUNCTIONS
---------------------------------------------------------------------------*/
#if defined(XP_OS2)
/*
** These functions need to be called at the start and end of a thread.
** An EXCEPTIONREGISTRATIONRECORD must be declared on the stack and its
** address passed to the two functions.
*/
NSPR_API(void) PR_OS2_SetFloatExcpHandler(EXCEPTIONREGISTRATIONRECORD* e);
NSPR_API(void) PR_OS2_UnsetFloatExcpHandler(EXCEPTIONREGISTRATIONRECORD* e);
#endif /* XP_OS2 */

/* I think PR_GetMonitorEntryCount is useless. All you really want is this... */
#define PR_InMonitor(m)     (PR_GetMonitorEntryCount(m) > 0)

/*---------------------------------------------------------------------------
** Special X-Lock hack for client
---------------------------------------------------------------------------*/

#ifdef XP_UNIX
extern void PR_XLock(void);
extern void PR_XUnlock(void);
extern PRBool PR_XIsLocked(void);
#endif /* XP_UNIX */

PR_END_EXTERN_C

#endif /* pprthred_h___ */
