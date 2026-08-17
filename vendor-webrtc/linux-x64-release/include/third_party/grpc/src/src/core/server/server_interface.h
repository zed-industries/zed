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

#ifndef GRPC_SRC_CORE_SERVER_SERVER_INTERFACE_H
#define GRPC_SRC_CORE_SERVER_SERVER_INTERFACE_H

#include <grpc/impl/compression_types.h>
#include <grpc/support/port_platform.h>

#include "src/core/channelz/channelz.h"
#include "src/core/lib/channel/channel_args.h"

namespace grpc_core {

class ServerCallTracerFactory;

// This class is a hack to avoid a circular dependency that would be
// caused by the code in call.cc depending directly on the server code.
// TODO(roth): After the call v3 migration, find a cleaner way to do this.
class ServerInterface {
 public:
  virtual ~ServerInterface() = default;

  virtual const ChannelArgs& channel_args() const = 0;
  virtual channelz::ServerNode* channelz_node() const = 0;
  virtual ServerCallTracerFactory* server_call_tracer_factory() const = 0;
  virtual grpc_compression_options compression_options() const = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_SERVER_SERVER_INTERFACE_H
