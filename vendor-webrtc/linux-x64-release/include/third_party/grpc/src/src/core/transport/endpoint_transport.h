// Copyright 2025 gRPC authors.
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

#ifndef GRPC_SRC_CORE_TRANSPORT_ENDPOINT_TRANSPORT_H
#define GRPC_SRC_CORE_TRANSPORT_ENDPOINT_TRANSPORT_H

#include <grpc/impl/grpc_types.h>

#include <memory>
#include <string>

#include "src/core/lib/channel/channel_args.h"

namespace grpc_core {

class Server;

// Comma separated list of transport protocols in order of most preferred to
// least preferred.
#define GRPC_ARG_PREFERRED_TRANSPORT_PROTOCOLS \
  "grpc.preferred_transport_protocols"

// EndpointTransport is a transport that operates over EventEngine::Endpoint
// objects.
// This interface is an iterim thing whilst call-v3 is finished and we flesh
// out next protocol negotiation in all transport stacks. At that point this
// interface will change so that we can run many kinds of EndpointTransport
// on one listener, and negotiate protocol with one connector.
class EndpointTransport {
 public:
  virtual absl::StatusOr<grpc_channel*> ChannelCreate(
      std::string target, const ChannelArgs& args) = 0;
  virtual absl::StatusOr<int> AddPort(Server* server, std::string addr,
                                      const ChannelArgs& args) = 0;
  virtual ~EndpointTransport() = default;
};

class EndpointTransportRegistry {
 private:
  using TransportMap =
      std::map<std::string, std::unique_ptr<EndpointTransport>>;

 public:
  class Builder {
   public:
    void RegisterTransport(std::string name,
                           std::unique_ptr<EndpointTransport> transport) {
      if (transports_.find(name) != transports_.end()) {
        LOG(FATAL) << "Duplicate endpoint transport registration: " << name;
      }
      transports_[name] = std::move(transport);
    }

    EndpointTransportRegistry Build() {
      return EndpointTransportRegistry(std::move(transports_));
    }

   private:
    TransportMap transports_;
  };

  EndpointTransport* GetTransport(absl::string_view name) const {
    auto it = transports_.find(std::string(name));
    if (it == transports_.end()) {
      return nullptr;
    }
    return it->second.get();
  }

 private:
  explicit EndpointTransportRegistry(TransportMap transports)
      : transports_(std::move(transports)) {}

  TransportMap transports_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_TRANSPORT_ENDPOINT_TRANSPORT_H
