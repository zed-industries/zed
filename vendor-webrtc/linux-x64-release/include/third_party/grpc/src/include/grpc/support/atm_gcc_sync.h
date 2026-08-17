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

#ifndef GRPC_SUPPORT_ATM_GCC_SYNC_H
#define GRPC_SUPPORT_ATM_GCC_SYNC_H

/* variant of atm_platform.h for gcc and gcc-like compilers with __sync_*
   interface */
#include <grpc/support/port_platform.h>

typedef intptr_t gpr_atm;
#define GPR_ATM_MAX INTPTR_MAX
#define GPR_ATM_MIN INTPTR_MIN

#define GPR_ATM_COMPILE_BARRIER_() __asm__ __volatile__("" : : : "memory")

#if defined(__i386) || defined(__x86_64__)
/* All loads are acquire loads and all stores are release stores.  */
#define GPR_ATM_LS_BARRIER_() GPR_ATM_COMPILE_BARRIER_()
#else
#define GPR_ATM_LS_BARRIER_() gpr_atm_full_barrier()
#endif

#define gpr_atm_full_barrier() (__sync_synchronize())

static __inline gpr_atm gpr_atm_acq_load(const gpr_atm* p) {
  gpr_atm value = *p;
  GPR_ATM_LS_BARRIER_();
  return value;
}

static __inline gpr_atm gpr_atm_no_barrier_load(const gpr_atm* p) {
  gpr_atm value = *p;
  GPR_ATM_COMPILE_BARRIER_();
  return value;
}

static __inline void gpr_atm_rel_store(gpr_atm* p, gpr_atm value) {
  GPR_ATM_LS_BARRIER_();
  *p = value;
}

static __inline void gpr_atm_no_barrier_store(gpr_atm* p, gpr_atm value) {
  GPR_ATM_COMPILE_BARRIER_();
  *p = value;
}

#undef GPR_ATM_LS_BARRIER_
#undef GPR_ATM_COMPILE_BARRIER_

#define gpr_atm_no_barrier_fetch_add(p, delta) \
  gpr_atm_full_fetch_add((p), (delta))
#define gpr_atm_full_fetch_add(p, delta) (__sync_fetch_and_add((p), (delta)))

#define gpr_atm_no_barrier_cas(p, o, n) gpr_atm_acq_cas((p), (o), (n))
#define gpr_atm_acq_cas(p, o, n) (__sync_bool_compare_and_swap((p), (o), (n)))
#define gpr_atm_rel_cas(p, o, n) gpr_atm_acq_cas((p), (o), (n))
#define gpr_atm_full_cas(p, o, n) gpr_atm_acq_cas((p), (o), (n))

static __inline gpr_atm gpr_atm_full_xchg(gpr_atm* p, gpr_atm n) {
  gpr_atm cur;
  do {
    cur = gpr_atm_acq_load(p);
  } while (!gpr_atm_rel_cas(p, cur, n));
  return cur;
}

#endif /* GRPC_SUPPORT_ATM_GCC_SYNC_H */
