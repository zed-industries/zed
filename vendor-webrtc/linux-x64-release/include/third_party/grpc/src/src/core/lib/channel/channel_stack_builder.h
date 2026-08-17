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

#ifndef GRPC_SRC_CORE_LIB_CHANNEL_CHANNEL_STACK_BUILDER_H
#define GRPC_SRC_CORE_LIB_CHANNEL_CHANNEL_STACK_BUILDER_H

#include <grpc/support/port_platform.h>

#include <string>
#include <vector>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/surface/channel_stack_type.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

// Build a channel stack.
// Allows interested parties to add filters to the stack, and to query an
// in-progress build.
// Carries some useful context for the channel stack, such as a target string
// and a transport.
class ChannelStackBuilder {
 public:
  // Initialize with a name.
  // channel_args *must be* preconditioned already.
  ChannelStackBuilder(const char* name, grpc_channel_stack_type type,
                      const ChannelArgs& channel_args);

  const char* name() const { return name_; }

  // Set the target string.
  ChannelStackBuilder& SetTarget(const char* target);

  // Query the target.
  absl::string_view target() const { return target_; }

  // Query the channel args.
  const ChannelArgs& channel_args() const { return args_; }

  // Mutable vector of proposed stack entries.
  std::vector<const grpc_channel_filter*>* mutable_stack() { return &stack_; }

  // Immutable vector of proposed stack entries.
  const std::vector<const grpc_channel_filter*>& stack() const {
    return stack_;
  }

  // The type of channel stack being built.
  grpc_channel_stack_type channel_stack_type() const { return type_; }

  // TODO(ctiller): re-evaluate the need for AppendFilter, PrependFilter.
  // Their usefulness is largely zero now that we have ordering constraints in
  // channel init.

  // Helper to add a filter to the front of the stack.
  void PrependFilter(const grpc_channel_filter* filter);

  // Helper to add a filter to the end of the stack.
  void AppendFilter(const grpc_channel_filter* filter);

  // Build the channel stack.
  // After success, *result holds the new channel stack,
  // prefix_bytes are allocated before the channel stack,
  // destroy is as per grpc_channel_stack_init
  // On failure, *result is nullptr.
  virtual absl::StatusOr<RefCountedPtr<grpc_channel_stack>> Build() = 0;

 protected:
  ~ChannelStackBuilder() = default;

 private:
  static std::string unknown_target() { return "unknown"; }

  // The name of the stack
  const char* const name_;
  // The type of stack being built
  const grpc_channel_stack_type type_;
  // The target
  std::string target_{unknown_target()};
  // Channel args
  ChannelArgs args_;
  // The in-progress stack
  std::vector<const grpc_channel_filter*> stack_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_CHANNEL_CHANNEL_STACK_BUILDER_H
