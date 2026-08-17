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

#ifndef GRPC_SRC_CORE_HANDSHAKER_HANDSHAKER_FACTORY_H
#define GRPC_SRC_CORE_HANDSHAKER_HANDSHAKER_FACTORY_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"

// A handshaker factory is used to create handshakers.

// TODO(ctiller): HandshakeManager is forward declared in this file. When
// EventEngine lands IO support we ought to be able to include
// handshake_manager.h here and eliminate the HandshakeManager dependency - we
// cannot right now because HandshakeManager names too many iomgr types.

namespace grpc_core {

class HandshakeManager;

class HandshakerFactory {
 public:
  // Enum representing the priority of the handshakers.
  // The order of the handshakers is decided by the priority.
  // For example kPreTCPConnect handshakers are called before kTCPConnect and so
  // on.
  enum class HandshakerPriority : int {
    // Handshakers that should be called before a TCP connect. Applicable mainly
    // for Client handshakers.
    kPreTCPConnectHandshakers,
    // Handshakers responsible for the actual TCP connect establishment.
    // Applicable mainly for Client handshakers.
    kTCPConnectHandshakers,
    // Handshakers responsible for the actual HTTP connect established.
    // Applicable mainly for Client handshakers.
    kHTTPConnectHandshakers,
    // Handshakers that should be called before security handshakes but after
    // connect establishment. Applicable mainly for Server handshakers
    // currently.
    kReadAheadSecurityHandshakers,
    // Handshakers that are responsible for post connect security handshakes.
    // Applicable for both Client and Server handshakers.
    kSecurityHandshakers,
    // TEMPORARY HACK -- DO NOT USE
    // Currently, handshakers that need to hijack the endpoint's fd and
    // exit early (which generally run at priority kSecurityHandshakers)
    // need to call grpc_tcp_destroy_and_release_fd(), which asserts
    // that the endpoint is an iomgr endpoint.  If another handshaker
    // has wrapped the endpoint before then, this assertion fails.  So
    // for now, we introduce a new priority here for handshakers that
    // need to wrap the endpoint, to make sure that they run after
    // handshakers that hijack the fd and exit early.
    // TODO(hork): As part of migrating to the EventEngine endpoint API,
    // remove this priority.  In the EE API, handshakers that want to
    // hijack the fd will do so via the query interface, so we can just
    // have any wrapper endpoints forward query interfaces to the wrapped
    // endpoint, so that it's not a problem if the endpoint is wrapped
    // before a handshaker needs to hijack the fd.
    kTemporaryHackDoNotUseEndpointWrappingHandshakers,
  };

  virtual void AddHandshakers(const ChannelArgs& args,
                              grpc_pollset_set* interested_parties,
                              HandshakeManager* handshake_mgr) = 0;
  // Return the priority associated with the handshaker.
  virtual HandshakerPriority Priority() = 0;
  virtual ~HandshakerFactory() = default;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_HANDSHAKER_HANDSHAKER_FACTORY_H
