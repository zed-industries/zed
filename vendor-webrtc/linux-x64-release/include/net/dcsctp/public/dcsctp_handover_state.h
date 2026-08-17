/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PUBLIC_DCSCTP_HANDOVER_STATE_H_
#define NET_DCSCTP_PUBLIC_DCSCTP_HANDOVER_STATE_H_

#include <cstdint>
#include <string>
#include <vector>

#include "rtc_base/strong_alias.h"

namespace dcsctp {

// Stores state snapshot of a dcSCTP socket. The snapshot can be used to
// recreate the socket - possibly in another process. This state should be
// treaded as opaque - the calling client should not inspect or alter it except
// for serialization. Serialization is not provided by dcSCTP. If needed it has
// to be implemented in the calling client.
struct DcSctpSocketHandoverState {
  enum class SocketState {
    kClosed,
    kConnected,
  };
  SocketState socket_state = SocketState::kClosed;

  uint32_t my_verification_tag = 0;
  uint32_t my_initial_tsn = 0;
  uint32_t peer_verification_tag = 0;
  uint32_t peer_initial_tsn = 0;
  uint64_t tie_tag = 0;

  struct Capabilities {
    bool partial_reliability = false;
    bool message_interleaving = false;
    bool reconfig = false;
    bool zero_checksum = false;
    uint16_t negotiated_maximum_incoming_streams = 0;
    uint16_t negotiated_maximum_outgoing_streams = 0;
  };
  Capabilities capabilities;

  struct OutgoingStream {
    uint32_t id = 0;
    uint32_t next_ssn = 0;
    uint32_t next_unordered_mid = 0;
    uint32_t next_ordered_mid = 0;
    uint16_t priority = 0;
  };
  struct Transmission {
    uint32_t next_tsn = 0;
    uint32_t next_reset_req_sn = 0;
    uint32_t cwnd = 0;
    uint32_t rwnd = 0;
    uint32_t ssthresh = 0;
    uint32_t partial_bytes_acked = 0;
    std::vector<OutgoingStream> streams;
  };
  Transmission tx;

  struct OrderedStream {
    uint32_t id = 0;
    uint32_t next_ssn = 0;
  };
  struct UnorderedStream {
    uint32_t id = 0;
  };
  struct Receive {
    bool seen_packet = false;
    uint32_t last_cumulative_acked_tsn = 0;
    uint32_t last_assembled_tsn = 0;
    uint32_t last_completed_deferred_reset_req_sn = 0;
    uint32_t last_completed_reset_req_sn = 0;
    std::vector<OrderedStream> ordered_streams;
    std::vector<UnorderedStream> unordered_streams;
  };
  Receive rx;
};

// A list of possible reasons for a socket to be not ready for handover.
enum class HandoverUnreadinessReason : uint32_t {
  kWrongConnectionState = 1,
  kSendQueueNotEmpty = 2,
  kPendingStreamResetRequest = 4,
  kDataTrackerTsnBlocksPending = 8,
  kPendingStreamReset = 16,
  kReassemblyQueueDeliveredTSNsGap = 32,
  kStreamResetDeferred = 64,
  kOrderedStreamHasUnassembledChunks = 128,
  kUnorderedStreamHasUnassembledChunks = 256,
  kRetransmissionQueueOutstandingData = 512,
  kRetransmissionQueueFastRecovery = 1024,
  kRetransmissionQueueNotEmpty = 2048,
  kMax = kRetransmissionQueueNotEmpty,
};

// Return value of `DcSctpSocketInterface::GetHandoverReadiness`. Set of
// `HandoverUnreadinessReason` bits. When no bit is set, the socket is in the
// state in which a snapshot of the state can be made by
// `GetHandoverStateAndClose()`.
class HandoverReadinessStatus
    : public webrtc::StrongAlias<class HandoverReadinessStatusTag, uint32_t> {
 public:
  // Constructs an empty `HandoverReadinessStatus` which represents ready state.
  constexpr HandoverReadinessStatus()
      : webrtc::StrongAlias<class HandoverReadinessStatusTag, uint32_t>(0) {}
  // Constructs status object that contains a single reason for not being
  // handover ready.
  constexpr explicit HandoverReadinessStatus(HandoverUnreadinessReason reason)
      : webrtc::StrongAlias<class HandoverReadinessStatusTag, uint32_t>(
            static_cast<uint32_t>(reason)) {}

  // Convenience methods
  constexpr bool IsReady() const { return value() == 0; }
  constexpr bool Contains(HandoverUnreadinessReason reason) const {
    return value() & static_cast<uint32_t>(reason);
  }
  HandoverReadinessStatus& Add(HandoverUnreadinessReason reason) {
    return Add(HandoverReadinessStatus(reason));
  }
  HandoverReadinessStatus& Add(HandoverReadinessStatus status) {
    value() |= status.value();
    return *this;
  }
  std::string ToString() const;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PUBLIC_DCSCTP_HANDOVER_STATE_H_
