//
// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_HEALTH_CHECK_CLIENT_H
#define GRPC_SRC_CORE_LOAD_BALANCING_HEALTH_CHECK_CLIENT_H

#include <grpc/support/port_platform.h>

#include <memory>

#include "src/core/lib/channel/channel_args.h"
#include "src/core/load_balancing/subchannel_interface.h"
#include "src/core/util/work_serializer.h"

namespace grpc_core {

// Interface for LB policies to access health check data from a subchannel.
// The data is reported from via a Health.Watch stream established on the
// subchannel whenever an LB policy registers a watcher.
//
// To use this, an LB policy will implement its own subclass of
// SubchannelInterface::ConnectivityStateWatcherInterface, which will
// receive connectivity state updates with health check status taken
// into account.  It will then register that watcher with the subchannel
// like this:
//   subchannel->AddDataWatcher(
//       MakeHealthCheckWatcher(
//           work_serializer(), channel_args,
//           std::make_unique<MyConnectivityStateWatcherSubclass>(...)));

std::unique_ptr<SubchannelInterface::DataWatcherInterface>
MakeHealthCheckWatcher(
    std::shared_ptr<WorkSerializer> work_serializer, const ChannelArgs& args,
    std::unique_ptr<SubchannelInterface::ConnectivityStateWatcherInterface>
        watcher);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_HEALTH_CHECK_CLIENT_H
