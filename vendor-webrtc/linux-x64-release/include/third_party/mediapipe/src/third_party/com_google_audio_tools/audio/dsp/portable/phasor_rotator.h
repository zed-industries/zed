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

/* Sine wave generator using phasor-rotator algorithm.
 *
 * This library implements an efficient oscillator using the phasor-rotator
 * algorithm: define the rotator r = exp(i theta) and recursively compute
 *
 *   p[0] = 1,
 *   p[n] = r * p[n - 1].
 *
 * Then the real and imaginary parts of p[n] are cosine and sine waves with
 * theta radians per sample:
 *
 *   Re{p[n]} = cos(n * theta),  Im{p[n]} = sin(n * theta).
 */

#ifndef AUDIO_DSP_PORTABLE_PHASOR_ROTATOR_H_
#define AUDIO_DSP_PORTABLE_PHASOR_ROTATOR_H_

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  float rotator[2];
  float phasor[2];
  int phasor_renormalize_counter;
} PhasorRotator;

/* Initializes the phasor-rotator with specified frequency. */
void PhasorRotatorInit(
    PhasorRotator* oscillator, float frequency_hz, float sample_rate_hz);

/* Cosine of the current phase. */
static float PhasorRotatorCos(const PhasorRotator* oscillator) {
  return oscillator->phasor[0];
}

/* Sine of the current phase. */
static float PhasorRotatorSin(const PhasorRotator* oscillator) {
  return oscillator->phasor[1];
}

/* Advances the phasor-rotator to the next sample. */
static void PhasorRotatorNext(PhasorRotator* oscillator);

/* Renormalizes phasor to unit amplitude. */
void PhasorRotatorRenormalize(PhasorRotator* oscillator);

/* Implementation details only below this line. ----------------------------- */

static void PhasorRotatorNext(PhasorRotator* oscillator) {
  /* Complex multiplication `phasor *= rotator`. */
  float next_state = oscillator->rotator[0] * oscillator->phasor[0]
      - oscillator->rotator[1] * oscillator->phasor[1];
  oscillator->phasor[1] = oscillator->rotator[1] * oscillator->phasor[0]
      + oscillator->rotator[0] * oscillator->phasor[1];
  oscillator->phasor[0] = next_state;

  /* Because the algorithm is recursive, round-off error is accumulated. The
   * accumulated error does not amplify or decay, it grows as a random walk at a
   * constant rate. This causes the phasor to slowly drift away from the exact
   * value. To correct for error in amplitude, we occasionally renormalize the
   * phasor to unit magnitude.
   */
  ++oscillator->phasor_renormalize_counter;
  if (oscillator->phasor_renormalize_counter >= 1000) {
    PhasorRotatorRenormalize(oscillator);
  }
}

#ifdef __cplusplus
}  /* extern "C" */
#endif
#endif /* AUDIO_DSP_PORTABLE_PHASOR_ROTATOR_H_ */
