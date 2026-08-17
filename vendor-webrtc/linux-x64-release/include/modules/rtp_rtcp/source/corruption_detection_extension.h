/*
 * Copyright 2024 The WebRTC project authors. All rights reserved.
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the LICENSE file in the root of the source
 * tree. An additional intellectual property rights grant can be found
 * in the file PATENTS.  All contributing project authors may
 * be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_CORRUPTION_DETECTION_EXTENSION_H_
#define MODULES_RTP_RTCP_SOURCE_CORRUPTION_DETECTION_EXTENSION_H_

#include <cstddef>
#include <cstdint>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/rtp_parameters.h"
#include "api/transport/rtp/corruption_detection_message.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"

namespace webrtc {

// RTP Corruption Detection Header Extension.
//
// The class reads and writes the corruption detection RTP header extension.
// The class implements traits so that the class is compatible with being an
// argument to the templated `RtpPacket::GetExtension` and
// `RtpPacketToSend::SetExtension` methods.
class CorruptionDetectionExtension {
 public:
  using value_type = CorruptionDetectionMessage;

  static constexpr RTPExtensionType kId = kRtpExtensionCorruptionDetection;
  static constexpr uint8_t kMaxValueSizeBytes = 16;

  static constexpr absl::string_view Uri() {
    return RtpExtension::kCorruptionDetectionUri;
  }
  static bool Parse(ArrayView<const uint8_t> data,
                    CorruptionDetectionMessage* message);
  static bool Write(ArrayView<uint8_t> data,
                    const CorruptionDetectionMessage& message);
  // Size of the header extension in bytes.
  static size_t ValueSize(const CorruptionDetectionMessage& message);
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_CORRUPTION_DETECTION_EXTENSION_H_
