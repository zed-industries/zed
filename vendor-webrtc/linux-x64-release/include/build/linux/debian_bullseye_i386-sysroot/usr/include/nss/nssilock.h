/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
** nssilock.h - Instrumented locking functions for NSS
**
** Description:
**    nssilock provides instrumentation for locks and monitors in
**    the NSS libraries. The instrumentation, when enabled, causes
**    each call to the instrumented function to record data about
**    the call to an external file. The external file
**    subsequently used to extract performance data and other
**    statistical information about the operation of locks used in
**    the nss library.
**
**    To enable compilation with instrumentation, build NSS with
**    the compile time switch NEED_NSS_ILOCK defined.
**
**    say:  "gmake OS_CFLAGS+=-DNEED_NSS_ILOCK" at make time.
**
**    At runtime, to enable recording from nssilock, one or more
**    environment variables must be set. For each nssILockType to
**    be recorded, an environment variable of the form NSS_ILOCK_x
**    must be set to 1. For example:
**
**       set NSS_ILOCK_Cert=1
**
**    nssilock uses PRLOG is used to record to trace data. The
**    PRLogModule name associated with nssilock data is: "nssilock".
**    To enable recording of nssilock data you will need to set the
**    environment variable NSPR_LOG_MODULES to enable
**    recording for the nssilock log module. Similarly, you will
**    need to set the environment variable NSPR_LOG_FILE to specify
**    the filename to receive the recorded data. See prlog.h for usage.
**    Example:
**
**        export NSPR_LOG_MODULES=nssilock:6
**        export NSPR_LOG_FILE=xxxLogfile
**
** Operation:
**    nssilock wraps calls to NSPR's PZLock and PZMonitor functions
**    with similarly named functions: PZ_NewLock(), etc. When NSS is
**    built with lock instrumentation enabled, the PZ* functions are
**    compiled into NSS; when lock instrumentation is disabled,
**    calls to PZ* functions are directly mapped to PR* functions
**    and the instrumentation arguments to the PZ* functions are
**    compiled away.
**
**
** File Format:
**    The format of the external file is implementation
**    dependent. Where NSPR's PR_LOG() function is used, the file
**    contains data defined for PR_LOG() plus the data written by
**    the wrapped function. On some platforms and under some
**    circumstances, platform dependent logging or
**    instrumentation probes may be used. In any case, the
**    relevant data provided by the lock instrumentation is:
**
**      lockType, func, address, duration, line, file [heldTime]
**
**    where:
**
**       lockType: a character representation of nssILockType for the
**       call. e.g. ... "cert"
**
**       func: the function doing the tracing. e.g. "NewLock"
**
**       address: address of the instrumented lock or monitor
**
**       duration: is how long was spent in the instrumented function,
**       in PRIntervalTime "ticks".
**
**       line: the line number within the calling function
**
**       file: the file from which the call was made
**
**       heldTime: how long the lock/monitor was held. field
**       present only for PZ_Unlock() and PZ_ExitMonitor().
**
** Design Notes:
**    The design for lock instrumentation was influenced by the
**    need to gather performance data on NSS 3.x. It is intended
**    that the effort to modify NSS to use lock instrumentation
**    be minimized. Existing calls to locking functions need only
**    have their names changed to the instrumentation function
**    names.
**
** Private NSS Interface:
**    nssilock.h defines a private interface for use by NSS.
**    nssilock.h is experimental in nature and is subject to
**    change or revocation without notice. ... Don't mess with
**    it.
**
*/

/*
 * $Id:
 */

#ifndef _NSSILOCK_H_
#define _NSSILOCK_H_

#include "utilrename.h"
#include "prtypes.h"
#include "prmon.h"
#include "prlock.h"
#include "prcvar.h"

#include "nssilckt.h"

PR_BEGIN_EXTERN_C

#if defined(NEED_NSS_ILOCK)

#define PZ_NewLock(t) pz_NewLock((t), __FILE__, __LINE__)
extern PZLock *
pz_NewLock(
    nssILockType ltype,
    char *file,
    PRIntn line);

#define PZ_Lock(k) pz_Lock((k), __FILE__, __LINE__)
extern void
pz_Lock(
    PZLock *lock,
    char *file,
    PRIntn line);

#define PZ_Unlock(k) pz_Unlock((k), __FILE__, __LINE__)
extern PRStatus
pz_Unlock(
    PZLock *lock,
    char *file,
    PRIntn line);

#define PZ_DestroyLock(k) pz_DestroyLock((k), __FILE__, __LINE__)
extern void
pz_DestroyLock(
    PZLock *lock,
    char *file,
    PRIntn line);

#define PZ_NewCondVar(l) pz_NewCondVar((l), __FILE__, __LINE__)
extern PZCondVar *
pz_NewCondVar(
    PZLock *lock,
    char *file,
    PRIntn line);

#define PZ_DestroyCondVar(v) pz_DestroyCondVar((v), __FILE__, __LINE__)
extern void
pz_DestroyCondVar(
    PZCondVar *cvar,
    char *file,
    PRIntn line);

#define PZ_WaitCondVar(v, t) pz_WaitCondVar((v), (t), __FILE__, __LINE__)
extern PRStatus
pz_WaitCondVar(
    PZCondVar *cvar,
    PRIntervalTime timeout,
    char *file,
    PRIntn line);

#define PZ_NotifyCondVar(v) pz_NotifyCondVar((v), __FILE__, __LINE__)
extern PRStatus
pz_NotifyCondVar(
    PZCondVar *cvar,
    char *file,
    PRIntn line);

#define PZ_NotifyAllCondVar(v) pz_NotifyAllCondVar((v), __FILE__, __LINE__)
extern PRStatus
pz_NotifyAllCondVar(
    PZCondVar *cvar,
    char *file,
    PRIntn line);

#define PZ_NewMonitor(t) pz_NewMonitor((t), __FILE__, __LINE__)
extern PZMonitor *
pz_NewMonitor(
    nssILockType ltype,
    char *file,
    PRIntn line);

#define PZ_DestroyMonitor(m) pz_DestroyMonitor((m), __FILE__, __LINE__)
extern void
pz_DestroyMonitor(
    PZMonitor *mon,
    char *file,
    PRIntn line);

#define PZ_EnterMonitor(m) pz_EnterMonitor((m), __FILE__, __LINE__)
extern void
pz_EnterMonitor(
    PZMonitor *mon,
    char *file,
    PRIntn line);

#define PZ_ExitMonitor(m) pz_ExitMonitor((m), __FILE__, __LINE__)
extern PRStatus
pz_ExitMonitor(
    PZMonitor *mon,
    char *file,
    PRIntn line);

#define PZ_InMonitor(m) (PZ_GetMonitorEntryCount(m) > 0)
#define PZ_GetMonitorEntryCount(m) pz_GetMonitorEntryCount((m), __FILE__, __LINE__)
extern PRIntn
pz_GetMonitorEntryCount(
    PZMonitor *mon,
    char *file,
    PRIntn line);

#define PZ_Wait(m, i) pz_Wait((m), ((i)), __FILE__, __LINE__)
extern PRStatus
pz_Wait(
    PZMonitor *mon,
    PRIntervalTime ticks,
    char *file,
    PRIntn line);

#define PZ_Notify(m) pz_Notify((m), __FILE__, __LINE__)
extern PRStatus
pz_Notify(
    PZMonitor *mon,
    char *file,
    PRIntn line);

#define PZ_NotifyAll(m) pz_NotifyAll((m), __FILE__, __LINE__)
extern PRStatus
pz_NotifyAll(
    PZMonitor *mon,
    char *file,
    PRIntn line);

#define PZ_TraceFlush() pz_TraceFlush()
extern void pz_TraceFlush(void);

#else /* NEED_NSS_ILOCK */

#define PZ_NewLock(t) PR_NewLock()
#define PZ_DestroyLock(k) PR_DestroyLock((k))
#define PZ_Lock(k) PR_Lock((k))
#define PZ_Unlock(k) PR_Unlock((k))

#define PZ_NewCondVar(l) PR_NewCondVar((l))
#define PZ_DestroyCondVar(v) PR_DestroyCondVar((v))
#define PZ_WaitCondVar(v, t) PR_WaitCondVar((v), (t))
#define PZ_NotifyCondVar(v) PR_NotifyCondVar((v))
#define PZ_NotifyAllCondVar(v) PR_NotifyAllCondVar((v))

#define PZ_NewMonitor(t) PR_NewMonitor()
#define PZ_DestroyMonitor(m) PR_DestroyMonitor((m))
#define PZ_EnterMonitor(m) PR_EnterMonitor((m))
#define PZ_ExitMonitor(m) PR_ExitMonitor((m))
#define PZ_InMonitor(m) PR_InMonitor((m))
#define PZ_Wait(m, t) PR_Wait(((m)), ((t)))
#define PZ_Notify(m) PR_Notify((m))
#define PZ_NotifyAll(m) PR_Notify((m))
#define PZ_TraceFlush() /* nothing */

#endif /* NEED_NSS_ILOCK */

PR_END_EXTERN_C
#endif /* _NSSILOCK_H_ */
