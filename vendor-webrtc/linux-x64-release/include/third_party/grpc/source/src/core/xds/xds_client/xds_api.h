//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_API_H
#define GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_API_H

#include "absl/strings/string_view.h"
#include "envoy/config/core/v3/base.upb.h"
#include "src/core/xds/xds_client/xds_bootstrap.h"
#include "upb/mem/arena.h"

namespace grpc_core {

void PopulateXdsNode(const XdsBootstrap::Node* node,
                     absl::string_view user_agent_name,
                     absl::string_view user_agent_version,
                     envoy_config_core_v3_Node* node_msg, upb_Arena* arena);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_API_H
