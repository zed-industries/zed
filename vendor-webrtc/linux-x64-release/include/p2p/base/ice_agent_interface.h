/*
 *  Copyright 2022 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_ICE_AGENT_INTERFACE_H_
#define P2P_BASE_ICE_AGENT_INTERFACE_H_

#include <cstdint>

#include "api/array_view.h"
#include "p2p/base/connection.h"
#include "p2p/base/ice_switch_reason.h"
#include "p2p/base/transport_description.h"

namespace webrtc {

// IceAgentInterface provides methods that allow an ICE controller to manipulate
// the connections available to a transport, and used by the transport to
// transfer data.
class IceAgentInterface {
 public:
  virtual ~IceAgentInterface() = default;

  // Get the time when the last ping was sent.
  // This is only needed in some scenarios if the agent decides to ping on its
  // own, eg. in some switchover scenarios. Otherwise the ICE controller could
  // keep this state on its own.
  // TODO(bugs.webrtc.org/14367): route extra pings through the ICE controller.
  virtual int64_t GetLastPingSentMs() const = 0;

  // Get the ICE role of this ICE agent.
  virtual IceRole GetIceRole() const = 0;

  // Called when a pingable connection first becomes available.
  virtual void OnStartedPinging() = 0;

  // Update the state of all available connections.
  virtual void UpdateConnectionStates() = 0;

  // Update the internal state of the ICE agent. An ICE controller should call
  // this at the end of a sequence of actions to combine several mutations into
  // a single state refresh.
  // TODO(bugs.webrtc.org/14431): ICE agent state updates should be internal to
  // the agent. If batching is necessary, use a more appropriate interface.
  virtual void UpdateState() = 0;

  // Reset the given connections to a state of newly connected connections.
  // - STATE_WRITE_INIT
  // - receving = false
  // - throw away all pending request
  // - reset RttEstimate
  //
  // Keep the following unchanged:
  // - connected
  // - remote_candidate
  // - statistics
  //
  // SignalStateChange will not be triggered.
  virtual void ForgetLearnedStateForConnections(
      ArrayView<const Connection* const> connections) = 0;

  // Send a STUN ping request for the given connection.
  virtual void SendPingRequest(const Connection* connection) = 0;

  // Switch the transport to use the given connection.
  virtual void SwitchSelectedConnection(const Connection* new_connection,
                                        IceSwitchReason reason) = 0;

  // Prune away the given connections. Returns true if pruning is permitted and
  // successfully performed.
  virtual bool PruneConnections(
      ArrayView<const Connection* const> connections) = 0;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::IceAgentInterface;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_ICE_AGENT_INTERFACE_H_
