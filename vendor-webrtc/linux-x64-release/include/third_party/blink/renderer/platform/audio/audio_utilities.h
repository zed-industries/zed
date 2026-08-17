/*
 * Copyright (C) 2010, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_UTILITIES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_UTILITIES_H_

#include <cstddef>

#include "base/time/time.h"
#include "third_party/blink/public/common/mediastream/media_devices.h"
#include "third_party/blink/public/platform/web_audio_latency_hint.h"
#include "third_party/blink/public/platform/web_audio_sink_descriptor.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink::audio_utilities {

// How to do rounding when converting time to sample frame.
enum SampleFrameRounding {
  // Round to nearest integer
  kRoundToNearest,
  // Round down
  kRoundDown,
  // Round up
  kRoundUp
};

// Standard functions for converting to and from decibel values from linear.
PLATFORM_EXPORT float LinearToDecibels(float);
PLATFORM_EXPORT float DecibelsToLinear(float);

// timeConstant is the time it takes a first-order linear time-invariant system
// to reach the value 1 - 1/e (around 63.2%) given a step input response.
// discreteTimeConstantForSampleRate() will return the discrete time-constant
// for the specific sampleRate.
PLATFORM_EXPORT double DiscreteTimeConstantForSampleRate(double time_constant,
                                                         double sample_rate);

// Convert the time to a sample frame at the given sample rate.
PLATFORM_EXPORT size_t
TimeToSampleFrame(double time,
                  double sample_rate,
                  enum SampleFrameRounding rounding = kRoundToNearest);

// Calculate a buffer duration given the number of frames and a sample rate.
// The only reason we have it here is because it takes sample_rate as float.
// Otherwise, media::AudioTimestampHelper::FramesToTime would be just fine.
PLATFORM_EXPORT
base::TimeDelta FramesToTime(int64_t frames, float sample_rate);

// Check that |sampleRate| is a valid rate for AudioBuffers.
PLATFORM_EXPORT bool IsValidAudioBufferSampleRate(float sample_rate);

// Return max/min sample rate supported by AudioBuffers.
PLATFORM_EXPORT float MinAudioBufferSampleRate();
PLATFORM_EXPORT float MaxAudioBufferSampleRate();

PLATFORM_EXPORT const std::string GetSinkIdForTracing(
    blink::WebAudioSinkDescriptor sink_descriptor);

PLATFORM_EXPORT const std::string GetSinkInfoForTracing(
    blink::WebAudioSinkDescriptor sink_descriptor,
    blink::WebAudioLatencyHint latency_hint,
    int channel_count,
    float sample_rate,
    int buffer_size);

PLATFORM_EXPORT const std::string GetDeviceEnumerationForTracing(
    const Vector<WebMediaDeviceInfo>& device_infos);

}  // namespace blink::audio_utilities

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_UTILITIES_H_
