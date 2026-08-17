/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_RX_REASSEMBLY_QUEUE_H_
#define NET_DCSCTP_RX_REASSEMBLY_QUEUE_H_

#include <stddef.h>

#include <cstdint>
#include <memory>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/common/internal_types.h"
#include "net/dcsctp/common/sequence_numbers.h"
#include "net/dcsctp/packet/chunk/forward_tsn_common.h"
#include "net/dcsctp/packet/data.h"
#include "net/dcsctp/packet/parameter/outgoing_ssn_reset_request_parameter.h"
#include "net/dcsctp/packet/parameter/reconfiguration_response_parameter.h"
#include "net/dcsctp/public/dcsctp_message.h"
#include "net/dcsctp/rx/reassembly_streams.h"
#include "rtc_base/containers/flat_set.h"

namespace dcsctp {

// Contains the received DATA chunks that haven't yet been reassembled, and
// reassembles chunks when possible.
//
// The actual assembly is handled by an implementation of the
// `ReassemblyStreams` interface.
//
// Except for reassembling fragmented messages, this class will also handle two
// less common operations; To handle the receiver-side of partial reliability
// (limited number of retransmissions or limited message lifetime) as well as
// stream resetting, which is used when a sender wishes to close a data channel.
//
// Partial reliability is handled when a FORWARD-TSN or I-FORWARD-TSN chunk is
// received, and it will simply delete any chunks matching the parameters in
// that chunk. This is mainly implemented in ReassemblyStreams.
//
// Resetting streams is handled when a RECONFIG chunks is received, with an
// "Outgoing SSN Reset Request" parameter. That parameter will contain a list of
// streams to reset, and a `sender_last_assigned_tsn`. If this TSN is not yet
// seen, the stream cannot be directly reset, and this class will respond that
// the reset is "deferred". But if this TSN provided is known, the stream can be
// immediately be reset.
//
// The ReassemblyQueue has a maximum size, as it would otherwise be an DoS
// attack vector where a peer could consume all memory of the other peer by
// sending a lot of ordered chunks, but carefully withholding an early one. It
// also has a watermark limit, which the caller can query is the number of bytes
// is above that limit. This is used by the caller to be selective in what to
// add to the reassembly queue, so that it's not exhausted. The caller is
// expected to call `is_full` prior to adding data to the queue and to act
// accordingly if the queue is full.
class ReassemblyQueue {
 public:
  // When the queue is filled over this fraction (of its maximum size), the
  // socket should restrict incoming data to avoid filling up the queue.
  static constexpr float kHighWatermarkLimit = 0.9;

  ReassemblyQueue(absl::string_view log_prefix,
                  size_t max_size_bytes,
                  bool use_message_interleaving = false);

  // Adds a data chunk to the queue, with a `tsn` and other parameters in
  // `data`.
  void Add(TSN tsn, Data data);

  // Indicates if the reassembly queue has any reassembled messages that can be
  // retrieved by calling `FlushMessages`.
  bool HasMessages() const { return !reassembled_messages_.empty(); }

  // Returns any reassembled messages.
  std::vector<DcSctpMessage> FlushMessages();

  // Handle a ForwardTSN chunk, when the sender has indicated that the received
  // (this class) should forget about some chunks. This is used to implement
  // partial reliability.
  void HandleForwardTsn(
      TSN new_cumulative_tsn,
      webrtc::ArrayView<const AnyForwardTsnChunk::SkippedStream>
          skipped_streams);

  // Resets the provided streams and leaves deferred reset processing, if
  // enabled.
  void ResetStreamsAndLeaveDeferredReset(
      webrtc::ArrayView<const StreamID> stream_ids);

  // Enters deferred reset processing.
  void EnterDeferredReset(TSN sender_last_assigned_tsn,
                          webrtc::ArrayView<const StreamID> streams);

  // The number of payload bytes that have been queued. Note that the actual
  // memory usage is higher due to additional overhead of tracking received
  // data.
  size_t queued_bytes() const { return queued_bytes_; }

  // The remaining bytes until the queue has reached the watermark limit.
  size_t remaining_bytes() const { return watermark_bytes_ - queued_bytes_; }

  // Indicates if the queue is full. Data should not be added to the queue when
  // it's full.
  bool is_full() const { return queued_bytes_ >= max_size_bytes_; }

  // Indicates if the queue is above the watermark limit, which is a certain
  // percentage of its size.
  bool is_above_watermark() const { return queued_bytes_ >= watermark_bytes_; }

  // Returns the watermark limit, in bytes.
  size_t watermark_bytes() const { return watermark_bytes_; }

  HandoverReadinessStatus GetHandoverReadiness() const;

  void AddHandoverState(DcSctpSocketHandoverState& state);
  void RestoreFromState(const DcSctpSocketHandoverState& state);

 private:
  struct DeferredResetStreams {
    DeferredResetStreams(UnwrappedTSN sender_last_assigned_tsn,
                         webrtc::flat_set<StreamID> streams)
        : sender_last_assigned_tsn(sender_last_assigned_tsn),
          streams(std::move(streams)) {}

    UnwrappedTSN sender_last_assigned_tsn;
    webrtc::flat_set<StreamID> streams;
    std::vector<absl::AnyInvocable<void(void)>> deferred_actions;
  };

  bool IsConsistent() const;
  void AddReassembledMessage(webrtc::ArrayView<const UnwrappedTSN> tsns,
                             DcSctpMessage message);

  const absl::string_view log_prefix_;
  const size_t max_size_bytes_;
  const size_t watermark_bytes_;
  UnwrappedTSN::Unwrapper tsn_unwrapper_;

  // Messages that have been reassembled, and will be returned by
  // `FlushMessages`.
  std::vector<DcSctpMessage> reassembled_messages_;

  // If present, "deferred reset processing" mode is active.
  std::optional<DeferredResetStreams> deferred_reset_streams_;

  // The number of "payload bytes" that are in this queue, in total.
  size_t queued_bytes_ = 0;

  // The actual implementation of ReassemblyStreams.
  std::unique_ptr<ReassemblyStreams> streams_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_RX_REASSEMBLY_QUEUE_H_
