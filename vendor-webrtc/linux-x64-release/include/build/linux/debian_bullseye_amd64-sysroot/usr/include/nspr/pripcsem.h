/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * File: pripcsem.h
 *
 * Description: named semaphores for interprocess
 * synchronization
 *
 * Unrelated processes obtain access to a shared semaphore
 * by specifying its name.
 *
 * Our goal is to support named semaphores on at least
 * Unix and Win32 platforms.  The implementation will use
 * one of the three native semaphore APIs: POSIX, System V,
 * and Win32.
 *
 * Because POSIX named semaphores have kernel persistence,
 * we are forced to have a delete function in this API.
 */

#ifndef pripcsem_h___
#define pripcsem_h___

#include "prtypes.h"
#include "prio.h"

PR_BEGIN_EXTERN_C

/*
 * PRSem is an opaque structure that represents a named
 * semaphore.
 */
typedef struct PRSem PRSem;

/*
 * PR_OpenSemaphore --
 *
 * Create or open a named semaphore with the specified name.
 * A handle to the semaphore is returned.
 *
 * If the named semaphore doesn't exist and the PR_SEM_CREATE
 * flag is specified, the named semaphore is created.  The
 * created semaphore needs to be removed from the system with
 * a PR_DeleteSemaphore call.
 *
 * If PR_SEM_CREATE is specified, the third argument is the
 * access permission bits of the new semaphore (same
 * interpretation as the mode argument to PR_Open) and the
 * fourth argument is the initial value of the new semaphore.
 * If PR_SEM_CREATE is not specified, the third and fourth
 * arguments are ignored.
 */

#define PR_SEM_CREATE 0x1  /* create if not exist */
#define PR_SEM_EXCL   0x2  /* fail if already exists */

NSPR_API(PRSem *) PR_OpenSemaphore(
    const char *name, PRIntn flags, PRIntn mode, PRUintn value);

/*
 * PR_WaitSemaphore --
 *
 * If the value of the semaphore is > 0, decrement the value and return.
 * If the value is 0, sleep until the value becomes > 0, then decrement
 * the value and return.
 *
 * The "test and decrement" operation is performed atomically.
 */

NSPR_API(PRStatus) PR_WaitSemaphore(PRSem *sem);

/*
 * PR_PostSemaphore --
 *
 * Increment the value of the named semaphore by 1.
 */

NSPR_API(PRStatus) PR_PostSemaphore(PRSem *sem);

/*
 * PR_CloseSemaphore --
 *
 * Close a named semaphore handle.
 */

NSPR_API(PRStatus) PR_CloseSemaphore(PRSem *sem);

/*
 * PR_DeleteSemaphore --
 *
 * Remove a named semaphore from the system.
 */

NSPR_API(PRStatus) PR_DeleteSemaphore(const char *name);

PR_END_EXTERN_C

#endif /* pripcsem_h___ */
