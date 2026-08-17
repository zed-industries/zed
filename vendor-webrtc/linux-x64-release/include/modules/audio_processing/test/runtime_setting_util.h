/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_RUNTIME_SETTING_UTIL_H_
#define MODULES_AUDIO_PROCESSING_TEST_RUNTIME_SETTING_UTIL_H_

#include "api/audio/audio_processing.h"
#include "modules/audio_processing/test/protobuf_utils.h"

namespace webrtc {

void ReplayRuntimeSetting(AudioProcessing* apm,
                          const webrtc::audioproc::RuntimeSetting& setting);
}

#endif  // MODULES_AUDIO_PROCESSING_TEST_RUNTIME_SETTING_UTIL_H_
