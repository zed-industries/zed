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

/* Biquad filter (second-order section). */

#ifndef AUDIO_DSP_PORTABLE_BIQUAD_FILTER_H_
#define AUDIO_DSP_PORTABLE_BIQUAD_FILTER_H_

#ifdef __cplusplus
extern "C" {
#endif

/* Filter coefficients in second-order section (biquad) representation,
 *
 *          b0 + b1 z^-1 + b2 z^-2
 *   H(z) = ----------------------.
 *           1 + a1 z^-1 + a2 z^-2
 */
typedef struct {
  float b0;
  float b1;
  float b2;
  float a1;
  float a2;
} BiquadFilterCoeffs;

/* Coefficients for the identity filter, H(z) = 1. */
extern const BiquadFilterCoeffs kBiquadFilterIdentityCoeffs;

typedef struct {
  float z[2];
} BiquadFilterState;

/* Initializes biquad filter state variables to zero. */
static void BiquadFilterInitZero(BiquadFilterState* state) {
  state->z[0] = 0.0f;
  state->z[1] = 0.0f;
}

/* Processes one sample.
 * NOTE: This function is marked `static` [the C analogy for `inline`] to
 * encourage the compiler to inline it.
 */
static float BiquadFilterProcessOneSample(const BiquadFilterCoeffs* coeffs,
                                          BiquadFilterState* state,
                                          float input_sample) {
  const float next_state = input_sample
      - coeffs->a1 * state->z[0]
      - coeffs->a2 * state->z[1];
  const float output_sample = coeffs->b0 * next_state
      + coeffs->b1 * state->z[0]
      + coeffs->b2 * state->z[1];
  state->z[1] = state->z[0];
  state->z[0] = next_state;
  return output_sample;
}

#ifdef __cplusplus
}  /* extern "C" */
#endif
#endif /* AUDIO_DSP_PORTABLE_BIQUAD_FILTER_H_ */
