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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_WEIGHTED_TARGET_WEIGHTED_TARGET_H
#define GRPC_SRC_CORE_LOAD_BALANCING_WEIGHTED_TARGET_WEIGHTED_TARGET_H

#include <grpc/support/port_platform.h>

#include "src/core/resolver/endpoint_addresses.h"

// Channel arg key indicating the weighted_target child name.
#define GRPC_ARG_LB_WEIGHTED_TARGET_CHILD \
  GRPC_ARG_NO_SUBCHANNEL_PREFIX "lb_weighted_target_child"

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_WEIGHTED_TARGET_WEIGHTED_TARGET_H
