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

#ifndef _NSSILCKT_H_
#define _NSSILCKT_H_

#include "utilrename.h"
#include "prtypes.h"
#include "prmon.h"
#include "prlock.h"
#include "prcvar.h"

typedef enum {
    nssILockArena = 0,
    nssILockSession = 1,
    nssILockObject = 2,
    nssILockRefLock = 3,
    nssILockCert = 4,
    nssILockCertDB = 5,
    nssILockDBM = 6,
    nssILockCache = 7,
    nssILockSSL = 8,
    nssILockList = 9,
    nssILockSlot = 10,
    nssILockFreelist = 11,
    nssILockOID = 12,
    nssILockAttribute = 13,
    nssILockPK11cxt = 14, /* pk11context */
    nssILockRWLock = 15,
    nssILockOther = 16,
    nssILockSelfServ = 17,
    nssILockKeyDB = 18,
    nssILockLast /* don't use this one! */
} nssILockType;

/*
** conditionally compile in nssilock features
*/
#if defined(NEED_NSS_ILOCK)

/*
** Declare operation type enumerator
** enumerations identify the function being performed
*/
typedef enum {
    FlushTT = 0,
    NewLock = 1,
    Lock = 2,
    Unlock = 3,
    DestroyLock = 4,
    NewCondVar = 5,
    WaitCondVar = 6,
    NotifyCondVar = 7,
    NotifyAllCondVar = 8,
    DestroyCondVar = 9,
    NewMonitor = 10,
    EnterMonitor = 11,
    ExitMonitor = 12,
    Notify = 13,
    NotifyAll = 14,
    Wait = 15,
    DestroyMonitor = 16
} nssILockOp;

/*
** Declare the trace record
*/
struct pzTrace_s {
    PRUint32 threadID;       /* PR_GetThreadID() */
    nssILockOp op;           /* operation being performed */
    nssILockType ltype;      /* lock type identifier */
    PRIntervalTime callTime; /* time spent in function */
    PRIntervalTime heldTime; /* lock held time, or -1 */
    void *lock;              /* address of lock structure */
    PRIntn line;             /* line number */
    char file[24];           /* filename */
};

/*
** declare opaque types. See: nssilock.c
*/
typedef struct pzlock_s PZLock;
typedef struct pzcondvar_s PZCondVar;
typedef struct pzmonitor_s PZMonitor;

#else /* NEED_NSS_ILOCK */

#define PZLock PRLock
#define PZCondVar PRCondVar
#define PZMonitor PRMonitor

#endif /* NEED_NSS_ILOCK */

#endif /* _NSSILCKT_H_ */
