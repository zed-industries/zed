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

#ifndef GRPC_SUPPORT_ATM_H
#define GRPC_SUPPORT_ATM_H

/** This interface provides atomic operations and barriers.
   It is internal to gpr support code and should not be used outside it.

   If an operation with acquire semantics precedes another memory access by the
   same thread, the operation will precede that other access as seen by other
   threads.

   If an operation with release semantics follows another memory access by the
   same thread, the operation will follow that other access as seen by other
   threads.

   Routines with "acq" or "full" in the name have acquire semantics.  Routines
   with "rel" or "full" in the name have release semantics.  Routines with
   "no_barrier" in the name have neither acquire not release semantics.

   The routines may be implemented as macros.

   // Atomic operations act on an integral_type gpr_atm that is guaranteed to
   // be the same size as a pointer.
   typedef intptr_t gpr_atm;

   // A memory barrier, providing both acquire and release semantics, but not
   // otherwise acting on memory.
   void gpr_atm_full_barrier(void);

   // Atomically return *p, with acquire semantics.
   gpr_atm gpr_atm_acq_load(gpr_atm *p);
   gpr_atm gpr_atm_no_barrier_load(gpr_atm *p);

   // Atomically set *p = value, with release semantics.
   void gpr_atm_rel_store(gpr_atm *p, gpr_atm value);

   // Atomically add delta to *p, and return the old value of *p, with
   // the barriers specified.
   gpr_atm gpr_atm_no_barrier_fetch_add(gpr_atm *p, gpr_atm delta);
   gpr_atm gpr_atm_full_fetch_add(gpr_atm *p, gpr_atm delta);

   // Atomically, if *p==o, set *p=n and return non-zero otherwise return 0,
   // with the barriers specified if the operation succeeds.
   int gpr_atm_no_barrier_cas(gpr_atm *p, gpr_atm o, gpr_atm n);
   int gpr_atm_acq_cas(gpr_atm *p, gpr_atm o, gpr_atm n);
   int gpr_atm_rel_cas(gpr_atm *p, gpr_atm o, gpr_atm n);
   int gpr_atm_full_cas(gpr_atm *p, gpr_atm o, gpr_atm n);

   // Atomically, set *p=n and return the old value of *p
   gpr_atm gpr_atm_full_xchg(gpr_atm *p, gpr_atm n);
*/

#include <grpc/support/port_platform.h>

#if defined(GPR_GCC_ATOMIC)
#include <grpc/support/atm_gcc_atomic.h>  // IWYU pragma: export
#elif defined(GPR_GCC_SYNC)
#include <grpc/support/atm_gcc_sync.h>  // IWYU pragma: export
#elif defined(GPR_WINDOWS_ATOMIC)
#include <grpc/support/atm_windows.h>  // IWYU pragma: export
#else
#error could not determine platform for atm
#endif

#endif /* GRPC_SUPPORT_ATM_H */
