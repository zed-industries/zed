/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef AUDIO_DSP_ATTACK_RELEASE_ENVELOPE_H_
#define AUDIO_DSP_ATTACK_RELEASE_ENVELOPE_H_

#include "audio/linear_filters/biquad_filter.h"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

struct AttackReleaseEnvelopeParams {
  AttackReleaseEnvelopeParams()
    : attack_s(0.05f),
      release_s(0.2f),
      interpolation_rate_hz(200.0f) {}

  AttackReleaseEnvelopeParams(float attack_seconds, float release_seconds)
    : attack_s(attack_seconds),
      release_s(release_seconds),
      interpolation_rate_hz(200.0f) {}
  // Time constant when input <= output.
  float attack_s;
  // Time constant when input > output.
  float release_s;
  // The rate at which coefficents can change on parameter change.
  float interpolation_rate_hz;
};

// Computes an approximate envelope of a rectified signal with an asymmetrical
// time constant.
  // attack_s and release_s are time constants for the filter in seconds. When
  // input > output, the attack coefficient is used. When input < output, the
  // release coefficient is used.
// TODO: Add multichannel support.
// TODO: Add an Init function and make this class a little less
// bare-bones.
class AttackReleaseEnvelope {
 public:
  AttackReleaseEnvelope(const AttackReleaseEnvelopeParams& params,
                        float sample_rate_hz);

  AttackReleaseEnvelope(float attack_s, float release_s, float sample_rate_hz)
    : AttackReleaseEnvelope(AttackReleaseEnvelopeParams(attack_s, release_s),
                            sample_rate_hz) {}

  // Note that the rectified signal is not guaranteed to be positive after a
  // recent change in filter coefficients.
  void SetAttackTimeSeconds(float attack_s);
  void SetReleaseTimeSeconds(float release_s);

  // Note that this leaves the time constants set to the last values passed
  // to SetAttackTimeSeconds (or the constructor).
  void Reset() {
    envelope_ = 0;
    attack_param_smoother_.SetSteadyStateCondition(attack_);
    release_param_smoother_.SetSteadyStateCondition(release_);
  }
  // Process a single sample.
  float Output(float input);

 private:
  // State variable.
  float envelope_;

  float attack_;
  float release_;
  float sample_rate_hz_;

  linear_filters::BiquadFilter<float> attack_param_smoother_;
  linear_filters::BiquadFilter<float> release_param_smoother_;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_ATTACK_RELEASE_ENVELOPE_H_
