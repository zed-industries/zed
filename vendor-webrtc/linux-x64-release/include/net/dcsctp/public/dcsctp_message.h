/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PUBLIC_DCSCTP_MESSAGE_H_
#define NET_DCSCTP_PUBLIC_DCSCTP_MESSAGE_H_

#include <cstdint>
#include <utility>
#include <vector>

#include "api/array_view.h"
#include "net/dcsctp/public/types.h"

namespace dcsctp {

// An SCTP message is a group of bytes sent and received as a whole on a
// specified stream identifier (`stream_id`), and with a payload protocol
// identifier (`ppid`).
class DcSctpMessage {
 public:
  DcSctpMessage(StreamID stream_id, PPID ppid, std::vector<uint8_t> payload)
      : stream_id_(stream_id), ppid_(ppid), payload_(std::move(payload)) {}

  DcSctpMessage(DcSctpMessage&& other) = default;
  DcSctpMessage& operator=(DcSctpMessage&& other) = default;
  DcSctpMessage(const DcSctpMessage&) = delete;
  DcSctpMessage& operator=(const DcSctpMessage&) = delete;

  // The stream identifier to which the message is sent.
  StreamID stream_id() const { return stream_id_; }

  // The payload protocol identifier (ppid) associated with the message.
  PPID ppid() const { return ppid_; }

  // The payload of the message.
  webrtc::ArrayView<const uint8_t> payload() const { return payload_; }

  // When destructing the message, extracts the payload.
  std::vector<uint8_t> ReleasePayload() && { return std::move(payload_); }

 private:
  StreamID stream_id_;
  PPID ppid_;
  std::vector<uint8_t> payload_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_PUBLIC_DCSCTP_MESSAGE_H_
