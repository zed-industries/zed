// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_THREAD_QUOTA_H
#define GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_THREAD_QUOTA_H

#include <grpc/support/port_platform.h>

#include <cstddef>
#include <limits>

#include "absl/base/thread_annotations.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"

namespace grpc_core {

// Tracks the amount of threads in a resource quota.
class ThreadQuota : public RefCounted<ThreadQuota> {
 public:
  ThreadQuota();
  ~ThreadQuota() override;

  ThreadQuota(const ThreadQuota&) = delete;
  ThreadQuota& operator=(const ThreadQuota&) = delete;

  // Set the maximum number of threads that can be used by this quota.
  // If there are more, new reservations will fail until the quota is available.
  void SetMax(size_t new_max);

  // Try to allocate some number of threads.
  // Returns true if the allocation succeeded, false otherwise.
  bool Reserve(size_t num_threads);

  // Release some number of threads.
  void Release(size_t num_threads);

 private:
  Mutex mu_;
  size_t allocated_ ABSL_GUARDED_BY(mu_) = 0;
  size_t max_ ABSL_GUARDED_BY(mu_) = std::numeric_limits<size_t>::max();
};

using ThreadQuotaPtr = RefCountedPtr<ThreadQuota>;

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_THREAD_QUOTA_H
