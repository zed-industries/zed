/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prsem_h___
#define prsem_h___

/*
** API for counting semaphores. Semaphores are counting synchronizing
** variables based on a lock and a condition variable.  They are lightweight
** contention control for a given count of resources.
*/
#include "prtypes.h"

PR_BEGIN_EXTERN_C

typedef struct PRSemaphore PRSemaphore;

/*
** Create a new semaphore object.
*/
NSPR_API(PRSemaphore*) PR_NewSem(PRUintn value);

/*
** Destroy the given semaphore object.
**
*/
NSPR_API(void) PR_DestroySem(PRSemaphore *sem);

/*
** Wait on a Semaphore.
**
** This routine allows a calling thread to wait or proceed depending upon the
** state of the semahore sem. The thread can proceed only if the counter value
** of the semaphore sem is currently greater than 0. If the value of semaphore
** sem is positive, it is decremented by one and the routine returns immediately
** allowing the calling thread to continue. If the value of semaphore sem is 0,
** the calling thread blocks awaiting the semaphore to be released by another
** thread.
**
** This routine can return PR_PENDING_INTERRUPT if the waiting thread
** has been interrupted.
*/
NSPR_API(PRStatus) PR_WaitSem(PRSemaphore *sem);

/*
** This routine increments the counter value of the semaphore. If other threads
** are blocked for the semaphore, then the scheduler will determine which ONE
** thread will be unblocked.
*/
NSPR_API(void) PR_PostSem(PRSemaphore *sem);

/*
** Returns the value of the semaphore referenced by sem without affecting
** the state of the semaphore.  The value represents the semaphore vaule
F** at the time of the call, but may not be the actual value when the
** caller inspects it.
*/
NSPR_API(PRUintn) PR_GetValueSem(PRSemaphore *sem);

PR_END_EXTERN_C

#endif /* prsem_h___ */
