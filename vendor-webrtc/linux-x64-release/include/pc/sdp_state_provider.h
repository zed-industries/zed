/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_SDP_STATE_PROVIDER_H_
#define PC_SDP_STATE_PROVIDER_H_

#include <string>

#include "api/jsep.h"
#include "api/peer_connection_interface.h"

namespace webrtc {

// This interface provides access to the state of an SDP offer/answer
// negotiation.
//
// All the functions are const, so using this interface serves as
// assurance that the user is not modifying the state.
class SdpStateProvider {
 public:
  virtual ~SdpStateProvider() {}

  virtual PeerConnectionInterface::SignalingState signaling_state() const = 0;

  virtual const SessionDescriptionInterface* local_description() const = 0;
  virtual const SessionDescriptionInterface* remote_description() const = 0;
  virtual const SessionDescriptionInterface* current_local_description()
      const = 0;
  virtual const SessionDescriptionInterface* current_remote_description()
      const = 0;
  virtual const SessionDescriptionInterface* pending_local_description()
      const = 0;
  virtual const SessionDescriptionInterface* pending_remote_description()
      const = 0;

  // Whether an ICE restart has been asked for. Used in CreateOffer.
  virtual bool NeedsIceRestart(const std::string& content_name) const = 0;
  // Whether an ICE restart was indicated in the remote offer.
  // Used in CreateAnswer.
  virtual bool IceRestartPending(const std::string& content_name) const = 0;
  virtual std::optional<SSLRole> GetDtlsRole(const std::string& mid) const = 0;
};

}  // namespace webrtc

#endif  // PC_SDP_STATE_PROVIDER_H_
