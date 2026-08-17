/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_SCALABILITY_MODE_H_
#define API_VIDEO_CODECS_SCALABILITY_MODE_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>

#include "absl/strings/string_view.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Supported scalability modes. Most applications should use the
// PeerConnection-level apis where scalability mode is represented as a string.
// This list of currently recognized modes is intended for the api boundary
// between webrtc and injected encoders. Any application usage outside of
// injected encoders is strongly discouraged.
enum class ScalabilityMode : uint8_t {
  kL1T1,
  kL1T2,
  kL1T3,
  kL2T1,
  kL2T1h,
  kL2T1_KEY,
  kL2T2,
  kL2T2h,
  kL2T2_KEY,
  kL2T2_KEY_SHIFT,
  kL2T3,
  kL2T3h,
  kL2T3_KEY,
  kL3T1,
  kL3T1h,
  kL3T1_KEY,
  kL3T2,
  kL3T2h,
  kL3T2_KEY,
  kL3T3,
  kL3T3h,
  kL3T3_KEY,
  kS2T1,
  kS2T1h,
  kS2T2,
  kS2T2h,
  kS2T3,
  kS2T3h,
  kS3T1,
  kS3T1h,
  kS3T2,
  kS3T2h,
  kS3T3,
  kS3T3h,
};

inline constexpr ScalabilityMode kAllScalabilityModes[] = {
    // clang-format off
    ScalabilityMode::kL1T1,
    ScalabilityMode::kL1T2,
    ScalabilityMode::kL1T3,
    ScalabilityMode::kL2T1,
    ScalabilityMode::kL2T1h,
    ScalabilityMode::kL2T1_KEY,
    ScalabilityMode::kL2T2,
    ScalabilityMode::kL2T2h,
    ScalabilityMode::kL2T2_KEY,
    ScalabilityMode::kL2T2_KEY_SHIFT,
    ScalabilityMode::kL2T3,
    ScalabilityMode::kL2T3h,
    ScalabilityMode::kL2T3_KEY,
    ScalabilityMode::kL3T1,
    ScalabilityMode::kL3T1h,
    ScalabilityMode::kL3T1_KEY,
    ScalabilityMode::kL3T2,
    ScalabilityMode::kL3T2h,
    ScalabilityMode::kL3T2_KEY,
    ScalabilityMode::kL3T3,
    ScalabilityMode::kL3T3h,
    ScalabilityMode::kL3T3_KEY,
    ScalabilityMode::kS2T1,
    ScalabilityMode::kS2T1h,
    ScalabilityMode::kS2T2,
    ScalabilityMode::kS2T2h,
    ScalabilityMode::kS2T3,
    ScalabilityMode::kS2T3h,
    ScalabilityMode::kS3T1,
    ScalabilityMode::kS3T1h,
    ScalabilityMode::kS3T2,
    ScalabilityMode::kS3T2h,
    ScalabilityMode::kS3T3,
    ScalabilityMode::kS3T3h,
    // clang-format on
};

inline constexpr size_t kScalabilityModeCount =
    sizeof(kAllScalabilityModes) / sizeof(ScalabilityMode);

RTC_EXPORT
absl::string_view ScalabilityModeToString(ScalabilityMode scalability_mode);
RTC_EXPORT
absl::string_view ScalabilityModeToString(
    std::optional<ScalabilityMode> scalability_mode);

}  // namespace webrtc

#endif  // API_VIDEO_CODECS_SCALABILITY_MODE_H_
