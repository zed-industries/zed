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

#ifndef GRPC_SRC_CORE_RESOLVER_SERVER_ADDRESS_H
#define GRPC_SRC_CORE_RESOLVER_SERVER_ADDRESS_H

#include <grpc/support/port_platform.h>

#include "src/core/resolver/endpoint_addresses.h"

namespace grpc_core {

// For backward compatibility only.
// TODO(roth): Remove this file when all callers have been updated.
using ServerAddress = EndpointAddresses;
using ServerAddressList = EndpointAddressesList;

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_RESOLVER_SERVER_ADDRESS_H
