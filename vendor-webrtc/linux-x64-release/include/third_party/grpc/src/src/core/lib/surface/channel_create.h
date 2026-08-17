//
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
//

#ifndef GRPC_SRC_CORE_LIB_SURFACE_CHANNEL_CREATE_H
#define GRPC_SRC_CORE_LIB_SURFACE_CHANNEL_CREATE_H

#include <grpc/support/port_platform.h>

#include <string>

#include "absl/status/statusor.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/surface/channel.h"
#include "src/core/lib/surface/channel_stack_type.h"

#define GRPC_ARG_USE_V3_STACK "grpc.internal.use_v3_stack"

namespace grpc_core {

class Transport;

// Creates a client channel.
absl::StatusOr<RefCountedPtr<Channel>> ChannelCreate(
    std::string target, ChannelArgs args,
    grpc_channel_stack_type channel_stack_type, Transport* optional_transport);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SURFACE_CHANNEL_CREATE_H
