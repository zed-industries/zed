// Copyright 2022 The gRPC Authors
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

#ifndef GRPC_SRC_CORE_UTIL_NOTIFICATION_H
#define GRPC_SRC_CORE_UTIL_NOTIFICATION_H

#include <grpc/support/port_platform.h>

#include "absl/time/clock.h"
#include "absl/time/time.h"
#include "src/core/util/sync.h"

namespace grpc_core {

// Polyfill for absl::Notification until we can use that type.
class Notification {
 public:
  void Notify() {
    MutexLock lock(&mu_);
    notified_ = true;
    cv_.SignalAll();
  }

  void WaitForNotification() {
    MutexLock lock(&mu_);
    while (!notified_) {
      cv_.Wait(&mu_);
    }
  }

  bool WaitForNotificationWithTimeout(absl::Duration timeout) {
    auto now = absl::Now();
    auto deadline = now + timeout;
    MutexLock lock(&mu_);
    while (!notified_ && now < deadline) {
      cv_.WaitWithTimeout(&mu_, deadline - now);
      now = absl::Now();
    }
    return notified_;
  }

  bool HasBeenNotified() {
    MutexLock lock(&mu_);
    return notified_;
  }

 private:
  Mutex mu_;
  CondVar cv_;
  bool notified_ = false;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_NOTIFICATION_H
