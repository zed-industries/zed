/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prolock_h___
#define prolock_h___

#include "prtypes.h"

PR_BEGIN_EXTERN_C

/*
** A locking mechanism, built on the existing PRLock definition,
** is provided that will permit applications to define a Lock
** Hierarchy (or Lock Ordering) schema. An application designed
** using the Ordered Lock functions will terminate with a
** diagnostic message when a lock inversion condition is
** detected.
**
** The lock ordering detection is compile-time enabled only. In
** optimized builds of NSPR, the Ordered Lock functions map
** directly to PRLock functions, providing no lock order
** detection.
**
** The Ordered Lock Facility is compiled in when DEBUG is defined at
** compile-time. Ordered Lock can be forced on in optimized builds by
** defining FORCE_NSPR_ORDERED_LOCK at compile-time. Both the
** application using Ordered Lock and NSPR must be compiled with the
** facility enabled to achieve the desired results.
**
** Application designers should use the macro interfaces to the Ordered
** Lock facility to ensure that it is compiled out in optimized builds.
**
** Application designers are responsible for defining their own
** lock hierarchy.
**
** Ordered Lock is thread-safe and SMP safe.
**
** See Also: prlock.h
**
** /lth. 10-Jun-1998.
**
*/

/*
** Opaque type for ordered lock.
** ... Don't even think of looking in here.
**
*/

#if defined(DEBUG) || defined(FORCE_NSPR_ORDERED_LOCKS)
typedef void * PROrderedLock;
#else
/*
** Map PROrderedLock and methods onto PRLock when ordered locking
** is not compiled in.
**
*/
#include "prlock.h"

typedef PRLock PROrderedLock;
#endif

/* -----------------------------------------------------------------------
** FUNCTION: PR_CreateOrderedLock() -- Create an Ordered Lock
**
** DESCRIPTION: PR_CreateOrderedLock() creates an ordered lock.
**
** INPUTS:
**  order: user defined order of this lock.
**  name: name of the lock. For debugging purposes.
**
** OUTPUTS: returned
**
** RETURNS: PR_OrderedLock pointer
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_ORDERED_LOCKS)
#define PR_CREATE_ORDERED_LOCK(order,name)\
    PR_CreateOrderedLock((order),(name))
#else
#define PR_CREATE_ORDERED_LOCK(order) PR_NewLock()
#endif

NSPR_API(PROrderedLock *)
PR_CreateOrderedLock(
    PRInt32 order,
    const char *name
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_DestroyOrderedLock() -- Destroy an Ordered Lock
**
** DESCRIPTION: PR_DestroyOrderedLock() destroys the ordered lock
** referenced by lock.
**
** INPUTS: lock: pointer to a PROrderedLock
**
** OUTPUTS: the lock is destroyed
**
** RETURNS: void
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_ORDERED_LOCKS)
#define PR_DESTROY_ORDERED_LOCK(lock) PR_DestroyOrderedLock((lock))
#else
#define PR_DESTROY_ORDERED_LOCK(lock) PR_DestroyLock((lock))
#endif

NSPR_API(void)
PR_DestroyOrderedLock(
    PROrderedLock *lock
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_LockOrderedLock() -- Lock an ordered lock
**
** DESCRIPTION: PR_LockOrderedLock() locks the ordered lock
** referenced by lock. If the order of lock is less than or equal
** to the order of the highest lock held by the locking thread,
** the function asserts.
**
** INPUTS: lock: a pointer to a PROrderedLock
**
** OUTPUTS: The lock is held or the function asserts.
**
** RETURNS: void
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_ORDERED_LOCKS)
#define PR_LOCK_ORDERED_LOCK(lock) PR_LockOrderedLock((lock))
#else
#define PR_LOCK_ORDERED_LOCK(lock) PR_Lock((lock))
#endif

NSPR_API(void)
PR_LockOrderedLock(
    PROrderedLock *lock
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_UnlockOrderedLock() -- unlock and Ordered Lock
**
** DESCRIPTION: PR_UnlockOrderedLock() unlocks the lock referenced
** by lock.
**
** INPUTS: lock: a pointer to a PROrderedLock
**
** OUTPUTS: the lock is unlocked
**
** RETURNS:
**  PR_SUCCESS
**  PR_FAILURE
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_ORDERED_LOCKS)
#define PR_UNLOCK_ORDERED_LOCK(lock) PR_UnlockOrderedLock((lock))
#else
#define PR_UNLOCK_ORDERED_LOCK(lock) PR_Unlock((lock))
#endif

NSPR_API(PRStatus)
PR_UnlockOrderedLock(
    PROrderedLock *lock
);

PR_END_EXTERN_C

#endif /* prolock_h___ */
