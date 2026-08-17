// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_PRUNE_PROPOSAL_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_PRUNE_PROPOSAL_H_

#include <ostream>
#include <string>
#include <vector>

#include "third_party/webrtc/api/array_view.h"
#include "third_party/webrtc/p2p/base/connection.h"
#include "third_party/webrtc/rtc_base/system/rtc_export.h"
#include "third_party/webrtc_overrides/p2p/base/ice_connection.h"
#include "third_party/webrtc_overrides/p2p/base/ice_proposal.h"

namespace blink {

// A proposal to discard some ICE connections.
class RTC_EXPORT IcePruneProposal : public IceProposal {
 public:
  IcePruneProposal(
      const webrtc::ArrayView<const webrtc::Connection*> connections_to_prune,
      bool reply_expected);

  IcePruneProposal(const IcePruneProposal&) = default;

  ~IcePruneProposal() override = default;

  // The ICE connections that will be discarded. Once pruned, these are no
  // longer viable candidates for a STUN ping or to switch the transport to.
  const webrtc::ArrayView<const IceConnection> connections_to_prune() const {
    return connections_to_prune_;
  }

  std::string ToString() const;
  // Pretty printing for unit test matchers.
  friend void PrintTo(const IcePruneProposal& proposal, std::ostream* os) {
    *os << proposal.ToString();
  }

 private:
  std::vector<IceConnection> connections_to_prune_;
};

}  // namespace blink

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_PRUNE_PROPOSAL_H_
