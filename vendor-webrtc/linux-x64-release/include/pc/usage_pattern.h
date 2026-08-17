/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_USAGE_PATTERN_H_
#define PC_USAGE_PATTERN_H_

#include "api/peer_connection_interface.h"

namespace webrtc {

class PeerConnectionObserver;

// A bit in the usage pattern is registered when its defining event occurs
// at least once.
enum class UsageEvent : int {
  TURN_SERVER_ADDED = 0x01,
  STUN_SERVER_ADDED = 0x02,
  DATA_ADDED = 0x04,
  AUDIO_ADDED = 0x08,
  VIDEO_ADDED = 0x10,
  // `SetLocalDescription` returns successfully.
  SET_LOCAL_DESCRIPTION_SUCCEEDED = 0x20,
  // `SetRemoteDescription` returns successfully.
  SET_REMOTE_DESCRIPTION_SUCCEEDED = 0x40,
  // A local candidate (with type host, server-reflexive, or relay) is
  // collected.
  CANDIDATE_COLLECTED = 0x80,
  // A remote candidate is successfully added via `AddIceCandidate`.
  ADD_ICE_CANDIDATE_SUCCEEDED = 0x100,
  ICE_STATE_CONNECTED = 0x200,
  CLOSE_CALLED = 0x400,
  // A local candidate with private IP is collected.
  PRIVATE_CANDIDATE_COLLECTED = 0x800,
  // A remote candidate with private IP is added, either via AddiceCandidate
  // or from the remote description.
  REMOTE_PRIVATE_CANDIDATE_ADDED = 0x1000,
  // A local mDNS candidate is collected.
  MDNS_CANDIDATE_COLLECTED = 0x2000,
  // A remote mDNS candidate is added, either via AddIceCandidate or from the
  // remote description.
  REMOTE_MDNS_CANDIDATE_ADDED = 0x4000,
  // A local candidate with IPv6 address is collected.
  IPV6_CANDIDATE_COLLECTED = 0x8000,
  // A remote candidate with IPv6 address is added, either via AddIceCandidate
  // or from the remote description.
  REMOTE_IPV6_CANDIDATE_ADDED = 0x10000,
  // A remote candidate (with type host, server-reflexive, or relay) is
  // successfully added, either via AddIceCandidate or from the remote
  // description.
  REMOTE_CANDIDATE_ADDED = 0x20000,
  // An explicit host-host candidate pair is selected, i.e. both the local and
  // the remote candidates have the host type. This does not include candidate
  // pairs formed with equivalent prflx remote candidates, e.g. a host-prflx
  // pair where the prflx candidate has the same base as a host candidate of
  // the remote peer.
  DIRECT_CONNECTION_SELECTED = 0x40000,
  MAX_VALUE = 0x80000,
};

class UsagePattern {
 public:
  void NoteUsageEvent(UsageEvent event);
  void ReportUsagePattern(PeerConnectionObserver* observer) const;

 private:
  int usage_event_accumulator_ = 0;
};

}  // namespace webrtc
#endif  // PC_USAGE_PATTERN_H_
