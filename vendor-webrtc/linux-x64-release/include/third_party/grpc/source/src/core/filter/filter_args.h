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

#ifndef GRPC_SRC_CORE_FILTER_FILTER_ARGS_H
#define GRPC_SRC_CORE_FILTER_FILTER_ARGS_H

#include "src/core/filter/blackboard.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/util/match.h"

namespace grpc_core {

// Filter arguments that are independent of channel args.
// Here-in should be things that depend on the filters location in the stack, or
// things that are ephemeral and disjoint from overall channel args.
class FilterArgs {
 public:
  FilterArgs() : FilterArgs(nullptr, nullptr, nullptr) {}
  FilterArgs(grpc_channel_stack* channel_stack,
             grpc_channel_element* channel_element,
             size_t (*channel_stack_filter_instance_number)(
                 grpc_channel_stack*, grpc_channel_element*),
             const Blackboard* old_blackboard = nullptr,
             Blackboard* new_blackboard = nullptr)
      : impl_(ChannelStackBased{channel_stack, channel_element,
                                channel_stack_filter_instance_number}),
        old_blackboard_(old_blackboard),
        new_blackboard_(new_blackboard) {}
  // While we're moving to call-v3 we need to have access to
  // grpc_channel_stack & friends here. That means that we can't rely on this
  // type signature from interception_chain.h, which means that we need a way
  // of constructing this object without naming it ===> implicit construction.
  // TODO(ctiller): remove this once we're fully on call-v3
  // NOLINTNEXTLINE(google-explicit-constructor)
  FilterArgs(size_t instance_id, const Blackboard* old_blackboard = nullptr,
             Blackboard* new_blackboard = nullptr)
      : impl_(V3Based{instance_id}),
        old_blackboard_(old_blackboard),
        new_blackboard_(new_blackboard) {}

  ABSL_DEPRECATED("Direct access to channel stack is deprecated")
  grpc_channel_stack* channel_stack() const {
    return std::get<ChannelStackBased>(impl_).channel_stack;
  }

  // Get the instance id of this filter.
  // This id is unique amongst all filters /of the same type/ and densely
  // packed (starting at 0) for a given channel stack instantiation.
  // eg. for a stack with filter types A B C A B D A the instance ids would be
  // 0 0 0 1 1 0 2.
  // This is useful for filters that need to store per-instance data in a
  // parallel data structure.
  size_t instance_id() const {
    return Match(
        impl_,
        [](const ChannelStackBased& cs) {
          return cs.channel_stack_filter_instance_number(cs.channel_stack,
                                                         cs.channel_element);
        },
        [](const V3Based& v3) { return v3.instance_id; });
  }

  // If a filter state object of type T exists for key from a previous
  // filter stack, retains it for the new filter stack we're constructing.
  // Otherwise, invokes create_func() to create a new filter state
  // object for the new filter stack.  Returns the new filter state object.
  template <typename T>
  RefCountedPtr<T> GetOrCreateState(
      const std::string& key,
      absl::FunctionRef<RefCountedPtr<T>()> create_func) {
    RefCountedPtr<T> state;
    if (old_blackboard_ != nullptr) state = old_blackboard_->Get<T>(key);
    if (state == nullptr) state = create_func();
    if (new_blackboard_ != nullptr) new_blackboard_->Set(key, state);
    return state;
  }

 private:
  friend class ChannelFilter;

  struct ChannelStackBased {
    grpc_channel_stack* channel_stack;
    grpc_channel_element* channel_element;
    size_t (*channel_stack_filter_instance_number)(grpc_channel_stack*,
                                                   grpc_channel_element*);
  };

  struct V3Based {
    size_t instance_id;
  };

  using Impl = std::variant<ChannelStackBased, V3Based>;
  Impl impl_;

  const Blackboard* old_blackboard_ = nullptr;
  Blackboard* new_blackboard_ = nullptr;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_FILTER_FILTER_ARGS_H
