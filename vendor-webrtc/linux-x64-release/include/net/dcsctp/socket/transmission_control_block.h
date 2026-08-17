/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_SOCKET_TRANSMISSION_CONTROL_BLOCK_H_
#define NET_DCSCTP_SOCKET_TRANSMISSION_CONTROL_BLOCK_H_

#include <cstdint>
#include <functional>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/functional/bind_front.h"
#include "absl/strings/string_view.h"
#include "api/task_queue/task_queue_base.h"
#include "net/dcsctp/common/sequence_numbers.h"
#include "net/dcsctp/packet/chunk/cookie_echo_chunk.h"
#include "net/dcsctp/packet/sctp_packet.h"
#include "net/dcsctp/public/dcsctp_options.h"
#include "net/dcsctp/public/dcsctp_socket.h"
#include "net/dcsctp/rx/data_tracker.h"
#include "net/dcsctp/rx/reassembly_queue.h"
#include "net/dcsctp/socket/capabilities.h"
#include "net/dcsctp/socket/context.h"
#include "net/dcsctp/socket/heartbeat_handler.h"
#include "net/dcsctp/socket/packet_sender.h"
#include "net/dcsctp/socket/stream_reset_handler.h"
#include "net/dcsctp/timer/timer.h"
#include "net/dcsctp/tx/retransmission_error_counter.h"
#include "net/dcsctp/tx/retransmission_queue.h"
#include "net/dcsctp/tx/retransmission_timeout.h"
#include "net/dcsctp/tx/send_queue.h"

namespace dcsctp {

// The TransmissionControlBlock (TCB) represents an open connection to a peer,
// and holds all the resources for that. If the connection is e.g. shutdown,
// closed or restarted, this object will be deleted and/or replaced.
class TransmissionControlBlock : public Context {
 public:
  TransmissionControlBlock(TimerManager& timer_manager,
                           absl::string_view log_prefix,
                           const DcSctpOptions& options,
                           const Capabilities& capabilities,
                           DcSctpSocketCallbacks& callbacks,
                           SendQueue& send_queue,
                           VerificationTag my_verification_tag,
                           TSN my_initial_tsn,
                           VerificationTag peer_verification_tag,
                           TSN peer_initial_tsn,
                           size_t a_rwnd,
                           TieTag tie_tag,
                           PacketSender& packet_sender,
                           std::function<bool()> is_connection_established);

  // Implementation of `Context`.
  bool is_connection_established() const override {
    return is_connection_established_();
  }
  TSN my_initial_tsn() const override { return my_initial_tsn_; }
  TSN peer_initial_tsn() const override { return peer_initial_tsn_; }
  DcSctpSocketCallbacks& callbacks() const override { return callbacks_; }
  void ObserveRTT(webrtc::TimeDelta rtt) override;
  webrtc::TimeDelta current_rto() const override { return rto_.rto(); }
  bool IncrementTxErrorCounter(absl::string_view reason) override {
    return tx_error_counter_.Increment(reason);
  }
  void ClearTxErrorCounter() override { tx_error_counter_.Clear(); }
  SctpPacket::Builder PacketBuilder() const override {
    return SctpPacket::Builder(peer_verification_tag_, options_);
  }
  bool HasTooManyTxErrors() const override {
    return tx_error_counter_.IsExhausted();
  }
  void Send(SctpPacket::Builder& builder) override {
    packet_sender_.Send(builder,
                        /*write_checksum=*/!capabilities_.zero_checksum);
  }

  // Other accessors
  DataTracker& data_tracker() { return data_tracker_; }
  ReassemblyQueue& reassembly_queue() { return reassembly_queue_; }
  RetransmissionQueue& retransmission_queue() { return retransmission_queue_; }
  StreamResetHandler& stream_reset_handler() { return stream_reset_handler_; }
  HeartbeatHandler& heartbeat_handler() { return heartbeat_handler_; }
  size_t cwnd() const { return retransmission_queue_.cwnd(); }
  webrtc::TimeDelta current_srtt() const { return rto_.srtt(); }

  // Returns this socket's verification tag, set in all packet headers.
  VerificationTag my_verification_tag() const { return my_verification_tag_; }
  // Returns the peer's verification tag, which should be in received packets.
  VerificationTag peer_verification_tag() const {
    return peer_verification_tag_;
  }
  // All negotiated supported capabilities.
  const Capabilities& capabilities() const { return capabilities_; }
  // A 64-bit tie-tag, used to e.g. detect reconnections.
  TieTag tie_tag() const { return tie_tag_; }

  // Sends a SACK, if there is a need to.
  void MaybeSendSack();

  // Sends a FORWARD-TSN, if it is needed and allowed (rate-limited).
  void MaybeSendForwardTsn(SctpPacket::Builder& builder, webrtc::Timestamp now);

  // Will be set while the socket is in kCookieEcho state. In this state, there
  // can only be a single packet outstanding, and it must contain the COOKIE
  // ECHO chunk as the first chunk in that packet, until the COOKIE ACK has been
  // received, which will make the socket call `ClearCookieEchoChunk`.
  void SetCookieEchoChunk(CookieEchoChunk chunk) {
    cookie_echo_chunk_ = std::move(chunk);
  }

  // Called when the COOKIE ACK chunk has been received, to allow further
  // packets to be sent.
  void ClearCookieEchoChunk() { cookie_echo_chunk_ = std::nullopt; }

  bool has_cookie_echo_chunk() const { return cookie_echo_chunk_.has_value(); }

  void MaybeSendFastRetransmit();

  // Fills `builder` (which may already be filled with control chunks) with
  // other control and data chunks, and sends packets as much as can be
  // allowed by the congestion control algorithm.
  void SendBufferedPackets(SctpPacket::Builder& builder, webrtc::Timestamp now);

  // As above, but without passing in a builder. If `cookie_echo_chunk_` is
  // present, then only one packet will be sent, with this chunk as the first
  // chunk.
  void SendBufferedPackets(webrtc::Timestamp now) {
    SctpPacket::Builder builder(peer_verification_tag_, options_);
    SendBufferedPackets(builder, now);
  }

  // Returns a textual representation of this object, for logging.
  std::string ToString() const;

  HandoverReadinessStatus GetHandoverReadiness() const;

  void AddHandoverState(DcSctpSocketHandoverState& state);
  void RestoreFromState(const DcSctpSocketHandoverState& handover_state);

 private:
  // Will be called when the retransmission timer (t3-rtx) expires.
  webrtc::TimeDelta OnRtxTimerExpiry();
  // Will be called when the delayed ack timer expires.
  webrtc::TimeDelta OnDelayedAckTimerExpiry();

  const absl::string_view log_prefix_;
  const DcSctpOptions options_;
  TimerManager& timer_manager_;
  // Negotiated capabilities that both peers support.
  const Capabilities capabilities_;
  DcSctpSocketCallbacks& callbacks_;
  // The data retransmission timer, called t3-rtx in SCTP.
  const std::unique_ptr<Timer> t3_rtx_;
  // Delayed ack timer, which triggers when acks should be sent (when delayed).
  const std::unique_ptr<Timer> delayed_ack_timer_;
  const VerificationTag my_verification_tag_;
  const TSN my_initial_tsn_;
  const VerificationTag peer_verification_tag_;
  const TSN peer_initial_tsn_;
  // Nonce, used to detect reconnections.
  const TieTag tie_tag_;
  const std::function<bool()> is_connection_established_;
  PacketSender& packet_sender_;
  // Rate limiting of FORWARD-TSN. Next can be sent at or after this timestamp.
  webrtc::Timestamp limit_forward_tsn_until_ = webrtc::Timestamp::Zero();

  RetransmissionTimeout rto_;
  RetransmissionErrorCounter tx_error_counter_;
  DataTracker data_tracker_;
  ReassemblyQueue reassembly_queue_;
  RetransmissionQueue retransmission_queue_;
  StreamResetHandler stream_reset_handler_;
  HeartbeatHandler heartbeat_handler_;

  // Only valid when the socket state == State::kCookieEchoed. In this state,
  // the socket must wait for COOKIE ACK to continue sending any packets (not
  // including a COOKIE ECHO). So if `cookie_echo_chunk_` is present, the
  // SendBufferedChunks will always only just send one packet, with this chunk
  // as the first chunk in the packet.
  std::optional<CookieEchoChunk> cookie_echo_chunk_ = std::nullopt;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_SOCKET_TRANSMISSION_CONTROL_BLOCK_H_
