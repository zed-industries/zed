// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_PROPOSAL_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_PROPOSAL_H_

namespace blink {

// Represents a proposed ICE controller action.
class IceProposal {
 public:
  explicit IceProposal(bool reply_expected) : reply_expected_(reply_expected) {}
  virtual ~IceProposal() = default;

  // Whether a reply is expected in response to this proposal. The ICE
  // controller will proceed without waiting if a reply is not expected. If a
  // reply is expected, the ICE controller will take no further action until a
  // response is received.
  bool reply_expected() const { return reply_expected_; }

 protected:
  const bool reply_expected_;
};

}  // namespace blink

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_PROPOSAL_H_
