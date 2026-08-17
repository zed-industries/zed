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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_BIQUAD_PROCESSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_BIQUAD_PROCESSOR_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "third_party/blink/renderer/platform/audio/audio_dsp_kernel.h"
#include "third_party/blink/renderer/platform/audio/audio_dsp_kernel_processor.h"
#include "third_party/blink/renderer/platform/audio/biquad.h"

namespace blink {

// BiquadProcessor is an AudioDSPKernelProcessor which uses Biquad objects to
// implement several common filters.

class BiquadProcessor final : public AudioDSPKernelProcessor {
 public:
  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  enum class FilterType {
    kLowPass = 0,
    kHighPass = 1,
    kBandPass = 2,
    kLowShelf = 3,
    kHighShelf = 4,
    kPeaking = 5,
    kNotch = 6,
    kAllpass = 7,
    kMaxValue = kAllpass,
  };

  BiquadProcessor(float sample_rate,
                  uint32_t number_of_channels,
                  unsigned render_quantum_frames,
                  AudioParamHandler& frequency,
                  AudioParamHandler& q,
                  AudioParamHandler& gain,
                  AudioParamHandler& detune);
  ~BiquadProcessor() override;

  std::unique_ptr<AudioDSPKernel> CreateKernel() override;

  void Initialize() override;
  void Process(const AudioBus* source,
               AudioBus* destination,
               uint32_t frames_to_process) override;
  void ProcessOnlyAudioParams(uint32_t frames_to_process) override;
  void Reset() override;

  // Get the magnitude and phase response of the filter at the given
  // set of frequencies (in Hz). The phase response is in radians.
  void GetFrequencyResponse(int n_frequencies,
                            const float* frequency_hz,
                            float* mag_response,
                            float* phase_response);

  void CheckForDirtyCoefficients();

  bool FilterCoefficientsDirty() const { return filter_coefficients_dirty_; }
  bool HasSampleAccurateValues() const { return has_sample_accurate_values_; }
  bool IsAudioRate() const { return is_audio_rate_; }

  AudioParamHandler& Parameter1() { return *parameter1_; }
  AudioParamHandler& Parameter2() { return *parameter2_; }
  AudioParamHandler& Parameter3() { return *parameter3_; }
  AudioParamHandler& Parameter4() { return *parameter4_; }

  FilterType GetType() const { return type_; }
  void SetType(FilterType);

 private:
  FilterType type_ = FilterType::kLowPass;

  scoped_refptr<AudioParamHandler> parameter1_;
  scoped_refptr<AudioParamHandler> parameter2_;
  scoped_refptr<AudioParamHandler> parameter3_;
  scoped_refptr<AudioParamHandler> parameter4_;

  // so DSP kernels know when to re-compute coefficients
  bool filter_coefficients_dirty_ = true;

  // Set to true if any of the filter parameters are sample-accurate.
  bool has_sample_accurate_values_ = false;

  // Set to true if any of the filter parameters are a-rate.
  bool is_audio_rate_;

  bool has_just_reset_ = true;

  // Cache previous parameter values to allow us to skip recomputing filter
  // coefficients when parameters are not changing
  float previous_parameter1_ = std::numeric_limits<float>::quiet_NaN();
  float previous_parameter2_ = std::numeric_limits<float>::quiet_NaN();
  float previous_parameter3_ = std::numeric_limits<float>::quiet_NaN();
  float previous_parameter4_ = std::numeric_limits<float>::quiet_NaN();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_BIQUAD_PROCESSOR_H_
