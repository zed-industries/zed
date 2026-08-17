/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prinit_h___
#define prinit_h___

#include "prthread.h"
#include "prtypes.h"
#include "prwin16.h"
#include <stdio.h>

PR_BEGIN_EXTERN_C

/************************************************************************/
/**************************IDENTITY AND VERSIONING***********************/
/************************************************************************/

/*
** NSPR's name, this should persist until at least the turn of the
** century.
*/
#define PR_NAME     "NSPR"

/*
** NSPR's version is used to determine the likelihood that the version you
** used to build your component is anywhere close to being compatible with
** what is in the underlying library.
**
** The format of the version string is
**     "<major version>.<minor version>[.<patch level>] [<Beta>]"
*/
#define PR_VERSION  "4.29"
#define PR_VMAJOR   4
#define PR_VMINOR   29
#define PR_VPATCH   0
#define PR_BETA     PR_FALSE

/*
** PRVersionCheck
**
** The basic signature of the function that is called to provide version
** checking. The result will be a boolean that indicates the likelihood
** that the underling library will perform as the caller expects.
**
** The only argument is a string, which should be the verson identifier
** of the library in question. That string will be compared against an
** equivalent string that represents the actual build version of the
** exporting library.
**
** The result will be the logical union of the directly called library
** and all dependent libraries.
*/

typedef PRBool (*PRVersionCheck)(const char*);

/*
** PR_VersionCheck
**
** NSPR's existance proof of the version check function.
**
** Note that NSPR has no cooperating dependencies.
*/

NSPR_API(PRBool) PR_VersionCheck(const char *importedVersion);

/*
 * Returns a const string of the NSPR library version.
 */
NSPR_API(const char*) PR_GetVersion(void);


/************************************************************************/
/*******************************INITIALIZATION***************************/
/************************************************************************/

/*
** Initialize the runtime. Attach a thread object to the currently
** executing native thread of type "type".
**
** The specificaiton of 'maxPTDs' is ignored.
*/
NSPR_API(void) PR_Init(
    PRThreadType type, PRThreadPriority priority, PRUintn maxPTDs);

/*
** And alternate form of initialization, one that may become the default if
** not the only mechanism, provides a method to get the NSPR runtime init-
** ialized and place NSPR between the caller and the runtime library. This
** allows main() to be treated as any other thread root function, signalling
** its compeletion by returning and allowing the runtime to coordinate the
** completion of the other threads of the runtime.
**
** The priority of the main (or primordial) thread will be PR_PRIORITY_NORMAL.
** The thread may adjust its own priority by using PR_SetPriority(), though
** at this time the support for priorities is somewhat weak.
**
** The specificaiton of 'maxPTDs' is ignored.
**
** The value returned by PR_Initialize is the value returned from the root
** function, 'prmain'.
*/

typedef PRIntn (PR_CALLBACK *PRPrimordialFn)(PRIntn argc, char **argv);

NSPR_API(PRIntn) PR_Initialize(
    PRPrimordialFn prmain, PRIntn argc, char **argv, PRUintn maxPTDs);

/*
** Return PR_TRUE if PR_Init has already been called.
*/
NSPR_API(PRBool) PR_Initialized(void);

/*
 * Perform a graceful shutdown of NSPR.  PR_Cleanup() may be called by
 * the primordial thread near the end of the main() function.
 *
 * PR_Cleanup() attempts to synchronize the natural termination of
 * process.  It does that by blocking the caller, if and only if it is
 * the primordial thread, until the number of user threads has dropped
 * to zero.  When the primordial thread returns from main(), the process
 * will immediately and silently exit.  That is, it will (if necessary)
 * forcibly terminate any existing threads and exit without significant
 * blocking and there will be no error messages or core files.
 *
 * PR_Cleanup() returns PR_SUCCESS if NSPR is successfully shutdown,
 * or PR_FAILURE if the calling thread of this function is not the
 * primordial thread.
 */
NSPR_API(PRStatus) PR_Cleanup(void);

/*
** Disable Interrupts
**      Disables timer signals used for pre-emptive scheduling.
*/
NSPR_API(void) PR_DisableClockInterrupts(void);

/*
** Enables Interrupts
**      Enables timer signals used for pre-emptive scheduling.
*/
NSPR_API(void) PR_EnableClockInterrupts(void);

/*
** Block Interrupts
**      Blocks the timer signal used for pre-emptive scheduling
*/
NSPR_API(void) PR_BlockClockInterrupts(void);

/*
** Unblock Interrupts
**      Unblocks the timer signal used for pre-emptive scheduling
*/
NSPR_API(void) PR_UnblockClockInterrupts(void);

/*
** Create extra virtual processor threads. Generally used with MP systems.
*/
NSPR_API(void) PR_SetConcurrency(PRUintn numCPUs);

/*
** Control the method and size of the file descriptor (PRFileDesc*)
** cache used by the runtime. Setting 'high' to zero is for performance,
** any other value probably for debugging (see memo on FD caching).
*/
NSPR_API(PRStatus) PR_SetFDCacheSize(PRIntn low, PRIntn high);

/*
 * Cause an immediate, nongraceful, forced termination of the process.
 * It takes a PRIntn argument, which is the exit status code of the
 * process.
 */
NSPR_API(void) PR_ProcessExit(PRIntn status);

/*
** Abort the process in a non-graceful manner. This will cause a core file,
** call to the debugger or other moral equivalent as well as causing the
** entire process to stop.
*/
NSPR_API(void) PR_Abort(void);

/*
 ****************************************************************
 *
 * Module initialization:
 *
 ****************************************************************
 */

typedef struct PRCallOnceType {
    PRIntn initialized;
    PRInt32 inProgress;
    PRStatus status;
} PRCallOnceType;

typedef PRStatus (PR_CALLBACK *PRCallOnceFN)(void);

typedef PRStatus (PR_CALLBACK *PRCallOnceWithArgFN)(void *arg);

NSPR_API(PRStatus) PR_CallOnce(
    PRCallOnceType *once,
    PRCallOnceFN    func
);

NSPR_API(PRStatus) PR_CallOnceWithArg(
    PRCallOnceType      *once,
    PRCallOnceWithArgFN  func,
    void                *arg
);


PR_END_EXTERN_C

#endif /* prinit_h___ */
