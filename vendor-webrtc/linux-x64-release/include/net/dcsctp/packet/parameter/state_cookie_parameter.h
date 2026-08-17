/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_PARAMETER_STATE_COOKIE_PARAMETER_H_
#define NET_DCSCTP_PACKET_PARAMETER_STATE_COOKIE_PARAMETER_H_
#include <stddef.h>
#include <stdint.h>

#include <cstdint>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/packet/parameter/parameter.h"
#include "net/dcsctp/packet/tlv_trait.h"

namespace dcsctp {

// https://tools.ietf.org/html/rfc4960#section-3.3.3.1
struct StateCookieParameterConfig : ParameterConfig {
  static constexpr int kType = 7;
  static constexpr size_t kHeaderSize = 4;
  static constexpr size_t kVariableLengthAlignment = 1;
};

class StateCookieParameter : public Parameter,
                             public TLVTrait<StateCookieParameterConfig> {
 public:
  static constexpr int kType = StateCookieParameterConfig::kType;

  explicit StateCookieParameter(webrtc::ArrayView<const uint8_t> data)
      : data_(data.begin(), data.end()) {}

  static std::optional<StateCookieParameter> Parse(
      webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;

  webrtc::ArrayView<const uint8_t> data() const { return data_; }

 private:
  std::vector<uint8_t> data_;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_PARAMETER_STATE_COOKIE_PARAMETER_H_
