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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_EQUAL_POWER_PANNER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_EQUAL_POWER_PANNER_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/audio/panner.h"

namespace blink {

// Common type of stereo panner as found in normal audio mixing equipment.

class EqualPowerPanner final : public Panner {
 public:
  explicit EqualPowerPanner(float sample_rate);

  void Pan(double azimuth,
           double elevation,
           const AudioBus* input_bus,
           AudioBus* output_buf,
           uint32_t frames_to_process,
           AudioBus::ChannelInterpretation) override;
  void PanWithSampleAccurateValues(base::span<double> azimuth,
                                   base::span<double> elevation,
                                   const AudioBus* input_bus,
                                   AudioBus* output_bus,
                                   uint32_t frames_to_process,
                                   AudioBus::ChannelInterpretation) override;

  void Reset() override {}

  double TailTime() const override { return 0; }
  double LatencyTime() const override { return 0; }
  bool RequiresTailProcessing() const override { return false; }

 private:
  void CalculateDesiredGain(double& desired_gain_l,
                            double& desired_gain_r,
                            double azimuth,
                            int number_of_input_channels);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_EQUAL_POWER_PANNER_H_
