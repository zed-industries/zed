//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CLIENT_CHANNEL_CONNECTOR_H
#define GRPC_SRC_CORE_CLIENT_CHANNEL_CONNECTOR_H

#include <grpc/support/port_platform.h>

#include "src/core/channelz/channelz.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/lib/iomgr/resolved_address.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/time.h"

namespace grpc_core {

// Interface for connection-establishment functionality.
// Each transport that supports client channels (e.g., not inproc) must
// supply an implementation of this.
class SubchannelConnector : public InternallyRefCounted<SubchannelConnector> {
 public:
  struct Args {
    // Address to connect to.
    grpc_resolved_address* address;
    // Set of pollsets interested in this connection.
    grpc_pollset_set* interested_parties;
    // Deadline for connection.
    Timestamp deadline;
    // Channel args to be passed to handshakers and transport.
    ChannelArgs channel_args;
  };

  struct Result {
    // The connected transport.
    Transport* transport = nullptr;
    // Channel args to be passed to filters.
    ChannelArgs channel_args;
    // Channelz socket node of the connected transport, if any.
    RefCountedPtr<channelz::SocketNode> socket_node;

    void Reset() {
      if (transport != nullptr) {
        transport->Orphan();
        transport = nullptr;
      }
      channel_args = ChannelArgs();
      socket_node.reset();
    }
  };

  // Attempts to connect.
  // When complete, populates *result and invokes notify.
  // Only one connection attempt may be in progress at any one time.
  virtual void Connect(const Args& args, Result* result,
                       grpc_closure* notify) = 0;

  // Cancels any in-flight connection attempt and shuts down the
  // connector.
  virtual void Shutdown(grpc_error_handle error) = 0;

  void Orphan() override {
    Shutdown(GRPC_ERROR_CREATE("Subchannel disconnected"));
    Unref();
  }
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CLIENT_CHANNEL_CONNECTOR_H
