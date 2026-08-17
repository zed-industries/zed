/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_ERROR_CAUSE_INVALID_STREAM_IDENTIFIER_CAUSE_H_
#define NET_DCSCTP_PACKET_ERROR_CAUSE_INVALID_STREAM_IDENTIFIER_CAUSE_H_
#include <stddef.h>
#include <stdint.h>

#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/packet/error_cause/error_cause.h"
#include "net/dcsctp/packet/tlv_trait.h"
#include "net/dcsctp/public/types.h"

namespace dcsctp {

// https://tools.ietf.org/html/rfc4960#section-3.3.10.1
struct InvalidStreamIdentifierCauseConfig : public ParameterConfig {
  static constexpr int kType = 1;
  static constexpr size_t kHeaderSize = 8;
  static constexpr size_t kVariableLengthAlignment = 0;
};

class InvalidStreamIdentifierCause
    : public Parameter,
      public TLVTrait<InvalidStreamIdentifierCauseConfig> {
 public:
  static constexpr int kType = InvalidStreamIdentifierCauseConfig::kType;

  explicit InvalidStreamIdentifierCause(StreamID stream_id)
      : stream_id_(stream_id) {}

  static std::optional<InvalidStreamIdentifierCause> Parse(
      webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;

  StreamID stream_id() const { return stream_id_; }

 private:
  StreamID stream_id_;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_ERROR_CAUSE_INVALID_STREAM_IDENTIFIER_CAUSE_H_
