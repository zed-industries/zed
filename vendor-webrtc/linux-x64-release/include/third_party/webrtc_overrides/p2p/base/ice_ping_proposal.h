// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_PING_PROPOSAL_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_PING_PROPOSAL_H_

#include <optional>
#include <ostream>
#include <string>

#include "third_party/webrtc/p2p/base/ice_controller_interface.h"
#include "third_party/webrtc/rtc_base/system/rtc_export.h"
#include "third_party/webrtc_overrides/p2p/base/ice_connection.h"
#include "third_party/webrtc_overrides/p2p/base/ice_proposal.h"

namespace blink {

// A proposal to send a STUN ping on an ICE connection. Optionally indicates the
// duration until the next time when a connection will be selected to be pinged.
class RTC_EXPORT IcePingProposal : public IceProposal {
 public:
  IcePingProposal(const webrtc::IceControllerInterface::PingResult& ping_result,
                  bool reply_expected);

  IcePingProposal(const IcePingProposal&) = default;

  ~IcePingProposal() override = default;

  // The connection that will be pinged.
  std::optional<const IceConnection> connection() const { return connection_; }
  // An optional duration to wait until the next time that a connection is
  // selected and pinged. This could be a different connection.
  std::optional<int> recheck_delay_ms() const { return recheck_delay_ms_; }

  std::string ToString() const;
  // Pretty printing for unit test matchers.
  friend void PrintTo(const IcePingProposal& proposal, std::ostream* os) {
    *os << proposal.ToString();
  }

 private:
  std::optional<IceConnection> connection_;
  std::optional<int> recheck_delay_ms_;
};

}  // namespace blink

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_PING_PROPOSAL_H_
