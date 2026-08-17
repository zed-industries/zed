/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_STATIONARITY_ESTIMATOR_H_
#define MODULES_AUDIO_PROCESSING_AEC3_STATIONARITY_ESTIMATOR_H_

#include <stddef.h>

#include <array>
#include <atomic>
#include <memory>

#include "api/array_view.h"
#include "modules/audio_processing/aec3/aec3_common.h"  // kFftLengthBy2Plus1...
#include "modules/audio_processing/aec3/reverb_model.h"
#include "rtc_base/checks.h"

namespace webrtc {

class ApmDataDumper;
struct SpectrumBuffer;

class StationarityEstimator {
 public:
  StationarityEstimator();
  ~StationarityEstimator();

  // Reset the stationarity estimator.
  void Reset();

  // Update just the noise estimator. Usefull until the delay is known
  void UpdateNoiseEstimator(
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> spectrum);

  // Update the flag indicating whether this current frame is stationary. For
  // getting a more robust estimation, it looks at future and/or past frames.
  void UpdateStationarityFlags(
      const SpectrumBuffer& spectrum_buffer,
      ArrayView<const float> render_reverb_contribution_spectrum,
      int idx_current,
      int num_lookahead);

  // Returns true if the current band is stationary.
  bool IsBandStationary(size_t band) const {
    return stationarity_flags_[band] && (hangovers_[band] == 0);
  }

  // Returns true if the current block is estimated as stationary.
  bool IsBlockStationary() const;

 private:
  static constexpr int kWindowLength = 13;
  // Returns the power of the stationary noise spectrum at a band.
  float GetStationarityPowerBand(size_t k) const { return noise_.Power(k); }

  // Get an estimation of the stationarity for the current band by looking
  // at the past/present/future available data.
  bool EstimateBandStationarity(const SpectrumBuffer& spectrum_buffer,
                                ArrayView<const float> average_reverb,
                                const std::array<int, kWindowLength>& indexes,
                                size_t band) const;

  // True if all bands at the current point are stationary.
  bool AreAllBandsStationary();

  // Update the hangover depending on the stationary status of the current
  // frame.
  void UpdateHangover();

  // Smooth the stationarity detection by looking at neighbouring frequency
  // bands.
  void SmoothStationaryPerFreq();

  class NoiseSpectrum {
   public:
    NoiseSpectrum();
    ~NoiseSpectrum();

    // Reset the noise power spectrum estimate state.
    void Reset();

    // Update the noise power spectrum with a new frame.
    void Update(
        ArrayView<const std::array<float, kFftLengthBy2Plus1>> spectrum);

    // Get the noise estimation power spectrum.
    ArrayView<const float> Spectrum() const { return noise_spectrum_; }

    // Get the noise power spectrum at a certain band.
    float Power(size_t band) const {
      RTC_DCHECK_LT(band, noise_spectrum_.size());
      return noise_spectrum_[band];
    }

   private:
    // Get the update coefficient to be used for the current frame.
    float GetAlpha() const;

    // Update the noise power spectrum at a certain band with a new frame.
    float UpdateBandBySmoothing(float power_band,
                                float power_band_noise,
                                float alpha) const;
    std::array<float, kFftLengthBy2Plus1> noise_spectrum_;
    size_t block_counter_;
  };

  static std::atomic<int> instance_count_;
  std::unique_ptr<ApmDataDumper> data_dumper_;
  NoiseSpectrum noise_;
  std::array<int, kFftLengthBy2Plus1> hangovers_;
  std::array<bool, kFftLengthBy2Plus1> stationarity_flags_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_STATIONARITY_ESTIMATOR_H_
