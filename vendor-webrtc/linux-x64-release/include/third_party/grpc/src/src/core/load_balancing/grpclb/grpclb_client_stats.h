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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_GRPCLB_CLIENT_STATS_H
#define GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_GRPCLB_CLIENT_STATS_H

#include <grpc/support/atm.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <memory>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/container/inlined_vector.h"
#include "src/core/util/memory.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/sync.h"

namespace grpc_core {

class GrpcLbClientStats final : public RefCounted<GrpcLbClientStats> {
 public:
  struct DropTokenCount {
    UniquePtr<char> token;
    int64_t count;

    DropTokenCount(UniquePtr<char> token, int64_t count)
        : token(std::move(token)), count(count) {}
  };

  typedef absl::InlinedVector<DropTokenCount, 10> DroppedCallCounts;

  void AddCallStarted();
  void AddCallFinished(bool finished_with_client_failed_to_send,
                       bool finished_known_received);

  void AddCallDropped(const char* token);

  void Get(int64_t* num_calls_started, int64_t* num_calls_finished,
           int64_t* num_calls_finished_with_client_failed_to_send,
           int64_t* num_calls_finished_known_received,
           std::unique_ptr<DroppedCallCounts>* drop_token_counts);

  // A destruction function to use as the user_data key when attaching
  // client stats to a grpc_mdelem.
  static void Destroy(void* arg) {
    static_cast<GrpcLbClientStats*>(arg)->Unref();
  }

 private:
  gpr_atm num_calls_started_ = 0;
  gpr_atm num_calls_finished_ = 0;
  gpr_atm num_calls_finished_with_client_failed_to_send_ = 0;
  gpr_atm num_calls_finished_known_received_ = 0;
  Mutex drop_count_mu_;  // Guards drop_token_counts_.
  std::unique_ptr<DroppedCallCounts> drop_token_counts_
      ABSL_GUARDED_BY(drop_count_mu_);
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_GRPCLB_CLIENT_STATS_H
