/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_DYNAMICS_COMPRESSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_DYNAMICS_COMPRESSOR_H_

#include <array>
#include <memory>

#include "third_party/blink/renderer/platform/audio/audio_array.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class AudioBus;

// DynamicsCompressor implements a flexible audio dynamics compression effect
// such as is commonly used in musical production and game audio. It lowers the
// volume of the loudest parts of the signal and raises the volume of the
// softest parts, making the sound richer, fuller, and more controlled.

class PLATFORM_EXPORT DynamicsCompressor final {
  USING_FAST_MALLOC(DynamicsCompressor);

 public:
  enum {
    kParamThreshold,
    kParamKnee,
    kParamRatio,
    kParamAttack,
    kParamRelease,
    kParamReduction,
    kParamLast
  };

  DynamicsCompressor(float sample_rate, unsigned number_of_channels);
  DynamicsCompressor(const DynamicsCompressor&) = delete;
  DynamicsCompressor& operator=(const DynamicsCompressor&) = delete;

  // Performs stereo-linked compression.
  void Process(const AudioBus* source_bus,
               AudioBus* destination_bus,
               unsigned frames_to_process);
  void Reset();
  void SetNumberOfChannels(unsigned);

  void SetParameterValue(unsigned parameter_id, float value);
  float ParameterValue(unsigned parameter_id) const;

  float SampleRate() const;
  float Nyquist() const;

  double TailTime() const;
  double LatencyTime() const;

  bool RequiresTailProcessing() const;

 protected:
  void InitializeParameters();

  void SetPreDelayTime(float);

  // Static compression curve.
  float KneeCurve(float x, float k) const;
  float Saturate(float x, float k) const;
  float KAtSlope(float desired_slope) const;

  float UpdateStaticCurveParameters(float db_threshold,
                                    float db_knee,
                                    float ratio);

  unsigned number_of_channels_;

  // parameters_ holds the tweakable compressor parameters.
  std::array<float, kParamLast> parameters_;

  float sample_rate_;

  std::unique_ptr<const float*[]> source_channels_;
  std::unique_ptr<float*[]> destination_channels_;

  float detector_average_;
  float compressor_gain_;

  // Metering
  float metering_release_k_;
  float metering_gain_;

  // Lookahead section.
  enum { kMaxPreDelayFrames = 1024 };
  enum { kMaxPreDelayFramesMask = kMaxPreDelayFrames - 1 };
  enum {
    kDefaultPreDelayFrames = 256
  };  // SetPreDelayTime() will override this initial value
  unsigned last_pre_delay_frames_ = kDefaultPreDelayFrames;

  Vector<std::unique_ptr<AudioFloatArray>> pre_delay_buffers_;
  int pre_delay_read_index_ = 0;
  int pre_delay_write_index_ = kDefaultPreDelayFrames;

  float db_max_attack_compression_diff_;

  // Amount of input change in dB required for 1 dB of output change.
  // This applies to the portion of the curve above db_knee_threshold_ (see
  // below).
  float ratio_;
  float slope_;  // Inverse ratio.

  // The input to output change below the threshold is linear 1:1.
  float linear_threshold_;
  float db_threshold_;

  // db_knee_ is the number of dB above the threshold before we enter the
  // "ratio" portion of the curve.
  // db_knee_threshold_ = db_threshold_ + db_knee_
  // The portion between db_threshold_ and db_knee_threshold_ is the "soft knee"
  // portion of the curve which transitions smoothly from the linear portion to
  // the ratio portion.
  float db_knee_;
  float knee_threshold_;
  float db_knee_threshold_;
  float db_yknee_threshold_;

  // Internal parameter for the knee portion of the curve.
  float knee_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_DYNAMICS_COMPRESSOR_H_
