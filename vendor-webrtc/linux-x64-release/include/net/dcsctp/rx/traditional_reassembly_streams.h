/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_RX_TRADITIONAL_REASSEMBLY_STREAMS_H_
#define NET_DCSCTP_RX_TRADITIONAL_REASSEMBLY_STREAMS_H_
#include <stddef.h>
#include <stdint.h>

#include <map>
#include <string>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/common/sequence_numbers.h"
#include "net/dcsctp/packet/chunk/forward_tsn_common.h"
#include "net/dcsctp/packet/data.h"
#include "net/dcsctp/rx/reassembly_streams.h"

namespace dcsctp {

// Handles reassembly of incoming data when interleaved message sending
// is not enabled on the association, i.e. when RFC8260 is not in use and
// RFC4960 is to be followed.
class TraditionalReassemblyStreams : public ReassemblyStreams {
 public:
  TraditionalReassemblyStreams(absl::string_view log_prefix,
                               OnAssembledMessage on_assembled_message);

  int Add(UnwrappedTSN tsn, Data data) override;

  size_t HandleForwardTsn(
      UnwrappedTSN new_cumulative_ack_tsn,
      webrtc::ArrayView<const AnyForwardTsnChunk::SkippedStream>
          skipped_streams) override;

  void ResetStreams(webrtc::ArrayView<const StreamID> stream_ids) override;

  HandoverReadinessStatus GetHandoverReadiness() const override;
  void AddHandoverState(DcSctpSocketHandoverState& state) override;
  void RestoreFromState(const DcSctpSocketHandoverState& state) override;

 private:
  using ChunkMap = std::map<UnwrappedTSN, Data>;

  // Base class for `UnorderedStream` and `OrderedStream`.
  class StreamBase {
   protected:
    explicit StreamBase(TraditionalReassemblyStreams* parent)
        : parent_(*parent) {}

    size_t AssembleMessage(ChunkMap::iterator start, ChunkMap::iterator end);
    size_t AssembleMessage(UnwrappedTSN tsn, Data data);
    TraditionalReassemblyStreams& parent_;
  };

  // Manages all received data for a specific unordered stream, and assembles
  // messages when possible.
  class UnorderedStream : StreamBase {
   public:
    explicit UnorderedStream(TraditionalReassemblyStreams* parent)
        : StreamBase(parent) {}
    int Add(UnwrappedTSN tsn, Data data);
    // Returns the number of bytes removed from the queue.
    size_t EraseTo(UnwrappedTSN tsn);
    bool has_unassembled_chunks() const { return !chunks_.empty(); }

   private:
    // Given an iterator to any chunk within the map, try to assemble a message
    // into `reassembled_messages` containing it and - if successful - erase
    // those chunks from the stream chunks map.
    //
    // Returns the number of bytes that were assembled.
    size_t TryToAssembleMessage(ChunkMap::iterator iter);

    ChunkMap chunks_;
  };

  // Manages all received data for a specific ordered stream, and assembles
  // messages when possible.
  class OrderedStream : StreamBase {
   public:
    explicit OrderedStream(TraditionalReassemblyStreams* parent,
                           SSN next_ssn = SSN(0))
        : StreamBase(parent), next_ssn_(ssn_unwrapper_.Unwrap(next_ssn)) {}
    int Add(UnwrappedTSN tsn, Data data);
    size_t EraseTo(SSN ssn);
    void Reset() {
      ssn_unwrapper_.Reset();
      next_ssn_ = ssn_unwrapper_.Unwrap(SSN(0));
    }
    SSN next_ssn() const { return next_ssn_.Wrap(); }
    bool has_unassembled_chunks() const { return !chunks_by_ssn_.empty(); }

   private:
    // Try to assemble one or several messages in order from the stream.
    // Returns the number of bytes assembled if a message was assembled.
    size_t TryToAssembleMessage();
    size_t TryToAssembleMessages();
    // Same as above but when inserting the first complete message avoid
    // insertion into the map.
    size_t TryToAssembleMessagesFastpath(UnwrappedSSN ssn,
                                         UnwrappedTSN tsn,
                                         Data data);
    // This must be an ordered container to be able to iterate in SSN order.
    std::map<UnwrappedSSN, ChunkMap> chunks_by_ssn_;
    UnwrappedSSN::Unwrapper ssn_unwrapper_;
    UnwrappedSSN next_ssn_;
  };

  const absl::string_view log_prefix_;

  // Callback for when a message has been assembled.
  const OnAssembledMessage on_assembled_message_;

  // All unordered and ordered streams, managing not-yet-assembled data.
  std::map<StreamID, UnorderedStream> unordered_streams_;
  std::map<StreamID, OrderedStream> ordered_streams_;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_RX_TRADITIONAL_REASSEMBLY_STREAMS_H_
