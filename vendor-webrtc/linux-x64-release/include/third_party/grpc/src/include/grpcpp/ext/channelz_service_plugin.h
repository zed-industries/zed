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

#ifndef GRPCPP_EXT_CHANNELZ_SERVICE_PLUGIN_H
#define GRPCPP_EXT_CHANNELZ_SERVICE_PLUGIN_H

#include <grpc/support/port_platform.h>
#include <grpcpp/impl/server_builder_plugin.h>
#include <grpcpp/impl/server_initializer.h>
#include <grpcpp/support/config.h>

namespace grpc {
namespace channelz {
namespace experimental {

/// Add channelz server plugin to \a ServerBuilder. This function should
/// be called at static initialization time. This service is experimental
/// for now. Track progress in https://github.com/grpc/grpc/issues/15988.
void InitChannelzService();

}  // namespace experimental
}  // namespace channelz
}  // namespace grpc

#endif  // GRPCPP_EXT_CHANNELZ_SERVICE_PLUGIN_H
