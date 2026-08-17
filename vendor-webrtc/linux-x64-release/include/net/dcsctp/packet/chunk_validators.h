/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_CHUNK_VALIDATORS_H_
#define NET_DCSCTP_PACKET_CHUNK_VALIDATORS_H_

#include "net/dcsctp/packet/chunk/sack_chunk.h"

namespace dcsctp {
// Validates and cleans SCTP chunks.
class ChunkValidators {
 public:
  // Given a SackChunk, will return `true` if it's valid, and `false` if not.
  static bool Validate(const SackChunk& sack);

  // Given a SackChunk, it will return a cleaned and validated variant of it.
  // RFC4960 doesn't say anything about validity of SACKs or if the Gap ACK
  // blocks must be sorted, and non-overlapping. While they always are in
  // well-behaving implementations, this can't be relied on.
  //
  // This method internally calls `Validate`, which means that you can always
  // pass a SackChunk to this method (valid or not), and use the results.
  static SackChunk Clean(SackChunk&& sack);
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_CHUNK_VALIDATORS_H_
