/* ISO C11 Standard: 7.26 - Thread support library  <threads.h>.
   Copyright (C) 2018-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

#ifndef _THREADS_H
#define _THREADS_H	1

#include <features.h>
#include <time.h>

__BEGIN_DECLS

#include <bits/pthreadtypes-arch.h>
#include <bits/types/struct_timespec.h>

#ifndef __cplusplus
# define thread_local _Thread_local
#endif

#define TSS_DTOR_ITERATIONS 4
typedef unsigned int tss_t;
typedef void (*tss_dtor_t) (void*);

typedef unsigned long int thrd_t;
typedef int (*thrd_start_t) (void*);

/* Exit and error codes.  */
enum
{
  thrd_success  = 0,
  thrd_busy     = 1,
  thrd_error    = 2,
  thrd_nomem    = 3,
  thrd_timedout = 4
};

/* Mutex types.  */
enum
{
  mtx_plain     = 0,
  mtx_recursive = 1,
  mtx_timed     = 2
};

typedef struct
{
  int __data __ONCE_ALIGNMENT;
} once_flag;
#define ONCE_FLAG_INIT { 0 }

typedef union
{
  char __size[__SIZEOF_PTHREAD_MUTEX_T];
  long int __align __LOCK_ALIGNMENT;
} mtx_t;

typedef union
{
  char __size[__SIZEOF_PTHREAD_COND_T];
  __extension__ long long int __align __LOCK_ALIGNMENT;
} cnd_t;

/* Threads functions.  */

/* Create a new thread executing the function __FUNC.  Arguments for __FUNC
   are passed through __ARG.  If succesful, __THR is set to new thread
   identifier.  */
extern int thrd_create (thrd_t *__thr, thrd_start_t __func, void *__arg);

/* Check if __LHS and __RHS point to the same thread.  */
extern int thrd_equal (thrd_t __lhs, thrd_t __rhs);

/* Return current thread identifier.  */
extern thrd_t thrd_current (void);

/* Block current thread execution for at least the time pointed by
   __TIME_POINT.  The current thread may resume if receives a signal.  In
   that case, if __REMAINING is not NULL, the remaining time is stored in
   the object pointed by it.  */
extern int thrd_sleep (const struct timespec *__time_point,
		       struct timespec *__remaining);

/* Terminate current thread execution, cleaning up any thread local
   storage and freeing resources.  Returns the value specified in __RES.  */
extern void thrd_exit (int __res) __attribute__ ((__noreturn__));

/* Detach the thread identified by __THR from the current environment
   (it does not allow join or wait for it).  */
extern int thrd_detach (thrd_t __thr);

/* Block current thread until execution of __THR is complete.  In case that
   __RES is not NULL, will store the return value of __THR when exiting.  */
extern int thrd_join (thrd_t __thr, int *__res);

/* Stop current thread execution and call the scheduler to decide which
   thread should execute next.  The current thread may be selected by the
   scheduler to keep running.  */
extern void thrd_yield (void);

#ifdef __USE_EXTERN_INLINES
/* Optimizations.  */
__extern_inline int
thrd_equal (thrd_t __thread1, thrd_t __thread2)
{
  return __thread1 == __thread2;
}
#endif


/* Mutex functions.  */

/* Creates a new mutex object with type __TYPE.  If successful the new
   object is pointed by __MUTEX.  */
extern int mtx_init (mtx_t *__mutex, int __type);

/* Block the current thread until the mutex pointed to by __MUTEX is
   unlocked.  In that case current thread will not be blocked.  */
extern int mtx_lock (mtx_t *__mutex);

/* Block the current thread until the mutex pointed by __MUTEX is unlocked
   or time pointed by __TIME_POINT is reached.  In case the mutex is unlock,
   the current thread will not be blocked.  */
extern int mtx_timedlock (mtx_t *__restrict __mutex,
			  const struct timespec *__restrict __time_point);

/* Try to lock the mutex pointed by __MUTEX without blocking.  If the mutex
   is free the current threads takes control of it, otherwise it returns
   immediately.  */
extern int mtx_trylock (mtx_t *__mutex);

/* Unlock the mutex pointed by __MUTEX.  It may potentially awake other
   threads waiting on this mutex.  */
extern int mtx_unlock (mtx_t *__mutex);

/* Destroy the mutex object pointed by __MUTEX.  */
extern void mtx_destroy (mtx_t *__mutex);


/* Call function __FUNC exactly once, even if invoked from several threads.
   All calls must be made with the same __FLAGS object.  */
extern void call_once (once_flag *__flag, void (*__func)(void));


/* Condition variable functions.  */

/* Initialize new condition variable pointed by __COND.  */
extern int cnd_init (cnd_t *__cond);

/* Unblock one thread that currently waits on condition variable pointed
   by __COND.  */
extern int cnd_signal (cnd_t *__cond);

/* Unblock all threads currently waiting on condition variable pointed by
   __COND.  */
extern int cnd_broadcast (cnd_t *__cond);

/* Block current thread on the condition variable pointed by __COND.  */
extern int cnd_wait (cnd_t *__cond, mtx_t *__mutex);

/* Block current thread on the condition variable until condition variable
   pointed by __COND is signaled or time pointed by __TIME_POINT is
   reached.  */
extern int cnd_timedwait (cnd_t *__restrict __cond,
			  mtx_t *__restrict __mutex,
			  const struct timespec *__restrict __time_point);

/* Destroy condition variable pointed by __cond and free all of its
   resources.  */
extern void cnd_destroy (cnd_t *__COND);


/* Thread specific storage functions.  */

/* Create new thread-specific storage key and stores it in the object pointed
   by __TSS_ID.  If __DESTRUCTOR is not NULL, the function will be called when
   the thread terminates.  */
extern int tss_create (tss_t *__tss_id, tss_dtor_t __destructor);

/* Return the value held in thread-specific storage for the current thread
   identified by __TSS_ID.  */
extern void *tss_get (tss_t __tss_id);

/* Sets the value of the thread-specific storage identified by __TSS_ID for
   the current thread to __VAL.  */
extern int tss_set (tss_t __tss_id, void *__val);

/* Destroys the thread-specific storage identified by __TSS_ID.  The
   destructor is not called until thrd_exit is called.  */
extern void tss_delete (tss_t __tss_id);

__END_DECLS

#endif /* _THREADS_H */
