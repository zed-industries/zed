/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prcmon_h___
#define prcmon_h___

/*
** Interface to cached monitors. Cached monitors use an address to find a
** given PR monitor. In this way a monitor can be associated with another
** object without preallocating a monitor for all objects.
**
** A hash table is used to quickly map addresses to individual monitors
** and the system automatically grows the hash table as needed.
**
** Cache monitors are about 5 times slower to use than uncached monitors.
*/
#include "prmon.h"
#include "prinrval.h"

PR_BEGIN_EXTERN_C

/**
** Like PR_EnterMonitor except use the "address" to find a monitor in the
** monitor cache. If successful, returns the PRMonitor now associated
** with "address". Note that you must PR_CExitMonitor the address to
** release the monitor cache entry (otherwise the monitor cache will fill
** up). This call will return NULL if the monitor cache needs to be
** expanded and the system is out of memory.
*/
NSPR_API(PRMonitor*) PR_CEnterMonitor(void *address);

/*
** Like PR_ExitMonitor except use the "address" to find a monitor in the
** monitor cache.
*/
NSPR_API(PRStatus) PR_CExitMonitor(void *address);

/*
** Like PR_Wait except use the "address" to find a monitor in the
** monitor cache.
*/
NSPR_API(PRStatus) PR_CWait(void *address, PRIntervalTime timeout);

/*
** Like PR_Notify except use the "address" to find a monitor in the
** monitor cache.
*/
NSPR_API(PRStatus) PR_CNotify(void *address);

/*
** Like PR_NotifyAll except use the "address" to find a monitor in the
** monitor cache.
*/
NSPR_API(PRStatus) PR_CNotifyAll(void *address);

/*
** Set a callback to be invoked each time a monitor is recycled from the cache
** freelist, with the monitor's cache-key passed in address.
*/
NSPR_API(void) PR_CSetOnMonitorRecycle(void (PR_CALLBACK *callback)(void *address));

PR_END_EXTERN_C

#endif /* prcmon_h___ */
