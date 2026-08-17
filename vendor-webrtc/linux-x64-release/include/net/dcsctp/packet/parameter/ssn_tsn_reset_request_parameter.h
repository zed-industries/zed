/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_PARAMETER_SSN_TSN_RESET_REQUEST_PARAMETER_H_
#define NET_DCSCTP_PACKET_PARAMETER_SSN_TSN_RESET_REQUEST_PARAMETER_H_
#include <stddef.h>
#include <stdint.h>

#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/common/internal_types.h"
#include "net/dcsctp/packet/parameter/parameter.h"
#include "net/dcsctp/packet/tlv_trait.h"

namespace dcsctp {

// https://tools.ietf.org/html/rfc6525#section-4.3
struct SSNTSNResetRequestParameterConfig : ParameterConfig {
  static constexpr int kType = 15;
  static constexpr size_t kHeaderSize = 8;
  static constexpr size_t kVariableLengthAlignment = 0;
};

class SSNTSNResetRequestParameter
    : public Parameter,
      public TLVTrait<SSNTSNResetRequestParameterConfig> {
 public:
  static constexpr int kType = SSNTSNResetRequestParameterConfig::kType;

  explicit SSNTSNResetRequestParameter(
      ReconfigRequestSN request_sequence_number)
      : request_sequence_number_(request_sequence_number) {}

  static std::optional<SSNTSNResetRequestParameter> Parse(
      webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;

  ReconfigRequestSN request_sequence_number() const {
    return request_sequence_number_;
  }

 private:
  ReconfigRequestSN request_sequence_number_;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_PARAMETER_SSN_TSN_RESET_REQUEST_PARAMETER_H_
