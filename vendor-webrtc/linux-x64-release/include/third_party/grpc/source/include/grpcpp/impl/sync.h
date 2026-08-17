//
//
// Copyright 2019 gRPC authors.
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

#ifndef GRPCPP_IMPL_SYNC_H
#define GRPCPP_IMPL_SYNC_H

#include <grpc/support/port_platform.h>

#ifdef GPR_HAS_PTHREAD_H
#include <pthread.h>
#endif

#include <grpc/support/sync.h>
#include <grpc/support/time.h>

#include <mutex>

#include "absl/log/absl_check.h"
#include "absl/synchronization/mutex.h"

// The core library is not accessible in C++ codegen headers, and vice versa.
// Thus, we need to have duplicate headers with similar functionality.
// Make sure any change to this file is also reflected in
// src/core/util/sync.h too.
//
// Whenever possible, prefer "src/core/util/sync.h" over this file,
// since in core we do not rely on g_core_codegen_interface and hence do not
// pay the costs of virtual function calls.

namespace grpc {
namespace internal {

#ifdef GPR_ABSEIL_SYNC

using Mutex = absl::Mutex;
using MutexLock = absl::MutexLock;
using ReleasableMutexLock = absl::ReleasableMutexLock;
using CondVar = absl::CondVar;

#else

class ABSL_LOCKABLE Mutex {
 public:
  Mutex() { gpr_mu_init(&mu_); }
  ~Mutex() { gpr_mu_destroy(&mu_); }

  Mutex(const Mutex&) = delete;
  Mutex& operator=(const Mutex&) = delete;

  void Lock() ABSL_EXCLUSIVE_LOCK_FUNCTION() { gpr_mu_lock(&mu_); }
  void Unlock() ABSL_UNLOCK_FUNCTION() { gpr_mu_unlock(&mu_); }

 private:
  union {
    gpr_mu mu_;
    std::mutex do_not_use_sth_;
#ifdef GPR_HAS_PTHREAD_H
    pthread_mutex_t do_not_use_pth_;
#endif
  };

  friend class CondVar;
};

class ABSL_SCOPED_LOCKABLE MutexLock {
 public:
  explicit MutexLock(Mutex* mu) ABSL_EXCLUSIVE_LOCK_FUNCTION(mu) : mu_(mu) {
    mu_->Lock();
  }
  ~MutexLock() ABSL_UNLOCK_FUNCTION() { mu_->Unlock(); }

  MutexLock(const MutexLock&) = delete;
  MutexLock& operator=(const MutexLock&) = delete;

 private:
  Mutex* const mu_;
};

class ABSL_SCOPED_LOCKABLE ReleasableMutexLock {
 public:
  explicit ReleasableMutexLock(Mutex* mu) ABSL_EXCLUSIVE_LOCK_FUNCTION(mu)
      : mu_(mu) {
    mu_->Lock();
  }
  ~ReleasableMutexLock() ABSL_UNLOCK_FUNCTION() {
    if (!released_) mu_->Unlock();
  }

  ReleasableMutexLock(const ReleasableMutexLock&) = delete;
  ReleasableMutexLock& operator=(const ReleasableMutexLock&) = delete;

  void Release() ABSL_UNLOCK_FUNCTION() {
    ABSL_DCHECK(!released_);
    released_ = true;
    mu_->Unlock();
  }

 private:
  Mutex* const mu_;
  bool released_ = false;
};

class CondVar {
 public:
  CondVar() { gpr_cv_init(&cv_); }
  ~CondVar() { gpr_cv_destroy(&cv_); }

  CondVar(const CondVar&) = delete;
  CondVar& operator=(const CondVar&) = delete;

  void Signal() { gpr_cv_signal(&cv_); }
  void SignalAll() { gpr_cv_broadcast(&cv_); }

  void Wait(Mutex* mu) {
    gpr_cv_wait(&cv_, &mu->mu_, gpr_inf_future(GPR_CLOCK_REALTIME));
  }

 private:
  gpr_cv cv_;
};

#endif  // GPR_ABSEIL_SYNC

template <typename Predicate>
GRPC_DEPRECATED("incompatible with thread safety analysis")
static void WaitUntil(CondVar* cv, Mutex* mu, Predicate pred) {
  while (!pred()) {
    cv->Wait(mu);
  }
}

}  // namespace internal
}  // namespace grpc

#endif  // GRPCPP_IMPL_SYNC_H
