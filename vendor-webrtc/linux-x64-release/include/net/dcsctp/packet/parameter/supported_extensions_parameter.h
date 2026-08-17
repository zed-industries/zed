/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_PARAMETER_SUPPORTED_EXTENSIONS_PARAMETER_H_
#define NET_DCSCTP_PACKET_PARAMETER_SUPPORTED_EXTENSIONS_PARAMETER_H_
#include <stddef.h>

#include <algorithm>
#include <cstdint>
#include <iterator>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/packet/parameter/parameter.h"
#include "net/dcsctp/packet/tlv_trait.h"

namespace dcsctp {

// https://tools.ietf.org/html/rfc5061#section-4.2.7
struct SupportedExtensionsParameterConfig : ParameterConfig {
  static constexpr int kType = 0x8008;
  static constexpr size_t kHeaderSize = 4;
  static constexpr size_t kVariableLengthAlignment = 1;
};

class SupportedExtensionsParameter
    : public Parameter,
      public TLVTrait<SupportedExtensionsParameterConfig> {
 public:
  static constexpr int kType = SupportedExtensionsParameterConfig::kType;

  explicit SupportedExtensionsParameter(std::vector<uint8_t> chunk_types)
      : chunk_types_(std::move(chunk_types)) {}

  static std::optional<SupportedExtensionsParameter> Parse(
      webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;

  bool supports(uint8_t chunk_type) const {
    return std::find(chunk_types_.begin(), chunk_types_.end(), chunk_type) !=
           chunk_types_.end();
  }

  webrtc::ArrayView<const uint8_t> chunk_types() const { return chunk_types_; }

 private:
  std::vector<uint8_t> chunk_types_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_PARAMETER_SUPPORTED_EXTENSIONS_PARAMETER_H_
