/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_PARAMETER_HEARTBEAT_INFO_PARAMETER_H_
#define NET_DCSCTP_PACKET_PARAMETER_HEARTBEAT_INFO_PARAMETER_H_
#include <stddef.h>

#include <cstdint>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/packet/parameter/parameter.h"
#include "net/dcsctp/packet/tlv_trait.h"

namespace dcsctp {

// https://tools.ietf.org/html/rfc4960#section-3.3.5
struct HeartbeatInfoParameterConfig : ParameterConfig {
  static constexpr int kType = 1;
  static constexpr size_t kHeaderSize = 4;
  static constexpr size_t kVariableLengthAlignment = 1;
};

class HeartbeatInfoParameter : public Parameter,
                               public TLVTrait<HeartbeatInfoParameterConfig> {
 public:
  static constexpr int kType = HeartbeatInfoParameterConfig::kType;

  explicit HeartbeatInfoParameter(webrtc::ArrayView<const uint8_t> info)
      : info_(info.begin(), info.end()) {}

  static std::optional<HeartbeatInfoParameter> Parse(
      webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;

  webrtc::ArrayView<const uint8_t> info() const { return info_; }

 private:
  std::vector<uint8_t> info_;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_PARAMETER_HEARTBEAT_INFO_PARAMETER_H_
