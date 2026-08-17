/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
** File:        nsrwlock.h
** Description: API to basic reader-writer lock functions of NSS.
**  These are re-entrant reader writer locks; that is,
**  If I hold the write lock, I can ask for it and get it again.
**  If I hold the write lock, I can also ask for and get a read lock.
**      I can then release the locks in any order (read or write).
**  I must release each lock type as many times as I acquired it.
**  Otherwise, these are normal reader/writer locks.
**
** For deadlock detection, locks should be ranked, and no lock may be aquired
** while I hold a lock of higher rank number.
** If you don't want that feature, always use NSS_RWLOCK_RANK_NONE.
** Lock name is for debugging, and is optional (may be NULL)
**/

#ifndef nssrwlk_h___
#define nssrwlk_h___

#include "utilrename.h"
#include "prtypes.h"
#include "nssrwlkt.h"

#define NSS_RWLOCK_RANK_NONE 0

/* SEC_BEGIN_PROTOS */
PR_BEGIN_EXTERN_C

/***********************************************************************
** FUNCTION:    NSSRWLock_New
** DESCRIPTION:
**  Returns a pointer to a newly created reader-writer lock object.
** INPUTS:      Lock rank
**      Lock name
** OUTPUTS:     void
** RETURN:      NSSRWLock*
**   If the lock cannot be created because of resource constraints, NULL
**   is returned.
**
***********************************************************************/
extern NSSRWLock *NSSRWLock_New(PRUint32 lock_rank, const char *lock_name);

/***********************************************************************
** FUNCTION:    NSSRWLock_AtomicCreate
** DESCRIPTION:
**  Given the address of a NULL pointer to a NSSRWLock,
**  atomically initializes that pointer to a newly created NSSRWLock.
**  Returns the value placed into that pointer, or NULL.
**
** INPUTS:      address of NSRWLock pointer
**              Lock rank
**      Lock name
** OUTPUTS:     NSSRWLock*
** RETURN:      NSSRWLock*
**   If the lock cannot be created because of resource constraints,
**   the pointer will be left NULL.
**
***********************************************************************/
extern NSSRWLock *
nssRWLock_AtomicCreate(NSSRWLock **prwlock,
                       PRUint32 lock_rank,
                       const char *lock_name);

/***********************************************************************
** FUNCTION:    NSSRWLock_Destroy
** DESCRIPTION:
**  Destroys a given RW lock object.
** INPUTS:      NSSRWLock *lock - Lock to be freed.
** OUTPUTS:     void
** RETURN:      None
***********************************************************************/
extern void NSSRWLock_Destroy(NSSRWLock *lock);

/***********************************************************************
** FUNCTION:    NSSRWLock_LockRead
** DESCRIPTION:
**  Apply a read lock (non-exclusive) on a RWLock
** INPUTS:      NSSRWLock *lock - Lock to be read-locked.
** OUTPUTS:     void
** RETURN:      None
***********************************************************************/
extern void NSSRWLock_LockRead(NSSRWLock *lock);

/***********************************************************************
** FUNCTION:    NSSRWLock_LockWrite
** DESCRIPTION:
**  Apply a write lock (exclusive) on a RWLock
** INPUTS:      NSSRWLock *lock - Lock to write-locked.
** OUTPUTS:     void
** RETURN:      None
***********************************************************************/
extern void NSSRWLock_LockWrite(NSSRWLock *lock);

/***********************************************************************
** FUNCTION:    NSSRWLock_UnlockRead
** DESCRIPTION:
**  Release a Read lock. Unlocking an unlocked lock has undefined results.
** INPUTS:      NSSRWLock *lock - Lock to unlocked.
** OUTPUTS:     void
** RETURN:      void
***********************************************************************/
extern void NSSRWLock_UnlockRead(NSSRWLock *lock);

/***********************************************************************
** FUNCTION:    NSSRWLock_UnlockWrite
** DESCRIPTION:
**  Release a Write lock. Unlocking an unlocked lock has undefined results.
** INPUTS:      NSSRWLock *lock - Lock to unlocked.
** OUTPUTS:     void
** RETURN:      void
***********************************************************************/
extern void NSSRWLock_UnlockWrite(NSSRWLock *lock);

/***********************************************************************
** FUNCTION:    NSSRWLock_HaveWriteLock
** DESCRIPTION:
**  Tells caller whether the current thread holds the write lock, or not.
** INPUTS:      NSSRWLock *lock - Lock to test.
** OUTPUTS:     void
** RETURN:      PRBool  PR_TRUE IFF the current thread holds the write lock.
***********************************************************************/

extern PRBool NSSRWLock_HaveWriteLock(NSSRWLock *rwlock);

/* SEC_END_PROTOS */
PR_END_EXTERN_C

#endif /* nsrwlock_h___ */
