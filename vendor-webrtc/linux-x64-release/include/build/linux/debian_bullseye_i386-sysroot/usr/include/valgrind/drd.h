/*
  ----------------------------------------------------------------

  Notice that the following BSD-style license applies to this one
  file (drd.h) only.  The rest of Valgrind is licensed under the
  terms of the GNU General Public License, version 2, unless
  otherwise indicated.  See the COPYING file in the source
  distribution for details.

  ----------------------------------------------------------------

  This file is part of DRD, a Valgrind tool for verification of
  multithreaded programs.

  Copyright (C) 2006-2017 Bart Van Assche <bvanassche@acm.org>.
  All rights reserved.

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions
  are met:

  1. Redistributions of source code must retain the above copyright
  notice, this list of conditions and the following disclaimer.

  2. The origin of this software must not be misrepresented; you must
  not claim that you wrote the original software.  If you use this
  software in a product, an acknowledgment in the product
  documentation would be appreciated but is not required.

  3. Altered source versions must be plainly marked as such, and must
  not be misrepresented as being the original software.

  4. The name of the author may not be used to endorse or promote
  products derived from this software without specific prior written
  permission.

  THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS
  OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
  WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
  ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
  DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
  DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE
  GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
  INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
  NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
  SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

  ----------------------------------------------------------------

  Notice that the above BSD-style license applies to this one file
  (drd.h) only.  The entire rest of Valgrind is licensed under
  the terms of the GNU General Public License, version 2.  See the
  COPYING file in the source distribution for details.

  ----------------------------------------------------------------
*/

#ifndef __VALGRIND_DRD_H
#define __VALGRIND_DRD_H


#include "valgrind.h"


/** Obtain the thread ID assigned by Valgrind's core. */
#define DRD_GET_VALGRIND_THREADID                                          \
    (unsigned)VALGRIND_DO_CLIENT_REQUEST_EXPR(0,                           \
                                   VG_USERREQ__DRD_GET_VALGRIND_THREAD_ID, \
                                   0, 0, 0, 0, 0)

/** Obtain the thread ID assigned by DRD. */
#define DRD_GET_DRD_THREADID                                            \
    (unsigned)VALGRIND_DO_CLIENT_REQUEST_EXPR(0,                        \
                                   VG_USERREQ__DRD_GET_DRD_THREAD_ID,   \
                                   0, 0, 0, 0, 0)


/** Tell DRD not to complain about data races for the specified variable. */
#define DRD_IGNORE_VAR(x) ANNOTATE_BENIGN_RACE_SIZED(&(x), sizeof(x), "")

/** Tell DRD to no longer ignore data races for the specified variable. */
#define DRD_STOP_IGNORING_VAR(x)                                       \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_FINISH_SUPPRESSION, \
                                   &(x), sizeof(x), 0, 0, 0)

/**
 * Tell DRD to trace all memory accesses for the specified variable
 * until the memory that was allocated for the variable is freed.
 */
#define DRD_TRACE_VAR(x)                                             \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_START_TRACE_ADDR, \
                                   &(x), sizeof(x), 0, 0, 0)

/**
 * Tell DRD to stop tracing memory accesses for the specified variable.
 */
#define DRD_STOP_TRACING_VAR(x)                                       \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_STOP_TRACE_ADDR, \
                                   &(x), sizeof(x), 0, 0, 0)

/**
 * @defgroup RaceDetectionAnnotations Data race detection annotations.
 *
 * @see See also the source file <a href="http://code.google.com/p/data-race-test/source/browse/trunk/dynamic_annotations/dynamic_annotations.h</a>

 * in the ThreadSanitizer project.
 */
/*@{*/

#ifndef __HELGRIND_H

/**
 * Tell DRD to insert a happens-before mark. addr is the address of an object
 * that is not a pthread synchronization object.
 */
#define ANNOTATE_HAPPENS_BEFORE(addr)                                       \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATE_HAPPENS_BEFORE, \
                                   addr, 0, 0, 0, 0)

/**
 * Tell DRD that the memory accesses executed after this annotation will
 * happen after all memory accesses performed before all preceding
 * ANNOTATE_HAPPENS_BEFORE(addr). addr is the address of an object that is not
 * a pthread synchronization object. Inserting a happens-after annotation
 * before any other thread has passed by a happens-before annotation for the
 * same address is an error.
 */
#define ANNOTATE_HAPPENS_AFTER(addr)                                       \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATE_HAPPENS_AFTER, \
                                   addr, 0, 0, 0, 0)

#else /* __HELGRIND_H */

#undef ANNOTATE_CONDVAR_LOCK_WAIT
#undef ANNOTATE_CONDVAR_WAIT
#undef ANNOTATE_CONDVAR_SIGNAL
#undef ANNOTATE_CONDVAR_SIGNAL_ALL
#undef ANNOTATE_PURE_HAPPENS_BEFORE_MUTEX
#undef ANNOTATE_PUBLISH_MEMORY_RANGE
#undef ANNOTATE_BARRIER_INIT
#undef ANNOTATE_BARRIER_WAIT_BEFORE
#undef ANNOTATE_BARRIER_WAIT_AFTER
#undef ANNOTATE_BARRIER_DESTROY
#undef ANNOTATE_PCQ_CREATE
#undef ANNOTATE_PCQ_DESTROY
#undef ANNOTATE_PCQ_PUT
#undef ANNOTATE_PCQ_GET
#undef ANNOTATE_BENIGN_RACE
#undef ANNOTATE_BENIGN_RACE_SIZED
#undef ANNOTATE_IGNORE_READS_BEGIN
#undef ANNOTATE_IGNORE_READS_END
#undef ANNOTATE_IGNORE_WRITES_BEGIN
#undef ANNOTATE_IGNORE_WRITES_END
#undef ANNOTATE_IGNORE_READS_AND_WRITES_BEGIN
#undef ANNOTATE_IGNORE_READS_AND_WRITES_END
#undef ANNOTATE_NEW_MEMORY
#undef ANNOTATE_TRACE_MEMORY
#undef ANNOTATE_THREAD_NAME

#endif /* __HELGRIND_H */

/**
 * Tell DRD that waiting on the condition variable at address cv has succeeded
 * and a lock on the mutex at address mtx is now held. Since DRD always inserts
 * a happens before relation between the pthread_cond_signal() or
 * pthread_cond_broadcast() call that wakes up a pthread_cond_wait() or
 * pthread_cond_timedwait() call and the woken up thread, this macro has been
 * defined such that it has no effect.
 */
#define ANNOTATE_CONDVAR_LOCK_WAIT(cv, mtx) do { } while(0)

/**
 * Tell DRD that the condition variable at address cv is about to be signaled.
 */
#define ANNOTATE_CONDVAR_SIGNAL(cv) do { } while(0)

/**
 * Tell DRD that the condition variable at address cv is about to be signaled.
 */
#define ANNOTATE_CONDVAR_SIGNAL_ALL(cv) do { } while(0)

/**
 * Tell DRD that waiting on condition variable at address cv succeeded and that
 * the memory operations performed after this annotation should be considered
 * to happen after the matching ANNOTATE_CONDVAR_SIGNAL(cv). Since this is the
 * default behavior of DRD, this macro and the macro above have been defined
 * such that they have no effect.
 */
#define ANNOTATE_CONDVAR_WAIT(cv) do { } while(0)

/**
 * Tell DRD to consider the memory operations that happened before a mutex
 * unlock event and after the subsequent mutex lock event on the same mutex as
 * ordered. This is how DRD always behaves, so this macro has been defined
 * such that it has no effect.
 */
#define ANNOTATE_PURE_HAPPENS_BEFORE_MUTEX(mtx) do { } while(0)

/** Deprecated -- don't use this annotation. */
#define ANNOTATE_MUTEX_IS_USED_AS_CONDVAR(mtx) do { } while(0)

/**
 * Tell DRD to handle the specified memory range like a pure happens-before
 * detector would do. Since this is how DRD always behaves, this annotation
 * has been defined such that it has no effect.
 */
#define ANNOTATE_PUBLISH_MEMORY_RANGE(addr, size) do { } while(0)

/** Deprecated -- don't use this annotation. */
#define ANNOTATE_UNPUBLISH_MEMORY_RANGE(addr, size) do { } while(0)

/** Deprecated -- don't use this annotation. */
#define ANNOTATE_SWAP_MEMORY_RANGE(addr, size) do { } while(0)

#ifndef __HELGRIND_H

/** Tell DRD that a reader-writer lock object has been initialized. */
#define ANNOTATE_RWLOCK_CREATE(rwlock)                                     \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATE_RWLOCK_CREATE, \
                                   rwlock, 0, 0, 0, 0);

/** Tell DRD that a reader-writer lock object has been destroyed. */
#define ANNOTATE_RWLOCK_DESTROY(rwlock)                                     \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATE_RWLOCK_DESTROY, \
                                   rwlock, 0, 0, 0, 0);

/**
 * Tell DRD that a reader-writer lock has been acquired. is_w == 1 means that
 * a write lock has been obtained, is_w == 0 means that a read lock has been
 * obtained.
 */
#define ANNOTATE_RWLOCK_ACQUIRED(rwlock, is_w)                               \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATE_RWLOCK_ACQUIRED, \
                                   rwlock, is_w, 0, 0, 0)

#endif /* __HELGRIND_H */

/**
 * Tell DRD that a reader lock has been acquired on a reader-writer
 * synchronization object.
 */
#define ANNOTATE_READERLOCK_ACQUIRED(rwlock) ANNOTATE_RWLOCK_ACQUIRED(rwlock, 0)

/**
 * Tell DRD that a writer lock has been acquired on a reader-writer
 * synchronization object.
 */
#define ANNOTATE_WRITERLOCK_ACQUIRED(rwlock) ANNOTATE_RWLOCK_ACQUIRED(rwlock, 1)

#ifndef __HELGRIND_H

/**
 * Tell DRD that a reader-writer lock is about to be released. is_w == 1 means
 * that a write lock is about to be released, is_w == 0 means that a read lock
 * is about to be released.
 */
#define ANNOTATE_RWLOCK_RELEASED(rwlock, is_w)                               \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATE_RWLOCK_RELEASED, \
                                   rwlock, is_w, 0, 0, 0);

#endif /* __HELGRIND_H */

/**
 * Tell DRD that a reader lock is about to be released.
 */
#define ANNOTATE_READERLOCK_RELEASED(rwlock) ANNOTATE_RWLOCK_RELEASED(rwlock, 0)

/**
 * Tell DRD that a writer lock is about to be released.
 */
#define ANNOTATE_WRITERLOCK_RELEASED(rwlock) ANNOTATE_RWLOCK_RELEASED(rwlock, 1)

/** Tell DRD that a semaphore object is going to be initialized. */
#define ANNOTATE_SEM_INIT_PRE(sem, value)                                 \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATE_SEM_INIT_PRE, \
                                   sem, value, 0, 0, 0);

/** Tell DRD that a semaphore object has been destroyed. */
#define ANNOTATE_SEM_DESTROY_POST(sem)                                        \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATE_SEM_DESTROY_POST, \
                                   sem, 0, 0, 0, 0);

/** Tell DRD that a semaphore is going to be acquired. */
#define ANNOTATE_SEM_WAIT_PRE(sem)                                        \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATE_SEM_WAIT_PRE, \
                                   sem, 0, 0, 0, 0)

/** Tell DRD that a semaphore has been acquired. */
#define ANNOTATE_SEM_WAIT_POST(sem)                                        \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATE_SEM_WAIT_POST, \
                                   sem, 0, 0, 0, 0)

/** Tell DRD that a semaphore is going to be released. */
#define ANNOTATE_SEM_POST_PRE(sem)                                        \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATE_SEM_POST_PRE, \
                                   sem, 0, 0, 0, 0)

/*
 * Report that a barrier has been initialized with a given barrier count.  The
 * third argument specifies whether or not reinitialization is allowed, that
 * is, whether or not it is allowed to call barrier_init() several times
 * without calling barrier_destroy().
 */
#define ANNOTATE_BARRIER_INIT(barrier, count, reinitialization_allowed) \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATION_UNIMP,    \
                                   "ANNOTATE_BARRIER_INIT", barrier,    \
                                   count, reinitialization_allowed, 0)

/* Report that a barrier has been destroyed. */
#define ANNOTATE_BARRIER_DESTROY(barrier)                               \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATION_UNIMP,    \
                                   "ANNOTATE_BARRIER_DESTROY",          \
                                   barrier, 0, 0, 0)

/* Report that the calling thread is about to start waiting for a barrier. */
#define ANNOTATE_BARRIER_WAIT_BEFORE(barrier)                           \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATION_UNIMP,    \
                                   "ANNOTATE_BARRIER_WAIT_BEFORE",      \
                                   barrier, 0, 0, 0)

/* Report that the calling thread has just finished waiting for a barrier. */
#define ANNOTATE_BARRIER_WAIT_AFTER(barrier)                            \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_ANNOTATION_UNIMP,    \
                                   "ANNOTATE_BARRIER_WAIT_AFTER",       \
                                   barrier, 0, 0, 0)

/**
 * Tell DRD that a FIFO queue has been created. The abbreviation PCQ stands for
 * <em>producer-consumer</em>.
 */
#define ANNOTATE_PCQ_CREATE(pcq) do { } while(0)

/** Tell DRD that a FIFO queue has been destroyed. */
#define ANNOTATE_PCQ_DESTROY(pcq) do { } while(0)

/**
 * Tell DRD that an element has been added to the FIFO queue at address pcq.
 */
#define ANNOTATE_PCQ_PUT(pcq) do { } while(0)

/**
 * Tell DRD that an element has been removed from the FIFO queue at address pcq,
 * and that DRD should insert a happens-before relationship between the memory
 * accesses that occurred before the corresponding ANNOTATE_PCQ_PUT(pcq)
 * annotation and the memory accesses after this annotation. Correspondence
 * between PUT and GET annotations happens in FIFO order. Since locking
 * of the queue is needed anyway to add elements to or to remove elements from
 * the queue, for DRD all four FIFO annotations are defined as no-ops.
 */
#define ANNOTATE_PCQ_GET(pcq) do { } while(0)

/**
 * Tell DRD that data races at the specified address are expected and must not
 * be reported.
 */
#define ANNOTATE_BENIGN_RACE(addr, descr) \
   ANNOTATE_BENIGN_RACE_SIZED(addr, sizeof(*addr), descr)

/* Same as ANNOTATE_BENIGN_RACE(addr, descr), but applies to
   the memory range [addr, addr + size). */
#define ANNOTATE_BENIGN_RACE_SIZED(addr, size, descr)                   \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_START_SUPPRESSION,   \
                                   addr, size, 0, 0, 0)

/** Tell DRD to ignore all reads performed by the current thread. */
#define ANNOTATE_IGNORE_READS_BEGIN()                                \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_RECORD_LOADS,     \
                                   0, 0, 0, 0, 0);


/** Tell DRD to no longer ignore the reads performed by the current thread. */
#define ANNOTATE_IGNORE_READS_END()                                  \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_RECORD_LOADS,     \
                                   1, 0, 0, 0, 0);

/** Tell DRD to ignore all writes performed by the current thread. */
#define ANNOTATE_IGNORE_WRITES_BEGIN()                                \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_RECORD_STORES,     \
                                   0, 0, 0, 0, 0)

/** Tell DRD to no longer ignore the writes performed by the current thread. */
#define ANNOTATE_IGNORE_WRITES_END()                                  \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_RECORD_STORES,     \
                                   1, 0, 0, 0, 0)

/** Tell DRD to ignore all memory accesses performed by the current thread. */
#define ANNOTATE_IGNORE_READS_AND_WRITES_BEGIN() \
   do { ANNOTATE_IGNORE_READS_BEGIN(); ANNOTATE_IGNORE_WRITES_BEGIN(); } while(0)

/**
 * Tell DRD to no longer ignore the memory accesses performed by the current
 * thread.
 */
#define ANNOTATE_IGNORE_READS_AND_WRITES_END() \
   do { ANNOTATE_IGNORE_READS_END(); ANNOTATE_IGNORE_WRITES_END(); } while(0)

/**
 * Tell DRD that size bytes starting at addr has been allocated by a custom
 * memory allocator.
 */
#define ANNOTATE_NEW_MEMORY(addr, size)                           \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_CLEAN_MEMORY,  \
                                   addr, size, 0, 0, 0)

/** Ask DRD to report every access to the specified address. */
#define ANNOTATE_TRACE_MEMORY(addr) DRD_TRACE_VAR(*(char*)(addr))

/**
 * Tell DRD to assign the specified name to the current thread. This name will
 * be used in error messages printed by DRD.
 */
#define ANNOTATE_THREAD_NAME(name)                                      \
   VALGRIND_DO_CLIENT_REQUEST_STMT(VG_USERREQ__DRD_SET_THREAD_NAME,     \
                                   name, 0, 0, 0, 0)

/*@}*/


/* !! ABIWARNING !! ABIWARNING !! ABIWARNING !! ABIWARNING !!
   This enum comprises an ABI exported by Valgrind to programs
   which use client requests.  DO NOT CHANGE THE ORDER OF THESE
   ENTRIES, NOR DELETE ANY -- add new ones at the end.
*/
enum {
   /* Ask the DRD tool to discard all information about memory accesses   */
   /* and client objects for the specified range. This client request is  */
   /* binary compatible with the similarly named Helgrind client request. */
   VG_USERREQ__DRD_CLEAN_MEMORY = VG_USERREQ_TOOL_BASE('H','G'),
   /* args: Addr, SizeT. */

   /* Ask the DRD tool the thread ID assigned by Valgrind. */
   VG_USERREQ__DRD_GET_VALGRIND_THREAD_ID = VG_USERREQ_TOOL_BASE('D','R'),
   /* args: none. */
   /* Ask the DRD tool the thread ID assigned by DRD. */
   VG_USERREQ__DRD_GET_DRD_THREAD_ID,
   /* args: none. */

   /* To tell the DRD tool to suppress data race detection on the */
   /* specified address range. */
   VG_USERREQ__DRD_START_SUPPRESSION,
   /* args: start address, size in bytes */
   /* To tell the DRD tool no longer to suppress data race detection on */
   /* the specified address range. */
   VG_USERREQ__DRD_FINISH_SUPPRESSION,
   /* args: start address, size in bytes */

   /* To ask the DRD tool to trace all accesses to the specified range. */
   VG_USERREQ__DRD_START_TRACE_ADDR,
   /* args: Addr, SizeT. */
   /* To ask the DRD tool to stop tracing accesses to the specified range. */
   VG_USERREQ__DRD_STOP_TRACE_ADDR,
   /* args: Addr, SizeT. */

   /* Tell DRD whether or not to record memory loads in the calling thread. */
   VG_USERREQ__DRD_RECORD_LOADS,
   /* args: Bool. */
   /* Tell DRD whether or not to record memory stores in the calling thread. */
   VG_USERREQ__DRD_RECORD_STORES,
   /* args: Bool. */

   /* Set the name of the thread that performs this client request. */
   VG_USERREQ__DRD_SET_THREAD_NAME,
   /* args: null-terminated character string. */

   /* Tell DRD that a DRD annotation has not yet been implemented. */
   VG_USERREQ__DRD_ANNOTATION_UNIMP,
   /* args: char*. */

   /* Tell DRD that a user-defined semaphore synchronization object
    * is about to be created. */
   VG_USERREQ__DRD_ANNOTATE_SEM_INIT_PRE,
   /* args: Addr, UInt value. */
   /* Tell DRD that a user-defined semaphore synchronization object
    * has been destroyed. */
   VG_USERREQ__DRD_ANNOTATE_SEM_DESTROY_POST,
   /* args: Addr. */
   /* Tell DRD that a user-defined semaphore synchronization
    * object is going to be acquired (semaphore wait). */
   VG_USERREQ__DRD_ANNOTATE_SEM_WAIT_PRE,
   /* args: Addr. */
   /* Tell DRD that a user-defined semaphore synchronization
    * object has been acquired (semaphore wait). */
   VG_USERREQ__DRD_ANNOTATE_SEM_WAIT_POST,
   /* args: Addr. */
   /* Tell DRD that a user-defined semaphore synchronization
    * object is about to be released (semaphore post). */
   VG_USERREQ__DRD_ANNOTATE_SEM_POST_PRE,
   /* args: Addr. */

   /* Tell DRD to ignore the inter-thread ordering introduced by a mutex. */
   VG_USERREQ__DRD_IGNORE_MUTEX_ORDERING,
   /* args: Addr. */

   /* Tell DRD that a user-defined reader-writer synchronization object
    * has been created. */
   VG_USERREQ__DRD_ANNOTATE_RWLOCK_CREATE
      = VG_USERREQ_TOOL_BASE('H','G') + 256 + 14,
   /* args: Addr. */
   /* Tell DRD that a user-defined reader-writer synchronization object
    * is about to be destroyed. */
   VG_USERREQ__DRD_ANNOTATE_RWLOCK_DESTROY
      = VG_USERREQ_TOOL_BASE('H','G') + 256 + 15,
   /* args: Addr. */
   /* Tell DRD that a lock on a user-defined reader-writer synchronization
    * object has been acquired. */
   VG_USERREQ__DRD_ANNOTATE_RWLOCK_ACQUIRED
      = VG_USERREQ_TOOL_BASE('H','G') + 256 + 17,
   /* args: Addr, Int is_rw. */
   /* Tell DRD that a lock on a user-defined reader-writer synchronization
    * object is about to be released. */
   VG_USERREQ__DRD_ANNOTATE_RWLOCK_RELEASED
      = VG_USERREQ_TOOL_BASE('H','G') + 256 + 18,
   /* args: Addr, Int is_rw. */

   /* Tell DRD that a Helgrind annotation has not yet been implemented. */
   VG_USERREQ__HELGRIND_ANNOTATION_UNIMP
      = VG_USERREQ_TOOL_BASE('H','G') + 256 + 32,
   /* args: char*. */

   /* Tell DRD to insert a happens-before annotation. */
   VG_USERREQ__DRD_ANNOTATE_HAPPENS_BEFORE
      = VG_USERREQ_TOOL_BASE('H','G') + 256 + 33,
   /* args: Addr. */
   /* Tell DRD to insert a happens-after annotation. */
   VG_USERREQ__DRD_ANNOTATE_HAPPENS_AFTER
      = VG_USERREQ_TOOL_BASE('H','G') + 256 + 34,
   /* args: Addr. */

};


/**
 * @addtogroup RaceDetectionAnnotations
 */
/*@{*/

#ifdef __cplusplus
/* ANNOTATE_UNPROTECTED_READ is the preferred way to annotate racy reads.

   Instead of doing
   ANNOTATE_IGNORE_READS_BEGIN();
   ... = x;
   ANNOTATE_IGNORE_READS_END();
   one can use
   ... = ANNOTATE_UNPROTECTED_READ(x); */
template <typename T>
inline T ANNOTATE_UNPROTECTED_READ(const volatile T& x) {
   ANNOTATE_IGNORE_READS_BEGIN();
   const T result = x;
   ANNOTATE_IGNORE_READS_END();
   return result;
}
/* Apply ANNOTATE_BENIGN_RACE_SIZED to a static variable. */
#define ANNOTATE_BENIGN_RACE_STATIC(static_var, description)		\
   namespace {								\
      static class static_var##_annotator				\
      {									\
      public:								\
	 static_var##_annotator()					\
	 {								\
	    ANNOTATE_BENIGN_RACE_SIZED(&static_var, sizeof(static_var),	\
				       #static_var ": " description);	\
	 }								\
      } the_##static_var##_annotator;					\
   }
#endif

/*@}*/

#endif /* __VALGRIND_DRD_H */
