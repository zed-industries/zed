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

#ifndef GRPC_SRC_CORE_CLIENT_CHANNEL_LOAD_BALANCED_CALL_DESTINATION_H
#define GRPC_SRC_CORE_CLIENT_CHANNEL_LOAD_BALANCED_CALL_DESTINATION_H

#include "absl/functional/any_invocable.h"
#include "src/core/call/call_destination.h"
#include "src/core/client_channel/client_channel.h"
#include "src/core/lib/promise/context.h"
#include "src/core/load_balancing/lb_policy.h"

namespace grpc_core {

// Context type for LB on_commit callback.
// TODO(ctiller): make this a struct, so we don't accidentally alias context
// types
using LbOnCommit = absl::AnyInvocable<void()>;
template <>
struct ContextType<LbOnCommit> {};

class LoadBalancedCallDestination final : public UnstartedCallDestination {
 public:
  explicit LoadBalancedCallDestination(ClientChannel::PickerObservable picker)
      : picker_(std::move(picker)) {}

  void Orphaned() override {}

  void StartCall(UnstartedCallHandler unstarted_handler) override;

 private:
  ClientChannel::PickerObservable picker_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CLIENT_CHANNEL_LOAD_BALANCED_CALL_DESTINATION_H
