//
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
//

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_GRPCLB_H
#define GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_GRPCLB_H

#include <grpc/support/port_platform.h>

/// Channel arg indicating if a target corresponding to the address is grpclb
/// loadbalancer. The type of this arg is an integer and the value is treated as
/// a bool.
#define GRPC_ARG_ADDRESS_IS_GRPCLB_LOAD_BALANCER \
  "grpc.address_is_grpclb_load_balancer"
/// Channel arg indicating if a target corresponding to the address is a backend
/// received from a balancer. The type of this arg is an integer and the value
/// is treated as a bool.
#define GRPC_ARG_ADDRESS_IS_BACKEND_FROM_GRPCLB_LOAD_BALANCER \
  "grpc.address_is_backend_from_grpclb_load_balancer"

// For use in tests.
#define GRPC_ARG_GRPCLB_SUBCHANNEL_CACHE_INTERVAL_MS \
  "grpc.internal.grpclb_subchannel_cache_interval_ms"

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_GRPCLB_H
