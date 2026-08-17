// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_CONTROLLER_OBSERVER_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_CONTROLLER_OBSERVER_H_

#include "base/memory/scoped_refptr.h"

#include "third_party/webrtc_overrides/p2p/base/ice_connection.h"
#include "third_party/webrtc_overrides/p2p/base/ice_interaction_interface.h"
#include "third_party/webrtc_overrides/p2p/base/ice_ping_proposal.h"
#include "third_party/webrtc_overrides/p2p/base/ice_prune_proposal.h"
#include "third_party/webrtc_overrides/p2p/base/ice_switch_proposal.h"

namespace blink {

// An interface for an observer of actions performed by an ICE controller. This
// interface mirrors the events fired on the JS ICE controller.
class IceControllerObserverInterface {
 public:
  virtual ~IceControllerObserverInterface() = default;

  // Once attached, the observer will receive ICE controller events from the
  // observed ICE controller. The observer can respond to events where the ICE
  // controller expects such responses through the interaction agent supplied in
  // this method.
  virtual void OnObserverAttached(
      scoped_refptr<IceInteractionInterface> agent) {}
  // The observer will no longer receive ICE controller events for the observed
  // ICE controller.
  virtual void OnObserverDetached() {}

  // A connection was added to the ICE controller.
  virtual void OnConnectionAdded(const IceConnection& connection) {}
  // A state information of a connection for the ICE controller has changed.
  virtual void OnConnectionUpdated(const IceConnection& connection) {}
  // The active connection used for the ICE transport changed.
  virtual void OnConnectionSwitched(const IceConnection& connection) {}
  // A connection for this ICE controller was destroyed.
  virtual void OnConnectionDestroyed(const IceConnection& connection) {}

  // The ICE controller intends to send a STUN ping on a connection indicated in
  // the ping proposal.
  virtual void OnPingProposal(const IcePingProposal& ping_proposal) {}
  // The ICE controller intends to switch the ICE transport to the connection
  // indicated in the switch proposal.
  virtual void OnSwitchProposal(const IceSwitchProposal& switch_proposal) {}
  // The ICE controller intends to prune away the connections indicated in
  // the prune proposal.
  virtual void OnPruneProposal(const IcePruneProposal& prune_proposal) {}
};

}  // namespace blink

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_CONTROLLER_OBSERVER_H_
