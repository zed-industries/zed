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

#ifndef GRPC_SRC_CORE_CLIENT_CHANNEL_SUBCHANNEL_INTERFACE_INTERNAL_H
#define GRPC_SRC_CORE_CLIENT_CHANNEL_SUBCHANNEL_INTERFACE_INTERNAL_H

#include <grpc/support/port_platform.h>

#include "src/core/client_channel/subchannel.h"
#include "src/core/load_balancing/subchannel_interface.h"
#include "src/core/util/unique_type_name.h"

namespace grpc_core {

// Internal interface for watching data of a particular type for this
// subchannel.
class InternalSubchannelDataWatcherInterface
    : public SubchannelInterface::DataWatcherInterface {
 public:
  virtual UniqueTypeName type() const = 0;

  // Tells the watcher which subchannel to register itself with.
  virtual void SetSubchannel(Subchannel* subchannel) = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CLIENT_CHANNEL_SUBCHANNEL_INTERFACE_INTERNAL_H
