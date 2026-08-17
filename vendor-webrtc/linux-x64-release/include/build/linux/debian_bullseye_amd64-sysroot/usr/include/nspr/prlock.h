/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
** File:        prlock.h
** Description: API to basic locking functions of NSPR.
**
**
** NSPR provides basic locking mechanisms for thread synchronization.  Locks
** are lightweight resource contention controls that prevent multiple threads
** from accessing something (code/data) simultaneously.
**/

#ifndef prlock_h___
#define prlock_h___

#include "prtypes.h"

PR_BEGIN_EXTERN_C

/**********************************************************************/
/************************* TYPES AND CONSTANTS ************************/
/**********************************************************************/

/*
 * PRLock --
 *
 *     NSPR represents the lock as an opaque entity to the client of the
 *     API.  All routines operate on a pointer to this opaque entity.
 */

typedef struct PRLock PRLock;

/**********************************************************************/
/****************************** FUNCTIONS *****************************/
/**********************************************************************/

/***********************************************************************
** FUNCTION:    PR_NewLock
** DESCRIPTION:
**  Returns a pointer to a newly created opaque lock object.
** INPUTS:      void
** OUTPUTS:     void
** RETURN:      PRLock*
**   If the lock can not be created because of resource constraints, NULL
**   is returned.
**
***********************************************************************/
NSPR_API(PRLock*) PR_NewLock(void);

/***********************************************************************
** FUNCTION:    PR_DestroyLock
** DESCRIPTION:
**  Destroys a given opaque lock object.
** INPUTS:      PRLock *lock
**              Lock to be freed.
** OUTPUTS:     void
** RETURN:      None
***********************************************************************/
NSPR_API(void) PR_DestroyLock(PRLock *lock);

/***********************************************************************
** FUNCTION:    PR_Lock
** DESCRIPTION:
**  Lock a lock.
** INPUTS:      PRLock *lock
**              Lock to locked.
** OUTPUTS:     void
** RETURN:      None
***********************************************************************/
NSPR_API(void) PR_Lock(PRLock *lock);

/***********************************************************************
** FUNCTION:    PR_Unlock
** DESCRIPTION:
**  Unlock a lock.  Unlocking an unlocked lock has undefined results.
** INPUTS:      PRLock *lock
**              Lock to unlocked.
** OUTPUTS:     void
** RETURN:      PR_STATUS
**              Returns PR_FAILURE if the caller does not own the lock.
***********************************************************************/
NSPR_API(PRStatus) PR_Unlock(PRLock *lock);

/***********************************************************************
** MACRO:    PR_ASSERT_CURRENT_THREAD_OWNS_LOCK
** DESCRIPTION:
**  If the current thread owns |lock|, this assertion is guaranteed to
**  succeed.  Otherwise, the behavior of this function is undefined.
** INPUTS:      PRLock *lock
**              Lock to assert ownership of.
** OUTPUTS:     void
** RETURN:      None
***********************************************************************/
#if defined(DEBUG) || defined(FORCE_PR_ASSERT)
#define PR_ASSERT_CURRENT_THREAD_OWNS_LOCK(/* PrLock* */ lock) \
    PR_AssertCurrentThreadOwnsLock(lock)
#else
#define PR_ASSERT_CURRENT_THREAD_OWNS_LOCK(/* PrLock* */ lock)
#endif

/* Don't call this function directly. */
NSPR_API(void) PR_AssertCurrentThreadOwnsLock(PRLock *lock);

PR_END_EXTERN_C

#endif /* prlock_h___ */
