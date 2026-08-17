/*
 *  Copyright 2022 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_ACTIVE_ICE_CONTROLLER_INTERFACE_H_
#define P2P_BASE_ACTIVE_ICE_CONTROLLER_INTERFACE_H_


#include "p2p/base/connection.h"
#include "p2p/base/ice_switch_reason.h"
#include "p2p/base/ice_transport_internal.h"
#include "p2p/base/transport_description.h"

namespace webrtc {

// ActiveIceControllerInterface defines the methods for a module that actively
// manages the connection used by an ICE transport.
//
// An active ICE controller receives updates from the ICE transport when
//   - the connections state is mutated
//   - a new connection should be selected as a result of an external event (eg.
//     a different connection nominated by the remote peer)
//
// The active ICE controller takes the appropriate decisions and requests the
// ICE agent to perform the necessary actions through the IceAgentInterface.
class ActiveIceControllerInterface {
 public:
  virtual ~ActiveIceControllerInterface() = default;

  // Sets the current ICE configuration.
  virtual void SetIceConfig(const IceConfig& config) = 0;

  // Called when a new connection is added to the ICE transport.
  virtual void OnConnectionAdded(const Connection* connection) = 0;

  // Called when the transport switches that connection in active use.
  virtual void OnConnectionSwitched(const Connection* connection) = 0;

  // Called when a connection is destroyed.
  virtual void OnConnectionDestroyed(const Connection* connection) = 0;

  // Called when a STUN ping has been sent on a connection. This does not
  // indicate that a STUN response has been received.
  virtual void OnConnectionPinged(const Connection* connection) = 0;

  // Called when one of the following changes for a connection.
  // - rtt estimate
  // - write state
  // - receiving
  // - connected
  // - nominated
  virtual void OnConnectionUpdated(const Connection* connection) = 0;

  // Compute "STUN_ATTR_USE_CANDIDATE" for a STUN ping on the given connection.
  virtual bool GetUseCandidateAttribute(const Connection* connection,
                                        NominationMode mode,
                                        IceMode remote_ice_mode) const = 0;

  // Called to enque a request to pick and switch to the best available
  // connection.
  virtual void OnSortAndSwitchRequest(IceSwitchReason reason) = 0;

  // Called to pick and switch to the best available connection immediately.
  virtual void OnImmediateSortAndSwitchRequest(IceSwitchReason reason) = 0;

  // Called to switch to the given connection immediately without checking for
  // the best available connection.
  virtual bool OnImmediateSwitchRequest(IceSwitchReason reason,
                                        const Connection* selected) = 0;

  // Only for unit tests
  virtual const Connection* FindNextPingableConnection() = 0;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::ActiveIceControllerInterface;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_ACTIVE_ICE_CONTROLLER_INTERFACE_H_
