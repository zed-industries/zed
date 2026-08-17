/*
   ----------------------------------------------------------------

   Notice that the above BSD-style license applies to this one file
   (helgrind.h) only.  The entire rest of Valgrind is licensed under
   the terms of the GNU General Public License, version 2.  See the
   COPYING file in the source distribution for details.

   ----------------------------------------------------------------

   This file is part of Helgrind, a Valgrind tool for detecting errors
   in threaded programs.

   Copyright (C) 2007-2017 OpenWorks LLP
      info@open-works.co.uk

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
   (helgrind.h) only.  The entire rest of Valgrind is licensed under
   the terms of the GNU General Public License, version 2.  See the
   COPYING file in the source distribution for details.

   ---------------------------------------------------------------- 
*/

#ifndef __HELGRIND_H
#define __HELGRIND_H

#include "valgrind.h"

/* !! ABIWARNING !! ABIWARNING !! ABIWARNING !! ABIWARNING !!
   This enum comprises an ABI exported by Valgrind to programs
   which use client requests.  DO NOT CHANGE THE ORDER OF THESE
   ENTRIES, NOR DELETE ANY -- add new ones at the end. */
typedef
   enum {
      VG_USERREQ__HG_CLEAN_MEMORY = VG_USERREQ_TOOL_BASE('H','G'),

      /* The rest are for Helgrind's internal use.  Not for end-user
         use.  Do not use them unless you are a Valgrind developer. */

      /* Notify the tool what this thread's pthread_t is. */
      _VG_USERREQ__HG_SET_MY_PTHREAD_T = VG_USERREQ_TOOL_BASE('H','G') 
                                         + 256,
      _VG_USERREQ__HG_PTH_API_ERROR,              /* char*, int */
      _VG_USERREQ__HG_PTHREAD_JOIN_POST,          /* pthread_t of quitter */
      _VG_USERREQ__HG_PTHREAD_MUTEX_INIT_POST,    /* pth_mx_t*, long mbRec */
      _VG_USERREQ__HG_PTHREAD_MUTEX_DESTROY_PRE,  /* pth_mx_t*, long isInit */
      _VG_USERREQ__HG_PTHREAD_MUTEX_UNLOCK_PRE,   /* pth_mx_t* */
      _VG_USERREQ__HG_PTHREAD_MUTEX_UNLOCK_POST,  /* pth_mx_t* */
      _VG_USERREQ__HG_PTHREAD_MUTEX_ACQUIRE_PRE,  /* void*, long isTryLock */
      _VG_USERREQ__HG_PTHREAD_MUTEX_ACQUIRE_POST, /* void* */
      _VG_USERREQ__HG_PTHREAD_COND_SIGNAL_PRE,    /* pth_cond_t* */
      _VG_USERREQ__HG_PTHREAD_COND_BROADCAST_PRE, /* pth_cond_t* */
      _VG_USERREQ__HG_PTHREAD_COND_WAIT_PRE,     /* pth_cond_t*, pth_mx_t* */
      _VG_USERREQ__HG_PTHREAD_COND_WAIT_POST,    /* pth_cond_t*, pth_mx_t* */
      _VG_USERREQ__HG_PTHREAD_COND_DESTROY_PRE,   /* pth_cond_t*, long isInit */
      _VG_USERREQ__HG_PTHREAD_RWLOCK_INIT_POST,   /* pth_rwlk_t* */
      _VG_USERREQ__HG_PTHREAD_RWLOCK_DESTROY_PRE, /* pth_rwlk_t* */
      _VG_USERREQ__HG_PTHREAD_RWLOCK_LOCK_PRE,    /* pth_rwlk_t*, long isW */
      _VG_USERREQ__HG_PTHREAD_RWLOCK_ACQUIRED,    /* void*, long isW */
      _VG_USERREQ__HG_PTHREAD_RWLOCK_RELEASED,    /* void* */
      _VG_USERREQ__HG_PTHREAD_RWLOCK_UNLOCK_POST, /* pth_rwlk_t* */
      _VG_USERREQ__HG_POSIX_SEM_INIT_POST,        /* sem_t*, ulong value */
      _VG_USERREQ__HG_POSIX_SEM_DESTROY_PRE,      /* sem_t* */
      _VG_USERREQ__HG_POSIX_SEM_RELEASED,         /* void* */
      _VG_USERREQ__HG_POSIX_SEM_ACQUIRED,         /* void* */
      _VG_USERREQ__HG_PTHREAD_BARRIER_INIT_PRE,   /* pth_bar_t*, ulong, ulong */
      _VG_USERREQ__HG_PTHREAD_BARRIER_WAIT_PRE,   /* pth_bar_t* */
      _VG_USERREQ__HG_PTHREAD_BARRIER_DESTROY_PRE, /* pth_bar_t* */
      _VG_USERREQ__HG_PTHREAD_SPIN_INIT_OR_UNLOCK_PRE,  /* pth_slk_t* */
      _VG_USERREQ__HG_PTHREAD_SPIN_INIT_OR_UNLOCK_POST, /* pth_slk_t* */
      _VG_USERREQ__HG_PTHREAD_SPIN_LOCK_PRE,      /* pth_slk_t* */
      _VG_USERREQ__HG_PTHREAD_SPIN_LOCK_POST,     /* pth_slk_t* */
      _VG_USERREQ__HG_PTHREAD_SPIN_DESTROY_PRE,   /* pth_slk_t* */
      _VG_USERREQ__HG_CLIENTREQ_UNIMP,            /* char* */
      _VG_USERREQ__HG_USERSO_SEND_PRE,        /* arbitrary UWord SO-tag */
      _VG_USERREQ__HG_USERSO_RECV_POST,       /* arbitrary UWord SO-tag */
      _VG_USERREQ__HG_USERSO_FORGET_ALL,      /* arbitrary UWord SO-tag */
      _VG_USERREQ__HG_RESERVED2,              /* Do not use */
      _VG_USERREQ__HG_RESERVED3,              /* Do not use */
      _VG_USERREQ__HG_RESERVED4,              /* Do not use */
      _VG_USERREQ__HG_ARANGE_MAKE_UNTRACKED, /* Addr a, ulong len */
      _VG_USERREQ__HG_ARANGE_MAKE_TRACKED,   /* Addr a, ulong len */
      _VG_USERREQ__HG_PTHREAD_BARRIER_RESIZE_PRE, /* pth_bar_t*, ulong */
      _VG_USERREQ__HG_CLEAN_MEMORY_HEAPBLOCK, /* Addr start_of_block */
      _VG_USERREQ__HG_PTHREAD_COND_INIT_POST,  /* pth_cond_t*, pth_cond_attr_t*/
      _VG_USERREQ__HG_GNAT_MASTER_HOOK,       /* void*d,void*m,Word ml */
      _VG_USERREQ__HG_GNAT_MASTER_COMPLETED_HOOK, /* void*s,Word ml */
      _VG_USERREQ__HG_GET_ABITS,              /* Addr a,Addr abits, ulong len */
      _VG_USERREQ__HG_PTHREAD_CREATE_BEGIN,
      _VG_USERREQ__HG_PTHREAD_CREATE_END,
      _VG_USERREQ__HG_PTHREAD_MUTEX_LOCK_PRE,     /* pth_mx_t*,long isTryLock */
      _VG_USERREQ__HG_PTHREAD_MUTEX_LOCK_POST,    /* pth_mx_t *,long tookLock */
      _VG_USERREQ__HG_PTHREAD_RWLOCK_LOCK_POST,  /* pth_rwlk_t*,long isW,long */
      _VG_USERREQ__HG_PTHREAD_RWLOCK_UNLOCK_PRE,  /* pth_rwlk_t* */
      _VG_USERREQ__HG_POSIX_SEM_POST_PRE,         /* sem_t* */
      _VG_USERREQ__HG_POSIX_SEM_POST_POST,        /* sem_t* */
      _VG_USERREQ__HG_POSIX_SEM_WAIT_PRE,         /* sem_t* */
      _VG_USERREQ__HG_POSIX_SEM_WAIT_POST,        /* sem_t*, long tookLock */
      _VG_USERREQ__HG_PTHREAD_COND_SIGNAL_POST,   /* pth_cond_t* */
      _VG_USERREQ__HG_PTHREAD_COND_BROADCAST_POST,/* pth_cond_t* */
      _VG_USERREQ__HG_RTLD_BIND_GUARD,            /* int flags */
      _VG_USERREQ__HG_RTLD_BIND_CLEAR,            /* int flags */
      _VG_USERREQ__HG_GNAT_DEPENDENT_MASTER_JOIN  /* void*d, void*m */
   } Vg_TCheckClientRequest;


/*----------------------------------------------------------------*/
/*---                                                          ---*/
/*--- Implementation-only facilities.  Not for end-user use.   ---*/
/*--- For end-user facilities see below (the next section in   ---*/
/*--- this file.)                                              ---*/
/*---                                                          ---*/
/*----------------------------------------------------------------*/

/* Do a client request.  These are macros rather than a functions so
   as to avoid having an extra frame in stack traces.

   NB: these duplicate definitions in hg_intercepts.c.  But here, we
   have to make do with weaker typing (no definition of Word etc) and
   no assertions, whereas in helgrind.h we can use those facilities.
   Obviously it's important the two sets of definitions are kept in
   sync.

   The commented-out asserts should actually hold, but unfortunately
   they can't be allowed to be visible here, because that would
   require the end-user code to #include <assert.h>.
*/

#define DO_CREQ_v_W(_creqF, _ty1F,_arg1F)                \
   do {                                                  \
      long int _arg1;                                    \
      /* assert(sizeof(_ty1F) == sizeof(long int)); */   \
      _arg1 = (long int)(_arg1F);                        \
      VALGRIND_DO_CLIENT_REQUEST_STMT(                   \
                                 (_creqF),               \
                                 _arg1, 0,0,0,0);        \
   } while (0)

#define DO_CREQ_W_W(_resF, _dfltF, _creqF, _ty1F,_arg1F) \
   do {                                                  \
      long int _arg1;                                    \
      /* assert(sizeof(_ty1F) == sizeof(long int)); */   \
      _arg1 = (long int)(_arg1F);                        \
      _qzz_res = VALGRIND_DO_CLIENT_REQUEST_EXPR(        \
                                 (_dfltF),               \
                                 (_creqF),               \
                                 _arg1, 0,0,0,0);        \
      _resF = _qzz_res;                                  \
   } while (0)

#define DO_CREQ_v_WW(_creqF, _ty1F,_arg1F, _ty2F,_arg2F) \
   do {                                                  \
      long int _arg1, _arg2;                             \
      /* assert(sizeof(_ty1F) == sizeof(long int)); */   \
      /* assert(sizeof(_ty2F) == sizeof(long int)); */   \
      _arg1 = (long int)(_arg1F);                        \
      _arg2 = (long int)(_arg2F);                        \
      VALGRIND_DO_CLIENT_REQUEST_STMT(                   \
                                 (_creqF),               \
                                 _arg1,_arg2,0,0,0);     \
   } while (0)

#define DO_CREQ_v_WWW(_creqF, _ty1F,_arg1F,              \
                      _ty2F,_arg2F, _ty3F, _arg3F)       \
   do {                                                  \
      long int _arg1, _arg2, _arg3;                      \
      /* assert(sizeof(_ty1F) == sizeof(long int)); */   \
      /* assert(sizeof(_ty2F) == sizeof(long int)); */   \
      /* assert(sizeof(_ty3F) == sizeof(long int)); */   \
      _arg1 = (long int)(_arg1F);                        \
      _arg2 = (long int)(_arg2F);                        \
      _arg3 = (long int)(_arg3F);                        \
      VALGRIND_DO_CLIENT_REQUEST_STMT(                   \
                                 (_creqF),               \
                                 _arg1,_arg2,_arg3,0,0); \
   } while (0)

#define DO_CREQ_W_WWW(_resF, _dfltF, _creqF, _ty1F,_arg1F, \
                      _ty2F,_arg2F, _ty3F, _arg3F)       \
   do {                                                  \
      long int _qzz_res;                                 \
      long int _arg1, _arg2, _arg3;                      \
      /* assert(sizeof(_ty1F) == sizeof(long int)); */   \
      _arg1 = (long int)(_arg1F);                        \
      _arg2 = (long int)(_arg2F);                        \
      _arg3 = (long int)(_arg3F);                        \
      _qzz_res = VALGRIND_DO_CLIENT_REQUEST_EXPR(        \
                                 (_dfltF),               \
                                 (_creqF),               \
                                 _arg1,_arg2,_arg3,0,0); \
      _resF = _qzz_res;                                  \
   } while (0)



#define _HG_CLIENTREQ_UNIMP(_qzz_str)                    \
   DO_CREQ_v_W(_VG_USERREQ__HG_CLIENTREQ_UNIMP,          \
               (char*),(_qzz_str))


/*----------------------------------------------------------------*/
/*---                                                          ---*/
/*--- Helgrind-native requests.  These allow access to         ---*/
/*--- the same set of annotation primitives that are used      ---*/
/*--- to build the POSIX pthread wrappers.                     ---*/
/*---                                                          ---*/
/*----------------------------------------------------------------*/

/* ----------------------------------------------------------
   For describing ordinary mutexes (non-rwlocks).  For rwlock
   descriptions see ANNOTATE_RWLOCK_* below.
   ---------------------------------------------------------- */

/* Notify here immediately after mutex creation.  _mbRec == 0 for a
   non-recursive mutex, 1 for a recursive mutex. */
#define VALGRIND_HG_MUTEX_INIT_POST(_mutex, _mbRec)          \
   DO_CREQ_v_WW(_VG_USERREQ__HG_PTHREAD_MUTEX_INIT_POST,     \
                void*,(_mutex), long,(_mbRec))

/* Notify here immediately before mutex acquisition.  _isTryLock == 0
   for a normal acquisition, 1 for a "try" style acquisition. */
#define VALGRIND_HG_MUTEX_LOCK_PRE(_mutex, _isTryLock)       \
   DO_CREQ_v_WW(_VG_USERREQ__HG_PTHREAD_MUTEX_ACQUIRE_PRE,   \
                void*,(_mutex), long,(_isTryLock))

/* Notify here immediately after a successful mutex acquisition. */
#define VALGRIND_HG_MUTEX_LOCK_POST(_mutex)                  \
   DO_CREQ_v_W(_VG_USERREQ__HG_PTHREAD_MUTEX_ACQUIRE_POST,   \
               void*,(_mutex))

/* Notify here immediately before a mutex release. */
#define VALGRIND_HG_MUTEX_UNLOCK_PRE(_mutex)                 \
   DO_CREQ_v_W(_VG_USERREQ__HG_PTHREAD_MUTEX_UNLOCK_PRE,     \
               void*,(_mutex))

/* Notify here immediately after a mutex release. */
#define VALGRIND_HG_MUTEX_UNLOCK_POST(_mutex)                \
   DO_CREQ_v_W(_VG_USERREQ__HG_PTHREAD_MUTEX_UNLOCK_POST,    \
               void*,(_mutex))

/* Notify here immediately before mutex destruction. */
#define VALGRIND_HG_MUTEX_DESTROY_PRE(_mutex)                \
   DO_CREQ_v_W(_VG_USERREQ__HG_PTHREAD_MUTEX_DESTROY_PRE,    \
               void*,(_mutex))

/* ----------------------------------------------------------
   For describing semaphores.
   ---------------------------------------------------------- */

/* Notify here immediately after semaphore creation. */
#define VALGRIND_HG_SEM_INIT_POST(_sem, _value)              \
   DO_CREQ_v_WW(_VG_USERREQ__HG_POSIX_SEM_INIT_POST,         \
                void*, (_sem), unsigned long, (_value))

/* Notify here immediately after a semaphore wait (an acquire-style
   operation) */
#define VALGRIND_HG_SEM_WAIT_POST(_sem)                      \
   DO_CREQ_v_W(_VG_USERREQ__HG_POSIX_SEM_ACQUIRED,           \
               void*,(_sem))

/* Notify here immediately before semaphore post (a release-style
   operation) */
#define VALGRIND_HG_SEM_POST_PRE(_sem)                       \
   DO_CREQ_v_W(_VG_USERREQ__HG_POSIX_SEM_RELEASED,           \
               void*,(_sem))

/* Notify here immediately before semaphore destruction. */
#define VALGRIND_HG_SEM_DESTROY_PRE(_sem)                    \
   DO_CREQ_v_W(_VG_USERREQ__HG_POSIX_SEM_DESTROY_PRE,        \
               void*, (_sem))

/* ----------------------------------------------------------
   For describing barriers.
   ---------------------------------------------------------- */

/* Notify here immediately before barrier creation.  _count is the
   capacity.  _resizable == 0 means the barrier may not be resized, 1
   means it may be. */
#define VALGRIND_HG_BARRIER_INIT_PRE(_bar, _count, _resizable) \
   DO_CREQ_v_WWW(_VG_USERREQ__HG_PTHREAD_BARRIER_INIT_PRE,   \
                 void*,(_bar),                               \
                 unsigned long,(_count),                     \
                 unsigned long,(_resizable))

/* Notify here immediately before arrival at a barrier. */
#define VALGRIND_HG_BARRIER_WAIT_PRE(_bar)                   \
   DO_CREQ_v_W(_VG_USERREQ__HG_PTHREAD_BARRIER_WAIT_PRE,     \
               void*,(_bar))

/* Notify here immediately before a resize (change of barrier
   capacity).  If _newcount >= the existing capacity, then there is no
   change in the state of any threads waiting at the barrier.  If
   _newcount < the existing capacity, and >= _newcount threads are
   currently waiting at the barrier, then this notification is
   considered to also have the effect of telling the checker that all
   waiting threads have now moved past the barrier.  (I can't think of
   any other sane semantics.) */
#define VALGRIND_HG_BARRIER_RESIZE_PRE(_bar, _newcount)      \
   DO_CREQ_v_WW(_VG_USERREQ__HG_PTHREAD_BARRIER_RESIZE_PRE,  \
                void*,(_bar),                                \
                unsigned long,(_newcount))

/* Notify here immediately before barrier destruction. */
#define VALGRIND_HG_BARRIER_DESTROY_PRE(_bar)                \
   DO_CREQ_v_W(_VG_USERREQ__HG_PTHREAD_BARRIER_DESTROY_PRE,  \
               void*,(_bar))

/* ----------------------------------------------------------
   For describing memory ownership changes.
   ---------------------------------------------------------- */

/* Clean memory state.  This makes Helgrind forget everything it knew
   about the specified memory range.  Effectively this announces that
   the specified memory range now "belongs" to the calling thread, so
   that: (1) the calling thread can access it safely without
   synchronisation, and (2) all other threads must sync with this one
   to access it safely.  This is particularly useful for memory
   allocators that wish to recycle memory. */
#define VALGRIND_HG_CLEAN_MEMORY(_qzz_start, _qzz_len)       \
   DO_CREQ_v_WW(VG_USERREQ__HG_CLEAN_MEMORY,                 \
                void*,(_qzz_start),                          \
                unsigned long,(_qzz_len))

/* The same, but for the heap block starting at _qzz_blockstart.  This
   allows painting when we only know the address of an object, but not
   its size, which is sometimes the case in C++ code involving
   inheritance, and in which RTTI is not, for whatever reason,
   available.  Returns the number of bytes painted, which can be zero
   for a zero-sized block.  Hence, return values >= 0 indicate success
   (the block was found), and the value -1 indicates block not
   found, and -2 is returned when not running on Helgrind. */
#define VALGRIND_HG_CLEAN_MEMORY_HEAPBLOCK(_qzz_blockstart)  \
   (__extension__                                            \
   ({long int _npainted;                                     \
     DO_CREQ_W_W(_npainted, (-2)/*default*/,                 \
                 _VG_USERREQ__HG_CLEAN_MEMORY_HEAPBLOCK,     \
                            void*,(_qzz_blockstart));        \
     _npainted;                                              \
   }))

/* ----------------------------------------------------------
   For error control.
   ---------------------------------------------------------- */

/* Tell H that an address range is not to be "tracked" until further
   notice.  This puts it in the NOACCESS state, in which case we
   ignore all reads and writes to it.  Useful for ignoring ranges of
   memory where there might be races we don't want to see.  If the
   memory is subsequently reallocated via malloc/new/stack allocation,
   then it is put back in the trackable state.  Hence it is safe in
   the situation where checking is disabled, the containing area is
   deallocated and later reallocated for some other purpose. */
#define VALGRIND_HG_DISABLE_CHECKING(_qzz_start, _qzz_len)   \
   DO_CREQ_v_WW(_VG_USERREQ__HG_ARANGE_MAKE_UNTRACKED,       \
                 void*,(_qzz_start),                         \
                 unsigned long,(_qzz_len))

/* And put it back into the normal "tracked" state, that is, make it
   once again subject to the normal race-checking machinery.  This
   puts it in the same state as new memory allocated by this thread --
   that is, basically owned exclusively by this thread. */
#define VALGRIND_HG_ENABLE_CHECKING(_qzz_start, _qzz_len)    \
   DO_CREQ_v_WW(_VG_USERREQ__HG_ARANGE_MAKE_TRACKED,         \
                 void*,(_qzz_start),                         \
                 unsigned long,(_qzz_len))


/*  Checks the accessibility bits for addresses [zza..zza+zznbytes-1].
    If zzabits array is provided, copy the accessibility bits in zzabits.
   Return values:
     -2   if not running on helgrind
     -1   if any parts of zzabits is not addressable
     >= 0 : success.
   When success, it returns the nr of addressable bytes found.
      So, to check that a whole range is addressable, check
         VALGRIND_HG_GET_ABITS(addr,NULL,len) == len
      In addition, if you want to examine the addressability of each
      byte of the range, you need to provide a non NULL ptr as
      second argument, pointing to an array of unsigned char
      of length len.
      Addressable bytes are indicated with 0xff.
      Non-addressable bytes are indicated with 0x00.
*/
#define VALGRIND_HG_GET_ABITS(zza,zzabits,zznbytes)          \
   (__extension__                                            \
   ({long int _res;                                          \
      DO_CREQ_W_WWW(_res, (-2)/*default*/,                   \
                    _VG_USERREQ__HG_GET_ABITS,               \
                    void*,(zza), void*,(zzabits),            \
                    unsigned long,(zznbytes));               \
      _res;                                                  \
   }))

/* End-user request for Ada applications compiled with GNAT.
   Helgrind understands the Ada concept of Ada task dependencies and
   terminations. See Ada Reference Manual section 9.3 "Task Dependence 
   - Termination of Tasks".
   However, in some cases, the master of (terminated) tasks completes
   only when the application exits. An example of this is dynamically
   allocated tasks with an access type defined at Library Level.
   By default, the state of such tasks in Helgrind will be 'exited but
   join not done yet'. Many tasks in such a state are however causing
   Helgrind CPU and memory to increase significantly.
   VALGRIND_HG_GNAT_DEPENDENT_MASTER_JOIN can be used to indicate
   to Helgrind that a not yet completed master has however already
   'seen' the termination of a dependent : this is conceptually the
   same as a pthread_join and causes the cleanup of the dependent
   as done by Helgrind when a master completes.
   This allows to avoid the overhead in helgrind caused by such tasks.
   A typical usage for a master to indicate it has done conceptually a join
   with a dependent task before the master completes is:
      while not Dep_Task'Terminated loop
         ... do whatever to wait for Dep_Task termination.
      end loop;
      VALGRIND_HG_GNAT_DEPENDENT_MASTER_JOIN
        (Dep_Task'Identity,
         Ada.Task_Identification.Current_Task);
    Note that VALGRIND_HG_GNAT_DEPENDENT_MASTER_JOIN should be a binding
    to a C function built with the below macro. */
#define VALGRIND_HG_GNAT_DEPENDENT_MASTER_JOIN(_qzz_dep, _qzz_master) \
   DO_CREQ_v_WW(_VG_USERREQ__HG_GNAT_DEPENDENT_MASTER_JOIN,           \
                void*,(_qzz_dep),                                     \
                void*,(_qzz_master))

/*----------------------------------------------------------------*/
/*---                                                          ---*/
/*--- ThreadSanitizer-compatible requests                      ---*/
/*--- (mostly unimplemented)                                   ---*/
/*---                                                          ---*/
/*----------------------------------------------------------------*/

/* A quite-broad set of annotations, as used in the ThreadSanitizer
   project.  This implementation aims to be a (source-level)
   compatible implementation of the macros defined in:

   http://code.google.com/p/data-race-test/source
          /browse/trunk/dynamic_annotations/dynamic_annotations.h

   (some of the comments below are taken from the above file)

   The implementation here is very incomplete, and intended as a
   starting point.  Many of the macros are unimplemented.  Rather than
   allowing unimplemented macros to silently do nothing, they cause an
   assertion.  Intention is to implement them on demand.

   The major use of these macros is to make visible to race detectors,
   the behaviour (effects) of user-implemented synchronisation
   primitives, that the detectors could not otherwise deduce from the
   normal observation of pthread etc calls.

   Some of the macros are no-ops in Helgrind.  That's because Helgrind
   is a pure happens-before detector, whereas ThreadSanitizer uses a
   hybrid lockset and happens-before scheme, which requires more
   accurate annotations for correct operation.

   The macros are listed in the same order as in dynamic_annotations.h
   (URL just above).

   I should point out that I am less than clear about the intended
   semantics of quite a number of them.  Comments and clarifications
   welcomed!
*/

/* ----------------------------------------------------------------
   These four allow description of user-level condition variables,
   apparently in the style of POSIX's pthread_cond_t.  Currently
   unimplemented and will assert.
   ----------------------------------------------------------------
*/
/* Report that wait on the condition variable at address CV has
   succeeded and the lock at address LOCK is now held.  CV and LOCK
   are completely arbitrary memory addresses which presumably mean
   something to the application, but are meaningless to Helgrind. */
#define ANNOTATE_CONDVAR_LOCK_WAIT(cv, lock) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_CONDVAR_LOCK_WAIT")

/* Report that wait on the condition variable at CV has succeeded.
   Variant w/o lock. */
#define ANNOTATE_CONDVAR_WAIT(cv) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_CONDVAR_WAIT")

/* Report that we are about to signal on the condition variable at
   address CV. */
#define ANNOTATE_CONDVAR_SIGNAL(cv) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_CONDVAR_SIGNAL")
  
/* Report that we are about to signal_all on the condition variable at
   CV. */
#define ANNOTATE_CONDVAR_SIGNAL_ALL(cv) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_CONDVAR_SIGNAL_ALL")


/* ----------------------------------------------------------------
   Create completely arbitrary happens-before edges between threads.

   If threads T1 .. Tn all do ANNOTATE_HAPPENS_BEFORE(obj) and later
   (w.r.t. some notional global clock for the computation) thread Tm
   does ANNOTATE_HAPPENS_AFTER(obj), then Helgrind will regard all
   memory accesses done by T1 .. Tn before the ..BEFORE.. call as
   happening-before all memory accesses done by Tm after the
   ..AFTER.. call.  Hence Helgrind won't complain about races if Tm's
   accesses afterwards are to the same locations as accesses before by
   any of T1 .. Tn.

   OBJ is a machine word (unsigned long, or void*), is completely
   arbitrary, and denotes the identity of some synchronisation object
   you're modelling.

   You must do the _BEFORE call just before the real sync event on the
   signaller's side, and _AFTER just after the real sync event on the
   waiter's side.

   If none of the rest of these macros make sense to you, at least
   take the time to understand these two.  They form the very essence
   of describing arbitrary inter-thread synchronisation events to
   Helgrind.  You can get a long way just with them alone.

   See also, extensive discussion on semantics of this in 
   https://bugs.kde.org/show_bug.cgi?id=243935

   ANNOTATE_HAPPENS_BEFORE_FORGET_ALL(obj) is interim until such time
   as bug 243935 is fully resolved.  It instructs Helgrind to forget
   about any ANNOTATE_HAPPENS_BEFORE calls on the specified object, in
   effect putting it back in its original state.  Once in that state,
   a use of ANNOTATE_HAPPENS_AFTER on it has no effect on the calling
   thread.

   An implementation may optionally release resources it has
   associated with 'obj' when ANNOTATE_HAPPENS_BEFORE_FORGET_ALL(obj)
   happens.  Users are recommended to use
   ANNOTATE_HAPPENS_BEFORE_FORGET_ALL to indicate when a
   synchronisation object is no longer needed, so as to avoid
   potential indefinite resource leaks.
   ----------------------------------------------------------------
*/
#define ANNOTATE_HAPPENS_BEFORE(obj) \
   DO_CREQ_v_W(_VG_USERREQ__HG_USERSO_SEND_PRE, void*,(obj))

#define ANNOTATE_HAPPENS_AFTER(obj) \
   DO_CREQ_v_W(_VG_USERREQ__HG_USERSO_RECV_POST, void*,(obj))

#define ANNOTATE_HAPPENS_BEFORE_FORGET_ALL(obj) \
   DO_CREQ_v_W(_VG_USERREQ__HG_USERSO_FORGET_ALL, void*,(obj))

/* ----------------------------------------------------------------
   Memory publishing.  The TSan sources say:

     Report that the bytes in the range [pointer, pointer+size) are about
     to be published safely. The race checker will create a happens-before
     arc from the call ANNOTATE_PUBLISH_MEMORY_RANGE(pointer, size) to
     subsequent accesses to this memory.

   I'm not sure I understand what this means exactly, nor whether it
   is relevant for a pure h-b detector.  Leaving unimplemented for
   now.
   ----------------------------------------------------------------
*/
#define ANNOTATE_PUBLISH_MEMORY_RANGE(pointer, size) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_PUBLISH_MEMORY_RANGE")

/* DEPRECATED. Don't use it. */
/* #define ANNOTATE_UNPUBLISH_MEMORY_RANGE(pointer, size) */

/* DEPRECATED. Don't use it. */
/* #define ANNOTATE_SWAP_MEMORY_RANGE(pointer, size) */


/* ----------------------------------------------------------------
   TSan sources say:
   
     Instruct the tool to create a happens-before arc between
     MU->Unlock() and MU->Lock().  This annotation may slow down the
     race detector; normally it is used only when it would be
     difficult to annotate each of the mutex's critical sections
     individually using the annotations above.

   If MU is a posix pthread_mutex_t then Helgrind will do this anyway.
   In any case, leave as unimp for now.  I'm unsure about the intended
   behaviour.
   ---------------------------------------------------------------- 
*/
#define ANNOTATE_PURE_HAPPENS_BEFORE_MUTEX(mu) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_PURE_HAPPENS_BEFORE_MUTEX")

/* Deprecated. Use ANNOTATE_PURE_HAPPENS_BEFORE_MUTEX. */
/* #define ANNOTATE_MUTEX_IS_USED_AS_CONDVAR(mu) */


/* ----------------------------------------------------------------
   TSan sources say:
   
     Annotations useful when defining memory allocators, or when
     memory that was protected in one way starts to be protected in
     another.

     Report that a new memory at "address" of size "size" has been
     allocated.  This might be used when the memory has been retrieved
     from a free list and is about to be reused, or when a the locking
     discipline for a variable changes.

   AFAICS this is the same as VALGRIND_HG_CLEAN_MEMORY.
   ---------------------------------------------------------------- 
*/
#define ANNOTATE_NEW_MEMORY(address, size) \
   VALGRIND_HG_CLEAN_MEMORY((address), (size))


/* ----------------------------------------------------------------
   TSan sources say:

     Annotations useful when defining FIFO queues that transfer data
     between threads.

   All unimplemented.  Am not claiming to understand this (yet).
   ---------------------------------------------------------------- 
*/

/* Report that the producer-consumer queue object at address PCQ has
   been created.  The ANNOTATE_PCQ_* annotations should be used only
   for FIFO queues.  For non-FIFO queues use ANNOTATE_HAPPENS_BEFORE
   (for put) and ANNOTATE_HAPPENS_AFTER (for get). */
#define ANNOTATE_PCQ_CREATE(pcq) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_PCQ_CREATE")

/* Report that the queue at address PCQ is about to be destroyed. */
#define ANNOTATE_PCQ_DESTROY(pcq) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_PCQ_DESTROY")

/* Report that we are about to put an element into a FIFO queue at
   address PCQ. */
#define ANNOTATE_PCQ_PUT(pcq) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_PCQ_PUT")

/* Report that we've just got an element from a FIFO queue at address
   PCQ. */
#define ANNOTATE_PCQ_GET(pcq) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_PCQ_GET")


/* ----------------------------------------------------------------
   Annotations that suppress errors.  It is usually better to express
   the program's synchronization using the other annotations, but
   these can be used when all else fails.

   Currently these are all unimplemented.  I can't think of a simple
   way to implement them without at least some performance overhead.
   ----------------------------------------------------------------
*/

/* Report that we may have a benign race at "pointer", with size
   "sizeof(*(pointer))". "pointer" must be a non-void* pointer.  Insert at the
   point where "pointer" has been allocated, preferably close to the point
   where the race happens.  See also ANNOTATE_BENIGN_RACE_STATIC.

   XXX: what's this actually supposed to do?  And what's the type of
   DESCRIPTION?  When does the annotation stop having an effect?
*/
#define ANNOTATE_BENIGN_RACE(pointer, description) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_BENIGN_RACE")

/* Same as ANNOTATE_BENIGN_RACE(address, description), but applies to
   the memory range [address, address+size). */
#define ANNOTATE_BENIGN_RACE_SIZED(address, size, description) \
   VALGRIND_HG_DISABLE_CHECKING(address, size)

/* Request the analysis tool to ignore all reads in the current thread
   until ANNOTATE_IGNORE_READS_END is called.  Useful to ignore
   intentional racey reads, while still checking other reads and all
   writes. */
#define ANNOTATE_IGNORE_READS_BEGIN() \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_IGNORE_READS_BEGIN")

/* Stop ignoring reads. */
#define ANNOTATE_IGNORE_READS_END() \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_IGNORE_READS_END")

/* Similar to ANNOTATE_IGNORE_READS_BEGIN, but ignore writes. */
#define ANNOTATE_IGNORE_WRITES_BEGIN() \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_IGNORE_WRITES_BEGIN")

/* Stop ignoring writes. */
#define ANNOTATE_IGNORE_WRITES_END() \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_IGNORE_WRITES_END")

/* Start ignoring all memory accesses (reads and writes). */
#define ANNOTATE_IGNORE_READS_AND_WRITES_BEGIN() \
   do { \
      ANNOTATE_IGNORE_READS_BEGIN(); \
      ANNOTATE_IGNORE_WRITES_BEGIN(); \
   } while (0)

/* Stop ignoring all memory accesses. */
#define ANNOTATE_IGNORE_READS_AND_WRITES_END() \
   do { \
      ANNOTATE_IGNORE_WRITES_END(); \
      ANNOTATE_IGNORE_READS_END(); \
   } while (0)


/* ----------------------------------------------------------------
   Annotations useful for debugging.

   Again, so for unimplemented, partly for performance reasons.
   ----------------------------------------------------------------
*/

/* Request to trace every access to ADDRESS. */
#define ANNOTATE_TRACE_MEMORY(address) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_TRACE_MEMORY")

/* Report the current thread name to a race detector. */
#define ANNOTATE_THREAD_NAME(name) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_THREAD_NAME")


/* ----------------------------------------------------------------
   Annotations for describing behaviour of user-implemented lock
   primitives.  In all cases, the LOCK argument is a completely
   arbitrary machine word (unsigned long, or void*) and can be any
   value which gives a unique identity to the lock objects being
   modelled.

   We just pretend they're ordinary posix rwlocks.  That'll probably
   give some rather confusing wording in error messages, claiming that
   the arbitrary LOCK values are pthread_rwlock_t*'s, when in fact
   they are not.  Ah well.
   ----------------------------------------------------------------
*/
/* Report that a lock has just been created at address LOCK. */
#define ANNOTATE_RWLOCK_CREATE(lock)                         \
   DO_CREQ_v_W(_VG_USERREQ__HG_PTHREAD_RWLOCK_INIT_POST,     \
               void*,(lock))
    
/* Report that the lock at address LOCK is about to be destroyed. */
#define ANNOTATE_RWLOCK_DESTROY(lock)                        \
   DO_CREQ_v_W(_VG_USERREQ__HG_PTHREAD_RWLOCK_DESTROY_PRE,   \
               void*,(lock))

/* Report that the lock at address LOCK has just been acquired.
   is_w=1 for writer lock, is_w=0 for reader lock. */
#define ANNOTATE_RWLOCK_ACQUIRED(lock, is_w)                 \
  DO_CREQ_v_WW(_VG_USERREQ__HG_PTHREAD_RWLOCK_ACQUIRED,      \
               void*,(lock), unsigned long,(is_w))

/* Report that the lock at address LOCK is about to be released. */
#define ANNOTATE_RWLOCK_RELEASED(lock, is_w)                 \
  DO_CREQ_v_W(_VG_USERREQ__HG_PTHREAD_RWLOCK_RELEASED,       \
              void*,(lock)) /* is_w is ignored */


/* -------------------------------------------------------------
   Annotations useful when implementing barriers.  They are not
   normally needed by modules that merely use barriers.
   The "barrier" argument is a pointer to the barrier object.
   ----------------------------------------------------------------
*/

/* Report that the "barrier" has been initialized with initial
   "count".  If 'reinitialization_allowed' is true, initialization is
   allowed to happen multiple times w/o calling barrier_destroy() */
#define ANNOTATE_BARRIER_INIT(barrier, count, reinitialization_allowed) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_BARRIER_INIT")

/* Report that we are about to enter barrier_wait("barrier"). */
#define ANNOTATE_BARRIER_WAIT_BEFORE(barrier) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_BARRIER_DESTROY")

/* Report that we just exited barrier_wait("barrier"). */
#define ANNOTATE_BARRIER_WAIT_AFTER(barrier) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_BARRIER_DESTROY")

/* Report that the "barrier" has been destroyed. */
#define ANNOTATE_BARRIER_DESTROY(barrier) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_BARRIER_DESTROY")


/* ----------------------------------------------------------------
   Annotations useful for testing race detectors.
   ----------------------------------------------------------------
*/

/* Report that we expect a race on the variable at ADDRESS.  Use only
   in unit tests for a race detector. */
#define ANNOTATE_EXPECT_RACE(address, description) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_EXPECT_RACE")

/* A no-op. Insert where you like to test the interceptors. */
#define ANNOTATE_NO_OP(arg) \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_NO_OP")

/* Force the race detector to flush its state. The actual effect depends on
 * the implementation of the detector. */
#define ANNOTATE_FLUSH_STATE() \
   _HG_CLIENTREQ_UNIMP("ANNOTATE_FLUSH_STATE")

#endif /* __HELGRIND_H */
