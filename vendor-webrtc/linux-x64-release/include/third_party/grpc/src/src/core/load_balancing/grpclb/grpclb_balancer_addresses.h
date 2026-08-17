//
// Copyright 2019 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_GRPCLB_BALANCER_ADDRESSES_H
#define GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_GRPCLB_BALANCER_ADDRESSES_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include "src/core/lib/channel/channel_args.h"
#include "src/core/resolver/endpoint_addresses.h"

namespace grpc_core {

grpc_arg CreateGrpclbBalancerAddressesArg(
    const EndpointAddressesList* endpoint_list);
GRPC_MUST_USE_RESULT
ChannelArgs SetGrpcLbBalancerAddresses(const ChannelArgs& args,
                                       EndpointAddressesList endpoint_list);
const EndpointAddressesList* FindGrpclbBalancerAddressesInChannelArgs(
    const ChannelArgs& args);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_GRPCLB_BALANCER_ADDRESSES_H
