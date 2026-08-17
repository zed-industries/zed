/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_DATA_H_
#define NET_DCSCTP_PACKET_DATA_H_

#include <cstdint>
#include <utility>
#include <vector>

#include "net/dcsctp/common/internal_types.h"
#include "net/dcsctp/public/types.h"

namespace dcsctp {

// Represents data that is either received and extracted from a DATA/I-DATA
// chunk, or data that is supposed to be sent, and wrapped in a DATA/I-DATA
// chunk (depending on peer capabilities).
//
// The data wrapped in this structure is actually the same as the DATA/I-DATA
// chunk (actually the union of them), but to avoid having all components be
// aware of the implementation details of the different chunks, this abstraction
// is used instead. A notable difference is also that it doesn't carry a
// Transmission Sequence Number (TSN), as that is not known when a chunk is
// created (assigned late, just when sending), and that the TSNs in DATA/I-DATA
// are wrapped numbers, and within the library, unwrapped sequence numbers are
// preferably used.
struct Data {
  // Indicates if a chunk is the first in a fragmented message and maps to the
  // "beginning" flag in DATA/I-DATA chunk.
  using IsBeginning = webrtc::StrongAlias<class IsBeginningTag, bool>;

  // Indicates if a chunk is the last in a fragmented message  and maps to the
  // "end" flag in DATA/I-DATA chunk.
  using IsEnd = webrtc::StrongAlias<class IsEndTag, bool>;

  Data(StreamID stream_id,
       SSN ssn,
       MID mid,
       FSN fsn,
       PPID ppid,
       std::vector<uint8_t> payload,
       IsBeginning is_beginning,
       IsEnd is_end,
       IsUnordered is_unordered)
      : stream_id(stream_id),
        ssn(ssn),
        mid(mid),
        fsn(fsn),
        ppid(ppid),
        payload(std::move(payload)),
        is_beginning(is_beginning),
        is_end(is_end),
        is_unordered(is_unordered) {}

  // Move-only, to avoid accidental copies.
  Data(Data&& other) = default;
  Data& operator=(Data&& other) = default;

  // Creates a copy of this `Data` object.
  Data Clone() const {
    return Data(stream_id, ssn, mid, fsn, ppid, payload, is_beginning, is_end,
                is_unordered);
  }

  // The size of this data, which translates to the size of its payload.
  size_t size() const { return payload.size(); }

  // Stream Identifier.
  StreamID stream_id;

  // Stream Sequence Number (SSN), per stream, for ordered chunks. Defined by
  // RFC4960 and used only in DATA chunks (not I-DATA).
  SSN ssn;

  // Message Identifier (MID) per stream and ordered/unordered. Defined by
  // RFC8260, and used together with options.is_unordered and stream_id to
  // uniquely identify a message. Used only in I-DATA chunks (not DATA).
  MID mid;
  // Fragment Sequence Number (FSN) per stream and ordered/unordered, as above.
  FSN fsn;

  // Payload Protocol Identifier (PPID).
  PPID ppid;

  // The actual data payload.
  std::vector<uint8_t> payload;

  // If this data represents the first, last or a middle chunk.
  IsBeginning is_beginning;
  IsEnd is_end;
  // If this data is sent/received unordered.
  IsUnordered is_unordered;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_DATA_H_
