/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_SVC_SCALABILITY_MODE_UTIL_H_
#define MODULES_VIDEO_CODING_SVC_SCALABILITY_MODE_UTIL_H_

#include <optional>

#include "absl/strings/string_view.h"
#include "api/video_codecs/scalability_mode.h"
#include "api/video_codecs/video_codec.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

enum class ScalabilityModeResolutionRatio {
  kTwoToOne,    // The resolution ratio between spatial layers is 2:1.
  kThreeToTwo,  // The resolution ratio between spatial layers is 1.5:1.
};

static constexpr char kDefaultScalabilityModeStr[] = "L1T2";

// Scalability mode to be used if falling back to default scalability mode is
// unsupported.
static constexpr char kNoLayeringScalabilityModeStr[] = "L1T1";

RTC_EXPORT std::optional<ScalabilityMode> MakeScalabilityMode(
    int num_spatial_layers,
    int num_temporal_layers,
    InterLayerPredMode inter_layer_pred,
    std::optional<ScalabilityModeResolutionRatio> ratio,
    bool shift);

RTC_EXPORT std::optional<ScalabilityMode> ScalabilityModeFromString(
    absl::string_view scalability_mode_string);

InterLayerPredMode ScalabilityModeToInterLayerPredMode(
    ScalabilityMode scalability_mode);

int ScalabilityModeToNumSpatialLayers(ScalabilityMode scalability_mode);

int ScalabilityModeToNumTemporalLayers(ScalabilityMode scalability_mode);

std::optional<ScalabilityModeResolutionRatio> ScalabilityModeToResolutionRatio(
    ScalabilityMode scalability_mode);

bool ScalabilityModeIsShiftMode(ScalabilityMode scalability_mode);

ScalabilityMode LimitNumSpatialLayers(ScalabilityMode scalability_mode,
                                      int max_spatial_layers);

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_SVC_SCALABILITY_MODE_UTIL_H_
