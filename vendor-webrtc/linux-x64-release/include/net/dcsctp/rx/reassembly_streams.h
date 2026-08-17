/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_RX_REASSEMBLY_STREAMS_H_
#define NET_DCSCTP_RX_REASSEMBLY_STREAMS_H_

#include <stddef.h>
#include <stdint.h>

#include <functional>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/common/sequence_numbers.h"
#include "net/dcsctp/packet/chunk/forward_tsn_common.h"
#include "net/dcsctp/packet/data.h"
#include "net/dcsctp/public/dcsctp_handover_state.h"
#include "net/dcsctp/public/dcsctp_message.h"

namespace dcsctp {

// Implementations of this interface will be called when data is received, when
// data should be skipped/forgotten or when sequence number should be reset.
//
// As a result of these operations - mainly when data is received - the
// implementations of this interface should notify when a message has been
// assembled, by calling the provided callback of type `OnAssembledMessage`. How
// it assembles messages will depend on e.g. if a message was sent on an ordered
// or unordered stream.
//
// Implementations will - for each operation - indicate how much additional
// memory that has been used as a result of performing the operation. This is
// used to limit the maximum amount of memory used, to prevent out-of-memory
// situations.
class ReassemblyStreams {
 public:
  // This callback will be provided as an argument to the constructor of the
  // concrete class implementing this interface and should be called when a
  // message has been assembled as well as indicating from which TSNs this
  // message was assembled from.
  using OnAssembledMessage =
      std::function<void(webrtc::ArrayView<const UnwrappedTSN> tsns,
                         DcSctpMessage message)>;

  virtual ~ReassemblyStreams() = default;

  // Adds a data chunk to a stream as identified in `data`.
  // If it was the last remaining chunk in a message, reassemble one (or
  // several, in case of ordered chunks) messages.
  //
  // Returns the additional number of bytes added to the queue as a result of
  // performing this operation. If this addition resulted in messages being
  // assembled and delivered, this may be negative.
  virtual int Add(UnwrappedTSN tsn, Data data) = 0;

  // Called for incoming FORWARD-TSN/I-FORWARD-TSN chunks - when the sender
  // wishes the received to skip/forget about data up until the provided TSN.
  // This is used to implement partial reliability, such as limiting the number
  // of retransmissions or the an expiration duration. As a result of skipping
  // data, this may result in the implementation being able to assemble messages
  // in ordered streams.
  //
  // Returns the number of bytes removed from the queue as a result of
  // this operation.
  virtual size_t HandleForwardTsn(
      UnwrappedTSN new_cumulative_ack_tsn,
      webrtc::ArrayView<const AnyForwardTsnChunk::SkippedStream>
          skipped_streams) = 0;

  // Called for incoming (possibly deferred) RE_CONFIG chunks asking for
  // either a few streams, or all streams (when the list is empty) to be
  // reset - to have their next SSN or Message ID to be zero.
  virtual void ResetStreams(webrtc::ArrayView<const StreamID> stream_ids) = 0;

  virtual HandoverReadinessStatus GetHandoverReadiness() const = 0;
  virtual void AddHandoverState(DcSctpSocketHandoverState& state) = 0;
  virtual void RestoreFromState(const DcSctpSocketHandoverState& state) = 0;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_RX_REASSEMBLY_STREAMS_H_
