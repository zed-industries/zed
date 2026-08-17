//
//
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
//
//

#ifndef GRPC_SRC_CPP_EXT_CSM_CSM_OBSERVABILITY_H
#define GRPC_SRC_CPP_EXT_CSM_CSM_OBSERVABILITY_H

#include <grpc/support/port_platform.h>

#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_args.h"

namespace grpc {
namespace internal {

// EXPOSED FOR TESTING PURPOSES ONLY
// Returns true if the channel is a CSM channel.
bool CsmChannelTargetSelector(absl::string_view target);

// EXPOSED FOR TESTING PURPOSES ONLY
// Returns true if the server is a CSM server.
bool CsmServerSelector(const grpc_core::ChannelArgs& args);

}  // namespace internal
}  // namespace grpc

#endif  // GRPC_SRC_CPP_EXT_CSM_CSM_OBSERVABILITY_H
