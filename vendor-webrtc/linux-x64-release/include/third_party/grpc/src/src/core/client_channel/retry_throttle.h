//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CLIENT_CHANNEL_RETRY_THROTTLE_H
#define GRPC_SRC_CORE_CLIENT_CHANNEL_RETRY_THROTTLE_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <atomic>
#include <map>
#include <string>

#include "absl/base/thread_annotations.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"

namespace grpc_core {
namespace internal {

class ServerRetryThrottleMap;

/// Tracks retry throttling data for an individual server name.
class ServerRetryThrottleData final
    : public RefCounted<ServerRetryThrottleData> {
 public:
  ServerRetryThrottleData(uintptr_t max_milli_tokens,
                          uintptr_t milli_token_ratio, uintptr_t milli_tokens);
  ~ServerRetryThrottleData() override;

  /// Records a failure.  Returns true if it's okay to send a retry.
  bool RecordFailure();

  /// Records a success.
  void RecordSuccess();

  uintptr_t max_milli_tokens() const { return max_milli_tokens_; }
  uintptr_t milli_token_ratio() const { return milli_token_ratio_; }
  intptr_t milli_tokens() const {
    return milli_tokens_.load(std::memory_order_relaxed);
  }

 private:
  friend ServerRetryThrottleMap;

  void SetReplacement(RefCountedPtr<ServerRetryThrottleData> replacement);

  void GetReplacementThrottleDataIfNeeded(
      ServerRetryThrottleData** throttle_data);

  const uintptr_t max_milli_tokens_;
  const uintptr_t milli_token_ratio_;
  std::atomic<intptr_t> milli_tokens_;
  // A pointer to the replacement for this ServerRetryThrottleData entry.
  // If non-nullptr, then this entry is stale and must not be used.
  // We hold a reference to the replacement.
  std::atomic<ServerRetryThrottleData*> replacement_{nullptr};
};

/// Global map of server name to retry throttle data.
class ServerRetryThrottleMap final {
 public:
  static ServerRetryThrottleMap* Get();

  /// Returns the failure data for \a server_name, creating a new entry if
  /// needed.
  RefCountedPtr<ServerRetryThrottleData> GetDataForServer(
      const std::string& server_name, uintptr_t max_milli_tokens,
      uintptr_t milli_token_ratio);

 private:
  using StringToDataMap =
      std::map<std::string, RefCountedPtr<ServerRetryThrottleData>>;

  Mutex mu_;
  StringToDataMap map_ ABSL_GUARDED_BY(mu_);
};

}  // namespace internal
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CLIENT_CHANNEL_RETRY_THROTTLE_H
