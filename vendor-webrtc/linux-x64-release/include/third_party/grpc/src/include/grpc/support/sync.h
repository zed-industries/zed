/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#ifndef GRPC_SUPPORT_SYNC_H
#define GRPC_SUPPORT_SYNC_H

/* Platform-specific type declarations of gpr_mu and gpr_cv.   */
#include <grpc/support/port_platform.h>
#include <grpc/support/time.h> /* for gpr_timespec */

#ifdef __cplusplus
extern "C" {
#endif

/** Synchronization primitives for GPR.

   The type  gpr_mu              provides a non-reentrant mutex (lock).

   The type  gpr_cv              provides a condition variable.

   The type  gpr_once            provides for one-time initialization.

   The type gpr_event            provides one-time-setting, reading, and
                                 waiting of a void*, with memory barriers.

   The type gpr_refcount         provides an object reference counter,
                                 with memory barriers suitable to control
                                 object lifetimes.

   The type gpr_stats_counter    provides an atomic statistics counter. It
                                 provides no memory barriers.
 */

#include <grpc/support/sync_generic.h>  // IWYU pragma: export

#if defined(GPR_CUSTOM_SYNC)
#include <grpc/support/sync_custom.h>  // IWYU pragma: export
#elif defined(GPR_ABSEIL_SYNC)
#include <grpc/support/sync_abseil.h>  // IWYU pragma: export
#elif defined(GPR_POSIX_SYNC)
#include <grpc/support/sync_posix.h>  // IWYU pragma: export
#elif defined(GPR_WINDOWS)
#include <grpc/support/sync_windows.h>  // IWYU pragma: export
#else
#error Unable to determine platform for sync
#endif

/** --- Mutex interface ---

   At most one thread may hold an exclusive lock on a mutex at any given time.
   Actions taken by a thread that holds a mutex exclusively happen after
   actions taken by all previous holders of the mutex.  Variables of type
   gpr_mu are uninitialized when first declared.  */

/** Initialize *mu.  Requires:  *mu uninitialized.  */
GPRAPI void gpr_mu_init(gpr_mu* mu);

/** Cause *mu no longer to be initialized, freeing any memory in use.  Requires:
 *mu initialized; no other concurrent operation on *mu.  */
GPRAPI void gpr_mu_destroy(gpr_mu* mu);

/** Wait until no thread has a lock on *mu, cause the calling thread to own an
   exclusive lock on *mu, then return.  May block indefinitely or crash if the
   calling thread has a lock on *mu.  Requires:  *mu initialized.  */
GPRAPI void gpr_mu_lock(gpr_mu* mu);

/** Release an exclusive lock on *mu held by the calling thread.  Requires:  *mu
   initialized; the calling thread holds an exclusive lock on *mu.  */
GPRAPI void gpr_mu_unlock(gpr_mu* mu);

/** Without blocking, attempt to acquire an exclusive lock on *mu for the
   calling thread, then return non-zero iff success.  Fail, if any thread holds
   the lock; succeeds with high probability if no thread holds the lock.
   Requires:  *mu initialized.  */
GPRAPI int gpr_mu_trylock(gpr_mu* mu);

/** --- Condition variable interface ---

   A while-loop should be used with gpr_cv_wait() when waiting for conditions
   to become true.  See the example below.  Variables of type gpr_cv are
   uninitialized when first declared.  */

/** Initialize *cv.  Requires:  *cv uninitialized.  */
GPRAPI void gpr_cv_init(gpr_cv* cv);

/** Cause *cv no longer to be initialized, freeing any memory in use.  Requires:
 *cv initialized; no other concurrent operation on *cv.*/
GPRAPI void gpr_cv_destroy(gpr_cv* cv);

/** Atomically release *mu and wait on *cv.  When the calling thread is woken
   from *cv or the deadline abs_deadline is exceeded, execute gpr_mu_lock(mu)
   and return whether the deadline was exceeded.  Use
   abs_deadline==gpr_inf_future for no deadline.  abs_deadline can be either
   an absolute deadline, or a GPR_TIMESPAN.  May return even when not
   woken explicitly.  Requires:  *mu and *cv initialized; the calling thread
   holds an exclusive lock on *mu.  */
GPRAPI int gpr_cv_wait(gpr_cv* cv, gpr_mu* mu, gpr_timespec abs_deadline);

/** If any threads are waiting on *cv, wake at least one.
   Clients may treat this as an optimization of gpr_cv_broadcast()
   for use in the case where waking more than one waiter is not useful.
   Requires:  *cv initialized.  */
GPRAPI void gpr_cv_signal(gpr_cv* cv);

/** Wake all threads waiting on *cv.  Requires:  *cv initialized.  */
GPRAPI void gpr_cv_broadcast(gpr_cv* cv);

/** --- One-time initialization ---

   gpr_once must be declared with static storage class, and initialized with
   GPR_ONCE_INIT.  e.g.,
     static gpr_once once_var = GPR_ONCE_INIT;     */

/** Ensure that (*init_function)() has been called exactly once (for the
   specified gpr_once instance) and then return.
   If multiple threads call gpr_once() on the same gpr_once instance, one of
   them will call (*init_function)(), and the others will block until that call
   finishes.*/
GPRAPI void gpr_once_init(gpr_once* once, void (*init_function)(void));

/** --- One-time event notification ---

  These operations act on a gpr_event, which should be initialized with
  gpr_ev_init(), or with GPR_EVENT_INIT if static, e.g.,
       static gpr_event event_var = GPR_EVENT_INIT;
  It requires no destruction.  */

/** Initialize *ev. */
GPRAPI void gpr_event_init(gpr_event* ev);

/** Set *ev so that gpr_event_get() and gpr_event_wait() will return value.
   Requires:  *ev initialized; value != NULL; no prior or concurrent calls to
   gpr_event_set(ev, ...) since initialization.  */
GPRAPI void gpr_event_set(gpr_event* ev, void* value);

/** Return the value set by gpr_event_set(ev, ...), or NULL if no such call has
   completed.  If the result is non-NULL, all operations that occurred prior to
   the gpr_event_set(ev, ...) set will be visible after this call returns.
   Requires:  *ev initialized.  This operation is faster than acquiring a mutex
   on most platforms.  */
GPRAPI void* gpr_event_get(gpr_event* ev);

/** Wait until *ev is set by gpr_event_set(ev, ...), or abs_deadline is
   exceeded, then return gpr_event_get(ev).  Requires:  *ev initialized.  Use
   abs_deadline==gpr_inf_future for no deadline.  When the event has been
   signalled before the call, this operation is faster than acquiring a mutex
   on most platforms.  */
GPRAPI void* gpr_event_wait(gpr_event* ev, gpr_timespec abs_deadline);

/** --- Reference counting ---

   These calls act on the type gpr_refcount.  It requires no destruction.  */

/** Initialize *r to value n.  */
GPRAPI void gpr_ref_init(gpr_refcount* r, int n);

/** Increment the reference count *r.  Requires *r initialized. */
GPRAPI void gpr_ref(gpr_refcount* r);

/** Increment the reference count *r.  Requires *r initialized.
   Crashes if refcount is zero */
GPRAPI void gpr_ref_non_zero(gpr_refcount* r);

/** Increment the reference count *r by n.  Requires *r initialized, n > 0. */
GPRAPI void gpr_refn(gpr_refcount* r, int n);

/** Decrement the reference count *r and return non-zero iff it has reached
   zero. .  Requires *r initialized. */
GPRAPI int gpr_unref(gpr_refcount* r);

/** Return non-zero iff the reference count of *r is one, and thus is owned
   by exactly one object. */
GPRAPI int gpr_ref_is_unique(gpr_refcount* r);

/** --- Stats counters ---

   These calls act on the integral type gpr_stats_counter.  It requires no
   destruction.  Static instances may be initialized with
       gpr_stats_counter c = GPR_STATS_INIT;
   Beware:  These operations do not imply memory barriers.  Do not use them to
   synchronize other events.  */

/** Initialize *c to the value n. */
GPRAPI void gpr_stats_init(gpr_stats_counter* c, intptr_t n);

/** *c += inc.  Requires: *c initialized. */
GPRAPI void gpr_stats_inc(gpr_stats_counter* c, intptr_t inc);

/** Return *c.  Requires: *c initialized. */
GPRAPI intptr_t gpr_stats_read(const gpr_stats_counter* c);

/** ==================Example use of interface===================
   A producer-consumer queue of up to N integers,
   illustrating the use of the calls in this interface. */
#if 0

#define N 4

   typedef struct queue {
     gpr_cv non_empty;  /* Signalled when length becomes non-zero. */
     gpr_cv non_full;   /* Signalled when length becomes non-N. */
     gpr_mu mu;         /* Protects all fields below.
                            (That is, except during initialization or
                            destruction, the fields below should be accessed
                            only by a thread that holds mu.) */
     int head;           /* Index of head of queue 0..N-1. */
     int length;         /* Number of valid elements in queue 0..N. */
     int elem[N];        /* elem[head .. head+length-1] are queue elements. */
   } queue;

   /* Initialize *q. */
   void queue_init(queue *q) {
     gpr_mu_init(&q->mu);
     gpr_cv_init(&q->non_empty);
     gpr_cv_init(&q->non_full);
     q->head = 0;
     q->length = 0;
   }

   /* Free storage associated with *q. */
   void queue_destroy(queue *q) {
     gpr_mu_destroy(&q->mu);
     gpr_cv_destroy(&q->non_empty);
     gpr_cv_destroy(&q->non_full);
   }

   /* Wait until there is room in *q, then append x to *q. */
   void queue_append(queue *q, int x) {
     gpr_mu_lock(&q->mu);
     /* To wait for a predicate without a deadline, loop on the negation of the
        predicate, and use gpr_cv_wait(..., gpr_inf_future) inside the loop
        to release the lock, wait, and reacquire on each iteration.  Code that
        makes the condition true should use gpr_cv_broadcast() on the
        corresponding condition variable.  The predicate must be on state
        protected by the lock.  */
     while (q->length == N) {
       gpr_cv_wait(&q->non_full, &q->mu, gpr_inf_future);
     }
     if (q->length == 0) {  /* Wake threads blocked in queue_remove(). */
       /* It's normal to use gpr_cv_broadcast() or gpr_signal() while
          holding the lock. */
       gpr_cv_broadcast(&q->non_empty);
     }
     q->elem[(q->head + q->length) % N] = x;
     q->length++;
     gpr_mu_unlock(&q->mu);
   }

   /* If it can be done without blocking, append x to *q and return non-zero.
      Otherwise return 0. */
   int queue_try_append(queue *q, int x) {
     int result = 0;
     if (gpr_mu_trylock(&q->mu)) {
       if (q->length != N) {
         if (q->length == 0) {  /* Wake threads blocked in queue_remove(). */
           gpr_cv_broadcast(&q->non_empty);
         }
         q->elem[(q->head + q->length) % N] = x;
         q->length++;
         result = 1;
       }
       gpr_mu_unlock(&q->mu);
     }
     return result;
   }

   /* Wait until the *q is non-empty or deadline abs_deadline passes.  If the
      queue is non-empty, remove its head entry, place it in *head, and return
      non-zero.  Otherwise return 0.  */
   int queue_remove(queue *q, int *head, gpr_timespec abs_deadline) {
     int result = 0;
     gpr_mu_lock(&q->mu);
     /* To wait for a predicate with a deadline, loop on the negation of the
        predicate or until gpr_cv_wait() returns true.  Code that makes
        the condition true should use gpr_cv_broadcast() on the corresponding
        condition variable.  The predicate must be on state protected by the
        lock. */
     while (q->length == 0 &&
            !gpr_cv_wait(&q->non_empty, &q->mu, abs_deadline)) {
     }
     if (q->length != 0) {    /* Queue is non-empty. */
       result = 1;
       if (q->length == N) {  /* Wake threads blocked in queue_append(). */
         gpr_cv_broadcast(&q->non_full);
       }
       *head = q->elem[q->head];
       q->head = (q->head + 1) % N;
       q->length--;
     } /* else deadline exceeded */
     gpr_mu_unlock(&q->mu);
     return result;
   }
#endif /* 0 */

#ifdef __cplusplus
}  // extern "C"
#endif

#endif /* GRPC_SUPPORT_SYNC_H */
