/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_MIXER_AUDIO_FRAME_MANIPULATOR_H_
#define MODULES_AUDIO_MIXER_AUDIO_FRAME_MANIPULATOR_H_

#include <stddef.h>
#include <stdint.h>

#include "api/audio/audio_frame.h"

namespace webrtc {

// Updates the audioFrame's energy (based on its samples).
uint32_t AudioMixerCalculateEnergy(const AudioFrame& audio_frame);

// Ramps up or down the provided audio frame. Ramp(0, 1, frame) will
// linearly increase the samples in the frame from 0 to full volume.
void Ramp(float start_gain, float target_gain, AudioFrame* audio_frame);

// Downmixes or upmixes a frame between stereo and mono.
void RemixFrame(size_t target_number_of_channels, AudioFrame* frame);

}  // namespace webrtc

#endif  // MODULES_AUDIO_MIXER_AUDIO_FRAME_MANIPULATOR_H_
