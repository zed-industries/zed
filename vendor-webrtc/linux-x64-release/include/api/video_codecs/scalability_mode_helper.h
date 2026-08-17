/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_SCALABILITY_MODE_HELPER_H_
#define API_VIDEO_CODECS_SCALABILITY_MODE_HELPER_H_

#include <optional>

#include "absl/strings/string_view.h"
#include "api/video_codecs/scalability_mode.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Returns the number of spatial layers from the `scalability_mode_string`
// or nullopt if the given mode is unknown.
RTC_EXPORT std::optional<int> ScalabilityModeStringToNumSpatialLayers(
    absl::string_view scalability_mode_string);

// Returns the number of temporal layers from the `scalability_mode_string`
// or nullopt if the given mode is unknown.
RTC_EXPORT std::optional<int> ScalabilityModeStringToNumTemporalLayers(
    absl::string_view scalability_mode_string);

// Convert the `scalability_mode_string` to the scalability mode enum value
// or nullopt if the given mode is unknown.
RTC_EXPORT std::optional<ScalabilityMode> ScalabilityModeStringToEnum(
    absl::string_view scalability_mode_string);

}  // namespace webrtc

#endif  // API_VIDEO_CODECS_SCALABILITY_MODE_HELPER_H_
