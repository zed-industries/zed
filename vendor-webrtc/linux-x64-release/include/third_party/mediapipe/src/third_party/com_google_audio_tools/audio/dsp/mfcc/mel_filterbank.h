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


#include "audio/dsp/porting.h"  // auto-added.


#ifndef AUDIO_DSP_MFCC_MEL_FILTERBANK_H_
#define AUDIO_DSP_MFCC_MEL_FILTERBANK_H_

#include <vector>


namespace audio_dsp {

class MelFilterbank {
 public:
  MelFilterbank();
  bool Initialize(int fft_length,  // Number of unique FFT bins fftsize/2+1.
                  double sample_rate, int mel_channel_count,
                  double lower_frequency_limit, double upper_frequency_limit);

  // Takes a squared-magnitude spectrogram slice as input, computes a
  // triangular-mel-weighted linear-magnitude filterbank, and places the result
  // in mel.
  void Compute(const std::vector<double>& squared_magnitude_fft,
               std::vector<double>* mel) const;

  // Takes a triangular-mel-weighted linear-magnitude filterbank and estimates
  // the squared-magnitude spectrogram slice that corresponds to it. This is
  // merely an estimate, so EstimateInverse() followed by Compute() will yield a
  // good approximation to the original mel filterbank, but the sequence of
  // operations will not be a perfect roundtrip.
  void EstimateInverse(const std::vector<double>& mel,
                       std::vector<double>* squared_magnitude_fft) const;

 private:
  double FreqToMel(double freq) const;
  bool initialized_;
  int num_mel_channels_;
  double sample_rate_;
  int fft_length_;

  // Each FFT bin b contributes to two triangular mel channels, with
  // proportion weights_[b] going into mel channel band_mapper_[b], and
  // proportion (1 - weights_[b]) going into channel band_mapper_[b] + 1.
  // Thus, weights_ contains the weighting applied to each FFT bin for the
  // upper-half of the triangular band.
  std::vector<double> weights_;  // Right-side weight for this fft  bin.

  // FFT bin i contributes to the upper side of mel channel band_mapper_[i]
  std::vector<int> band_mapper_;

  // Holds the sum of all weights for a specific Mel channel. This includes the
  // weights on both the left and right sides of the triangle.
  std::vector<double> channel_weights_sum_;

  int start_index_;  // Lowest FFT bin used to calculate mel spectrum.
  int end_index_;  // Highest FFT bin used to calculate mel spectrum.

  MelFilterbank(const MelFilterbank&) = delete;
  MelFilterbank& operator=(const MelFilterbank&) = delete;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_MFCC_MEL_FILTERBANK_H_
