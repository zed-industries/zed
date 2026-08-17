//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_SRC_CORE_HANDSHAKER_HANDSHAKER_REGISTRY_H
#define GRPC_SRC_CORE_HANDSHAKER_HANDSHAKER_REGISTRY_H

#include <grpc/support/port_platform.h>

#include <memory>
#include <vector>

#include "src/core/handshaker/handshaker_factory.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"

namespace grpc_core {

typedef enum {
  HANDSHAKER_CLIENT = 0,
  HANDSHAKER_SERVER,
  NUM_HANDSHAKER_TYPES,  // Must be last.
} HandshakerType;

class HandshakerRegistry {
 public:
  class Builder {
   public:
    /// Registers a new handshaker factory.  Takes ownership.
    /// The priority of the handshaker will be used to order the handshakers
    /// in the list.
    void RegisterHandshakerFactory(HandshakerType handshaker_type,
                                   std::unique_ptr<HandshakerFactory> factory);

    HandshakerRegistry Build();

   private:
    std::vector<std::unique_ptr<HandshakerFactory>>
        factories_[NUM_HANDSHAKER_TYPES];
  };

  void AddHandshakers(HandshakerType handshaker_type, const ChannelArgs& args,
                      grpc_pollset_set* interested_parties,
                      HandshakeManager* handshake_mgr) const;

 private:
  HandshakerRegistry() = default;

  std::vector<std::unique_ptr<HandshakerFactory>>
      factories_[NUM_HANDSHAKER_TYPES];
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_HANDSHAKER_HANDSHAKER_REGISTRY_H
