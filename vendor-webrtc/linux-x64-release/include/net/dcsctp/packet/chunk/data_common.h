/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_CHUNK_DATA_COMMON_H_
#define NET_DCSCTP_PACKET_CHUNK_DATA_COMMON_H_
#include <stdint.h>

#include <utility>
#include <vector>

#include "api/array_view.h"
#include "net/dcsctp/packet/chunk/chunk.h"
#include "net/dcsctp/packet/data.h"

namespace dcsctp {

// Base class for DataChunk and IDataChunk
class AnyDataChunk : public Chunk {
 public:
  // Represents the "immediate ack" flag on DATA/I-DATA, from RFC7053.
  using ImmediateAckFlag = webrtc::StrongAlias<class ImmediateAckFlagTag, bool>;

  // Data chunk options.
  // See https://tools.ietf.org/html/rfc4960#section-3.3.1
  struct Options {
    Data::IsEnd is_end = Data::IsEnd(false);
    Data::IsBeginning is_beginning = Data::IsBeginning(false);
    IsUnordered is_unordered = IsUnordered(false);
    ImmediateAckFlag immediate_ack = ImmediateAckFlag(false);
  };

  TSN tsn() const { return tsn_; }

  Options options() const {
    Options options;
    options.is_end = data_.is_end;
    options.is_beginning = data_.is_beginning;
    options.is_unordered = data_.is_unordered;
    options.immediate_ack = immediate_ack_;
    return options;
  }

  StreamID stream_id() const { return data_.stream_id; }
  SSN ssn() const { return data_.ssn; }
  MID mid() const { return data_.mid; }
  FSN fsn() const { return data_.fsn; }
  PPID ppid() const { return data_.ppid; }
  webrtc::ArrayView<const uint8_t> payload() const { return data_.payload; }

  // Extracts the Data from the chunk, as a destructive action.
  Data extract() && { return std::move(data_); }

  AnyDataChunk(TSN tsn,
               StreamID stream_id,
               SSN ssn,
               MID mid,
               FSN fsn,
               PPID ppid,
               std::vector<uint8_t> payload,
               const Options& options)
      : tsn_(tsn),
        data_(stream_id,
              ssn,
              mid,
              fsn,
              ppid,
              std::move(payload),
              options.is_beginning,
              options.is_end,
              options.is_unordered),
        immediate_ack_(options.immediate_ack) {}

  AnyDataChunk(TSN tsn, Data data, bool immediate_ack)
      : tsn_(tsn), data_(std::move(data)), immediate_ack_(immediate_ack) {}

 protected:
  // Bits in `flags` header field.
  static constexpr int kFlagsBitEnd = 0;
  static constexpr int kFlagsBitBeginning = 1;
  static constexpr int kFlagsBitUnordered = 2;
  static constexpr int kFlagsBitImmediateAck = 3;

 private:
  TSN tsn_;
  Data data_;
  ImmediateAckFlag immediate_ack_;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_CHUNK_DATA_COMMON_H_
