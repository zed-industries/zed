/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_BACKGROUND_NOISE_H_
#define MODULES_AUDIO_CODING_NETEQ_BACKGROUND_NOISE_H_

#include <string.h>  // size_t

#include <memory>

#include "api/array_view.h"

namespace webrtc {

// Forward declarations.
class AudioMultiVector;
class PostDecodeVad;

// This class handles estimation of background noise parameters.
class BackgroundNoise {
 public:
  // TODO(hlundin): For 48 kHz support, increase kMaxLpcOrder to 10.
  // Will work anyway, but probably sound a little worse.
  static constexpr size_t kMaxLpcOrder = 8;  // 32000 / 8000 + 4.

  explicit BackgroundNoise(size_t num_channels);
  virtual ~BackgroundNoise();

  BackgroundNoise(const BackgroundNoise&) = delete;
  BackgroundNoise& operator=(const BackgroundNoise&) = delete;

  void Reset();

  // Updates the parameter estimates based on the signal currently in the
  // `sync_buffer`.
  // Returns true if the filter parameters are updated.
  bool Update(const AudioMultiVector& sync_buffer);

  // Generates background noise given a random vector and writes the output to
  // `buffer`.
  void GenerateBackgroundNoise(ArrayView<const int16_t> random_vector,
                               size_t channel,
                               int mute_slope,
                               bool too_many_expands,
                               size_t num_noise_samples,
                               int16_t* buffer);

  // Returns `energy_` for `channel`.
  int32_t Energy(size_t channel) const;

  // Sets the value of `mute_factor_` for `channel` to `value`.
  void SetMuteFactor(size_t channel, int16_t value);

  // Returns `mute_factor_` for `channel`.
  int16_t MuteFactor(size_t channel) const;

  // Returns a pointer to `filter_` for `channel`.
  const int16_t* Filter(size_t channel) const;

  // Returns a pointer to `filter_state_` for `channel`.
  const int16_t* FilterState(size_t channel) const;

  // Copies `input` to the filter state. Will not copy more than `kMaxLpcOrder`
  // elements.
  void SetFilterState(size_t channel, ArrayView<const int16_t> input);

  // Returns `scale_` for `channel`.
  int16_t Scale(size_t channel) const;

  // Returns `scale_shift_` for `channel`.
  int16_t ScaleShift(size_t channel) const;

  // Accessors.
  bool initialized() const { return initialized_; }

 private:
  static const int kThresholdIncrement = 229;  // 0.0035 in Q16.
  static const size_t kVecLen = 256;
  static const int kLogVecLen = 8;  // log2(kVecLen).
  static const size_t kResidualLength = 64;
  static const int16_t kLogResidualLength = 6;  // log2(kResidualLength)

  struct ChannelParameters {
    // Constructor.
    ChannelParameters() { Reset(); }

    void Reset() {
      energy = 2500;
      max_energy = 0;
      energy_update_threshold = 500000;
      low_energy_update_threshold = 0;
      memset(filter_state, 0, sizeof(filter_state));
      memset(filter, 0, sizeof(filter));
      filter[0] = 4096;
      mute_factor = 0;
      scale = 20000;
      scale_shift = 24;
    }

    int32_t energy;
    int32_t max_energy;
    int32_t energy_update_threshold;
    int32_t low_energy_update_threshold;
    int16_t filter_state[kMaxLpcOrder];
    int16_t filter[kMaxLpcOrder + 1];
    int16_t mute_factor;
    int16_t scale;
    int16_t scale_shift;
  };

  int32_t CalculateAutoCorrelation(const int16_t* signal,
                                   size_t length,
                                   int32_t* auto_correlation) const;

  // Increments the energy threshold by a factor 1 + `kThresholdIncrement`.
  void IncrementEnergyThreshold(size_t channel, int32_t sample_energy);

  // Updates the filter parameters.
  void SaveParameters(size_t channel,
                      const int16_t* lpc_coefficients,
                      const int16_t* filter_state,
                      int32_t sample_energy,
                      int32_t residual_energy);

  size_t num_channels_;
  std::unique_ptr<ChannelParameters[]> channel_parameters_;
  bool initialized_;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_BACKGROUND_NOISE_H_
