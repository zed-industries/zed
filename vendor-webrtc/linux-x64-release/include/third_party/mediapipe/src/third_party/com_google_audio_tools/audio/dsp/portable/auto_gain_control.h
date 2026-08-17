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

/* Auto gain control (AGC).
 *
 * This library implements an auto gain control (AGC), aka dynamic range control
 * (DRC). This is useful as an audio preprocessing step to automatically
 * normalize the audio volume level toward a smaller dynamic range, making quiet
 * audio louder and conversely loud audio quieter.
 *
 * This implementation uses a simple feed-forward structure. Simplified
 * algorithm pseudocode:
 *
 *   Input: audio waveform x[0], x[1], x[2], ...
 *   Output: normalized audio y[0], y[1], y[2], ...
 *   smoothed_power = 1
 *   for n = 0, 1, 2, ...
 *     power_sample = x[n]^2
 *     smoothed_power += smoother_coeff * (power_sample - smoothed_power)
 *     gain = (power_floor + smoothed_power)^(-agc_strength / 2)
 *     y[n] = x[n] * gain
 *
 * Non-simplified algorithm details: to warm-start the smoothed_power
 * quickly, we set it on the nth sample to
 *   smoothed_power = (1 + x[0]^2 + x[1]^2 + ... + x[n]^2) / (1 + n),
 * then switch to the above algorithm after one time constant.
 *
 * Example use:
 *   AutoGainControlState state;
 *   AutoGainControlInit(&state,
 *                       16000,   // Sample rate in Hz.
 *                       0.25f,   // Time constant in seconds.
 *                       0.5f,    // AGC strength.
 *                       1e-6f);  // Power floor.
 *
 *   float signal[num_samples] = ...
 *   for (i = 0; i < num_samples; ++i) {
 *     const float power_sample = signal[i] * signal[i];
 *     AutoGainControlProcessSample(&state, power_sample);
 *     signal[i] *= AutoGainControlGetGain(&state);
 *   }
 */

#ifndef AUDIO_DSP_PORTABLE_AUTO_GAIN_CONTROL_H_
#define AUDIO_DSP_PORTABLE_AUTO_GAIN_CONTROL_H_

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  float smoother_coeff;
  float exponent;
  float smoothed_power;
  float power_floor;
  int num_warm_up_samples;
  int warm_up_counter;
} AutoGainControlState;

/* Initializes state:
 *  - sample_rate_hz: Input sample rate in Hz.
 *  - time_constant_s: Time constant in seconds for how quickly the AGC adapts.
 *    A value between 0.01 and 3.0 is typical.
 *  - agc_strength: Strength (stiffness) of the AGC. A value between 0 and 1,
 *    where 1 is full normalization (very strong). A value between 0.3 and 0.8
 *    is typical.
 *  - power_floor: A small offset added to the power estimate to avoid
 *    excessive amplification. A value on the order of 1e-6 is typical.
 *
 * Returns 1 on success, 0 on failure.
 */
int AutoGainControlInit(AutoGainControlState* state,
                        float sample_rate_hz,
                        float time_constant_s,
                        float agc_strength,
                        float power_floor);

/* Resets the AGC to initial state. */
void AutoGainControlReset(AutoGainControlState* state);

/* Processes one power sample. This method should be called for every input
 * sample, passing its value squared (to get power):
 *
 *   float power_sample = input[n] * input[n];
 *   AutoGainControlProcessSample(&state, power_sample);
 */
static void AutoGainControlProcessSample(AutoGainControlState* state,
                                         float power_sample);


/* Computes the AGC amplitude gain factor. This method should be called for
 * every output sample, after having called AutoGainControlProcessSample:
 *
 *   float gain = AutoGainControlGetGain(&state);
 *   output[n] = input[n] * gain;
 *
 * NOTE: It is allowed to skip calls to this method (it has no effect on the AGC
 * state). For instance, the output may have lower sample rate than the input.
 */
float AutoGainControlGetGain(const AutoGainControlState* state);

/* Implementation details only below this line. ----------------------------- */

/* (Internal method exposed so that AutoGainControlProcessSample can inline.) */
void AutoGainControlWarmUpProcess(AutoGainControlState* state,
                                  float power_sample);

static void AutoGainControlProcessSample(AutoGainControlState* state,
                                         float power_sample) {
  if (state->warm_up_counter <= state->num_warm_up_samples) {
    AutoGainControlWarmUpProcess(state, power_sample);
    return;
  }

  /* Recursively compute the exponentially-weighted moving average
   *
   *   smoothed_power = smoother_coeff * sum_k (1 - smoother_coeff)^k x[n-k]^2,
   *
   * or in other words applying a one-pole smoothing filter.
   *
   * Note: This update line is equivalent to
   *  smoothed_power = smoother_coeff * power_sample
   *                   - (1 - smoother_coeff) * smoothed_power.
   */
  state->smoothed_power +=
      state->smoother_coeff * (power_sample - state->smoothed_power);
}

#ifdef __cplusplus
}  /* extern "C" */
#endif
#endif /* AUDIO_DSP_PORTABLE_AUTO_GAIN_CONTROL_H_ */

