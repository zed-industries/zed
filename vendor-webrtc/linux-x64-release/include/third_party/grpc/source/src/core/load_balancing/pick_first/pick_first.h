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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_PICK_FIRST_PICK_FIRST_H
#define GRPC_SRC_CORE_LOAD_BALANCING_PICK_FIRST_PICK_FIRST_H

#include <grpc/support/port_platform.h>

#include "src/core/resolver/endpoint_addresses.h"

// Internal channel arg to enable health checking in pick_first.
// Intended to be used by petiole policies (e.g., round_robin) that
// delegate to pick_first.
#define GRPC_ARG_INTERNAL_PICK_FIRST_ENABLE_HEALTH_CHECKING \
  GRPC_ARG_NO_SUBCHANNEL_PREFIX "pick_first_enable_health_checking"

// Internal channel arg to tell pick_first to omit the prefix it normally
// adds to error status messages.  Intended to be used by petiole policies
// (e.g., round_robin) that want to add their own prefixes.
#define GRPC_ARG_INTERNAL_PICK_FIRST_OMIT_STATUS_MESSAGE_PREFIX \
  GRPC_ARG_NO_SUBCHANNEL_PREFIX "pick_first_omit_status_message_prefix"

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_PICK_FIRST_PICK_FIRST_H
