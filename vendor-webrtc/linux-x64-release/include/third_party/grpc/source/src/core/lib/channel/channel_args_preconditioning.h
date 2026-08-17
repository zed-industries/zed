// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_CHANNEL_CHANNEL_ARGS_PRECONDITIONING_H
#define GRPC_SRC_CORE_LIB_CHANNEL_CHANNEL_ARGS_PRECONDITIONING_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include <functional>
#include <vector>

#include "src/core/lib/channel/channel_args.h"

namespace grpc_core {

// Registry of mutators for channel args.
// Surface APIs should call into this with channel args received from outside
// of gRPC, in order to prepare those channel args for the expectations of the
// gRPC internals.
class ChannelArgsPreconditioning {
 public:
  // Take channel args and mutate them.
  // Does not take ownership of the channel args passed in.
  // Returns a new channel args object that is owned by the caller.
  using Stage = std::function<ChannelArgs(ChannelArgs)>;

  class Builder {
   public:
    // Register a new channel args preconditioner.
    void RegisterStage(Stage stage);
    // Build out the preconditioners.
    ChannelArgsPreconditioning Build();

   private:
    std::vector<Stage> stages_;
  };

  // Take channel args and precondition them.
  // Does not take ownership of the channel args passed in.
  // Returns a new channel args object that is owned by the caller.
  ChannelArgs PreconditionChannelArgs(const grpc_channel_args* args) const;

 private:
  std::vector<Stage> stages_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_CHANNEL_CHANNEL_ARGS_PRECONDITIONING_H
