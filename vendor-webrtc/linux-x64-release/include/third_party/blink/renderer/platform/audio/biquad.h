/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_BIQUAD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_BIQUAD_H_

#include <sys/types.h>
#include <complex>
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/audio/audio_array.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// A basic biquad (two-zero / two-pole digital filter)
//
// It can be configured to a number of common and very useful filters:
//    lowpass, highpass, shelving, parameteric, notch, allpass, ...

class PLATFORM_EXPORT Biquad final {
  DISALLOW_NEW();

 public:
  explicit Biquad(unsigned render_quantum_frames);
  ~Biquad();

  void Process(const float* source_p,
               float* dest_p,
               uint32_t frames_to_process);

  bool HasSampleAccurateValues() const { return has_sample_accurate_values_; }
  void SetHasSampleAccurateValues(bool is_sample_accurate) {
    has_sample_accurate_values_ = is_sample_accurate;
  }

  // frequency is 0 - 1 normalized, resonance and db_gain are in decibels.
  // Q is a unitless quality factor.
  void SetLowpassParams(int, double frequency, double resonance);
  void SetHighpassParams(int, double frequency, double resonance);
  void SetBandpassParams(int, double frequency, double q);
  void SetLowShelfParams(int, double frequency, double db_gain);
  void SetHighShelfParams(int, double frequency, double db_gain);
  void SetPeakingParams(int, double frequency, double q, double db_gain);
  void SetAllpassParams(int, double frequency, double q);
  void SetNotchParams(int, double frequency, double q);

  // Resets filter state
  void Reset();

  // Compute tail frame based on the filter coefficents at index
  // |coef_index|.  The tail frame is the frame number where the
  // impulse response of the filter falls below a threshold value.
  // The maximum allowed frame value is given by |max_frame|.  This
  // limits how much work is done in computing the frame numer.
  double TailFrame(int coef_index, double max_frame);

  // Filter response at a set of n frequencies. The magnitude and
  // phase response are returned in magResponse and phaseResponse.
  // The phase response is in radians.
  void GetFrequencyResponse(int n_frequencies,
                            const float* frequency,
                            float* mag_response,
                            float* phase_response);

 private:
  void SetNormalizedCoefficients(int,
                                 double b0,
                                 double b1,
                                 double b2,
                                 double a0,
                                 double a1,
                                 double a2);

  // If true, the filter coefficients are (possibly) time-varying due to a
  // timeline automation on at least one filter parameter.
  bool has_sample_accurate_values_;

  // Filter coefficients. The filter is defined as
  //
  // y[n] + m_a1*y[n-1] + m_a2*y[n-2] = m_b0*x[n] + m_b1*x[n-1] + m_b2*x[n-2].
  AudioDoubleArray b0_;
  AudioDoubleArray b1_;
  AudioDoubleArray b2_;
  AudioDoubleArray a1_;
  AudioDoubleArray a2_;

#if BUILDFLAG(IS_MAC)
  void ProcessFast(const float* source_p,
                   float* dest_p,
                   uint32_t frames_to_process);
  void ProcessSliceFast(double* source_p,
                        double* dest_p,
                        double* coefficients_p,
                        uint32_t frames_to_process);

  AudioDoubleArray input_buffer_;
  AudioDoubleArray output_buffer_;

#endif
  // Filter memory
  double x1_;  // input delayed by 1 sample
  double x2_;  // input delayed by 2 samples
  double y1_;  // output delayed by 1 sample
  double y2_;  // output delayed by 2 samples
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_BIQUAD_H_
