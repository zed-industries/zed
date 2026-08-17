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

#ifndef GRPC_SRC_CPP_SERVER_CHANNELZ_CHANNELZ_SERVICE_H
#define GRPC_SRC_CPP_SERVER_CHANNELZ_CHANNELZ_SERVICE_H

#include <grpc/support/port_platform.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/support/status.h>

#include "src/proto/grpc/channelz/channelz.grpc.pb.h"

namespace grpc {

class ChannelzService final : public channelz::v1::Channelz::Service {
 private:
  // implementation of GetTopChannels rpc
  Status GetTopChannels(
      ServerContext* unused, const channelz::v1::GetTopChannelsRequest* request,
      channelz::v1::GetTopChannelsResponse* response) override;
  // implementation of GetServers rpc
  Status GetServers(ServerContext* unused,
                    const channelz::v1::GetServersRequest* request,
                    channelz::v1::GetServersResponse* response) override;
  // implementation of GetServer rpc
  Status GetServer(ServerContext* unused,
                   const channelz::v1::GetServerRequest* request,
                   channelz::v1::GetServerResponse* response) override;
  // implementation of GetServerSockets rpc
  Status GetServerSockets(
      ServerContext* unused,
      const channelz::v1::GetServerSocketsRequest* request,
      channelz::v1::GetServerSocketsResponse* response) override;
  // implementation of GetChannel rpc
  Status GetChannel(ServerContext* unused,
                    const channelz::v1::GetChannelRequest* request,
                    channelz::v1::GetChannelResponse* response) override;
  // implementation of GetSubchannel rpc
  Status GetSubchannel(ServerContext* unused,
                       const channelz::v1::GetSubchannelRequest* request,
                       channelz::v1::GetSubchannelResponse* response) override;
  // implementation of GetSocket rpc
  Status GetSocket(ServerContext* unused,
                   const channelz::v1::GetSocketRequest* request,
                   channelz::v1::GetSocketResponse* response) override;
};

}  // namespace grpc

#endif  // GRPC_SRC_CPP_SERVER_CHANNELZ_CHANNELZ_SERVICE_H
