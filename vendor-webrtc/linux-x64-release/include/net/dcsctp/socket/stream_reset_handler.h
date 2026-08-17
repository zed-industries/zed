/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_SOCKET_STREAM_RESET_HANDLER_H_
#define NET_DCSCTP_SOCKET_STREAM_RESET_HANDLER_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/functional/bind_front.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/units/time_delta.h"
#include "net/dcsctp/common/internal_types.h"
#include "net/dcsctp/packet/chunk/reconfig_chunk.h"
#include "net/dcsctp/packet/parameter/incoming_ssn_reset_request_parameter.h"
#include "net/dcsctp/packet/parameter/outgoing_ssn_reset_request_parameter.h"
#include "net/dcsctp/packet/parameter/reconfiguration_response_parameter.h"
#include "net/dcsctp/packet/sctp_packet.h"
#include "net/dcsctp/public/dcsctp_socket.h"
#include "net/dcsctp/rx/data_tracker.h"
#include "net/dcsctp/rx/reassembly_queue.h"
#include "net/dcsctp/socket/context.h"
#include "net/dcsctp/timer/timer.h"
#include "net/dcsctp/tx/retransmission_queue.h"
#include "rtc_base/containers/flat_set.h"

namespace dcsctp {

// StreamResetHandler handles sending outgoing stream reset requests (to close
// an SCTP stream, which translates to closing a data channel).
//
// It also handles incoming "outgoing stream reset requests", when the peer
// wants to close its data channel.
//
// Resetting streams is an asynchronous operation where the client will request
// a request a stream to be reset, but then it might not be performed exactly at
// this point. First, the sender might need to discard all messages that have
// been enqueued for this stream, or it may select to wait until all have been
// sent. At least, it must wait for the currently sending fragmented message to
// be fully sent, because a stream can't be reset while having received half a
// message. In the stream reset request, the "sender's last assigned TSN" is
// provided, which is simply the TSN for which the receiver should've received
// all messages before this value, before the stream can be reset. Since
// fragments can get lost or sent out-of-order, the receiver of a request may
// not have received all the data just yet, and then it will respond to the
// sender: "In progress". In other words, try again. The sender will then need
// to start a timer and try the very same request again (but with a new sequence
// number) until the receiver successfully performs the operation.
//
// All this can take some time, and may be driven by timers, so the client will
// ultimately be notified using callbacks.
//
// In this implementation, when a stream is reset, the queued but not-yet-sent
// messages will be discarded, but that may change in the future. RFC8831 allows
// both behaviors.
class StreamResetHandler {
 public:
  StreamResetHandler(absl::string_view log_prefix,
                     Context* context,
                     TimerManager* timer_manager,
                     DataTracker* data_tracker,
                     ReassemblyQueue* reassembly_queue,
                     RetransmissionQueue* retransmission_queue,
                     const DcSctpSocketHandoverState* handover_state = nullptr)
      : log_prefix_(log_prefix),
        ctx_(context),
        data_tracker_(data_tracker),
        reassembly_queue_(reassembly_queue),
        retransmission_queue_(retransmission_queue),
        reconfig_timer_(timer_manager->CreateTimer(
            "re-config",
            absl::bind_front(&StreamResetHandler::OnReconfigTimerExpiry, this),
            TimerOptions(webrtc::TimeDelta::Zero()))),
        next_outgoing_req_seq_nbr_(
            handover_state
                ? ReconfigRequestSN(handover_state->tx.next_reset_req_sn)
                : ReconfigRequestSN(*ctx_->my_initial_tsn())),
        last_processed_req_seq_nbr_(
            incoming_reconfig_request_sn_unwrapper_.Unwrap(
                handover_state
                    ? ReconfigRequestSN(
                          handover_state->rx.last_completed_reset_req_sn)
                    : ReconfigRequestSN(*ctx_->peer_initial_tsn() - 1))),
        last_processed_req_result_(
            ReconfigurationResponseParameter::Result::kSuccessNothingToDo) {}

  // Initiates reset of the provided streams. While there can only be one
  // ongoing stream reset request at any time, this method can be called at any
  // time and also multiple times. It will enqueue requests that can't be
  // directly fulfilled, and will asynchronously process them when any ongoing
  // request has completed.
  void ResetStreams(webrtc::ArrayView<const StreamID> outgoing_streams);

  // Creates a Reset Streams request that must be sent if returned. Will start
  // the reconfig timer. Will return std::nullopt if there is no need to
  // create a request (no streams to reset) or if there already is an ongoing
  // stream reset request that hasn't completed yet.
  std::optional<ReConfigChunk> MakeStreamResetRequest();

  // Called when handling and incoming RE-CONFIG chunk.
  void HandleReConfig(ReConfigChunk chunk);

  HandoverReadinessStatus GetHandoverReadiness() const;

  void AddHandoverState(DcSctpSocketHandoverState& state);

 private:
  using UnwrappedReconfigRequestSn = UnwrappedSequenceNumber<ReconfigRequestSN>;
  // Represents a stream request operation. There can only be one ongoing at
  // any time, and a sent request may either succeed, fail or result in the
  // receiver signaling that it can't process it right now, and then it will be
  // retried.
  class CurrentRequest {
   public:
    CurrentRequest(TSN sender_last_assigned_tsn, std::vector<StreamID> streams)
        : req_seq_nbr_(std::nullopt),
          sender_last_assigned_tsn_(sender_last_assigned_tsn),
          streams_(std::move(streams)) {}

    // Returns the current request sequence number, if this request has been
    // sent (check `has_been_sent` first). Will return 0 if the request is just
    // prepared (or scheduled for retransmission) but not yet sent.
    ReconfigRequestSN req_seq_nbr() const {
      return req_seq_nbr_.value_or(ReconfigRequestSN(0));
    }

    // The sender's last assigned TSN, from the retransmission queue. The
    // receiver uses this to know when all data up to this TSN has been
    // received, to know when to safely reset the stream.
    TSN sender_last_assigned_tsn() const { return sender_last_assigned_tsn_; }

    // The streams that are to be reset.
    const std::vector<StreamID>& streams() const { return streams_; }

    // If this request has been sent yet. If not, then it's either because it
    // has only been prepared and not yet sent, or because the received couldn't
    // apply the request, and then the exact same request will be retried, but
    // with a new sequence number.
    bool has_been_sent() const { return req_seq_nbr_.has_value(); }

    // If the receiver can't apply the request yet (and answered "In Progress"),
    // this will be called to prepare the request to be retransmitted at a later
    // time.
    void PrepareRetransmission() { req_seq_nbr_ = std::nullopt; }

    // If the request hasn't been sent yet, this assigns it a request number.
    void PrepareToSend(ReconfigRequestSN new_req_seq_nbr) {
      req_seq_nbr_ = new_req_seq_nbr;
    }

   private:
    // If this is set, this request has been sent. If it's not set, the request
    // has been prepared, but has not yet been sent. This is typically used when
    // the peer responded "in progress" and the same request (but a different
    // request number) must be sent again.
    std::optional<ReconfigRequestSN> req_seq_nbr_;
    // The sender's (that's us) last assigned TSN, from the retransmission
    // queue.
    TSN sender_last_assigned_tsn_;
    // The streams that are to be reset in this request.
    const std::vector<StreamID> streams_;
  };

  // Called to validate an incoming RE-CONFIG chunk.
  bool Validate(const ReConfigChunk& chunk);

  // Processes a stream stream reconfiguration chunk and may either return
  // std::nullopt (on protocol errors), or a list of responses - either 0, 1
  // or 2.
  std::optional<std::vector<ReconfigurationResponseParameter>> Process(
      const ReConfigChunk& chunk);

  // Creates the actual RE-CONFIG chunk. A request (which set `current_request`)
  // must have been created prior.
  ReConfigChunk MakeReconfigChunk();

  // Called to validate the `req_seq_nbr`, that it's the next in sequence. If it
  // fails to validate, and returns false, it will also add a response to
  // `responses`.
  bool ValidateReqSeqNbr(
      UnwrappedReconfigRequestSn req_seq_nbr,
      std::vector<ReconfigurationResponseParameter>& responses);

  // Called when this socket receives an outgoing stream reset request. It might
  // either be performed straight away, or have to be deferred, and the result
  // of that will be put in `responses`.
  void HandleResetOutgoing(
      const ParameterDescriptor& descriptor,
      std::vector<ReconfigurationResponseParameter>& responses);

  // Called when this socket receives an incoming stream reset request. This
  // isn't really supported, but a successful response is put in `responses`.
  void HandleResetIncoming(
      const ParameterDescriptor& descriptor,
      std::vector<ReconfigurationResponseParameter>& responses);

  // Called when receiving a response to an outgoing stream reset request. It
  // will either commit the stream resetting, if the operation was successful,
  // or will schedule a retry if it was deferred. And if it failed, the
  // operation will be rolled back.
  void HandleResponse(const ParameterDescriptor& descriptor);

  // Expiration handler for the Reconfig timer.
  webrtc::TimeDelta OnReconfigTimerExpiry();

  const absl::string_view log_prefix_;
  Context* ctx_;
  DataTracker* data_tracker_;
  ReassemblyQueue* reassembly_queue_;
  RetransmissionQueue* retransmission_queue_;
  UnwrappedReconfigRequestSn::Unwrapper incoming_reconfig_request_sn_unwrapper_;
  const std::unique_ptr<Timer> reconfig_timer_;

  // The next sequence number for outgoing stream requests.
  ReconfigRequestSN next_outgoing_req_seq_nbr_;

  // The current stream request operation.
  std::optional<CurrentRequest> current_request_;

  // For incoming requests - last processed request sequence number.
  UnwrappedReconfigRequestSn last_processed_req_seq_nbr_;
  // The result from last processed incoming request
  ReconfigurationResponseParameter::Result last_processed_req_result_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_SOCKET_STREAM_RESET_HANDLER_H_
