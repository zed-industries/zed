/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_ERROR_CAUSE_MISSING_MANDATORY_PARAMETER_CAUSE_H_
#define NET_DCSCTP_PACKET_ERROR_CAUSE_MISSING_MANDATORY_PARAMETER_CAUSE_H_
#include <stddef.h>

#include <cstdint>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/packet/error_cause/error_cause.h"
#include "net/dcsctp/packet/tlv_trait.h"

namespace dcsctp {

// https://tools.ietf.org/html/rfc4960#section-3.3.10.2
struct MissingMandatoryParameterCauseConfig : public ParameterConfig {
  static constexpr int kType = 2;
  static constexpr size_t kHeaderSize = 8;
  static constexpr size_t kVariableLengthAlignment = 2;
};

class MissingMandatoryParameterCause
    : public Parameter,
      public TLVTrait<MissingMandatoryParameterCauseConfig> {
 public:
  static constexpr int kType = MissingMandatoryParameterCauseConfig::kType;

  explicit MissingMandatoryParameterCause(
      webrtc::ArrayView<const uint16_t> missing_parameter_types)
      : missing_parameter_types_(missing_parameter_types.begin(),
                                 missing_parameter_types.end()) {}

  static std::optional<MissingMandatoryParameterCause> Parse(
      webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;

  webrtc::ArrayView<const uint16_t> missing_parameter_types() const {
    return missing_parameter_types_;
  }

 private:
  static constexpr size_t kMissingParameterSize = 2;
  std::vector<uint16_t> missing_parameter_types_;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_ERROR_CAUSE_MISSING_MANDATORY_PARAMETER_CAUSE_H_
