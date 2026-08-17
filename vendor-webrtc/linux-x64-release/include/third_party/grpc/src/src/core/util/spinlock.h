//
//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_UTIL_SPINLOCK_H
#define GRPC_SRC_CORE_UTIL_SPINLOCK_H

#include <grpc/support/atm.h>
#include <grpc/support/port_platform.h>

// Simple spinlock. No backoff strategy, gpr_spinlock_lock is almost always
// a concurrency code smell. Code must _never_ block while holding a spinlock
// as this could lead to a deadlock under a cooperative multithreading model.
struct gpr_spinlock {
  gpr_atm atm;
};
#ifdef __cplusplus
#define GPR_SPINLOCK_INITIALIZER (gpr_spinlock{0})
#else
#define GPR_SPINLOCK_INITIALIZER ((gpr_spinlock){0})
#endif
#define GPR_SPINLOCK_STATIC_INITIALIZER {0}

#define gpr_spinlock_trylock(lock) (gpr_atm_acq_cas(&(lock)->atm, 0, 1))
#define gpr_spinlock_unlock(lock) (gpr_atm_rel_store(&(lock)->atm, 0))
// Although the following code spins without any library or system calls, it
// still functions under cooperative multithreading. The principle is that
// the lock holder can't block, so it will be scheduled onto its system thread
// for the entire critical section. By the time another thread attempts a lock,
// it will either get it immediately or will be scheduled onto another system
// thread that is different from the current lockholder. There is no chance of
// waiting for a lockholder scheduled to the same system thread.
#define gpr_spinlock_lock(lock) \
  do {                          \
  } while (!gpr_spinlock_trylock((lock)))

#endif  // GRPC_SRC_CORE_UTIL_SPINLOCK_H
