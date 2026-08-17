/*
 * Atomic int and pointer operations.  Originally copied from HarfBuzz.
 *
 * Copyright © 2007  Chris Wilson
 * Copyright © 2009,2010  Red Hat, Inc.
 * Copyright © 2011,2012,2013  Google, Inc.
 *
 * Permission is hereby granted, without written agreement and without
 * license or royalty fees, to use, copy, modify, and distribute this
 * software and its documentation for any purpose, provided that the
 * above copyright notice and the following two paragraphs appear in
 * all copies of this software.
 *
 * IN NO EVENT SHALL THE COPYRIGHT HOLDER BE LIABLE TO ANY PARTY FOR
 * DIRECT, INDIRECT, SPECIAL, INCIDENTAL, OR CONSEQUENTIAL DAMAGES
 * ARISING OUT OF THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN
 * IF THE COPYRIGHT HOLDER HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 *
 * THE COPYRIGHT HOLDER SPECIFICALLY DISCLAIMS ANY WARRANTIES, INCLUDING,
 * BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
 * FITNESS FOR A PARTICULAR PURPOSE.  THE SOFTWARE PROVIDED HEREUNDER IS
 * ON AN "AS IS" BASIS, AND THE COPYRIGHT HOLDER HAS NO OBLIGATION TO
 * PROVIDE MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS, OR MODIFICATIONS.
 *
 * Contributor(s):
 *	Chris Wilson <chris@chris-wilson.co.uk>
 * Red Hat Author(s): Behdad Esfahbod
 * Google Author(s): Behdad Esfahbod
 */

#ifndef _FCMUTEX_H_
#define _FCMUTEX_H_

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#define FC_STMT_START do
#define FC_STMT_END while (0)

/* mutex */

/* We need external help for these */

#if 0


#elif !defined(FC_NO_MT) && defined(_MSC_VER) || defined(__MINGW32__)

#include "fcwindows.h"
typedef CRITICAL_SECTION fc_mutex_impl_t;
#define FC_MUTEX_IMPL_INIT	{ NULL, 0, 0, NULL, NULL, 0 }
#define fc_mutex_impl_init(M)	InitializeCriticalSection (M)
#define fc_mutex_impl_lock(M)	EnterCriticalSection (M)
#define fc_mutex_impl_unlock(M)	LeaveCriticalSection (M)
#define fc_mutex_impl_finish(M)	DeleteCriticalSection (M)


#elif !defined(FC_NO_MT) && (defined(HAVE_PTHREAD) || defined(__APPLE__))

#include <pthread.h>
typedef pthread_mutex_t fc_mutex_impl_t;
#define FC_MUTEX_IMPL_INIT	PTHREAD_MUTEX_INITIALIZER
#define fc_mutex_impl_init(M)	pthread_mutex_init (M, NULL)
#define fc_mutex_impl_lock(M)	pthread_mutex_lock (M)
#define fc_mutex_impl_unlock(M)	pthread_mutex_unlock (M)
#define fc_mutex_impl_finish(M)	pthread_mutex_destroy (M)


#elif !defined(FC_NO_MT) && defined(HAVE_INTEL_ATOMIC_PRIMITIVES)

#if defined(HAVE_SCHED_H) && defined(HAVE_SCHED_YIELD)
# include <sched.h>
# define FC_SCHED_YIELD() sched_yield ()
#else
# define FC_SCHED_YIELD() FC_STMT_START {} FC_STMT_END
#endif

/* This actually is not a totally awful implementation. */
typedef volatile int fc_mutex_impl_t;
#define FC_MUTEX_IMPL_INIT	0
#define fc_mutex_impl_init(M)	*(M) = 0
#define fc_mutex_impl_lock(M)	FC_STMT_START { while (__sync_lock_test_and_set((M), 1)) FC_SCHED_YIELD (); } FC_STMT_END
#define fc_mutex_impl_unlock(M)	__sync_lock_release (M)
#define fc_mutex_impl_finish(M)	FC_STMT_START {} FC_STMT_END


#elif !defined(FC_NO_MT)

#if defined(HAVE_SCHED_H) && defined(HAVE_SCHED_YIELD)
# include <sched.h>
# define FC_SCHED_YIELD() sched_yield ()
#else
# define FC_SCHED_YIELD() FC_STMT_START {} FC_STMT_END
#endif

#define FC_MUTEX_INT_NIL 1 /* Warn that fallback implementation is in use. */
typedef volatile int fc_mutex_impl_t;
#define FC_MUTEX_IMPL_INIT	0
#define fc_mutex_impl_init(M)	*(M) = 0
#define fc_mutex_impl_lock(M)	FC_STMT_START { while (*(M)) FC_SCHED_YIELD (); (*(M))++; } FC_STMT_END
#define fc_mutex_impl_unlock(M)	(*(M))--;
#define fc_mutex_impl_finish(M)	FC_STMT_START {} FC_STMT_END


#else /* FC_NO_MT */

typedef int fc_mutex_impl_t;
#define FC_MUTEX_IMPL_INIT	0
#define fc_mutex_impl_init(M)	FC_STMT_START {} FC_STMT_END
#define fc_mutex_impl_lock(M)	FC_STMT_START {} FC_STMT_END
#define fc_mutex_impl_unlock(M)	FC_STMT_START {} FC_STMT_END
#define fc_mutex_impl_finish(M)	FC_STMT_START {} FC_STMT_END

#endif


#define FC_MUTEX_INIT		{FC_MUTEX_IMPL_INIT}
typedef fc_mutex_impl_t FcMutex;
static inline void FcMutexInit   (FcMutex *m) { fc_mutex_impl_init (m);   }
static inline void FcMutexLock   (FcMutex *m) { fc_mutex_impl_lock (m);   }
static inline void FcMutexUnlock (FcMutex *m) { fc_mutex_impl_unlock (m); }
static inline void FcMutexFinish (FcMutex *m) { fc_mutex_impl_finish (m); }


#endif /* _FCMUTEX_H_ */
