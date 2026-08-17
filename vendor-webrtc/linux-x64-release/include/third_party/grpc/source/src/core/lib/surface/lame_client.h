//
//
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
//
//

#ifndef GRPC_SRC_CORE_LIB_SURFACE_LAME_CLIENT_H
#define GRPC_SRC_CORE_LIB_SURFACE_LAME_CLIENT_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include <memory>

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/promise_based_filter.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/transport/connectivity_state.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/sync.h"

#define GRPC_ARG_LAME_FILTER_ERROR "grpc.lame_filter_error"

namespace grpc_core {

// This filter becomes the entire channel stack for a channel that fails to be
// created. Every call returns failure.
class LameClientFilter : public ChannelFilter {
 public:
  static const grpc_channel_filter kFilter;

  static absl::string_view TypeName() { return "lame-client"; }

  explicit LameClientFilter(absl::Status error);

  static absl::StatusOr<std::unique_ptr<LameClientFilter>> Create(
      const ChannelArgs& args, ChannelFilter::Args filter_args);
  ArenaPromise<ServerMetadataHandle> MakeCallPromise(
      CallArgs call_args, NextPromiseFactory next_promise_factory) override;
  bool StartTransportOp(grpc_transport_op*) override;
  bool GetChannelInfo(const grpc_channel_info*) override;

 private:
  absl::Status error_;
  Mutex mu_;
  ConnectivityStateTracker state_tracker_ ABSL_GUARDED_BY(mu_);
};

extern const grpc_arg_pointer_vtable kLameFilterErrorArgVtable;

grpc_arg MakeLameClientErrorArg(grpc_error_handle* error);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SURFACE_LAME_CLIENT_H
