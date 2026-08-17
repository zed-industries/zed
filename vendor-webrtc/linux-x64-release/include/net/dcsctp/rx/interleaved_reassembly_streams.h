/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_RX_INTERLEAVED_REASSEMBLY_STREAMS_H_
#define NET_DCSCTP_RX_INTERLEAVED_REASSEMBLY_STREAMS_H_

#include <cstdint>
#include <map>
#include <string>
#include <utility>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/common/sequence_numbers.h"
#include "net/dcsctp/packet/chunk/forward_tsn_common.h"
#include "net/dcsctp/packet/data.h"
#include "net/dcsctp/rx/reassembly_streams.h"

namespace dcsctp {

// Handles reassembly of incoming data when interleaved message sending is
// enabled on the association, i.e. when RFC8260 is in use.
class InterleavedReassemblyStreams : public ReassemblyStreams {
 public:
  InterleavedReassemblyStreams(absl::string_view log_prefix,
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
  struct FullStreamId {
    const IsUnordered unordered;
    const StreamID stream_id;

    FullStreamId(IsUnordered unordered, StreamID stream_id)
        : unordered(unordered), stream_id(stream_id) {}

    friend bool operator<(FullStreamId a, FullStreamId b) {
      return a.unordered < b.unordered ||
             (!(a.unordered < b.unordered) && (a.stream_id < b.stream_id));
    }
  };

  class Stream {
   public:
    Stream(FullStreamId stream_id,
           InterleavedReassemblyStreams* parent,
           MID next_mid = MID(0))
        : stream_id_(stream_id),
          parent_(*parent),
          next_mid_(mid_unwrapper_.Unwrap(next_mid)) {}
    int Add(UnwrappedTSN tsn, Data data);
    size_t EraseTo(MID mid);
    void Reset() {
      mid_unwrapper_.Reset();
      next_mid_ = mid_unwrapper_.Unwrap(MID(0));
    }
    bool has_unassembled_chunks() const { return !chunks_by_mid_.empty(); }
    void AddHandoverState(DcSctpSocketHandoverState& state) const;

   private:
    using ChunkMap = std::map<FSN, std::pair<UnwrappedTSN, Data>>;

    // Try to assemble one message identified by `mid`.
    // Returns the number of bytes assembled if a message was assembled.
    size_t TryToAssembleMessage(UnwrappedMID mid);
    size_t AssembleMessage(ChunkMap& tsn_chunks);
    size_t AssembleMessage(UnwrappedTSN tsn, Data data);

    // Try to assemble one or several messages in order from the stream.
    // Returns the number of bytes assembled if one or more messages were
    // assembled.
    size_t TryToAssembleMessages();

    const FullStreamId stream_id_;
    InterleavedReassemblyStreams& parent_;
    std::map<UnwrappedMID, ChunkMap> chunks_by_mid_;
    UnwrappedMID::Unwrapper mid_unwrapper_;
    UnwrappedMID next_mid_;
  };

  Stream& GetOrCreateStream(const FullStreamId& stream_id);

  const absl::string_view log_prefix_;

  // Callback for when a message has been assembled.
  const OnAssembledMessage on_assembled_message_;

  // All unordered and ordered streams, managing not-yet-assembled data.
  std::map<FullStreamId, Stream> streams_;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_RX_INTERLEAVED_REASSEMBLY_STREAMS_H_
