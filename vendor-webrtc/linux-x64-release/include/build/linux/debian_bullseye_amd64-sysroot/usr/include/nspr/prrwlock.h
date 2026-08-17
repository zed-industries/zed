/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
** File:        prrwlock.h
** Description: API to basic reader-writer lock functions of NSPR.
**
**/

#ifndef prrwlock_h___
#define prrwlock_h___

#include "prtypes.h"

PR_BEGIN_EXTERN_C

/*
 * PRRWLock --
 *
 *  The reader writer lock, PRRWLock, is an opaque object to the clients
 *  of NSPR.  All routines operate on a pointer to this opaque entity.
 */


typedef struct PRRWLock PRRWLock;

#define PR_RWLOCK_RANK_NONE 0


/***********************************************************************
** FUNCTION:    PR_NewRWLock
** DESCRIPTION:
**  Returns a pointer to a newly created reader-writer lock object.
** INPUTS:      Lock rank
**              Lock name
** OUTPUTS:     void
** RETURN:      PRRWLock*
**   If the lock cannot be created because of resource constraints, NULL
**   is returned.
**
***********************************************************************/
NSPR_API(PRRWLock*) PR_NewRWLock(PRUint32 lock_rank, const char *lock_name);

/***********************************************************************
** FUNCTION:    PR_DestroyRWLock
** DESCRIPTION:
**  Destroys a given RW lock object.
** INPUTS:      PRRWLock *lock - Lock to be freed.
** OUTPUTS:     void
** RETURN:      None
***********************************************************************/
NSPR_API(void) PR_DestroyRWLock(PRRWLock *lock);

/***********************************************************************
** FUNCTION:    PR_RWLock_Rlock
** DESCRIPTION:
**  Apply a read lock (non-exclusive) on a RWLock
** INPUTS:      PRRWLock *lock - Lock to be read-locked.
** OUTPUTS:     void
** RETURN:      None
***********************************************************************/
NSPR_API(void) PR_RWLock_Rlock(PRRWLock *lock);

/***********************************************************************
** FUNCTION:    PR_RWLock_Wlock
** DESCRIPTION:
**  Apply a write lock (exclusive) on a RWLock
** INPUTS:      PRRWLock *lock - Lock to write-locked.
** OUTPUTS:     void
** RETURN:      None
***********************************************************************/
NSPR_API(void) PR_RWLock_Wlock(PRRWLock *lock);

/***********************************************************************
** FUNCTION:    PR_RWLock_Unlock
** DESCRIPTION:
**  Release a RW lock. Unlocking an unlocked lock has undefined results.
** INPUTS:      PRRWLock *lock - Lock to unlocked.
** OUTPUTS:     void
** RETURN:      void
***********************************************************************/
NSPR_API(void) PR_RWLock_Unlock(PRRWLock *lock);

PR_END_EXTERN_C

#endif /* prrwlock_h___ */
