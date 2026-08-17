/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_PARAMETER_RECONFIGURATION_RESPONSE_PARAMETER_H_
#define NET_DCSCTP_PACKET_PARAMETER_RECONFIGURATION_RESPONSE_PARAMETER_H_
#include <stddef.h>

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/common/internal_types.h"
#include "net/dcsctp/packet/parameter/parameter.h"
#include "net/dcsctp/packet/tlv_trait.h"

namespace dcsctp {

// https://tools.ietf.org/html/rfc6525#section-4.4
struct ReconfigurationResponseParameterConfig : ParameterConfig {
  static constexpr int kType = 16;
  static constexpr size_t kHeaderSize = 12;
  static constexpr size_t kVariableLengthAlignment = 4;
};

class ReconfigurationResponseParameter
    : public Parameter,
      public TLVTrait<ReconfigurationResponseParameterConfig> {
 public:
  static constexpr int kType = ReconfigurationResponseParameterConfig::kType;

  enum class Result {
    kSuccessNothingToDo = 0,
    kSuccessPerformed = 1,
    kDenied = 2,
    kErrorWrongSSN = 3,
    kErrorRequestAlreadyInProgress = 4,
    kErrorBadSequenceNumber = 5,
    kInProgress = 6,
  };

  ReconfigurationResponseParameter(ReconfigRequestSN response_sequence_number,
                                   Result result)
      : response_sequence_number_(response_sequence_number),
        result_(result),
        sender_next_tsn_(std::nullopt),
        receiver_next_tsn_(std::nullopt) {}

  explicit ReconfigurationResponseParameter(
      ReconfigRequestSN response_sequence_number,
      Result result,
      TSN sender_next_tsn,
      TSN receiver_next_tsn)
      : response_sequence_number_(response_sequence_number),
        result_(result),
        sender_next_tsn_(sender_next_tsn),
        receiver_next_tsn_(receiver_next_tsn) {}

  static std::optional<ReconfigurationResponseParameter> Parse(
      webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;

  ReconfigRequestSN response_sequence_number() const {
    return response_sequence_number_;
  }
  Result result() const { return result_; }
  std::optional<TSN> sender_next_tsn() const { return sender_next_tsn_; }
  std::optional<TSN> receiver_next_tsn() const { return receiver_next_tsn_; }

 private:
  static constexpr size_t kNextTsnHeaderSize = 8;
  ReconfigRequestSN response_sequence_number_;
  Result result_;
  std::optional<TSN> sender_next_tsn_;
  std::optional<TSN> receiver_next_tsn_;
};

absl::string_view ToString(ReconfigurationResponseParameter::Result result);

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_PARAMETER_RECONFIGURATION_RESPONSE_PARAMETER_H_
