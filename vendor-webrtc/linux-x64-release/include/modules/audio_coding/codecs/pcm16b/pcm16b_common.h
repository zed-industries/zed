/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_PCM16B_PCM16B_COMMON_H_
#define MODULES_AUDIO_CODING_CODECS_PCM16B_PCM16B_COMMON_H_

#include <vector>

#include "api/audio_codecs/audio_format.h"

namespace webrtc {
void Pcm16BAppendSupportedCodecSpecs(std::vector<AudioCodecSpec>* specs);
}

#endif  // MODULES_AUDIO_CODING_CODECS_PCM16B_PCM16B_COMMON_H_
