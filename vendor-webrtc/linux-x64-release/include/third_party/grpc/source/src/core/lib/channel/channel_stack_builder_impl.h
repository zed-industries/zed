// Copyright 2016 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_CHANNEL_CHANNEL_STACK_BUILDER_IMPL_H
#define GRPC_SRC_CORE_LIB_CHANNEL_CHANNEL_STACK_BUILDER_IMPL_H

#include <grpc/support/port_platform.h>

#include "absl/status/statusor.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/channel_stack_builder.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

class Blackboard;

// Build a channel stack.
// Allows interested parties to add filters to the stack, and to query an
// in-progress build.
// Carries some useful context for the channel stack, such as a target string
// and a transport.
class ChannelStackBuilderImpl final : public ChannelStackBuilder {
 public:
  using ChannelStackBuilder::ChannelStackBuilder;

  void SetBlackboards(const Blackboard* old_blackboard,
                      Blackboard* new_blackboard) {
    old_blackboard_ = old_blackboard;
    new_blackboard_ = new_blackboard;
  }

  // Build the channel stack.
  // After success, *result holds the new channel stack,
  // prefix_bytes are allocated before the channel stack,
  // initial_refs, destroy, destroy_arg are as per grpc_channel_stack_init
  // On failure, *result is nullptr.
  absl::StatusOr<RefCountedPtr<grpc_channel_stack>> Build() override;

 private:
  const Blackboard* old_blackboard_ = nullptr;
  Blackboard* new_blackboard_ = nullptr;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_CHANNEL_CHANNEL_STACK_BUILDER_IMPL_H
