/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_VIDEO_CODING_SVC_CREATE_SCALABILITY_STRUCTURE_H_
#define MODULES_VIDEO_CODING_SVC_CREATE_SCALABILITY_STRUCTURE_H_

#include <memory>
#include <optional>

#include "api/video_codecs/scalability_mode.h"
#include "modules/video_coding/svc/scalable_video_controller.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Creates a structure by name according to
// https://w3c.github.io/webrtc-svc/#scalabilitymodes*
// Returns nullptr for unknown name.
std::unique_ptr<ScalableVideoController> RTC_EXPORT
CreateScalabilityStructure(ScalabilityMode name);

// Returns description of the scalability structure identified by 'name',
// Return nullopt for unknown name.
std::optional<ScalableVideoController::StreamLayersConfig>
ScalabilityStructureConfig(ScalabilityMode name);

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_SVC_CREATE_SCALABILITY_STRUCTURE_H_
