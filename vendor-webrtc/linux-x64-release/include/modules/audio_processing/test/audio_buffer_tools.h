/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_AUDIO_BUFFER_TOOLS_H_
#define MODULES_AUDIO_PROCESSING_TEST_AUDIO_BUFFER_TOOLS_H_

#include <vector>

#include "api/array_view.h"
#include "api/audio/audio_processing.h"
#include "modules/audio_processing/audio_buffer.h"

namespace webrtc {
namespace test {

// Copies a vector into an audiobuffer.
void CopyVectorToAudioBuffer(const StreamConfig& stream_config,
                             ArrayView<const float> source,
                             AudioBuffer* destination);

// Extracts a vector from an audiobuffer.
void ExtractVectorFromAudioBuffer(const StreamConfig& stream_config,
                                  AudioBuffer* source,
                                  std::vector<float>* destination);

// Sets all values in `audio_buffer` to `value`.
void FillBuffer(float value, AudioBuffer& audio_buffer);

// Sets all values channel `channel` for `audio_buffer` to `value`.
void FillBufferChannel(float value, int channel, AudioBuffer& audio_buffer);

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_AUDIO_BUFFER_TOOLS_H_
