// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_CONNECTION_QUOTA_H
#define GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_CONNECTION_QUOTA_H

#include <grpc/support/port_platform.h>

#include <cstddef>
#include <limits>

#include "absl/base/thread_annotations.h"
#include "src/core/lib/resource_quota/memory_quota.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"

namespace grpc_core {

// Tracks the amount of threads in a resource quota.
class ConnectionQuota : public RefCounted<ConnectionQuota> {
 public:
  ConnectionQuota();
  ~ConnectionQuota() override = default;

  ConnectionQuota(const ConnectionQuota&) = delete;
  ConnectionQuota& operator=(const ConnectionQuota&) = delete;

  // Set the maximum number of allowed incoming connections on the server.
  void SetMaxIncomingConnections(int max_incoming_connections);

  // Returns true if the incoming connection is allowed to be accepted on the
  // server.
  bool AllowIncomingConnection(MemoryQuotaRefPtr mem_quota,
                               absl::string_view peer);

  // Mark connections as closed.
  void ReleaseConnections(int num_connections);

  int TestOnlyActiveIncomingConnections() const {
    return active_incoming_connections_;
  }

 private:
  std::atomic<int> active_incoming_connections_{0};
  std::atomic<int> max_incoming_connections_{std::numeric_limits<int>::max()};
};

using ConnectionQuotaRefPtr = RefCountedPtr<ConnectionQuota>;

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_CONNECTION_QUOTA_H
