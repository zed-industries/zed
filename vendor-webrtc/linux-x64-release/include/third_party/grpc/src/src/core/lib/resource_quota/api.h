// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_API_H
#define GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_API_H

#include <grpc/event_engine/endpoint_config.h>
#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include "src/core/config/core_configuration.h"
#include "src/core/lib/resource_quota/resource_quota.h"

typedef struct grpc_resource_quota grpc_resource_quota;

namespace grpc_core {

// TODO(ctiller): This is a hack. We need to do real accounting instead of
// hard coding.
constexpr size_t kResourceQuotaChannelSize = 34 * 1024;

// Retrieve the resource quota from the channel args.
// UB if not set.
ResourceQuotaRefPtr ResourceQuotaFromChannelArgs(const grpc_channel_args* args);

// Retrieve the resource quota from the EndpointConfig.
// Returns nullptr if not set.
ResourceQuotaRefPtr ResourceQuotaFromEndpointConfig(
    const grpc_event_engine::experimental::EndpointConfig& config);

void RegisterResourceQuota(CoreConfiguration::Builder* builder);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_API_H
