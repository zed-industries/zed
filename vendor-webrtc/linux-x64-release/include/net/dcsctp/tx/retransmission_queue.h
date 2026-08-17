/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_TX_RETRANSMISSION_QUEUE_H_
#define NET_DCSCTP_TX_RETRANSMISSION_QUEUE_H_

#include <cstdint>
#include <functional>
#include <map>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/common/sequence_numbers.h"
#include "net/dcsctp/packet/chunk/forward_tsn_chunk.h"
#include "net/dcsctp/packet/chunk/iforward_tsn_chunk.h"
#include "net/dcsctp/packet/chunk/sack_chunk.h"
#include "net/dcsctp/packet/data.h"
#include "net/dcsctp/public/dcsctp_handover_state.h"
#include "net/dcsctp/public/dcsctp_options.h"
#include "net/dcsctp/public/dcsctp_socket.h"
#include "net/dcsctp/timer/timer.h"
#include "net/dcsctp/tx/outstanding_data.h"
#include "net/dcsctp/tx/retransmission_timeout.h"
#include "net/dcsctp/tx/send_queue.h"

namespace dcsctp {

// The RetransmissionQueue manages all DATA/I-DATA chunks that are in-flight and
// schedules them to be retransmitted if necessary. Chunks are retransmitted
// when they have been lost for a number of consecutive SACKs, or when the
// retransmission timer, `t3_rtx` expires.
//
// As congestion control is tightly connected with the state of transmitted
// packets, that's also managed here to limit the amount of data that is
// in-flight (sent, but not yet acknowledged).
class RetransmissionQueue {
 public:
  static constexpr size_t kMinimumFragmentedPayload = 10;
  using State = OutstandingData::State;
  // Creates a RetransmissionQueue which will send data using `my_initial_tsn`
  // (or a value from `DcSctpSocketHandoverState` if given) as the first TSN
  // to use for sent fragments. It will poll data from `send_queue`. When SACKs
  // are received, it will estimate the RTT, and call `on_new_rtt`. When an
  // outstanding chunk has been ACKed, it will call
  // `on_clear_retransmission_counter` and will also use `t3_rtx`, which is the
  // SCTP retransmission timer to manage retransmissions.
  RetransmissionQueue(absl::string_view log_prefix,
                      DcSctpSocketCallbacks* callbacks,
                      TSN my_initial_tsn,
                      size_t a_rwnd,
                      SendQueue& send_queue,
                      std::function<void(webrtc::TimeDelta rtt)> on_new_rtt,
                      std::function<void()> on_clear_retransmission_counter,
                      Timer& t3_rtx,
                      const DcSctpOptions& options,
                      bool supports_partial_reliability = true,
                      bool use_message_interleaving = false);

  // Handles a received SACK. Returns true if the `sack` was processed and
  // false if it was discarded due to received out-of-order and not relevant.
  bool HandleSack(webrtc::Timestamp now, const SackChunk& sack);

  // Handles an expired retransmission timer.
  void HandleT3RtxTimerExpiry();

  bool has_data_to_be_fast_retransmitted() const {
    return outstanding_data_.has_data_to_be_fast_retransmitted();
  }

  // Returns a list of chunks to "fast retransmit" that would fit in one SCTP
  // packet with `bytes_in_packet` bytes available. The current value
  // of `cwnd` is ignored.
  std::vector<std::pair<TSN, Data>> GetChunksForFastRetransmit(
      size_t bytes_in_packet);

  // Returns a list of chunks to send that would fit in one SCTP packet with
  // `bytes_remaining_in_packet` bytes available. This may be further limited by
  // the congestion control windows. Note that `ShouldSendForwardTSN` must be
  // called prior to this method, to abandon expired chunks, as this method will
  // not expire any chunks.
  std::vector<std::pair<TSN, Data>> GetChunksToSend(
      webrtc::Timestamp now,
      size_t bytes_remaining_in_packet);

  // Returns the internal state of all queued chunks. This is only used in
  // unit-tests.
  std::vector<std::pair<TSN, OutstandingData::State>> GetChunkStatesForTesting()
      const {
    return outstanding_data_.GetChunkStatesForTesting();
  }

  // Returns the next TSN that will be allocated for sent DATA chunks.
  TSN next_tsn() const { return outstanding_data_.next_tsn().Wrap(); }

  TSN last_assigned_tsn() const {
    return UnwrappedTSN::AddTo(outstanding_data_.next_tsn(), -1).Wrap();
  }

  // Returns the size of the congestion window, in bytes. This is the number of
  // bytes that may be in-flight.
  size_t cwnd() const { return cwnd_; }

  // Overrides the current congestion window size.
  void set_cwnd(size_t cwnd) { cwnd_ = cwnd; }

  // Returns the current receiver window size.
  size_t rwnd() const { return rwnd_; }

  size_t rtx_packets_count() const { return rtx_packets_count_; }
  uint64_t rtx_bytes_count() const { return rtx_bytes_count_; }

  // How many inflight bytes there are, as sent on the wire as packets.
  size_t unacked_packet_bytes() const {
    return outstanding_data_.unacked_packet_bytes();
  }

  // Returns the number of DATA chunks that are in-flight.
  size_t unacked_items() const { return outstanding_data_.unacked_items(); }

  // Given the current time `now`, it will evaluate if there are chunks that
  // have expired and that need to be discarded. It returns true if a
  // FORWARD-TSN should be sent.
  bool ShouldSendForwardTsn(webrtc::Timestamp now);

  // Creates a FORWARD-TSN chunk.
  ForwardTsnChunk CreateForwardTsn() const {
    return outstanding_data_.CreateForwardTsn();
  }

  // Creates an I-FORWARD-TSN chunk.
  IForwardTsnChunk CreateIForwardTsn() const {
    return outstanding_data_.CreateIForwardTsn();
  }

  // See the SendQueue for a longer description of these methods related
  // to stream resetting.
  void PrepareResetStream(StreamID stream_id);
  bool HasStreamsReadyToBeReset() const;
  std::vector<StreamID> BeginResetStreams();
  void CommitResetStreams();
  void RollbackResetStreams();

  HandoverReadinessStatus GetHandoverReadiness() const;

  void AddHandoverState(DcSctpSocketHandoverState& state);
  void RestoreFromState(const DcSctpSocketHandoverState& state);

 private:
  enum class CongestionAlgorithmPhase {
    kSlowStart,
    kCongestionAvoidance,
  };

  bool IsConsistent() const;

  // Returns how large a chunk will be, serialized, carrying the data
  size_t GetSerializedChunkSize(const Data& data) const;

  // Indicates if the congestion control algorithm is in "fast recovery".
  bool is_in_fast_recovery() const {
    return fast_recovery_exit_tsn_.has_value();
  }

  // Indicates if the provided SACK is valid given what has previously been
  // received. If it returns false, the SACK is most likely a duplicate of
  // something already seen, so this returning false doesn't necessarily mean
  // that the SACK is illegal.
  bool IsSackValid(const SackChunk& sack) const;

  // When a SACK chunk is received, this method will be called which _may_ call
  // into the `RetransmissionTimeout` to update the RTO.
  void UpdateRTT(webrtc::Timestamp now, UnwrappedTSN cumulative_tsn_ack);

  // If the congestion control is in "fast recovery mode", this may be exited
  // now.
  void MaybeExitFastRecovery(UnwrappedTSN cumulative_tsn_ack);

  // If chunks have been ACKed, stop the retransmission timer.
  void StopT3RtxTimerOnIncreasedCumulativeTsnAck(
      UnwrappedTSN cumulative_tsn_ack);

  // Update the congestion control algorithm given as the cumulative ack TSN
  // value has increased, as reported in an incoming SACK chunk.
  void HandleIncreasedCumulativeTsnAck(size_t unacked_packet_bytes,
                                       size_t total_bytes_acked);
  // Update the congestion control algorithm, given as packet loss has been
  // detected, as reported in an incoming SACK chunk.
  void HandlePacketLoss(UnwrappedTSN highest_tsn_acked);
  // Update the view of the receiver window size.
  void UpdateReceiverWindow(uint32_t a_rwnd);
  // If there is data sent and not ACKED, ensure that the retransmission timer
  // is running.
  void StartT3RtxTimerIfOutstandingData();

  // Returns the current congestion control algorithm phase.
  CongestionAlgorithmPhase phase() const {
    return (cwnd_ <= ssthresh_)
               ? CongestionAlgorithmPhase::kSlowStart
               : CongestionAlgorithmPhase::kCongestionAvoidance;
  }

  DcSctpSocketCallbacks& callbacks_;
  const DcSctpOptions options_;
  // If the peer supports RFC3758 - SCTP Partial Reliability Extension.
  const bool partial_reliability_;
  const absl::string_view log_prefix_;
  // The size of the data chunk (DATA/I-DATA) header that is used.
  const size_t data_chunk_header_size_;
  // Called when a new RTT measurement has been done
  const std::function<void(webrtc::TimeDelta rtt)> on_new_rtt_;
  // Called when a SACK has been seen that cleared the retransmission counter.
  const std::function<void()> on_clear_retransmission_counter_;
  // The retransmission counter.
  Timer& t3_rtx_;
  // Unwraps TSNs
  UnwrappedTSN::Unwrapper tsn_unwrapper_;

  // Congestion Window. Number of bytes that may be in-flight (sent, not acked).
  size_t cwnd_;
  // Receive Window. Number of bytes available in the receiver's RX buffer.
  size_t rwnd_;
  // Slow Start Threshold. See RFC4960.
  size_t ssthresh_;
  // Partial Bytes Acked. See RFC4960.
  size_t partial_bytes_acked_;

  // See `dcsctp::Metrics`.
  size_t rtx_packets_count_ = 0;
  uint64_t rtx_bytes_count_ = 0;

  // If set, fast recovery is enabled until this TSN has been cumulative
  // acked.
  std::optional<UnwrappedTSN> fast_recovery_exit_tsn_ = std::nullopt;

  // The send queue.
  SendQueue& send_queue_;
  // All the outstanding data chunks that are in-flight and that have not been
  // cumulative acked. Note that it also contains chunks that have been acked in
  // gap ack blocks.
  OutstandingData outstanding_data_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_TX_RETRANSMISSION_QUEUE_H_
