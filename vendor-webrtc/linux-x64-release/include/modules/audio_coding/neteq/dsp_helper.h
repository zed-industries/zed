/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_DSP_HELPER_H_
#define MODULES_AUDIO_CODING_NETEQ_DSP_HELPER_H_

#include <stdint.h>
#include <string.h>

#include "modules/audio_coding/neteq/audio_multi_vector.h"
#include "modules/audio_coding/neteq/audio_vector.h"

namespace webrtc {

// This class contains various signal processing functions, all implemented as
// static methods.
class DspHelper {
 public:
  // Filter coefficients used when downsampling from the indicated sample rates
  // (8, 16, 32, 48 kHz) to 4 kHz. Coefficients are in Q12.
  static const int16_t kDownsample8kHzTbl[3];
  static const int16_t kDownsample16kHzTbl[5];
  static const int16_t kDownsample32kHzTbl[7];
  static const int16_t kDownsample48kHzTbl[7];

  // Constants used to mute and unmute over 5 samples. The coefficients are
  // in Q15.
  static const int kMuteFactorStart8kHz = 27307;
  static const int kMuteFactorIncrement8kHz = -5461;
  static const int kUnmuteFactorStart8kHz = 5461;
  static const int kUnmuteFactorIncrement8kHz = 5461;
  static const int kMuteFactorStart16kHz = 29789;
  static const int kMuteFactorIncrement16kHz = -2979;
  static const int kUnmuteFactorStart16kHz = 2979;
  static const int kUnmuteFactorIncrement16kHz = 2979;
  static const int kMuteFactorStart32kHz = 31208;
  static const int kMuteFactorIncrement32kHz = -1560;
  static const int kUnmuteFactorStart32kHz = 1560;
  static const int kUnmuteFactorIncrement32kHz = 1560;
  static const int kMuteFactorStart48kHz = 31711;
  static const int kMuteFactorIncrement48kHz = -1057;
  static const int kUnmuteFactorStart48kHz = 1057;
  static const int kUnmuteFactorIncrement48kHz = 1057;

  // Multiplies the signal with a gradually changing factor.
  // The first sample is multiplied with `factor` (in Q14). For each sample,
  // `factor` is increased (additive) by the `increment` (in Q20), which can
  // be negative. Returns the scale factor after the last increment.
  static int RampSignal(const int16_t* input,
                        size_t length,
                        int factor,
                        int increment,
                        int16_t* output);

  // Same as above, but with the samples of `signal` being modified in-place.
  static int RampSignal(int16_t* signal,
                        size_t length,
                        int factor,
                        int increment);

  // Same as above, but processes `length` samples from `signal`, starting at
  // `start_index`.
  static int RampSignal(AudioVector* signal,
                        size_t start_index,
                        size_t length,
                        int factor,
                        int increment);

  // Same as above, but for an AudioMultiVector.
  static int RampSignal(AudioMultiVector* signal,
                        size_t start_index,
                        size_t length,
                        int factor,
                        int increment);

  // Peak detection with parabolic fit. Looks for `num_peaks` maxima in `data`,
  // having length `data_length` and sample rate multiplier `fs_mult`. The peak
  // locations and values are written to the arrays `peak_index` and
  // `peak_value`, respectively. Both arrays must hold at least `num_peaks`
  // elements.
  static void PeakDetection(int16_t* data,
                            size_t data_length,
                            size_t num_peaks,
                            int fs_mult,
                            size_t* peak_index,
                            int16_t* peak_value);

  // Estimates the height and location of a maximum. The three values in the
  // array `signal_points` are used as basis for a parabolic fit, which is then
  // used to find the maximum in an interpolated signal. The `signal_points` are
  // assumed to be from a 4 kHz signal, while the maximum, written to
  // `peak_index` and `peak_value` is given in the full sample rate, as
  // indicated by the sample rate multiplier `fs_mult`.
  static void ParabolicFit(int16_t* signal_points,
                           int fs_mult,
                           size_t* peak_index,
                           int16_t* peak_value);

  // Calculates the sum-abs-diff for `signal` when compared to a displaced
  // version of itself. Returns the displacement lag that results in the minimum
  // distortion. The resulting distortion is written to `distortion_value`.
  // The values of `min_lag` and `max_lag` are boundaries for the search.
  static size_t MinDistortion(const int16_t* signal,
                              size_t min_lag,
                              size_t max_lag,
                              size_t length,
                              int32_t* distortion_value);

  // Mixes `length` samples from `input1` and `input2` together and writes the
  // result to `output`. The gain for `input1` starts at `mix_factor` (Q14) and
  // is decreased by `factor_decrement` (Q14) for each sample. The gain for
  // `input2` is the complement 16384 - mix_factor.
  static void CrossFade(const int16_t* input1,
                        const int16_t* input2,
                        size_t length,
                        int16_t* mix_factor,
                        int16_t factor_decrement,
                        int16_t* output);

  // Scales `input` with an increasing gain. Applies `factor` (Q14) to the first
  // sample and increases the gain by `increment` (Q20) for each sample. The
  // result is written to `output`. `length` samples are processed.
  static void UnmuteSignal(const int16_t* input,
                           size_t length,
                           int16_t* factor,
                           int increment,
                           int16_t* output);

  // Starts at unity gain and gradually fades out `signal`. For each sample,
  // the gain is reduced by `mute_slope` (Q14). `length` samples are processed.
  static void MuteSignal(int16_t* signal, int mute_slope, size_t length);

  // Downsamples `input` from `sample_rate_hz` to 4 kHz sample rate. The input
  // has `input_length` samples, and the method will write `output_length`
  // samples to `output`. Compensates for the phase delay of the downsampling
  // filters if `compensate_delay` is true. Returns -1 if the input is too short
  // to produce `output_length` samples, otherwise 0.
  static int DownsampleTo4kHz(const int16_t* input,
                              size_t input_length,
                              size_t output_length,
                              int input_rate_hz,
                              bool compensate_delay,
                              int16_t* output);

  DspHelper(const DspHelper&) = delete;
  DspHelper& operator=(const DspHelper&) = delete;

 private:
  // Table of constants used in method DspHelper::ParabolicFit().
  static const int16_t kParabolaCoefficients[17][3];
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_DSP_HELPER_H_
