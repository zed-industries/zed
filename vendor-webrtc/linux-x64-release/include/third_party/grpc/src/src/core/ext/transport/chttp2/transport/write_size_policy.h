// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_WRITE_SIZE_POLICY_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_WRITE_SIZE_POLICY_H

#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include "src/core/util/time.h"

namespace grpc_core {

class Chttp2WriteSizePolicy {
 public:
  // Smallest possible WriteTargetSize
  static constexpr size_t MinTarget() { return 32 * 1024; }
  // Largest possible WriteTargetSize
  static constexpr size_t MaxTarget() { return 16 * 1024 * 1024; }
  // How long should a write take to be considered "fast"
  static constexpr Duration FastWrite() { return Duration::Milliseconds(100); }
  // How long should a write take to be considered "slow"
  static constexpr Duration SlowWrite() { return Duration::Seconds(1); }
  // If a read is slow, what target time should we use to try and adjust back
  // to?
  static constexpr Duration TargetWriteTime() {
    return Duration::Milliseconds(300);
  }

  // What size should be targeted for the next write.
  size_t WriteTargetSize();
  // Notify the policy that a write of some size has begun.
  // EndWrite must be called when the write completes.
  void BeginWrite(size_t size);
  // Notify the policy that a write of some size has ended.
  void EndWrite(bool success);

 private:
  size_t current_target_ = 128 * 1024;
  Timestamp experiment_start_time_ = Timestamp::InfFuture();
  // State varies from -2...2
  // Every time we do a write faster than kFastWrite, we decrement
  // Every time we do a write slower than kSlowWrite, we increment
  // If we hit -2, we increase the target size and reset state to 0
  // If we hit 2, we decrease the target size and reset state to 0
  // In this way, we need two consecutive fast/slow operations to adjust,
  // denoising the signal significantly
  int8_t state_ = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_WRITE_SIZE_POLICY_H
