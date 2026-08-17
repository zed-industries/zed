/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * File:        prpdce.h
 * Description: This file is the API defined to allow for DCE (aka POSIX)
 *              thread emulation in an NSPR environment. It is not the
 *              intent that this be a fully supported API.
 */

#if !defined(PRPDCE_H)
#define PRPDCE_H

#include "prlock.h"
#include "prcvar.h"
#include "prtypes.h"
#include "prinrval.h"

PR_BEGIN_EXTERN_C

#define _PR_NAKED_CV_LOCK (PRLock*)0xdce1dce1

/*
** Test and acquire a lock.
**
** If the lock is acquired by the calling thread, the
** return value will be PR_SUCCESS. If the lock is
** already held, by another thread or this thread, the
** result will be PR_FAILURE.
*/
NSPR_API(PRStatus) PRP_TryLock(PRLock *lock);

/*
** Create a naked condition variable
**
** A "naked" condition variable is one that is not created bound
** to a lock. The CV created with this function is the only type
** that may be used in the subsequent "naked" condition variable
** operations (see PRP_NakedWait, PRP_NakedNotify, PRP_NakedBroadcast);
*/
NSPR_API(PRCondVar*) PRP_NewNakedCondVar(void);

/*
** Destroy a naked condition variable
**
** Destroy the condition variable created by PR_NewNakedCondVar.
*/
NSPR_API(void) PRP_DestroyNakedCondVar(PRCondVar *cvar);

/*
** Wait on a condition
**
** Wait on the condition variable 'cvar'. It is asserted that
** the lock protecting the condition 'lock' is held by the
** calling thread. If more time expires than that declared in
** 'timeout' the condition will be notified. Waits can be
** interrupted by another thread.
**
** NB: The CV ('cvar') must be one created using PR_NewNakedCondVar.
*/
NSPR_API(PRStatus) PRP_NakedWait(
    PRCondVar *cvar, PRLock *lock, PRIntervalTime timeout);

/*
** Notify a thread waiting on a condition
**
** Notify the condition specified 'cvar'.
**
** NB: The CV ('cvar') must be one created using PR_NewNakedCondVar.
*/
NSPR_API(PRStatus) PRP_NakedNotify(PRCondVar *cvar);

/*
** Notify all threads waiting on a condition
**
** Notify the condition specified 'cvar'.
**
** NB: The CV ('cvar') must be one created using PR_NewNakedCondVar.
*/
NSPR_API(PRStatus) PRP_NakedBroadcast(PRCondVar *cvar);

PR_END_EXTERN_C

#endif /* PRPDCE_H */
