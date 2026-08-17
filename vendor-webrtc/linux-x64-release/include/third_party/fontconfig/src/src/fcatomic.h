/*
 * Mutex operations.  Originally copied from HarfBuzz.
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

#ifndef _FCATOMIC_H_
#define _FCATOMIC_H_

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif


/* atomic_int */

/* We need external help for these */

#if 0

typedef <type> fc_atomic_int_t;
#define FC_ATOMIC_INT_FORMAT		"<printf format for fc_atomic_int_t>"
#define fc_atomic_int_add(AI, V)	o = (AI), (AI) += (V), o // atomic acquire/release

#define fc_atomic_ptr_get(P)		*(P) // atomic acquire
#define fc_atomic_ptr_cmpexch(P,O,N)	*(P) == (O) ? (*(P) = (N), FcTrue) : FcFalse // atomic release


#elif !defined(FC_NO_MT) && defined(HAVE_STDATOMIC_PRIMITIVES)

#include <stdatomic.h>

typedef atomic_int fc_atomic_int_t;
#define FC_ATOMIC_INT_FORMAT		"d"
#define fc_atomic_int_add(AI, V)	atomic_fetch_add_explicit (&(AI), (V), memory_order_acq_rel)

#define fc_atomic_ptr_get(P)		atomic_load_explicit ((_Atomic(void *)*) (P), memory_order_acquire)
static inline FcBool _fc_atomic_ptr_cmpexch(_Atomic(void *)*P, void * O, _Atomic(void *) N) {
  return atomic_compare_exchange_strong_explicit(P, &O, N, memory_order_release, memory_order_relaxed);
}
#define fc_atomic_ptr_cmpexch(P,O,N)	_fc_atomic_ptr_cmpexch ((_Atomic(void *)*) (P), (O), (N))

/* Casting -1 to _Atomic(int) produces a compiler error with Clang (but not GCC)
 * so we have to override FC_REF_CONSTANT_VALUE for stdatomic.h atomics.
 * See https://bugs.llvm.org/show_bug.cgi?id=40249. */
#define FC_REF_CONSTANT_VALUE (-1)

#elif !defined(FC_NO_MT) && defined(_MSC_VER) || defined(__MINGW32__)

#include "fcwindows.h"

typedef LONG fc_atomic_int_t;
#define FC_ATOMIC_INT_FORMAT		"ld"
#define fc_atomic_int_add(AI, V)	InterlockedExchangeAdd (&(AI), (V))

#define fc_atomic_ptr_get(P)		(InterlockedCompareExchangePointerAcquire ((void **) (P), NULL, NULL))
#define fc_atomic_ptr_cmpexch(P,O,N)	(InterlockedCompareExchangePointer ((void **) (P), (void *) (N), (void *) (O)) == (void *) (O))


#elif !defined(FC_NO_MT) && defined(__APPLE__)

#include <libkern/OSAtomic.h>
#include <AvailabilityMacros.h>

typedef int fc_atomic_int_t;
#define FC_ATOMIC_INT_FORMAT		"d"
#define fc_atomic_int_add(AI, V)	(OSAtomicAdd32Barrier ((V), &(AI)) - (V))

#if SIZEOF_VOID_P == 8
#define fc_atomic_ptr_get(P)		OSAtomicAdd64Barrier (0, (int64_t*)(P))
#elif SIZEOF_VOID_P == 4
#define fc_atomic_ptr_get(P)		OSAtomicAdd32Barrier (0, (int32_t*)(P))
#else
#error "SIZEOF_VOID_P not 4 or 8 (assumes CHAR_BIT is 8)"
#endif

#if (MAC_OS_X_VERSION_MIN_REQUIRED >= 1050 || __IPHONE_OS_VERSION_MIN_REQUIRED >= 20100)
#define fc_atomic_ptr_cmpexch(P,O,N)	OSAtomicCompareAndSwapPtrBarrier ((void *) (O), (void *) (N), (void **) (P))
#else
#if __LP64__
#define fc_atomic_ptr_cmpexch(P,O,N)	OSAtomicCompareAndSwap64Barrier ((int64_t) (O), (int64_t) (N), (int64_t *) (P))
#else
#define fc_atomic_ptr_cmpexch(P,O,N)	OSAtomicCompareAndSwap32Barrier ((int32_t) (O), (int32_t) (N), (int32_t *) (P))
#endif
#endif


#elif !defined(FC_NO_MT) && defined(HAVE_INTEL_ATOMIC_PRIMITIVES)

typedef int fc_atomic_int_t;
#define FC_ATOMIC_INT_FORMAT		"d"
#define fc_atomic_int_add(AI, V)	__sync_fetch_and_add (&(AI), (V))

#define fc_atomic_ptr_get(P)		(void *) (__sync_fetch_and_add ((P), 0))
#define fc_atomic_ptr_cmpexch(P,O,N)	__sync_bool_compare_and_swap ((P), (O), (N))


#elif !defined(FC_NO_MT) && defined(HAVE_SOLARIS_ATOMIC_OPS)

#include <atomic.h>
#include <mbarrier.h>

typedef unsigned int fc_atomic_int_t;
#define FC_ATOMIC_INT_FORMAT		"u"
#define fc_atomic_int_add(AI, V)	( ({__machine_rw_barrier ();}), atomic_add_int_nv (&(AI), (V)) - (V))

#define fc_atomic_ptr_get(P)		( ({__machine_rw_barrier ();}), (void *) *(P))
#define fc_atomic_ptr_cmpexch(P,O,N)	( ({__machine_rw_barrier ();}), atomic_cas_ptr ((P), (O), (N)) == (void *) (O) ? FcTrue : FcFalse)


#elif !defined(FC_NO_MT)

#define FC_ATOMIC_INT_NIL 1 /* Warn that fallback implementation is in use. */
typedef volatile int fc_atomic_int_t;
#define FC_ATOMIC_INT_FORMAT		"d"
#define fc_atomic_int_add(AI, V)	(((AI) += (V)) - (V))

#define fc_atomic_ptr_get(P)		((void *) *(P))
#define fc_atomic_ptr_cmpexch(P,O,N)	(* (void * volatile *) (P) == (void *) (O) ? (* (void * volatile *) (P) = (void *) (N), FcTrue) : FcFalse)


#else /* FC_NO_MT */

typedef int fc_atomic_int_t;
#define FC_ATOMIC_INT_FORMAT		"d"
#define fc_atomic_int_add(AI, V)	(((AI) += (V)) - (V))

#define fc_atomic_ptr_get(P)		((void *) *(P))
#define fc_atomic_ptr_cmpexch(P,O,N)	(* (void **) (P) == (void *) (O) ? (* (void **) (P) = (void *) (N), FcTrue) : FcFalse)

#endif

/* reference count */
#ifndef FC_REF_CONSTANT_VALUE
#define FC_REF_CONSTANT_VALUE ((fc_atomic_int_t) -1)
#endif
#define FC_REF_CONSTANT {FC_REF_CONSTANT_VALUE}
typedef struct _FcRef { fc_atomic_int_t count; } FcRef;
static inline void   FcRefInit     (FcRef *r, int v) { r->count = v; }
static inline int    FcRefInc      (FcRef *r) { return fc_atomic_int_add (r->count, +1); }
static inline int    FcRefDec      (FcRef *r) { return fc_atomic_int_add (r->count, -1); }
static inline int    FcRefAdd      (FcRef *r, int v) { return fc_atomic_int_add (r->count, v); }
static inline void   FcRefSetConst (FcRef *r) { r->count = FC_REF_CONSTANT_VALUE; }
static inline FcBool FcRefIsConst  (const FcRef *r) { return r->count == FC_REF_CONSTANT_VALUE; }

#endif /* _FCATOMIC_H_ */
