/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_PARAMETER_OUTGOING_SSN_RESET_REQUEST_PARAMETER_H_
#define NET_DCSCTP_PACKET_PARAMETER_OUTGOING_SSN_RESET_REQUEST_PARAMETER_H_
#include <stddef.h>
#include <stdint.h>

#include <cstdint>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/common/internal_types.h"
#include "net/dcsctp/packet/parameter/parameter.h"
#include "net/dcsctp/packet/tlv_trait.h"
#include "net/dcsctp/public/types.h"

namespace dcsctp {

// https://tools.ietf.org/html/rfc6525#section-4.1
struct OutgoingSSNResetRequestParameterConfig : ParameterConfig {
  static constexpr int kType = 13;
  static constexpr size_t kHeaderSize = 16;
  static constexpr size_t kVariableLengthAlignment = 2;
};

class OutgoingSSNResetRequestParameter
    : public Parameter,
      public TLVTrait<OutgoingSSNResetRequestParameterConfig> {
 public:
  static constexpr int kType = OutgoingSSNResetRequestParameterConfig::kType;

  explicit OutgoingSSNResetRequestParameter(
      ReconfigRequestSN request_sequence_number,
      ReconfigRequestSN response_sequence_number,
      TSN sender_last_assigned_tsn,
      std::vector<StreamID> stream_ids)
      : request_sequence_number_(request_sequence_number),
        response_sequence_number_(response_sequence_number),
        sender_last_assigned_tsn_(sender_last_assigned_tsn),
        stream_ids_(std::move(stream_ids)) {}

  static std::optional<OutgoingSSNResetRequestParameter> Parse(
      webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;

  ReconfigRequestSN request_sequence_number() const {
    return request_sequence_number_;
  }
  ReconfigRequestSN response_sequence_number() const {
    return response_sequence_number_;
  }
  TSN sender_last_assigned_tsn() const { return sender_last_assigned_tsn_; }
  webrtc::ArrayView<const StreamID> stream_ids() const { return stream_ids_; }

 private:
  static constexpr size_t kStreamIdSize = sizeof(uint16_t);

  ReconfigRequestSN request_sequence_number_;
  ReconfigRequestSN response_sequence_number_;
  TSN sender_last_assigned_tsn_;
  std::vector<StreamID> stream_ids_;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_PARAMETER_OUTGOING_SSN_RESET_REQUEST_PARAMETER_H_
