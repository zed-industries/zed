/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef nssrwlkt_h___
#define nssrwlkt_h___

#include "utilrename.h"
#include "nssilock.h"
/*
 * NSSRWLock --
 *
 *  The reader writer lock, NSSRWLock, is an opaque object to the clients
 *  of NSS.  All routines operate on a pointer to this opaque entity.
 */

typedef struct nssRWLockStr NSSRWLock;

#endif /* nsrwlock_h___ */
